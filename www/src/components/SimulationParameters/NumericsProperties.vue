<template>
  <div class="shadow sm:rounded-md sm:overflow-hidden">
    <div class="bg-white py-6 px-4 space-y-6 sm:p-6">
      <div>
        <h3 class="text-lg leading-6 font-medium text-gray-900">Numerics</h3>
        <p class="mt-1 text-sm text-gray-500">
        Defines the simulation numerical parameters, ie parameters relating to the algorithm solving the fluid dynamics equations.
        </p>
      </div>

      <div class="grid grid-cols-3 gap-6">
        <div class="col-span-3">
          <label for="n-vortons" class="block text-sm font-medium text-gray-700">Number of vortons)</label>
          <input type="number" min=0 step=1 v-model="n_vortons" name="n-vortons" id="n-vortons" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </div>

      </div>
    </div>

  </div>
</template>
<script>
export default {
  name: 'NumericsProperties',
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
    n_vortons: {
      get: function() { return this.copy.n_vortons; },
      set: function(v) {
        this.copy.n_vortons = v;
        this.update_model_value();
      }
    }
  },
  methods: {
    update_model_value: function() {
      try {
        this.$emit('update:modelValue', this.copy);
      } catch (e) {
        this.on_error("Error in NumericsProperties::update_model_value", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('on_error', {message, error});
    }
  }
}
</script>
