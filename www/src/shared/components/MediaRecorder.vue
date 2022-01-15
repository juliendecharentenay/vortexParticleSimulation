<template>
  <div>
    <!-- Download recording -->
   <div class="fixed z-10 inset-0 overflow-y-auto" aria-labelledby="modal-title" role="dialog" aria-modal="true" v-if="recording_url !== null">
     <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
       <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" aria-hidden="true"></div>

       <!-- This element is to trick the browser into centering the modal contents. -->
       <span class="hidden sm:inline-block sm:align-middle sm:h-screen" aria-hidden="true">&#8203;</span>

       <div class="inline-block align-bottom bg-white rounded-lg px-4 pt-5 pb-4 text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-sm sm:w-full sm:p-6">
         <div>
           <div class="mt-3 text-center sm:mt-5">
             <h3 class="text-lg leading-6 font-medium text-gray-900" id="modal-title">
               Recording
             </h3>
             <div class="mt-2">
               <p class="text-sm text-gray-500">
                 <a :href="recording_url" 
                    :download="filename"
                    :name="filename"
                    target="_self"
                    >Download recording</a>
               </p>
             </div>
           </div>
         </div>
         <div class="mt-5 sm:mt-6">
           <button type="button" class="inline-flex justify-center w-full rounded-md border border-transparent shadow-sm px-4 py-2 bg-indigo-600 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:text-sm"
             @click="close_recording();"
             >
             Close
           </button>
         </div>
       </div>
     </div>
   </div>
    <!-- Download recording ** End -->

  </div>
</template>
<script>
import GIF from "gif.js.optimized";

class MyGifRecorder {
  #count_frames;
  #frame_blobs;
  #frame_images;
  #state;
  #on_complete;
  #on_error;
  #width;
  #height;
  #last_time;

  constructor() {
    this.#count_frames = 0;
    this.#frame_images = [];
    this.#state = 0;
    this.#on_complete = null;
    this.#on_error = null;
    this.#width = null;
    this.#height = null;
    this.#last_time = null;
  }

