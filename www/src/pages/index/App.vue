<template>
  <div id="app">
    <!-- Page: home -->
    <div class="bg-white" v-if="page == 'home'">
      <main>
        <SetupToolbar>
          <Breadcrumb
            :home="{
              onClick: function () {
                this.navigate_to('home');
              }.bind(this),
            }"
            :pages="[]"
          />
        </SetupToolbar>
        <div class="mx-auto sm:px-6 lg:px-8 pt-2">
          <p>
            Welcome to the Vortex Particle Simulation GUI. <br />
            Select one of the scenario below to get started:
          </p>
        </div>

        <div class="relative pt-2 pb-2 px-4 sm:px-6 lg:pt-4 lg:pb-8 lg:px-8">
          <div
            class="
              mt-2
              max-w-lg
              mx-auto
              grid
              gap-5
              lg:grid-cols-3
              lg:max-w-none
            "
          >
            <VortexParticleSimulationScenario
              imageUrl="/assets/vortexparticlesimulation/vortexring.png"
              title="Vortex Ring"
              description="Simulation of a vortex ring that self propagates through its own vorticity. 
                           Vortex ring anulus diameter, thickness and vorticity magnitude can be 
                           modified"
              author="Julien de Charentenay, PhD"
              @select="on_select_scenario('InitialConditionVortexRing')"
            />
          </div>
        </div>
      </main>

      <FooterSection />
    </div>

    <!-- Page: setup -->
    <div class="bg-white" v-else-if="page == 'setup'">
      <main>
        <SetupToolbar>
          <Breadcrumb
            :home="{
              onClick: function () {
                this.navigate_to('home');
              }.bind(this),
            }"
            :pages="[
              {
                name: 'Setup',
                href: '#',
                onClick: function () {
                  this.navigate_to('setup');
                }.bind(this),
              },
            ]"
          />
        </SetupToolbar>
        <div class="mx-auto sm:px-6 lg:px-8 pt-2">
          <VortexParticleSimulationSimulationSetup />
          <div
            class="mt-6"
            v-if="
              simulation_initial_conditions_name == 'InitialConditionVortexRing'
            "
          >
            <VortexParticleSimulationVortexRingSetup
              v-if="
                simulation_initial_conditions_name ==
                'InitialConditionVortexRing'
              "
            />
            <div class="max-w-7xl mx-auto mt-6 flex flex-row-reverse">
              <button
                type="button"
                class="
                  px-6
                  py-3
                  border border-gray-300
                  shadow-sm
                  text-xs
                  font-medium
                  rounded
                  text-white
                  bg-indigo-600
                  hover:bg-indigo-700
                  focus:outline-none
                  focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500
                "
                @click="navigate_to('player')"
              >
                Start simulation
              </button>
            </div>
          </div>
          <p v-else>
            Initial condition name: {{ simulation_initial_conditions_name }} is
            not recognized.
          </p>
        </div>
      </main>
      <FooterSection />
    </div>

    <!-- page: player -->
    <div class="bg-white" v-else-if="page == 'player'">
      <SetupToolbar>
        <Breadcrumb
          :home="{
            onClick: function () {
              this.navigate_to('home');
            }.bind(this),
          }"
          :pages="[
            {
              name: 'Setup',
              href: '#',
              onClick: function () {
                this.navigate_to('setup');
              }.bind(this),
            },
            {
              name: 'Player',
              href: '#',
              onClick: function () {
                this.navigate_to('player');
              }.bind(this),
            },
          ]"
        />
      </SetupToolbar>
      <Player />
      <Solver />
    </div>

    <!-- Unknown page -->
    <div class="bg-white" v-else>
      <main>
        <SetupToolbar>
          <Breadcrumb
            :home="{
              onClick: function () {
                this.navigate_to('home');
              }.bind(this),
            }"
            :pages="[
              {
                name: 'Unknown',
                href: '#',
                onClick: function () {
                  this.navigate_to('unknown');
                }.bind(this),
              },
            ]"
          />
        </SetupToolbar>
        <div class="max-w-7xl mx-auto sm:px-6 lg:px-8 pt-2">
          <p>
            Welcome to the Vortex Particle Simulation GUI. The requested page
            {{ page }} is unknown. Please reload the page to continue.
          </p>
        </div>
      </main>

      <FooterSection />
    </div>

    <UndoRedo />
  </div>
</template>

<script>
import Breadcrumb from "@/components/Breadcrumb.vue";
import FooterSection from "@/components/FooterSection.vue";
import Player from "@/components/Player.vue";
import SetupToolbar from "@/components/SetupToolbar.vue";
import Solver from "@/components/Solver.vue";
import VortexParticleSimulationScenario from "@/components/VortexParticleSimulationScenario.vue";
import VortexParticleSimulationSimulationSetup from "@/components/VortexParticleSimulationSimulationSetup.vue";
import VortexParticleSimulationVortexRingSetup from "@/components/VortexParticleSimulationVortexRingSetup.vue";

import UndoRedo from "@/utils/undoredo/component.vue";

export default {
  name: "App",
  mounted: function () {
    // Add Undo/Redo calls
    document.addEventListener(
      "keydown",
      function (evt) {
        if (evt.ctrlKey && evt.key === "z") {
          this.$store.commit("undo");
        } else if (evt.ctrlKey && evt.key === "y") {
          this.$store.commit("redo");
        }
      }.bind(this)
    );
  },

  computed: {
    page: function () {
      return this.$store.state.page;
    },

    simulation_initial_conditions_name: function () {
      return this.$store.state.configuration.initial_conditions_name;
    },
  },

  methods: {
    on_select_scenario: function (name) {
      this.$store.commit("configuration/set_initial_conditions_name", name);
      this.navigate_to("setup");
    },
    navigate_to: function (name) {
      this.$store.commit("set_page", name);
    },
  },

  components: {
    Breadcrumb,
    FooterSection,
    VortexParticleSimulationScenario,
    VortexParticleSimulationSimulationSetup,
    VortexParticleSimulationVortexRingSetup,
    SetupToolbar,
    Solver,
    UndoRedo,
    Player,
  },
};
</script>
