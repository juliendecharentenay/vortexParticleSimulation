export function initial_state() {
  return {
    /*
     * solving: flag that indicates when solving
     */
    solving: false,

    /*
     * time_step: store the time_step (s)
     */
    time_step: 0.03,
  };
}

const state = () => initial_state();

const mutations = {
  emptyState(state) {
    this.replaceState({ ...state, ...initial_state() });
  },
  start(state) {
    state.solving = true;
  },
  stop(state) {
    state.solving = false;
  },
  time_step(state, time_step) {
    state.time_step = time_step;
  },
};

const actions = {
  initialize() {},
};

const getters = {};

export default {
  namespaced: true,
  state,
  mutations,
  actions,
  getters,
};
