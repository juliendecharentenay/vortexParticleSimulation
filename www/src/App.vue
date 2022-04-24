<template>
  <div class="absolute inset-0 bg-white">
    <div class="absolute inset-0 bg-green-900" id="app-viewer">
      <ViewerRender ref="viewerrender" 
        @on_error="on_error($event.message, $event.error)"
        @mousedown   ="simulation_f((s) => {s.on_mouse_down($event);})"
        @mousemove   ="simulation_f((s) => {s.on_mouse_move($event);})"
        @mouseup     ="simulation_f((s) => {s.on_mouse_up($event);})"
        @wheel       ="simulation_f((s) => {s.on_wheel($event);})"
        @touchstart  ="simulation_f((s) => {s.on_touch_event($event);})"
        @touchend    ="simulation_f((s) => {s.on_touch_event($event);})"
        @touchcancel ="simulation_f((s) => {s.on_touch_event($event);})"
        @touchmove   ="simulation_f((s) => {s.on_touch_event($event);})"
        />
    </div>

    <div class="absolute top-1 left-1">
      <ArrowsExpandIcon class="w-10 h-10" 
          :class="{'text-black': full_screen, 'text-neutral-400': ! full_screen}"
          @click="toggle_full_screen" />
    </div>

    <div class="absolute top-1 left-12">
      <p>{{ message }}</p>
    </div>

    <div class="absolute top-2 right-4 grid grid-cols-1 justify-items-end gap-2">
      <ControlStyleSelect v-model="control_style" />
      <ToggleWithLabelRight v-model="record" :disabled="simulating">{{ record ? "Record" : "View" }}</ToggleWithLabelRight>
      <div v-if="worker !== null">
        <RecordIcon @click="start"
                    class="h-10 w-10" v-if="record && ! simulating" />
        <PlayIcon @click="start" 
                  class="h-10 w-10" v-else-if="! simulating" />
        <RecordStopIcon @click="stop"
                  class="h-10 w-10" v-else-if="record" />
        <StopIcon @click="stop"
                  class="h-10 w-10" v-else />

      </div>
      <AdjustmentsIcon @click="openSimulationParameters" class="h-10 w-10" />
    </div>

    <div v-if="control_style === 'rc-heli' || control_style === 'rc-plane'">
      <JoyConAnimation
        @active="on_controller_left_active"
        @on_change="on_controller_left_change"
        @on_error="on_error($event.msg, $event.e)"
        >
        <div class="absolute bottom-14 left-14 sm:bottom-18 sm:left-18 md:left-24 lg:left-30">
          <JoyCon class="absolute left-0 bottom-0 w-20 h-20 opacity-100" 
                  :class="{'opacity-50': controller_left}"
            />
        </div>
      </JoyConAnimation>
    </div>

    <div v-if="control_style === 'rc-heli' || control_style === 'rc-plane'">
      <JoyConAnimation 
        @active="on_controller_right_active"
        @on_change="on_controller_right_change"
        @on_error="on_error($event.msg, $event.e)"
        >
        <div class="absolute bottom-20 right-14 sm:bottom-22 sm:right-18 md:right-24 lg:right-30">
          <JoyConYawTilt class="absolute right-0 bottom-0 w-20 h-20 opacity-100"
                         :class="{'opacity-50': controller_right}"
           />
        </div>
      </JoyConAnimation>
    </div>

    <div v-if="error !== null"
         class="absolute bg-black/70 inset-0">
      <ErrorAlert
        :message="error_message"
        :error="error"
        @on_close="clear_error()"
        class="max-w-4xl p-4 mx-auto top-1/3 " />
    </div>
    <div v-else-if="loading !== null"
         class="absolute bg-black/70 inset-0">
      <LoadingAlert
        :message="loading"
        @on_error="on_error($event.message, $event.error)"
        class="max-w-4xl p-4 mx-auto top-1/3 " />
    </div>
    <div v-else-if="simulation_parameters !== null"
         class="absolute bg-black/70 inset-0">
      <SimulationParametersAdjust
        v-model="simulation_parameters"
        @on_error="on_error($event.message, $event.error)"
        @on_update="updateSimulationParameters"
        @on_cancel="closeSimulationParameters"
        class="absolute inset-4 max-w-4xl mx-auto" />
    </div>

  </div>
  <MediaRecorder ref="media_recorder" 
                 @on_processing="loading = ($event ? 'Processing recording' : null);"
                 @error="on_error($event.msg, $event.e)" 
                 />
</template>

<script>
import WorkerSolver from '@/workers/Solver.worker.js';

import ErrorAlert from '@/components/ErrorAlert';
import JoyCon from '@/components/JoyCon';
import JoyConYawTilt from '@/components/JoyConYawTilt';
import JoyConAnimation from '@/components/JoyConAnimation';
import LoadingAlert from '@/components/LoadingAlert';
import ViewerRender from '@/components/ViewerRender';
import ToggleWithLabelRight from '@/components/ToggleWithLabelRight';
import ControlStyleSelect from '@/components/ControlStyleSelect';
import SimulationParametersAdjust from '@/components/SimulationParametersAdjust';

