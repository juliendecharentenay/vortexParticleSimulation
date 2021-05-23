/*
 * Undo-redo plugin as store module for ease of access
 *
 * Not mine.
 * Adapted from https://vuejsdevelopers.com/2017/11/13/vue-js-vuex-undo-redo/
 *              https://github.com/anthonygore/vuex-undo-redo
 */

const state = () => ({
  canRedo: false,
  canUndo: false,
});

const getters = {
  canUndo(state) {
    return state.canUndo;
  },
  canRedo(state) {
    return state.canRedo;
  },
};

const mutations = {
  setCanUndoRedo(state, payload) {
    state.canUndo = payload.canUndo;
    state.canRedo = payload.canRedo;
  },
  undo() {},
  redo() {},
};

const actions = {};

export default {
  // namespaced: true,
  state,
  mutations,
  actions,
  getters,
};
