<template>
  <canvas :id="canvas_id"></canvas>
</template>
<script>
export default {
  name: 'ViewerRender',
  props: [],
  data: function() { 
    return {
      canvas_id: "component-viewerrender-canvas",
    };
  },
  mounted: function() {
  },
  methods: {
    get_canvas: function() {
      return document.getElementById(this.canvas_id);
    },
    on_resize: function(w, h) {
      try {
        console.log(`Resize canvas to ${w}/${h}`);
        var c = this.get_canvas();
        c.width = w; c.height = h;
      } catch (e) {
        this.on_error("Error in ViewerRender::on_resize", e);
      }
    },
    on_error: function(msg, e) {
      this.$emit("on_error", {message: msg, error: e});
    }

  }
};
</script>
