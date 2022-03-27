<template>
  <div class="shadow sm:rounded-md sm:overflow-hidden">
    <div class="bg-white py-6 px-4 space-y-6 sm:p-6">
      <div>
        <h3 class="text-lg leading-6 font-medium text-gray-900">Physical Properties</h3>
        <p class="mt-1 text-sm text-gray-500">Defines the physical properties employed in the simulation, ie what is being simulated.</p>
      </div>

      <div class="grid grid-cols-3 gap-6">
        <div class="col-span-3 sm:col-span-2">
          <label for="viscosity" class="block text-sm font-medium text-gray-700">Viscosity</label>
          <input type="number" step=any min=0 v-model="viscosity" name="viscosity" id="viscosity" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </div>
      </div>
    </div>

  </div>
</template>
<script>
export default {
  name: 'PhysicalProperties',
  components: { },
  props: [ 'modelValue' ],
  emits: [ 'update:modelValue', 'on_error' ],
  data: function() {
    return {
      copy: null,
      };
  },
  created: function() {
    this.copy = JSON.parse(JSON.stringify(this.modelValue));
  },
  computed: {
    viscosity: {
      get: function() { return this.copy.viscosity; },
      set: function(v) {
        this.copy.viscosity = v;
        this.update_model_value();
      }
    }
  },
  methods: {
    update_model_value: function() {
      try {
        this.$emit('update:modelValue', this.copy);
      } catch (e) {
        this.on_error("Err in PhysicalProperties::update_model_value", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('on_error', {message, error});
    }
  }
}
</script>
