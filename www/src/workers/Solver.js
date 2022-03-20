
export class Solver {
  worker;
  wasm;
  simulation;
  interval;

  constructor(worker) {
    this.worker = worker;
    this.simulation = null;
    this.interval = null;
    this.wasm = null;
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
  }

  initialize() {
    if (this.simulation === null) {
      throw new Error("Solver::initialize - Simulation does not exist yet.");
    }
    this.simulation.initialize_solution();
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


