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

    <ViewerPlayerControl 
          :allow_recording="true"
          :allow_playing="true"
          :allow_rewind="false"
          :controls="true"
          :is_recording="recording"
          :is_playing="solving"
          :video_current_time="time"
          @on_record="record()"
          @on_play="start()"
          @on_stop="stop()"
          @error="on_error($event);"
        />

    <!-- Feedback -->
    <div class="absolute top-3 right-3 text-gray-400" id="player-feedback" v-if="time && iteration">
      {{ time.toFixed(1) }}s [Iteration {{ iteration }}, fps {{ fps }}]
    </div>

    <!-- fps Calculator -->
    <CalculateFps class="hidden"
         @fps="fps = $event"
         ref="calculate_fps" />

    <!-- Handle recording and downloading -->
    <MediaRecorder
         @error="on_error($event.msg, $event.e);"
         @on_processing="mediarecorder_on_processing($event)"
         ref="media_recorder" />

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
      this.#worker.postMessage({ make: this.#configuration, use_simulation_format: this.use_simulation_format() });
      this.#initialized = true;
    }
    this.#worker.postMessage({ start: time_step });
  }

  use_simulation_format() {
    let s = new URL(window.location).searchParams;
    let r = s.get("format");
    if (r === null) {r = "array_buffer";}
    return r;
  }

  stop() {
    this.#worker.postMessage({ stop: true });
  }
}

import CalculateFps from "@/shared/components/CalculateFps";
import ViewerPlayerControl from "@/shared/components/ViewerPlayerControl";
import MediaRecorder from "@/shared/components/MediaRecorder";

// import init, { shared_memory, viewer_start, viewer_draw, viewer_element_create } from "@/pkg/index.js";

export default {
  name: "Viewer",
  components: {
    ViewerPlayerControl,
    CalculateFps,
    MediaRecorder,
  },
  data: function () {
    return {
      viewer_id: "viewer",
      canvas_id: "viewer-canvas",
      solver: null,
      wasm: null,
      viewer: null,
      iteration: null,
      time: null,
      fps: null,
      processing: null,
      recording: false,
    };
  },

  computed: {
    solving: function () {
      return this.$store.state.solver.solving;
    },
  },

  mounted: function () {
    this.$store.dispatch("solver/initialize");

    import("@/pkg")
    .then((w) => { 
       this.wasm = w;
       this.viewer = this.wasm.Viewer.new(this.canvas_id);
       this.viewer.create(JSON.stringify({type: "VortonRender"}));
       // this.viewer.create(JSON.stringify({type: "Demo"}));
    })
    .catch(console.error);

    this.set_canvas_size();
    window.onresize = () => {this.set_canvas_size();};
  },

  beforeDestroy: function() {
    if (this.solver) { this.solver.terminate(); }
  },

  methods: {
    mediarecorder_on_processing: function(evt) {
      if (evt) { 
        this.processing = {'message': 'Processing video recording'}; 
      } else { 
        this.processing = null; 
      }
    },
    set_canvas_size: function() {
      var p = document.getElementById(this.viewer_id);
      var c = this.get_canvas();
      c.width = p.clientWidth;
      c.height = p.clientHeight;
    },

    get_canvas: function () {
      return document.getElementById(this.canvas_id);
    },

/*
    get_viewer_element: function () {
      return document.getElementById(this.viewer_id);
    },
*/

    stop: function () {
      this.$store.commit("solver/stop");
      this.get_solver().stop(this.$store.state.solver.time_step);
      if (this.recording) {
        this.$refs.media_recorder.stop();
        this.recording = false;
      }
    },

    start: function () {
      this.$store.commit("solver/start");
      this.get_solver().start(this.$store.state.solver.time_step);
    },

    record: function () {
      this.$store.commit("solver/start");
      this.get_solver().start(this.$store.state.solver.time_step);
      this.$refs.media_recorder.record(this.get_canvas().captureStream(30));
      this.recording = true;
    },

    /* Solver elements */
    get_solver: function () {
      if (this.solver === null) {
        this.solver = new SolverWebassembly(
          this.$store.getters["configuration/as_configuration"],
          (evt) => { this.on_message(evt); },
          (evt) => { this.on_error("Error in get_solver", evt); }
        );
      }
      return this.solver;
    },

    on_message: function(evt) {
      if (evt.data !== undefined) {
        if (evt.data.on_initialized || evt.data.on_iterated) {
          this.iteration = evt.data.iteration; this.time = evt.data.time;
          this.$refs.calculate_fps.tick();

        } else if (evt.data.on_simulation) {
          if (this.wasm !== null && this.viewer !== null) { 
            let s = this.wasm.Solver.from_json(evt.data.simulation);
            this.viewer.draw(s);
            if (this.recording) { this.$refs.media_recorder.capture(this.get_canvas()); }
          }

        } else if (evt.data.on_simulation_array_buffer) {
          if (this.wasm !== null && this.viewer !== null) { 
            let s = this.wasm.Solver.from_array_buffer(evt.data.simulation);
            this.viewer.draw(s);
            if (this.recording) { this.$refs.media_recorder.capture(this.get_canvas()); }
          }


        } else if (evt.data.on_simulation_shared_array_buffer) {
          if (this.wasm !== null && this.viewer !== null) { 
            let s = this.wasm.Solver.from_shared_array_buffer(evt.data.simulation);
            this.viewer.draw(s);
            if (this.recording) { this.$refs.media_recorder.capture(this.get_canvas()); }
          }

        } else if (evt.data.on_simulation_solver) {
          if (this.wasm !== null && this.viewer !== null) { 
            this.viewer.draw(evt.data.simulation);
            if (this.recording) { this.$refs.media_recorder.capture(this.get_canvas()); }
          }

        } else {
          console.log("Event " + evt.data + " is not handled");
        }
      }
    },

    on_error: function(msg, evt) {
      console.error("Viewer::on_error", msg, evt);
    }
  },
};
</script>
