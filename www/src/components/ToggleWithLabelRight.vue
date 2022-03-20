<!-- This example requires Tailwind CSS v2.0+ -->
<template>
  <SwitchGroup as="div" class="flex flex-row-reverse items-center">
    <Switch v-model="my_enabled" :class="[my_enabled ? 'bg-indigo-600' : 'bg-gray-200', 'relative inline-flex flex-shrink-0 h-4 w-8 border-2 border-transparent rounded-full cursor-pointer transition-colors ease-in-out duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500']">
      <span aria-hidden="true" :class="[my_enabled ? 'translate-x-3' : 'translate-x-0', 'pointer-events-none inline-block h-3 w-4 rounded-full bg-white shadow transform ring-0 transition ease-in-out duration-200']" />
    </Switch>
    <SwitchLabel as="span" class="mr-3">
      <slot></slot>
    </SwitchLabel>
  </SwitchGroup>
</template>

<script>
import { Switch, SwitchGroup, SwitchLabel } from '@headlessui/vue'

export default {
  components: {
    Switch,
    SwitchGroup,
    SwitchLabel,
  },
  props: {
    modelValue: {
      type: Boolean,
      required: true,
    }, 
    disabled: {
      type: Boolean,
      required: false,
      default: false,
    }
  },
  emits: ['update:modelValue'],
  /*
  data: function() {
    return {
      my_enabled: this.enabled,
    };
  },
  */
  computed: {
    my_enabled: {
      get: function() {return this.modelValue; },
      set: function(v) { if (! this.disabled) { this.$emit('update:modelValue', v); } },
    }
  }
}
</script>
