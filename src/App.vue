<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import {
    Sidebar,
    SidebarContent,
    SidebarInset,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarProvider,
} from '@/components/ui/sidebar';

const greetMsg = ref('');
const name = ref('');

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg.value = await invoke('greet', { name: name.value });
}
</script>

<template>
    <SidebarProvider>
        <Sidebar>
            <SidebarContent>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton>
                            <router-link to="/">Home</router-link>
                        </SidebarMenuButton>
                        <SidebarMenuButton>
                            <router-link to="/about">About</router-link>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarContent>
        </Sidebar>

        <SidebarInset class="bg-gray-100">
            <header
                class="flex h-16 shrink-0 items-center bg-white gap-2 border-b px-4 sticky top-0 z-50"
            >
                <Separator orientation="vertical" class="mr-2 h-4" />
            </header>

            <div class="flex flex-1 flex-col gap-4 p-4">
                <router-view></router-view>
            </div>
        </SidebarInset>
    </SidebarProvider>
</template>
