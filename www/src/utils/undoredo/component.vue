<template>
  <div class="display:none"></div>
</template>
<script>
export default {
  data: function () {
    return {
      m_done: [],
      m_undone: [],
      m_newMutation: true,
      m_ignoreMutation: false,
    };
  },
  props: [],
  mounted: function () {
    this.$store.subscribe((mutation) => {
      if (this.m_ignoreMutation) {
        // Ignore
      } else if (mutation.type === "emptyState") {
        // Ignore
      } else if (mutation.type === "setCanUndoRedo") {
        // Ignore
      } else if (mutation.type === "undo") {
        // Ignore
        this.undo();
      } else if (mutation.type === "redo") {
        // Ignore
        this.redo();
      } else {
        this.done(mutation);
      }
    });
  },
  methods: {
    done: function (mutation) {
      this.m_done.push(mutation);
      if (this.m_newMutation) {
        this.m_undone = [];
      }
      this.set_store();
    },
    undo: function () {
      if (this.m_done.length > 0) {
        this.m_ignoreMutation = true;
        this.m_newMutation = false;
        this.m_undone.push(this.m_done.pop());
        this.$store.commit("emptyState");
        this.m_done.forEach((mutation) => {
          this.$store.commit(`${mutation.type}`, mutation.payload);
        });
        this.m_newMutation = true;
        this.m_ignoreMutation = false;
        this.set_store();
      }
    },
    redo: function () {
      if (this.m_undone.length > 0) {
        let commit = this.m_undone.pop();
        this.m_newMutation = false;
        this.$store.commit(`${commit.type}`, commit.payload);
        this.m_newMutation = true;
        this.set_store();
      }
    },
    set_store: function () {
      this.m_ignoreMutation = true;
      this.$store.commit("setCanUndoRedo", {
        canUndo: this.m_done.length > 0,
        canRedo: this.m_undone.length > 0,
      });
      this.m_ignoreMutation = false;
    },
  },
};
</script>
