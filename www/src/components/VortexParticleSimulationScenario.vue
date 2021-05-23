<template>
  <div class="flex flex-col rounded-lg shadow-lg overflow-hidden">
    <div class="flex-shrink-0">
      <img class="h-48 w-full object-cover" :src="imageUrl" alt="" />
    </div>
    <div class="flex-1 bg-gray-200 p-6 flex flex-col justify-between">
      <div class="flex-1">
        <!--
        <p class="text-sm font-medium text-indigo-600">
          <a :href="post.category.href" class="hover:underline">
            {{ post.category.name }}
          </a>
        </p>
-->
        <a href="#" class="block mt-2" @click="$emit('select')">
          <p class="text-xl font-semibold text-gray-900">
            {{ title }}
          </p>
          <p class="mt-3 text-base text-gray-500">
            {{ description }}
          </p>
        </a>
      </div>
      <div class="mt-6 flex items-center">
        <div class="flex-shrink-0">
          <!--
          <a :href="post.author.href">
-->
          <span class="sr-only">{{ author }}</span>
          <!--
            <img class="h-10 w-10 rounded-full" :src="post.author.imageUrl" alt="" />
          </a>
-->
        </div>
        <div class="ml-3">
          <p class="text-sm font-medium text-gray-900">
            <!--
            <a :href="post.author.href" class="hover:underline">
-->
            {{ author }}
            <!--
            </a>
-->
          </p>
          <!--
          <div class="flex space-x-1 text-sm text-gray-500">
            <time :datetime="post.datetime">
              {{ post.date }}
            </time>
            <span aria-hidden="true">
              &middot;
            </span>
            <span> {{ post.readingTime }} read </span>
          </div>
-->
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "VortexParticleSimulationScenario",
  props: ["imageUrl", "title", "description", "author"],
};
</script>
