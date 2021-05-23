<template>
  <div class="bg-white border-b border-gray-200">
    <!-- Toolbar-->
    <div class="mx-auto h-16 flex flex-col justify-center">
      <div class="px-4 sm:px-6 lg:px-8">
        <div class="py-3 flex justify-between">
          <!-- Left buttons -->
          <div>
            <span
              class="
                relative
                z-0
                inline-flex
                shadow-sm
                rounded-md
                sm:shadow-none
                sm:space-x-3
              "
            >
              <span class="flex space-x-1">
                <slot></slot>
                <!-- Do not expose undo/redo just yet
                <button type="button" class="inline-flex items-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-xs font-medium rounded text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                        @click="undo()"
                        v-if="canUndo">
                  <Rewind class="-ml-0.5 mr-2 h-4 w-4 text-gray-400" aria-hidden="true" />
                  Undo
                </button>

                <button type="button" class="inline-flex items-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-xs font-medium rounded text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                        @click="redo()"
                        v-if="canRedo">
                  Redo
                  <FastForward class="ml-2 -mr-0.5 h-4 w-4 text-gray-400" aria-hidden="true" />
                </button>
-->
              </span>
              <!--
              <span class="flex space-x-3">
                <button type="button" class="inline-flex items-center px-2.5 py-1.5 border border-gray-300 shadow-sm text-xs font-medium rounded text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                        @click="$emit('start')" >
                  Start
                </button>
              </span>
-->
            </span>
          </div>

          <div class="flex-shrink-0 items-center">
            <a href="https://www.cfd-webassembly.com/">
              <span
                class="
                  block
                  text-lg
                  sm:text-xl
                  font-bold font-serif
                  not-italic
                  text-blue-800
                "
                >CFD-WebAssembly<span class="italic">.com</span></span
              >
            </a>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
// import FastForward from "@/components/heroicons/FastForward.vue";
// import Rewind from "@/components/heroicons/Rewind.vue";

export default {
  name: "SetupToolbar",
  computed: {
    canUndo: function () {
      return this.$store.state.undoredo.canUndo;
    },
    canRedo: function () {
      return this.$store.state.undoredo.canRedo;
    },
  },
  methods: {
    undo: function () {
      this.$store.commit("undo");
    },
    redo: function () {
      this.$store.commit("redo");
    },
  },
  components: {
    // FastForward,
    // Rewind
  },
};
</script>
