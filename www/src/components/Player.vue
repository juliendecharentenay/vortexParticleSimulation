<template>
  <div class="fixed inset-x-0 top-16 bottom-0">
    <div
      class="absolute inset-x-0 top-0 bottom-0 bg-gray-500"
      id="player-viewer"
    >
      <div class="absolute inset-x-10 top-10 bottom-10 br-gray-400 text-center">
        Viewer placeholder
      </div>
    </div>
    <!-- <div class="absolute inset-x-0 bottom-0 h-10 inset-x-0 bg-gray-700"><p>Bottom element</p></div> -->
    <div class="absolute inset-x-0 bottom-3" id="controls">
      <div class="flex">
        <span class="mx-auto relative z-0 inline-flex shadow-sm rounded-md">
          <button
            type="button"
            class="
              relative
              inline-flex
              items-center
              px-4
              py-2
              rounded-md
              border border-gray-300
              bg-white
              text-sm
              font-medium
              text-gray-700
              hover:bg-gray-50
              focus:z-10
              focus:outline-none
              focus:ring-1 focus:ring-indigo-500
              focus:border-indigo-500
            "
            @click="solving ? stop() : start()"
          >
            {{ solving ? "Stop" : "Play" }}
          </button>
        </span>
      </div>
    </div>
  </div>
</template>
<script>
export default {
  name: "Player",
  data: function () {
    return {};
  },

  computed: {
    solving: function () {
      return this.$store.state.solver.solving;
    },
  },

  mounted: function () {
    this.$store.dispatch("solver/initialize");
  },

  methods: {
    stop: function () {
      this.$store.commit("solver/stop");
    },

    start: function () {
      this.$store.commit("solver/start");
    },

    get_viewer_element: function () {
      return document.getElementById("player-viewer");
    },
  },
};
</script>
