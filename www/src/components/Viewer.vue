<template>
  <div class="fixed inset-x-0 top-16 bottom-0">
    <div
      class="absolute inset-x-0 top-0 bottom-0 bg-green-900"
      :id="viewer_id"
    >
      <canvas :id="canvas_id" class="absolute inset-x-0 top-0 bottom-0"></canvas>
      <!-- <div class="absolute inset-x-10 top-10 bottom-10 br-gray-400 text-center"> Viewer placeholder </div> -->

    </div>
    <!-- <div class="absolute inset-x-0 bottom-0 h-10 inset-x-0 bg-gray-700"><p>Bottom element</p></div> -->
    <div class="absolute inset-x-0 bottom-3" id="controls">
      <div class="flex">
        <span class="mx-auto relative z-0 inline-flex shadow-sm rounded-md">
          <button
            type="button"
            class="
              relative
              inline-flex
              items-center
              px-4
              py-2
              rounded-md
              border border-gray-300
              bg-white
              text-sm
              font-medium
              text-gray-700
              hover:bg-gray-50
              focus:z-10
              focus:outline-none
              focus:ring-1 focus:ring-indigo-500
              focus:border-indigo-500
            "
            @click="solving ? stop() : start()"
          >
            {{ solving ? "Stop" : "Simulate" }}
          </button>
        </span>
      </div>
    </div>

    <!-- Feedback -->
    <div class="absolute bottom-3 right-3 text-gray-400" id="player-feedback" v-if="time && iteration">
      {{ time.toFixed(1) }}s [Iteration {{ iteration }}]
    </div>
  </div>
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
  name: "Viewer",
  data: function () {
    return {
      viewer_id: "viewer",
      canvas_id: "viewer-canvas",
      solver: null,
      wasm: null,
      iteration: null,
      time: null
    };
  },

  computed: {
    solving: function () {
      return this.$store.state.solver.solving;
    },
  },

  mounted: function () {
    this.$store.dispatch("solver/initialize");

    this.wasm = import("@/pkg");
    this.wasm
    .then((w) => { 
       w.viewer_start(this.canvas_id); 
       // w.viewer_element_create(JSON.stringify({type: "Demo"}));
       w.viewer_element_create(JSON.stringify({type: "VortonRender"}));
    })
    .catch(console.error);

    this.set_canvas_size();
    window.onresize = () => {this.set_canvas_size();};
  },

  beforeDestroy: function() {
    if (this.solver) { this.solver.terminate(); }
  },

  methods: {
    set_canvas_size: function() {
      var p = this.get_viewer_element();
      var c = document.getElementById(this.canvas_id);
      c.width = p.clientWidth;
      c.height = p.clientHeight;
    },

    get_viewer_element: function () {
      return document.getElementById(this.viewer_id);
    },

    stop: function () {
      this.$store.commit("solver/stop");
      this.get_solver().stop(this.$store.state.solver.time_step);
    },

    start: function () {
      this.$store.commit("solver/start");
      this.get_solver().start(this.$store.state.solver.time_step);
    },

    /* Solver elements */
    get_solver: function () {
      if (this.solver === null) {
        this.solver = new SolverWebassembly(
          this.$store.getters["configuration/as_configuration"],
          (evt) => { this.on_message(evt); },
          (evt) => { this.on_error(evt); }
        );
      }
      return this.solver;
    },

    on_message: function(evt) {
      if (evt.data !== undefined) {
        if (evt.data.on_initialized || evt.data.on_iterated) {
          this.iteration = evt.data.iteration; this.time = evt.data.time;
        } else if (evt.data.on_simulation) {
          this.wasm.then((w) => {w.viewer_draw(evt.data.simulation);});
        } else {
          console.log("Event " + evt.data + " is not handled");
        }
      }
    },
    on_error: function(evt) {
      console.error("Viewer::on_error", evt);
    }
  },
};
</script>