  on_error(f) {this.#on_error = f; return this;}
  on_complete(f) {this.#on_complete = f; return this;}

  capture(canvas) {
    if (this.#width === null) {this.#width = canvas.width;}
    if (this.#height === null) {this.#height = canvas.height;}
    const t = new Date().getTime();
    var delay = 33;
    if (this.#last_time !== null) {delay = t - this.#last_time;}
    this.#last_time = t;
    this.#count_frames += 1;
    canvas.toBlob((b) => {
      try {
        var url = URL.createObjectURL(b);
        var img = new Image();
        img.width = this.#width;
        img.height = this.#height;
        img.src = url;
        this.#frame_images.push({url, img, delay});
        this.convert_to_gif()
      } catch (e) {
        this.handle_on_error("Error in MyGifRecorder::capture", e);
      }
    });
  }

  render() {
    this.#state = 1;
    this.convert_to_gif()
  }

  convert_to_gif() {
    try {
      if (this.#state === 1 && this.#frame_images.length >= this.#count_frames) {
        this.#state = 2;
        var gifjs = new GIF({workers: 2, quality: 10, workerScript: 'js/gif.worker/gif.worker.js'});
        gifjs.on('finished', (blob) => {
          try {
            for (var f of this.#frame_images) { URL.revokeObjectURL(f.url); }
            this.#frame_images = null;
            if (this.#on_complete) { this.#on_complete(blob); }
          } catch(e) {
            this.handle_on_error("Error in MyGifRecorder::convert_to_gif:gifjs.on_finished", e);
          }
        });

        if (this.#frame_images.length > 1) {
          this.#frame_images[0].delay = this.#frame_images[1].delay; // The first frame delay is set arbitrarily
        }

        for (var i of this.#frame_images) {
          if ('delay' in i) {
            gifjs.addFrame(i.img, {delay: i.delay});
          } else {
            gifjs.addFrame(i.img);
          }
        }

        gifjs.render();
      }
    } catch (e) {
      this.handle_on_error("Error in MyGifRecorder::convert_to_gif", e);
    }
  }

  handle_on_error(msg, e) {
    if (this.#on_error !== null) {
      this.#on_error(msg, e);
    } else {
      throw e;
    }
  }
}

export default {
  name: "MediaRecorder",
  components: {},
  props: {},
  data: function() {
    return {
      mediarecorder: null,
      recording_url: null,
      media_type: null,
      mediarecorder_filename: null,
      filename: null,
      my_recorder: null,
    };
  },
  mounted: function() {
    if (typeof MediaRecorder.isTypeSupported == 'function') {
      if (MediaRecorder.isTypeSupported("video/mp4")) {
        this.media_type = "video/mp4"; this.mediarecorder_filename = "video.mp4";
      } else if (MediaRecorder.isTypeSupported("video/webm")) {
        this.media_type = "video/webm"; this.mediarecorder_filename = "video.webm";
      } else {
        this.on_error("Unable to find supported video type", null);
      }
    } else {
      this.media_type = null; this.mediarecorder_filename = "video.mp4";
    }
  },

  methods: {
    record_using_mediarecorder: function(stream) {
      try {
        try {
          this.mediarecorder = new MediaRecorder(stream, { mimeType: "video/webm; codecs=vp9" });
        } catch (e) {
          this.mediarecorder = new MediaRecorder(stream, { mimeType: "video/webm" });
        }
        var chunks = [];
        this.mediarecorder.ondataavailable = (evt) => { 
              try {
                if (evt.data && evt.data.size > 0) {chunks.push(evt.data);} 
              } catch (e) {
                this.on_error("Error in mediarecorder.ondataavailable", e);
              }
        };
        this.mediarecorder.onstop = () => {
              try {
                this.$emit("on_processing", false);
                var blob = new Blob(chunks); // (this.media_type === null ? new Blob(chunks, {type: "video/mp4"}) : new Blob(chunks, {type: this.media_type }));
                this.recording_url = URL.createObjectURL(blob);
                this.filename = this.mediarecorder_filename;
              } catch (e) {
                this.on_error("Error in mediarecorder.onstop", e);
              }
        };
        this.mediarecorder.start();
      } catch (e) {
        this.on_error("Error in record_using_mediarecorder", e);
      }
    },

    record_manually: function() {
      try {
        this.my_recorder 
          = new MyGifRecorder()
            .on_complete((blob) => {
               try {
                 this.$emit("on_processing", false);
                 this.recording_url = URL.createObjectURL(blob);
                 this.filename = "video.gif";
               } catch (e) {
                 this.on_error("Error in record_manually:on_complete", e);
               }
            })
            .on_error(this.on_error);
      } catch (e) {
        this.on_error("Error in record_manually", e);
      }
    },

    record: function(stream) {
      try {
        const videoTrack = (stream.getVideoTracks().length === 1 ? stream.getVideoTracks()[0] : null);
        var use_mediarecorder = (videoTrack !== null) 
         && ((('getCapabilities' in videoTrack) && ('deviceId' in videoTrack.getCapabilities()))
             ||  ((! ('getCapabilities' in videoTrack)) && ('id' in videoTrack)) 
            );
        if (use_mediarecorder) {
          this.record_using_mediarecorder(stream);
        } else {
          this.record_manually();
        }
        
      } catch (e) {
        this.on_error("Error in record", e);
      }
    },

    capture: function(canvas) {
      try {
        if (this.my_recorder !== null) {this.my_recorder.capture(canvas);}
      } catch (e) {
         this.on_error("Error in capture", e);
      }
    },

    stop: function() {
      try {
        if ((this.mediarecorder !== null) || (this.my_recorder !== null)) {
          this.$emit("on_processing", true);
        }
        if (this.mediarecorder !== null) { this.mediarecorder.stop(); }
        if (this.my_recorder !== null) { this.my_recorder.render(); 
        }
      } catch (e) {
        this.on_error("Error in stop", e);
      }
    },

    close_recording: function() {
      try {
        URL.revokeObjectURL(this.recording_url);
        this.recording_url = null;
        this.mediarecorder = null;
        this.my_recorder = null;
      } catch (e) {
        this.on_error("Error in close_recording", e);
      }
    },

    on_error: function(msg, e) {
      this.$emit("error", {msg, e});
    }
  }
}
</script>

