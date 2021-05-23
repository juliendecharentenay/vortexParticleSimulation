<template>
  <form class="space-y-8 divide-y divide-gray-200">
    <div class="space-y-8 divide-y divide-gray-200 sm:space-y-5">
      <div>
        <div>
          <h3 class="text-lg leading-6 font-medium text-gray-900">
            Simulation Setup
          </h3>
          <p class="mt-1 text-sm text-gray-500">
            Select simulation parameters.
          </p>
        </div>

        <div
          class="
            max-w-7xl
            mx-auto
            mt-6
            grid grid-cols-1
            gap-y-6 gap-x-4
            sm:grid-cols-6
          "
        >
          <div class="sm:col-span-3">
            <label
              for="n_vortons"
              class="block text-sm font-medium text-gray-700"
            >
              Number of vortons
            </label>
            <div class="mt-1">
              <input
                type="number"
                step="any"
                min="0"
                v-model="n_vortons"
                name="n_vortons"
                id="n_vortons"
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
              for="viscosity"
              class="block text-sm font-medium text-gray-700"
            >
              Kinematic Viscosity (m<sup>2</sup>s<sup>-1</sup>)
            </label>
            <div class="mt-1">
              <input
                type="number"
                step="any"
                min="0"
                v-model="viscosity"
                name="viscosity"
                id="viscosity"
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

          <!--
          <div class="sm:col-span-2">
            <label for="domain_min_x" class="block text-sm font-medium text-gray-700">
              Domain lower vertex, X (m)
            </label>
            <div class="mt-1">
              <input type="number" step="any" v-model="domain_min_x" name="domain_min_x" id="domain_min_x" class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label for="domain_min_y" class="block text-sm font-medium text-gray-700">
              Domain lower vertex, Y (m)
            </label>
            <div class="mt-1">
              <input type="number" step="any" v-model="domain_min_y" name="domain_min_y" id="domain_min_y" class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label for="domain_min_z" class="block text-sm font-medium text-gray-700">
              Domain lower vertex, Z (m)
            </label>
            <div class="mt-1">
              <input type="number" step="any" v-model="domain_min_z" name="domain_min_z" id="domain_min_z" class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label for="domain_max_x" class="block text-sm font-medium text-gray-700">
              Domain upper vertex, X (m)
            </label>
            <div class="mt-1">
              <input type="number" step="any" v-model="domain_max_x" name="domain_max_x" id="domain_max_x" class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label for="domain_max_y" class="block text-sm font-medium text-gray-700">
              Domain upper vertex, Y (m)
            </label>
            <div class="mt-1">
              <input type="number" step="any" v-model="domain_max_y" name="domain_max_y" id="domain_max_y" class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" />
            </div>
          </div>

          <div class="sm:col-span-2">
            <label for="domain_max_z" class="block text-sm font-medium text-gray-700">
              Domain upper vertex, Z (m)
            </label>
            <div class="mt-1">
              <input type="number" step="any" v-model="domain_max_z" name="domain_max_z" id="domain_max_z" class="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md" />
            </div>
          </div>
-->
        </div>
      </div>
    </div>
  </form>
</template>

<script>
export default {
  name: "VortexParticleSimulationVortexRingSetup",
  computed: {
    n_vortons: {
      get() {
        return this.parameters().n_vortons;
      },
      set(value) {
        this.commit({ ...this.parameters(), n_vortons: parseFloat(value) });
      },
    }, // n_vortons
    viscosity: {
      get() {
        return this.parameters().viscosity;
      },
      set(value) {
        this.commit({ ...this.parameters(), viscosity: parseFloat(value) });
      },
    }, // viscosity
    domain_min_x: {
      get() {
        return this.parameters().domain.min[0];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          domain: {
            ...p.domain,
            min: [parseFloat(value), p.domain.min[1], p.domain.min[2]],
          },
        });
      },
    }, // domain_min_x
    domain_min_y: {
      get() {
        return this.parameters().domain.min[1];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          domain: {
            ...p.domain,
            min: [p.domain.min[0], parseFloat(value), p.domain.min[2]],
          },
        });
      },
    }, // domain_min_y
    domain_min_z: {
      get() {
        return this.parameters().domain.min[2];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          domain: {
            ...p.domain,
            min: [p.domain.min[0], p.domain.min[1], parseFloat(value)],
          },
        });
      },
    }, // domain_min_z
    domain_max_x: {
      get() {
        return this.parameters().domain.max[0];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          domain: {
            ...p.domain,
            max: [parseFloat(value), p.domain.max[1], p.domain.max[2]],
          },
        });
      },
    }, // domain_max_x
    domain_max_y: {
      get() {
        return this.parameters().domain.max[1];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          domain: {
            ...p.domain,
            max: [p.domain.max[0], parseFloat(value), p.domain.max[2]],
          },
        });
      },
    }, // domain_max_y
    domain_max_z: {
      get() {
        return this.parameters().domain.max[2];
      },
      set(value) {
        const p = this.parameters();
        this.commit({
          ...p,
          domain: {
            ...p.domain,
            max: [p.domain.max[0], p.domain.max[1], parseFloat(value)],
          },
        });
      },
    }, // domain_max_z
  },
  methods: {
    parameters() {
      return this.$store.state.configuration.simulation_setup;
    },
    commit(parms) {
      this.$store.commit("configuration/set_simulation_setup", parms);
    },
  },
};
</script>
