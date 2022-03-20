<template>
  <div>
    <svg xmlns="http://www.w3.org/2000/svg" 
      class="absolute top-0 left-0 h-10 w-10 -mt-5 -ml-5"
      :style="{'top': `${m.clientY.toFixed(0)}px`, 'left': `${m.clientX.toFixed(0)}px`}"
      viewBox="-10 -10 20 20" stroke="none" stroke-width="1"
      :key="m.timeStamp"
      v-for="m in markers"
      >
      <circle cx="0" cy="0" r="7" class="fill-slate-400" :style="{'opacity': `${m.opacity.toFixed(2)}`}" />
    </svg>
  </div>
</template>
<script>
export default {
  name: "TouchMarker",
  props: {
    markers: {
      type: Array,
      required: true,
    }
  }
};
</script>
