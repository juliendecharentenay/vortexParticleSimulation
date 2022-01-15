/*
 * WebWorker to handle the running of the solver in webassembly
 */
{
  // (a) Definition of variables
  let wasm = null;
  let wasm_solver = null;
  let initializing = false;
  let initialized = false;
  let starting = false;
  let interval = null;
  let timeout = null;
  let start_performance = null;
  let use_simulation_format = 1; // 0 -> json, 1 -> array buffer (default), 2 -> shared array buffer

  let postSimulation = () => {
    switch (use_simulation_format) {
      case 0: // json
        self.postMessage({ on_simulation: true, simulation: wasm_solver.to_json() });
        break;
      case 1: // array buffer
        self.postMessage({ on_simulation_array_buffer: true, simulation: wasm_solver.to_array_buffer() });
        break;
      case 2: // shared array buffer
        self.postMessage({ on_simulation_shared_array_buffer: true, simulation: wasm_solver.to_shared_array_buffer() });
        break;
      default: // Default: array buffer
        self.postMessage({ on_simulation_array_buffer: true, simulation: wasm_solver.to_array_buffer() });
    }
  };

  // (b) Handling function declaration
  let handle_make = (evt) => {
    initializing = true;
    if (evt.data.use_simulation_format.match(/^array_buffer$/i)) {
      use_simulation_format = 1;
    } else if (evt.data.use_simulation_format.match(/^json$/i)) {
      use_simulation_format = 0;
    } else if (evt.data.use_simulation_format.match(/^shared_array_buffer$/i)) {
      use_simulation_format = 2;
    } else {
      use_simulation_format = 1;
    }
    import("@/pkg")
    .then((w) => {
      wasm = w;
      wasm_solver = wasm.Solver.from_configuration(JSON.stringify(evt.data.make));
      self.postMessage({ on_initialized: true, iteration: wasm_solver.iteration(), time: wasm_solver.time() });
      postSimulation();
      initializing = false;
      initialized = true;
    });
  };

  let handle_start = (evt) => {
    if (initializing) {
      timeout = setTimeout(() => {
        timeout = null;
        handle_start(evt);
      }, 500);
    } else if (initialized) {
      starting = true;
      start_performance = performance.now();
      starting = false;
      interval = setInterval(() => {
        wasm_solver.step(evt.data.start);
        self.postMessage({ on_initialized: true, iteration: wasm_solver.iteration(), time: wasm_solver.time() });
        postSimulation();
      }, 0);
    } else {
      throw "Start is called before the solver is initialized";
    }
  };

  let handle_stop = (evt) => {
    if (starting) {
      setTimeout(() => {
        handle_stop(evt);
      }, 500);
    }
    if (interval) {
      clearInterval(interval);
      interval = null;
      console.log("Analysis runtime: " + (performance.now() - start_performance) + "ms");
    } else if (timeout) {
      clearTimeout(timeout);
      timeout = null;
    } else {
      throw "Stop is called before the solver is initialized";
    }
  };

  // (c) Message handling
  self.onmessage = (evt) => {
    if (evt.data instanceof Object) {
      // console.log("SolverWebassembly: Received event: ", evt.data);
      if (evt.data.make) {
        handle_make(evt);
      } else if (evt.data.start) {
        handle_start(evt);
      } else if (evt.data.stop) {
        handle_stop(evt);
      } else {
        throw "Event is not recognised " + JSON.stringify(evt.data);
      }
    }
  };
}
