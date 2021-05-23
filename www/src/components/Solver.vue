<template>
  <span></span>
</template>
<script>
import WorkerSolverWebassembly from "@/workers/SolverWebassembly.worker.js";

class SolverWebassembly {
  #initialized;
  #configuration;
  #worker;

  constructor(configuration, onmessage, onerror) {
    this.#initialized = false;
    this.#configuration = configuration;
    if (window.Worker) {
      this.#worker = new WorkerSolverWebassembly();
      this.#worker.onmessage = onmessage;
      this.#worker.onerror = onerror;
    } else {
      throw "WebWorkers are not supported by this web browser. Unable to proceed with solving";
    }
  }

  terminate() {
    this.#worker.terminate();
  }

  start(time_step) {
    if (!this.#initialized) {
      this.#worker.postMessage({ make: this.#configuration });
      this.#initialized = true;
    }
    this.#worker.postMessage({ start: time_step });
  }

  stop() {
    this.#worker.postMessage({ stop: true });
  }
}

export default {
  name: "Solver",
  data: function () {
    return {
      solver: null,
      mutation_unsubscribe: null,
    };
  },

  mounted: function () {
    this.mutation_unsubscribe = this.$store.subscribe((mutation) => {
      if (mutation.type === "solver/start") {
        this.get_solver().start(this.$store.state.solver.time_step);
      } else if (mutation.type === "solver/stop") {
        this.get_solver().stop(this.$store.state.solver.time_step);
      }
    });
  },

  beforeDestroy: function () {
    if (this.solver) {
      this.solver.terminate();
    }
    if (this.mutation_unsubscribe) {
      this.mutation_unsubscribe();
    }
  },

  methods: {
    handle_worker_error: function (evt) {
      console.log(
        "Worker returned an error in file " +
          evt.filename +
          ", line " +
          evt.lineno +
          ".\nError: " +
          evt.message
      );
    },
    handle_worker_message: function (evt) {
      if (evt.data instanceof Object) {
        console.log(
          "Received the following event from worker that has not been handled: ",
          evt
        );
      }
    },
    get_solver: function () {
      if (this.solver === null) {
        this.solver = new SolverWebassembly(
          this.$store.getters["configuration/as_configuration"],
          (evt) => {
            this.handle_worker_message(evt);
          },
          (evt) => {
            this.handle_worker_error(evt);
          }
        );
      }
      return this.solver;
    },
  },
};
</script>
