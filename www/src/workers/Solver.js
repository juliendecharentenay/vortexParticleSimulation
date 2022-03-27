
export class Solver {
  worker;
  wasm;
  simulation;
  interval;
  initialize;

  constructor(worker) {
    this.worker = worker;
    this.simulation = null;
    this.interval = null;
    this.wasm = null;
    this.initialize_on_the_fly = false;
    import("@/pkg")
    .then((w) => {
      this.wasm = w;
      this.worker.postMessage({ on_initialized: true });
    });
  }

  set_parameters(parameters) {
    if (this.wasm === null) {
      throw new Error("Wasm is not initialized in Solver::set_parameters");
    }
    if (this.simulation === null) {
      this.simulation = this.wasm.Simulation.default();
    }
    this.simulation.set_parameters(parameters);
    if (this.interval === null) {
      this.initialize_on_the_fly = false;
      this.simulation.initialize_solution();
      this.send_solution();
    } else {
      this.initialize_on_the_fly = true;
    }
  }

  send_solution() {
    if (this.simulation === null) {
      throw new Error("Solver::send_solution - Simulation does not exist yet.");
    }
    this.worker.postMessage({ solution: this.simulation.solution_to_arraybuffer()});
  }

  start() {
    if (this.simulation === null) {
      throw new Error("Solver::start - Simulation does not exist yet.");
    }
    if (this.interval !== null) {
      throw new Error("Solver::start - Interval already exists. Is a simulation running?");
    }
    var time_step = 1.0/30.0;
    this.interval = setInterval(() => {
      if (this.initialize_on_the_fly) {
        this.simulation.initialize_solution();
        this.initialize_on_the_fly = false;
      }
      this.simulation.step(time_step);
      this.worker.postMessage({ on_step: true, iteration: this.simulation.iteration(), time: this.simulation.time() });
      this.send_solution();
    }, 0);
  }

  stop() {
    if (this.interval === null) {
      throw new Error("Solver::stop - Stopping, but there is no simulation running");
    }
    clearInterval(this.interval); this.interval = null;
  }

}


