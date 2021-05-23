<!-- This example requires Tailwind CSS v2.0+ -->
<template>
  <nav class="flex" aria-label="Breadcrumb">
    <ol class="px-6 flex space-x-4 items-center">
      <li class="">
        <div>
          <a
            href="#"
            class="text-gray-400 hover:text-gray-500"
            @click="home.onClick()"
          >
            <HomeIcon class="flex-shrink-0 h-5 w-5" aria-hidden="true" />
            <span class="sr-only">Home</span>
          </a>
        </div>
      </li>
      <li v-for="page in pages" :key="page.name">
        <div class="flex items-center">
          <ChevronRightIcon
            class="flex-shrink-0 h-5 w-5 text-gray-400"
            aria-hidden="true"
          />
          <a
            :href="page.href"
            class="ml-4 text-sm font-medium text-gray-500 hover:text-gray-700"
            :aria-current="page.current ? 'page' : undefined"
            @click="page.onClick()"
          >
            {{ page.name }}
          </a>
        </div>
      </li>
    </ol>
  </nav>
</template>

<script>
// import { ChevronRightIcon, HomeIcon } from '@heroicons/vue/solid';
import HomeIcon from "@/components/heroicons/homeiconsolid.vue";
import ChevronRightIcon from "@/components/heroicons/ChevronRightIconSolid.vue";

export default {
  name: "Breadcrumb",
  props: ["home", "pages"],
  components: {
    ChevronRightIcon,
    HomeIcon,
  },
};
</script>
