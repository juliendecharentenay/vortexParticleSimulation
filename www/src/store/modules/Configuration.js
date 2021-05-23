/**
 *Configuration: information about the simulation configuration
 */

export function initial_state() {
  return {
    /*
     * SimulationSetup: defines the object containing the simulation parameters
     */
    simulation_setup: {
      n_vortons: 1000,
      domain: {
        min: [-10.0, -10.0, -10.0],
        max: [10.0, 10.0, 10.0],
      },
      viscosity: 1e-5,
    },

    /*
     * initial_conditions_name: defined the initial conditions
     */
    initial_conditions_name: "InitialConditionVortexRing",

    /*
     * InitialConditionsVortexRing: defines the parameters for a vortex ring initial condition
     */
    InitialConditionVortexRing: {
      center: [0.0, 0.0, 0.0],
      direction: [1.0, 0.0, 0.0],
      intensity: 1.0,
      radius: 1.0,
      thickness: 0.5,
    },
  };
}

const state = () => initial_state();

const mutations = {
  emptyState(state) {
    this.replaceState({ ...state, ...initial_state() });
  },
  set_initial_conditions_name(state, name) {
    state.initial_conditions_name = name;
  },
  set_simulation_setup(state, setup) {
    state.simulation_setup = setup;
  },
  set_initial_condition_vortex_ring(state, data) {
    state.InitialConditionsVortexRing = data;
  },
};

const actions = {};

const getters = {
  as_configuration: (state) => {
    let c = { ...state.simulation_setup };
    c.initial_conditions = {};
    c.initial_conditions[state.initial_conditions_name] =
      state[state.initial_conditions_name];

    return c;
  },
};

export default {
  namespaced: true,
  state,
  mutations,
  actions,
  getters,
};
