<template>
    <div
        class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 auto-rows-[minmax(auto,_max-content)]"
    >
        <div
            class="bg-white rounded-lg shadow-xl overflow-hidden max-w-lg w-full"
            v-for="img in imgList"
        >
            <div class="overflow-hidden">
                <img
                    class="cursor-pointer aspect-video w-full object-cover hover:scale-105 transition-transform duration-200"
                    :src="img.url"
                    :alt="img.title"
                />
            </div>
            <div class="p-6">
                <h2 class="cursor-default text-xl font-bold text-gray-800 mb-2">
                    {{ img.title }}
                </h2>
                <a
                    href="#"
                    @click.prevent="openLink(img.copyrightlink)"
                    class="hover:text-blue-400 text-gray-700 leading-tight mb-4 cursor-pointer"
                >
                    {{ img.copyright }}
                </a>
            </div>
        </div>
    </div>
    <div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min" />
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface BingImage {
    url: string;
    urlbase: string;
    copyright: string;
    copyrightlink: string;
    title: string;
}

const imgList = ref<BingImage[]>([]);

async function fetchImages() {
    imgList.value = await invoke('get_img_list', { number: 20 });
}
function openLink(url: string) {
    invoke('open_url', { url });
}

onMounted(() => {
    fetchImages();
});
</script>
