<template>
  <div>
    <!-- label for="control-style-select" class="block text-sm font-medium text-gray-700">Location</label -->
    <select id="control-style-select" name="control-style-select" 
            v-model="myControl"
            class="mt-1 block w-full pl-2 pr-10 py-0 text-sm border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 rounded-md">
      <option value="classic"  :selected="myControl === 'classic'">Classic controls</option>
      <option value="rc-plane" :selected="myControl === 'rc-plane'">R/C Plane control style</option>
      <option value="rc-heli"  :selected="myControl === 'rc-heli'">R/C Heli control style</option>
    </select>
  </div>
</template>
<script>
export default {
  name: "ContolStyleSelect",
  props: {
    modelValue: {
      type: String,
      required: true,
    }
  },
  emits: ['update:modelValue'],
  computed: {
    myControl: {
      get: function() { return this.modelValue; },
      set: function(v) { this.$emit('update:modelValue', v); },
    }
  },
};
</script>
