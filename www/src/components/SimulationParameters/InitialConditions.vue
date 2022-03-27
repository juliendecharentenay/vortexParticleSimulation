<template>
  <div class="shadow sm:rounded-md sm:overflow-hidden">
    <div class="bg-white py-6 px-4 space-y-6 sm:p-6">
      <div>
        <h3 class="text-lg leading-6 font-medium text-gray-900">Initial Conditions</h3>
        <p class="mt-1 text-sm text-gray-500">Defines the starting point of the simulation, ie how is the fluid set at the beginning of the simulation.<br/>
        The following defines the parameters for a vortex ring initialisation:
        </p>
      </div>

      <div class="grid grid-cols-3 gap-6">
        <div class="col">
          <label for="center-x" class="block text-sm font-medium text-gray-700">Center, X (m)</label>
          <input type="number" step=any v-model="center_x" name="center-x" id="center-x" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </div>

        <div class="col">
          <label for="center-y" class="block text-sm font-medium text-gray-700">Center, Y (m)</label>
          <input type="number" step=any v-model="center_y" name="center-y" id="center-y" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </div>

        <div class="col">
          <label for="center-z" class="block text-sm font-medium text-gray-700">Center, Z (m)</label>
          <input type="number" step=any v-model="center_z" name="center-z" id="center-z" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </div>

        <div class="col">
          <label for="direction-x" class="block text-sm font-medium text-gray-700">Direction, X (m)</label>
          <input type="number" step=any v-model="direction_x" name="direction-x" id="direction-x" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </div>

        <div class="col">
          <label for="direction-y" class="block text-sm font-medium text-gray-700">Direction, Y (m)</label>
          <input type="number" step=any v-model="direction_y" name="direction-y" id="direction-y" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </div>

        <div class="col">
          <label for="direction-z" class="block text-sm font-medium text-gray-700">Direction, Z (m)</label>
          <input type="number" step=any v-model="direction_z" name="direction-z" id="direction-z" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </div>

        <div class="col-span-3">
          <label for="intensity" class="block text-sm font-medium text-gray-700">Intensity (?)</label>
          <input type="number" min=0 step=any v-model="intensity" name="intensity" id="intensity" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </div>

        <div class="col-span-3">
          <label for="radius" class="block text-sm font-medium text-gray-700">Radius (m)</label>
          <input type="number" min=0 step=any v-model="radius" name="radius" id="radius" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </div>

        <div class="col-span-3">
          <label for="thickness" class="block text-sm font-medium text-gray-700">Thickness (m)</label>
          <input type="number" min=0 step=any v-model="thickness" name="thickness" id="thickness" class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </div>

      </div>
    </div>

  </div>
</template>
<script>
export default {
  name: 'InitialConditions',
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
    center_x: {
      get: function() {return this.copy.InitialConditionVortexRing.center.x; },
      set: function(v) {
        this.copy.InitialConditionVortexRing.center.x = v;
        this.update_model_value();
      }
    },

    center_y: {
      get: function() {return this.copy.InitialConditionVortexRing.center.y; },
      set: function(v) {
        this.copy.InitialConditionVortexRing.center.y = v;
        this.update_model_value();
      }
    },

    center_z: {
      get: function() {return this.copy.InitialConditionVortexRing.center.z; },
      set: function(v) {
        this.copy.InitialConditionVortexRing.center.z = v;
        this.update_model_value();
      }
    },

    direction_x: {
      get: function() {return this.copy.InitialConditionVortexRing.direction.x; },
      set: function(v) {
        this.copy.InitialConditionVortexRing.direction.x = v;
        this.update_model_value();
      }
    },

    direction_y: {
      get: function() {return this.copy.InitialConditionVortexRing.direction.y; },
      set: function(v) {
        this.copy.InitialConditionVortexRing.direction.y = v;
        this.update_model_value();
      }
    },

    direction_z: {
      get: function() {return this.copy.InitialConditionVortexRing.direction.z; },
      set: function(v) {
        this.copy.InitialConditionVortexRing.direction.z = v;
        this.update_model_value();
      }
    },

    intensity: {
      get: function() {return this.copy.InitialConditionVortexRing.intensity; },
      set: function(v) {
        this.copy.InitialConditionVortexRing.intensity = v;
        this.update_model_value();
      }
    },

    radius: {
      get: function() {return this.copy.InitialConditionVortexRing.radius; },
      set: function(v) {
        this.copy.InitialConditionVortexRing.radius = v;
        this.update_model_value();
      }
    },

    thickness: {
      get: function() {return this.copy.InitialConditionVortexRing.thickness; },
      set: function(v) {
        this.copy.InitialConditionVortexRing.thickness = v;
        this.update_model_value();
      }
    },

  },
  methods: {
    update_model_value: function() {
      try {
        this.$emit('update:modelValue', this.copy);
      } catch (e) {
        this.on_error("Error in InitialConditions::update_model_value", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('on_error', {message, error});
    }
  }
}
</script>