import MediaRecorder from "@/shared/components/MediaRecorder";

import RecordIcon from '@/components/RecordIcon';
import PlayIcon from "@/components/PlayIcon";
import StopIcon from "@/components/StopIcon";
import RecordStopIcon from "@/components/RecordStopIcon";

import { ArrowsExpandIcon, AdjustmentsIcon } from "@heroicons/vue/outline";

export default {
  name: 'App',

  data: function() {
    return {
      active: true,
      loading: null,
      error: null,
      error_message: null,
      simulation: null,
      simulation_parameters: null,
      worker: null,
      message: null,
      record: false,
      simulating: false,
      c_control_style: "classic",
      controller_right: false,
      controller_left: false,
      controller_signal: {
        time_stamp: null,
        dtms: null,
        yaw: 0.0,
        pitch: 0.0,
        roll: 0.0,
        forward: 0.0,
        lift: 0.0,
        weight: 0.0,

      },
      full_screen: false,
    };
  },

  components: {
    ErrorAlert,
    JoyCon,
    JoyConYawTilt,
    JoyConAnimation,
    LoadingAlert,
    ViewerRender,
    ToggleWithLabelRight,
    RecordIcon,
    RecordStopIcon,
    PlayIcon,
    StopIcon,
    ControlStyleSelect,
    ArrowsExpandIcon,
    AdjustmentsIcon,
    MediaRecorder,
    SimulationParametersAdjust,
  },

  mounted: function() {
    try {
      window.onresize = () => {this.on_resize();};
      this.on_resize();
      this.init_wasm();
      this.loop();

    } catch (e) {
      this.on_error("Error in mounted", e);
    }
  },

  beforeUnmount: function() {
    try {
      this.active = false;
    } catch (e) {
      this.on_error("Error in beforeUnmount", e);
    }
  },

  computed: {
    control_style: {
      get: function() { return this.c_control_style; },
      set: function(v) {
        this.controller_signal = {
            time_stamp: null,
            dtms: null,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            forward: 0.0, 
            lift: 0.0,
            weight: 0.0
          };
        switch (v) {
          case "classic":
          case "rc-plane":
            break;
          case "rc-heli":
            this.controller_signal = {...this.controller_signal, lift: 1.0, weight: 1.0 };
            break;
        }
        this.c_control_style = v;
      }
    }
  },

  methods: {
    openSimulationParameters: function() {
      try {
        if (this.simulation !== null) {
          this.simulation_parameters = JSON.parse(this.simulation.get_parameters());
        } else {
          throw new Error("Simulation is not available in updateSimulationParameters");
        }
      } catch (e) {
        this.on_error("Error in App::openSimulationParameters", e);
      }
    },

    updateSimulationParameters: function() {
      try {
        if (this.simulation !== null) {
          this.simulation.set_parameters(JSON.stringify(this.simulation_parameters));
          this.make();
          this.log("");
          this.simulation_parameters = null;
        } else {
          throw new Error("Simulation is not available in updateSimulationParameters");
        }
      } catch (e) {
        this.on_error("Error in App::updateSimulationParameters", e);
      }
    },

    closeSimulationParameters: function() {
      try {
        this.simulation_parameters = null;
      } catch (e) {
        this.on_error("Error in App::closeSimulationParameters", e);
      }
    },

    toggle_full_screen: function() {
      try {
        if (! this.full_screen) {
          var e = document.documentElement;
          if (e.requestFullScreen) {
            e.requestFullScreen();
          } else if (e.webkitRequestFullscreen) { /* Safari */
            e.webkitRequestFullscreen();
          } else if (e.msRequestFullscreen) { /* IE11 */
            e.msRequestFullscreen();
          }
        } else {
          if (document.exitFullscreen) {
            document.exitFullscreen();
          } else if (document.webkitExitFullscreen) { /* Safari */
            document.webkitExitFullscreen();
          } else if (document.msExitFullscreen) { /* IE11 */
            document.msExitFullscreen();
          }
        }
        this.full_screen = ! this.full_screen;
      } catch (e) {
        this.on_error("Error in App::toggle_full_screen", e);
      }
    },

    log: function(msg) {
      this.message = msg;
    },

    on_controller_left_active: function(evt) {
      this.controller_left = evt;
      if (! this.controller_left) { this.on_controller_left_change({result: [0.0, 0.0]}); }
    },

    on_controller_left_change: function(evt) {
      try {
        let res = (evt.result === null ? [0.0, 0.0] : evt.result);
        switch (this.control_style) {
          case "rc-plane":
            this.controller_signal = {...this.controller_signal, yaw: res[0], forward: res[1]};
            break;
          case "rc-heli":
            this.controller_signal = {...this.controller_signal, yaw: res[0], pitch: res[1]};
            break;
        }
      } catch (e) {
        this.on_error("Error in App::on_controller_left_change", e);
      }
    },

    on_controller_right_active: function(evt) {
      this.controller_right = evt;
      if (! this.controller_right) { this.on_controller_right_change({result: [0.0, 0.0]}); }
    },

    on_controller_right_change: function(evt) {
      try {
        let res = (evt.result === null ? [0.0, 0.0] : evt.result);
        switch (this.control_style) {
          case "rc-plane":
            this.controller_signal = {...this.controller_signal, pitch: res[1], roll: res[0]};
            break;
          case "rc-heli":
            this.controller_signal = {...this.controller_signal, roll: res[0], lift: res[1]};
            break;
        }
      } catch (e) {
        this.on_error("Error in App::on_controller_right_change", e);
      }
    },

    make: function() {
      try {
        if (this.worker !== null) {
          if (this.simulation !== null) {
            this.worker.postMessage({make: this.simulation.get_parameters()});
          } else {
            throw "Simulation is not initialized when worker is initialized";
          }
        }
      } catch(e) {
        this.on_error("Error in App::make", e);
      }
    },

    start: function() {
      try {
        if (this.worker !== null) {
          this.simulating = true;
          this.worker.postMessage({start: true});
          if (this.record) { this.$refs.media_recorder.record(this.$refs.viewerrender.get_canvas().captureStream(30)); }
        }
      } catch(e) {
        this.on_error("Error in App::start", e);
      }
    },

    stop: function() {
      try {
        if (this.record) { this.$refs.media_recorder.stop(); }
        this.simulating = false;
        if (this.worker !== null) {
          this.worker.postMessage({stop: true});
        }
      } catch(e) {
        this.on_error("Error in App::stop", e);
      }
    },

    init_wasm: function() {
      try {
        this.loading = "Loading WebAssembly module";
        import("@/pkg")
        .then((wasm) => {
          this.simulation = wasm.Simulation.default();
          this.simulation.initialize_viewer(this.$refs.viewerrender.canvas_id);
          this.simulation.create_view(JSON.stringify({type: "VortonRender"}));
          this.worker_init();
          this.loading = null;
          this.simulation.create_view(JSON.stringify({type: "SkyBox"}));
        })
        .catch((e) => {this.on_error("Error in App::init when importing pkg", e);});
      } catch(e) {
        this.on_error("Error in App::init_wasm", e);
      }
    },

    worker_init: function() {
      try {
        if (window.Worker) {
          this.loading = "Initializing worker";
          this.worker = new WorkerSolver();
          this.worker.onerror = (evt) => { 
            this.on_error("Error in worker", evt); 
          };
          this.worker.onmessage = (evt) => {
            try {
              if (evt.data !== undefined) {
                if (evt.data.on_initialized) {
                  this.loading = null;
                  this.make();

                } else if (evt.data.on_step) {
                  this.log(`Simulation step: ${evt.data.time.toFixed(2)}s [${evt.data.iteration}]`);

                } else if (evt.data.solution) {
                  if (this.simulation) {
                    this.simulation.solution_from_arraybuffer(evt.data.solution);
                  } else {
                    throw "Simulation is not initialized when getting solution";
                  }

                } else {
                  throw "Event " + evt.data + " is not supported";
                }
              }
            } catch (e) {
              this.on_error("Error in worker.onmessage", e);
            }
          }; // this.worker.onmessage
        } else {
          throw "WebWorkers are not supported by this web browser. Unable to proceed with solving";
        }
      } catch(e) {
        this.on_error("Error in App::worker_init", e);
      }
    },

    simulation_f: function(f) {
      try {
        if (this.simulation !== null) {
          f(this.simulation);
        }
      } catch (e) {
        this.on_error("Error in simulation", e);
      }
    },

    loop_handle_render: function() {
      try {
        const t = performance.now();
        if (this.controller_right || this.controller_left) {
          if (this.controller_signal.time_stamp === null) {
            this.controller_signal.time_stamp = t;
          } else {
            const dtms = t - this.controller_signal.time_stamp;
            this.controller_signal = {...this.controller_signal, time_stamp: t, dtms};
            if (this.simulation) { this.simulation.on_controller_signal(this.controller_signal); }
          }
        }
        this.controller_signal.time_stamp = t;

        if (this.simulation !== null) { this.simulation.draw(); }
        if (this.record) { this.$refs.media_recorder.capture(this.$refs.viewerrender.get_canvas()); }

      } catch (e) {
        this.on_error("Error in App::loop_handle_render", e);
      }
    },

    loop: function() {
      try {
        this.loop_handle_render();
        if (this.active) { window.requestAnimationFrame(() => {this.loop();});}
      } catch (e) {
        this.on_error("Error in App::loop", e);
      }
    },

    on_resize: function() {
      try {
        const p = document.getElementById("app-viewer");
        this.$refs.viewerrender.on_resize(p.clientWidth, p.clientHeight);
      } catch (e) {
        this.on_error("Error in App::on_resize", e);
      }
    },

    clear_error: function() {
      this.error = null; this.error_message = null;
    },

    on_error: function(msg, e) {
      this.error = e; this.error_message = msg;
    }
  }
}
</script>

