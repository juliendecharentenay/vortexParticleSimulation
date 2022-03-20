<template>
  <svg xmlns="http://www.w3.org/2000/svg"
     class="absolute top-0 left-0 w-40 -ml-20 h-40 -mt-20"
     :style="{'top': `${touch.start.clientY.toFixed(0)}px`, 'left': `${touch.start.clientX.toFixed(0)}px`}"
     viewBox="-30 -30 60 60"
     fill="none"
     stroke-width="1"
     ref="touch_indicator_svg"
     >
   <circle cx="0" cy="0" :r="outer_r" class="stroke-slate-50" />
   <circle cx="0" cy="0" :r="inner_r"  class="stroke-slate-400" />
  </svg>
</template>
<script>
export default {
  name: "TouchIndicator",
  props: {
    touch: {
      type: Object,
      required: true,
    }
  },
  data: function() {
    return {
      outer_r: 30,
      inner_r: 10,
    };
  },
  methods: {
    ratio: function() {
      return this.inner_r / this.outer_r;
    },
    width: function() {
      return this.$refs.touch_indicator_svg.clientWidth;
    },
    height: function() {
      return this.$refs.touch_indicator_svg.clientHeight;
    },
  }
};
</script>
