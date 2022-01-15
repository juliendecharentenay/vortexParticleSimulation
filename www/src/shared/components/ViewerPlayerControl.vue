<template>
  <div>
    <!-- Controls -->
    <div class="absolute bottom-0 left-0 right-0 mx-auto bg-gray-400" v-if="controls">
      <div class="flex pb-1 pl-2 pr-2 justify-between">
        <div class="flex-none flex items-end">
          <!-- Record button -->
          <a href="#" @click="$emit('on_record')" :class="{'hidden': (is_recording || (! allow_recording))}"><IconRecord c="h-10 w-10" /></a>
          <!-- Play button -->
          <a href="#" @click="$emit('on_play')" :class="{'hidden': (is_playing || (! allow_playing))}"><IconPlay c="h-10 w-10" /></a>
          <!-- Stop button -->
          <a href="#" @click="$emit('on_stop')" :class="{'hidden': (! is_playing && ! is_recording)}"><IconStop c="h-10 w-10" /></a>
          <!-- Restart -->
          <a href="#" @click="$emit('on_restart')" :class="{'hidden': (video_current_time < 0.1 || (! allow_rewind))}"><IconRewind c="h-10 w-10" /></a>
        </div>
        <div class="flex-auto text-center">
        </div>
        <div class="flex-none flex">
          <!-- Instructions -->
          <a href="#" @click="show_instruction_modal = true;"><IconHelp c="h-10 w-10" /></a>
        </div>
      </div>
    </div>
    <!-- Controls ** End -->

    <!-- Instruction step -->
   <div class="fixed z-10 inset-0 overflow-y-auto" aria-labelledby="modal-title" role="dialog" aria-modal="true" v-if="show_instruction_modal">
     <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
       <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" aria-hidden="true"></div>

       <!-- This element is to trick the browser into centering the modal contents. -->
       <span class="hidden sm:inline-block sm:align-middle sm:h-screen" aria-hidden="true">&#8203;</span>

       <div class="inline-block align-bottom bg-white rounded-lg px-4 pt-5 pb-4 text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-sm sm:w-full sm:p-6">
         <div>
           <div class="mt-3 text-center sm:mt-5">
             <h3 class="text-lg leading-6 font-medium text-gray-900" id="modal-title">
               Instructions
             </h3>
             <div class="mt-2">
               <div class="flex flex-col">
                 <div class="flex flex-row items-center" v-if="allow_recording">
                   <div class="flex-none">
                     <IconRecord c="h-10 w-10" />
                   </div>
                   <div class="text-sm text-gray-500 ml-3">Play and record the video.</div>
                 </div>

                 <div class="flex flex-row items-center" v-if="allow_playing">
                   <div class="flex-none">
                     <IconPlay c="h-10 w-10" />
                   </div>
                   <div class="text-sm text-gray-500 ml-3">Play without recording.</div>
                 </div>

                 <div class="flex flex-row items-center">
                   <div class="flex-none">
                     <IconStop c="h-10 w-10" />
                   </div>
                   <div class="text-sm text-gray-500 ml-3">Stop playing or recording.</div>
                 </div>

                 <div class="flex flex-row items-center" v-if="allow_rewind">
                   <div class="flex-none">
                     <IconRewind c="h-10 w-10" />
                   </div>
                   <div class="text-sm text-gray-500 ml-3">Rewind to the start.</div>
                 </div>

                 <div class="flex flex-row items-center">
                   <div class="flex-none">
                     <IconHelp c="h-10 w-10" />
                   </div>
                   <div class="text-sm text-gray-500 ml-3">Show instructions.</div>
                 </div>

               </div>
             </div>
           </div>
         </div>
         <div class="mt-5 sm:mt-6">
           <button type="button" class="inline-flex justify-center w-full rounded-md border border-transparent shadow-sm px-4 py-2 bg-indigo-600 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:text-sm"
             @click="show_instruction_modal = false;"
             >
             Ok
           </button>
         </div>
       </div>
     </div>
   </div>
    <!-- Instruction step ** End -->
  </div>
</template>

<script>
import IconRecord from "@/shared/icons/iconrecord";
import IconPlay from "@/shared/icons/iconplay";
import IconStop from "@/shared/icons/iconstop";
import IconRewind from "@/shared/icons/iconrewind";
// import IconSettings from "./icons/iconsettings";
import IconHelp from "@/shared/icons/iconhelp";

export default {
  name: "ViewerPlayerControl",
  components: {
    IconRecord,
    IconPlay,
    IconStop,
    IconRewind,
    // IconSettings,
    IconHelp,
  },
  props: ['allow_recording', 'allow_playing', 'allow_rewind', 'controls', 
          'is_recording', 'is_playing', 'video_current_time'],

  data: function() {
    return {
      show_instruction_modal: false,
    };
  },

  methods: {
    on_error: function(msg, e) {
      this.$emit("error", {msg, e});
    }
  }
}
</script>
