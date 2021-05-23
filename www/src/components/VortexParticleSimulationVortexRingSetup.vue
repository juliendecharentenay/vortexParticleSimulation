<template>
  <form class="space-y-8 divide-y divide-gray-200">
    <div class="space-y-8 divide-y divide-gray-200 sm:space-y-5">
      <div>
        <div>
          <h3 class="text-lg leading-6 font-medium text-gray-900">
            Vortex Ring Setup
          </h3>
          <p class="mt-1 text-sm text-gray-500">
            Select the vortex ring analysis parameters prior to starting the
            fluid simulation. The vortex intensity relates to the strength of
            vorticity for the vortex ring. The radius relates to the radius of
            the vortex ring and the thickness to the spread of the vortex
            intensity. The thickness should be less than the radius. The center
            indicates the location of the center of the vortex ring using its X,
            Y and Z coordinates. The direction nominates a vector that defines
            the direction of the vortex ring using its X, Y and Z coordinates.
            The direction vector will be normalized and default to [1.0, 0.0,
            0.0] if an invalid direction (i.e [0.0, 0.0, 0.0]) was specified.
          </p>
        </div>

        <div
          class="
            max-w-7xl
            mx-auto
            mt-6
            grid grid-cols-1
            gap-y-4 gap-x-4
            sm:grid-cols-6
          "
        >
          <div class="sm:col-span-6">
            <label
              for="intensity"
              class="block text-sm font-medium text-gray-700"
            >
              Vortex Ring Intensity (s<sup>-1</sup>)
            </label>
            <div class="mt-1">
              <input
                type="number"
                step="any"
                min="0"
                v-model="intensity"
                name="intensity"
                id="intensity"
                class="
                  shadow-sm
                  focus:ring-indigo-500
                  focus:border-indigo-500
                  block
                  w-full
                  sm:text-sm
                  border-gray-300
                  rounded-md
                "
              />
            </div>
          </div>

          <div class="sm:col-span-3">
            <label for="radius" class="block text-sm font-medium text-gray-700">
              Radius (m)
            </label>
            <div class="mt-1">
              <input
                type="number"
                step="any"
                min="0"
                v-model="radius"
                name="radius"
                id="radius"
                class="
                  shadow-sm
                  focus:ring-indigo-500
                  focus:border-indigo-500
                  block
                  w-full
                  sm:text-sm
                  border-gray-300
                  rounded-md
                "
              />
            </div>
          </div>

          <div class="sm:col-span-3">
            <label
              for="thickness"
              class="block text-sm font-medium text-gray-700"
            >
              Thickness (m)
            </label>
            <div class="mt-1">
              <input
                type="number"
                step="any"
                min="0"
                v-model="thickness"
                name="thickness"
                id="thickness"
                class="
                  shadow-sm
                  focus:ring-indigo-500
                  focus:border-indigo-500
                  block
                  w-full
                  sm:text-sm
                  border-gray-300
                  rounded-md
                "
              />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label
              for="center_x"
              class="block text-sm font-medium text-gray-700"
            >
              Center, X (m)
            </label>
            <div class="mt-1">
              <input
                type="number"
                step="any"
                v-model="center_x"
                name="center_x"
                id="center_x"
                class="
                  shadow-sm
                  focus:ring-indigo-500
                  focus:border-indigo-500
                  block
                  w-full
                  sm:text-sm
                  border-gray-300
                  rounded-md
                "
              />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label
              for="center_y"
              class="block text-sm font-medium text-gray-700"
            >
              Center, Y (m)
            </label>
            <div class="mt-1">
              <input
                type="number"
                step="any"
                v-model="center_y"
                name="center_y"
                id="center_y"
                class="
                  shadow-sm
                  focus:ring-indigo-500
                  focus:border-indigo-500
                  block
                  w-full
                  sm:text-sm
                  border-gray-300
                  rounded-md
                "
              />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label
              for="center_z"
              class="block text-sm font-medium text-gray-700"
            >
              Center, Z (m)
            </label>
            <div class="mt-1">
              <input
                type="number"
                step="any"
                v-model="center_z"
                name="center_z"
                id="center_z"
                class="
                  shadow-sm
                  focus:ring-indigo-500
                  focus:border-indigo-500
                  block
                  w-full
                  sm:text-sm
                  border-gray-300
                  rounded-md
                "
              />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label
              for="direction_x"
              class="block text-sm font-medium text-gray-700"
            >
              Direction, X (m)
            </label>
            <div class="mt-1">
              <input
                type="number"
                step="any"
                v-model="direction_x"
                name="direction_x"
                id="direction_x"
                class="
                  shadow-sm
                  focus:ring-indigo-500
                  focus:border-indigo-500
                  block
                  w-full
                  sm:text-sm
                  border-gray-300
                  rounded-md
                "
              />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label
              for="direction_y"
              class="block text-sm font-medium text-gray-700"
            >
              Direction, Y (m)
            </label>
            <div class="mt-1">
              <input
                type="number"
                step="any"
                v-model="direction_y"
                name="direction_y"
                id="direction_y"
                class="
                  shadow-sm
                  focus:ring-indigo-500
                  focus:border-indigo-500
                  block
                  w-full
                  sm:text-sm
                  border-gray-300
                  rounded-md
                "
              />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label
              for="direction_z"
              class="block text-sm font-medium text-gray-700"
            >
              Direction, Z (m)
            </label>
            <div class="mt-1">
              <input
                type="number"
                step="any"
                v-model="direction_z"
                name="direction_z"
                id="direction_z"
                class="
                  shadow-sm
                  focus:ring-indigo-500
                  focus:border-indigo-500
                  block
                  w-full
                  sm:text-sm
                  border-gray-300
                  rounded-md
                "
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </form>
</template>

<script>
export default {
  name: "VortexParticleSimulationVortexRingSetup",
  computed: {
    radius: {
      get() {
        return this.parameters().radius;
      },
      set(value) {
        this.commit({ ...this.parameters(), radius: parseFloat(value) });
      },
    }, // radius
    thickness: {
      get() {
        return this.parameters().thickness;
      },
      set(value) {
        this.commit({ ...this.parameters(), thickness: parseFloat(value) });
      },
    }, // thickness
    intensity: {
      get() {
        return this.parameters().intensity;
      },
      set(value) {
        this.commit({ ...this.parameters(), intensity: parseFloat(value) });
      },
    }, // intensity
    center_x: {
      get() {
        return this.parameters().center[0];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          center: [parseFloat(value), p.center[1], p.center[2]],
        });
      },
    }, // intensity
    center_y: {
      get() {
        return this.parameters().center[1];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          center: [p.center[0], parseFloat(value), p.center[2]],
        });
      },
    }, // center_y
    center_z: {
      get() {
        return this.parameters().center[2];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          center: [p.center[0], p.center[1], parseFloat(value)],
        });
      },
    }, // center_z
    direction_x: {
      get() {
        return this.parameters().direction[0];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          direction: [parseFloat(value), p.direction[1], p.direction[2]],
        });
      },
    }, // direction_x
    direction_y: {
      get() {
        return this.parameters().direction[1];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          direction: [p.direction[0], parseFloat(value), p.direction[2]],
        });
      },
    }, // direction_y
    direction_z: {
      get() {
        return this.parameters().direction[2];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          direction: [p.direction[0], p.direction[1], parseFloat(value)],
        });
      },
    }, // direction_z
  },
  methods: {
    parameters() {
      return this.$store.state.configuration.InitialConditionVortexRing;
    },
    commit(parms) {
      this.$store.commit(
        "configuration/set_initial_condition_vortex_ring",
        parms
      );
    },
  },
};
</script>
