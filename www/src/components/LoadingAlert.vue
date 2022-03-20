<!-- This example requires Tailwind CSS v2.0+ -->
<template>
  <div class="relative">
  <div class="rounded-md bg-green-50 p-4">
    <div class="flex">
      <div class="flex-shrink-0">
        <CheckCircleIcon class="h-5 w-5 text-green-400" aria-hidden="true" />
      </div>
      <div class="ml-3">
        <h3 class="text-sm font-medium text-green-800" id="component-loadingalert-message" v-if="message">{{ message }}</h3>
        <h3 class="text-sm font-medium text-green-800" v-else>Loading</h3>
        <div class="mt-2 text-sm text-green-700">
          <p>Please be patient</p>
        </div>
      </div>
    </div>
  </div>
  </div>
</template>

<script>
import { CheckCircleIcon } from '@heroicons/vue/solid'

export default {
  name: 'ErrorAlert',
  components: {
    CheckCircleIcon,
  },
  props: ['message'],
}
</script>
