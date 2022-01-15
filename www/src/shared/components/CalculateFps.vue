<template>
  <div>
  </div>
</template>
<script>
export default {
  name: "CalculateFps",
  data: function() {
    return {
      dt_ms: null,
      last_time: null,
    };
  },
  methods: {
    tick: function() {
      const t = new Date().getTime();
      if (this.last_time !== null) {
        const dt = t - this.last_time; // ms
        this.dt_ms = (this.dt_ms === null ? dt : (9*this.dt_ms + dt)/10);
        this.$emit("fps", Math.round(1000 / Math.max(1, this.dt_ms)));
      }
      this.last_time = t;
    },
    on_error: function(msg, e) {
      this.$emit("error", {msg, e});
    }
  }
}
</script>
