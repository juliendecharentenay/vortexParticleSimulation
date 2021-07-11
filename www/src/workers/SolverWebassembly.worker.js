/*
 * WebWorker to handle the running of the solver in webassembly
 */
{
  // (a) Definition of variables
  let wasm = null;
  let initializing = false;
  let initialized = false;
  let starting = false;
  let interval = null;
  let timeout = null;
  let start_performance = null;

  // (b) Handling function declaration
  let handle_make = (evt) => {
    initializing = true;
    wasm = import("@/pkg");
    wasm.then((w) => {
      w.make_from_configuration(JSON.stringify(evt.data.make));
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
      wasm.then((w) => {
        starting = false;
        interval = setInterval(() => {
          w.step(evt.data.start);
        }, 0);
      });
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
  onmessage = (evt) => {
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
