import Vue from "vue";
import Vuex from "vuex";

import Configuration from "./modules/Configuration.js";
import Solver from "./modules/Solver.js";
import UndoRedo from "@/utils/undoredo/store.js";

Vue.use(Vuex);

/**
 * Vuex module for handling actions from the Vortex Particles Simulation Gui
 */

function initial_state() {
  return {
    /*
     * page: defines the navigation within the page Index/App.vue
     */
    page: "home",
  };
}

export default new Vuex.Store({
  state: initial_state(),

  mutations: {
    emptyState(state) {
      console.log(this);
      console.log(state);
      this.replaceState({ ...state, ...initial_state() });
      console.log(state);
    },
    set_page(state, name) {
      state.page = name;
    },
  },

  getters: {
    page: (state) => {
      return state.page;
    },
  },

  actions: {},

  modules: {
    configuration: Configuration,
    solver: Solver,
    undoredo: UndoRedo,
  },
});
