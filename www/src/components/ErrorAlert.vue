<!-- This example requires Tailwind CSS v2.0+ -->
<template>
  <div class="relative">
  <div class="rounded-md bg-red-50 p-4">
    <div class="flex">
      <div class="flex-shrink-0">
        <XCircleIcon class="h-5 w-5 text-red-400" aria-hidden="true" />
      </div>
      <div class="ml-3">
        <h3 class="text-sm font-medium text-red-800">We are sorry, but the following error occured:</h3>
        <div class="mt-2 text-sm text-red-700">
          <p id="component-erroralert-message" v-if="message">{{ message }}</p>
          <p>Error code:<br/>{{ error }}</p>
        </div>
        <div class="mt-4">
          <div class="-mx-2 -my-1.5 flex">
            <button type="button" 
              class="ml-3 bg-red-50 px-2 py-1.5 rounded-md text-sm font-medium text-red-800 hover:bg-red-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-red-50 focus:ring-red-600"
              @click="$emit('on_close')"
              id="component-erroralert-dismissbtn"
              >Dismiss</button>
          </div>
        </div>
      </div>
    </div>
  </div>
  </div>
</template>

<script>
import { XCircleIcon } from '@heroicons/vue/solid'

export default {
  name: 'ErrorAlert',
  components: {
    XCircleIcon,
  },
  props: ['message', 'error' ],
}
</script>
