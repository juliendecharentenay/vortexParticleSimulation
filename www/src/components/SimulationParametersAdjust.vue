<template>
  <div class="bg-white flex flex-col">
    <div class="grow overflow-y-scroll p-2 lg:grid lg:grid-cols-12 lg:gap-x-5">
      <aside class="py-6 px-2 sm:px-6 lg:py-0 lg:px-0 lg:col-span-3">
        <nav class="space-y-1">
          <a href="#" 
            v-for="item in nav_items"
            :key="item.id"
            :class="[current === item.id ? 'bg-gray-100 text-indigo-700 hover:text-indigo-700 hover:bg-gray-50' : 'text-gray-900 hover:text-gray-900 hover:bg-gray-100', 'group rounded-md px-3 py-2 flex items-center text-sm font-medium']" 
            :aria-current="current === item.id ? 'page' : undefined"
            @click="current = item.id" >
            <span class="truncate">{{ item.name }}</span>
          </a>
        </nav>
      </aside>

      <div class="space-y-6 sm:px-6 lg:px-0 lg:col-span-9">
        <PhysicalProperties 
          v-model="physical_properties"
          @on_error="$emit('on_error', $event)"
          v-if="current === 'Properties'" />
        <InitialConditions 
          v-model="initial_conditions"
          @on_error="$emit('on_error', $event)"
          v-else-if="current === 'InitialConditions'" />
        <NumericsProperties 
          v-model="numerics_properties"
          @on_error="$emit('on_error', $event)"
          v-else-if="current === 'Numerics'" />
        <div v-else>
          Option {{ current }} is not supported
        </div>
      </div>
    </div>

    <div class="flex-none px-4 py-3 bg-gray-50 text-right sm:px-6">
      <button type="submit" class="mx-4 bg-white border border-gray-300 rounded-md shadow-sm py-2 px-4 inline-flex justify-center text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" @click="$emit('on_cancel')">Cancel</button>

      <button type="submit" class="bg-indigo-600 border border-transparent rounded-md shadow-sm py-2 px-4 inline-flex justify-center text-sm font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" @click="$emit('on_update')">Update</button>
    </div>

  </div>
</template>
<script>

import PhysicalProperties from './SimulationParameters/PhysicalProperties';
import InitialConditions from './SimulationParameters/InitialConditions';
import NumericsProperties from './SimulationParameters/NumericsProperties';

export default {
  name: 'SimulationParametersAdjust',
  components: { 
    PhysicalProperties,
    InitialConditions,
    NumericsProperties,
  },
  props: ['modelValue'],
  emits: ['update:modelValue', 'on_error', 'on_update', 'on_cancel'],
  data: function() {
    return {
      current: 'InitialConditions',
      copy: null,
    };
  },
  created: function() {
    this.copy = JSON.parse(JSON.stringify(this.modelValue));
  },
  computed: {
    nav_items: function() {
      return [
        {id: 'InitialConditions', name: 'Initial Conditions'},
        {id: 'Properties', name: 'PhysicalProperties'},
        {id: 'Numerics', name: 'Numerics'},
      ];
    },
    physical_properties: {
      get: function() {
        return {
          viscosity: this.copy.configuration.viscosity,
        };
      },
      set: function(v) {
        this.copy.configuration.viscosity = v.viscosity;
        this.$emit('update:modelValue', this.copy);
      }
    },
    initial_conditions: {
      get: function() {
        return this.copy.configuration.initial_conditions;
      },
      set: function(v) {
        this.copy.configuration.initial_conditions = v;
        this.$emit('update:modelValue', this.copy);
      },
    },
    numerics_properties: {
      get: function() {
        return {
          n_vortons: this.copy.configuration.n_vortons,
        };
      },
      set: function(v) {
        this.copy.configuration.n_vortons = v.n_vortons;
        this.update_model_value();
      },
    },
  },
  methods: {
    update_model_value: function() {
      try {
        this.$emit('update:modelValue', this.copy);
      } catch (e) {
        this.on_error("Error in SimulationParametersAdjust::update_model_value", e);
      }
    },
    on_error: function(message, error) {
      this.$emit('on_error', {message, error});
    }
  }
}
</script>
