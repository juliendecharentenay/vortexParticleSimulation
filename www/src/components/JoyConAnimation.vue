<template>
  <div>
    <TouchMarker :markers="joycon_markers" v-if="joycon_markers.length > 0" />
    <TouchIndicator
      ref="joycon_touch_indicator"
      :touch="joycon_touch"
      v-if="joycon_touch !== null" />
    <div
      @touchstart="onJoyConTouch"
      @touchmove="onJoyConTouch"
      @touchend="onJoyConTouch"
      @touchcancel="onJoyConTouch"
      :id="target_id"
      >
      <slot></slot>
    </div>
  </div>
</template>
<script>
import TouchMarker from "@/components/TouchMarker";
import TouchIndicator from "@/components/TouchIndicator";

export default {
  name: "JoyConAnimation",
  components: {
    TouchMarker,
    TouchIndicator,
  },
  props: {
  },
  data: function() {
    return {
      target_id: null,
      joycon_markers: [],
      joycon_touch: null,
      interval: null,
    };
  },
  emits: ['active', 'on_change', 'on_error'],
  mounted: function() {
    try {
      this.target_id = `target_${this.generateUID()}`;
    } catch (e) {
      this.on_error("Error in JoyConAnimation::mount", e);
    }
  },
  beforeUnmount: function() {
    try {
      this.clearInterval();
    } catch (e) {
      this.on_error("Error in JoyConAnimation::beforeUnmout", e);
    }
  },
  methods: {
    // From https://stackoverflow.com/questions/6248666/how-to-generate-short-uid-like-ax4j9z-in-js
    generateUID: function() {
      // I generate the UID from two parts here 
      // to ensure the random number provide enough bits.
      var firstPart = (Math.random() * 46656) | 0;
      var secondPart = (Math.random() * 46656) | 0;
      firstPart = ("000" + firstPart.toString(36)).slice(-3);
      secondPart = ("000" + secondPart.toString(36)).slice(-3);
      return firstPart + secondPart;
    },
    clearInterval: function() {
      try {
        if (this.interval !== null) {
          clearInterval(this.interval);
          this.interval = null;
        }
      } catch (e) {
        this.on_error("Error in JoyConAnimation::clearInterval", e);
      }
    },

    setInterval: function() {
      try {
        if (this.interval !== null) { this.clearInterval(); }
        this.interval = setInterval(() => {
          try {
            var t_now = performance.now();
            if (this.joycon_touch !== null) {
              this.joycon_markers.push({
                opacity: 1,
                timeNow: t_now,
                clientX: this.joycon_touch.current.clientX,
                clientY: this.joycon_touch.current.clientY,
                radiusX: this.joycon_touch.current.radiusX,
                radiusY: this.joycon_touch.current.radiusY,
                force:   this.joycon_touch.current.force,
              });
            }
            this.joycon_markers
            = this.joycon_markers
            .filter((m) => (m.timeNow + 300.0 > t_now))
            .map((m) => ({...m, opacity: (300.0 - (t_now - m.timeNow))/300.0}));
            if (this.joycon_markers.length === 0) {this.clearInterval();}
          } catch (e) {
            this.on_error("Error in JoyConAnimation::setInterval, interval function", e);
          }
        }, 0);
      } catch (e) {
        this.on_error("Error in JoyConAnimation::setInterval", e);
      }
    },

    handle: function() {
      try {
        var result = null;
        var indicator = this.$refs.joycon_touch_indicator;
        if (this.joycon_touch !== null && indicator !== undefined && indicator !== null) {
          var c = this.joycon_touch.current; var s = this.joycon_touch.start;
          var x = [c.clientX - s.clientX, -c.clientY + s.clientY];
          const r = Math.sqrt(x[0] ** 2 + x[1] ** 2);
          const rmax   = 0.5 * indicator.width();
          const ratio  = indicator.ratio();
          const rmin   = rmax * ratio;

          var alpha;
          if (r < rmin) {
            alpha = 0.0;
          } else {
            alpha = 1.0 / r * (Math.min(r, rmax) - rmin) / (rmax - rmin);
          }
          result = x.map((v) => alpha * v);
        }
        this.$emit('on_change', { result });
      } catch (e) {
        this.on_error("Error in JoyConAnimation::handle", e);
      }
    },

    onJoyConTouch: function(evt) {
      try {
        console.log("JoyConAnimation::onJoyConTouch");
        evt.preventDefault();
        switch (evt.type) {
          case "touchstart":
            if (this.joycon_touch === null) {
              var touch = evt.changedTouches[0];
              if (touch !== undefined) {
                this.joycon_touch = {
                  identifier: touch.identifier,
                  start: touch,
                  current: touch,
                };
                this.setInterval();
                this.$emit('active', true);
              }
            }
            break;
          case "touchmove":
            if (this.joycon_touch !== null) {
              let touch = Array.from(evt.touches).find((t) => t.identifier === this.joycon_touch.identifier);
              if (touch !== undefined) {
                this.joycon_touch = {...this.joycon_touch, current: touch, };
              }
            }
            break;
          case "touchend":
          case "touchcancel":
            if (this.joycon_touch !== null) {
              let touch = Array.from(evt.changedTouches).find((t) => t.identifier === this.joycon_touch.identifier);
              if (touch !== undefined) {
                this.joycon_touch = null;
                this.$emit('active', false);
              }
            }
            break;
        }
        this.handle();

      } catch (e) {
        this.on_error("Error in JoyConAnimation::onJoyConTouch", e);
      }
    },
    on_error: function(msg, e) {
      this.$emit('on_error', {msg, e});
    },
  }
};
  
</script>
