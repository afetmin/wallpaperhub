<script setup lang="ts">
import {
    Sidebar,
    SidebarContent,
    SidebarInset,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarProvider,
} from '@/components/ui/sidebar';
import { computed } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const isSettingsPage = computed(() => route.path === '/settings');
</script>

<template>
    <SidebarProvider>
        <Sidebar>
            <SidebarContent>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton class="text-lg">
                            <router-link to="/">Home</router-link>
                        </SidebarMenuButton>
                        <SidebarMenuButton class="text-md">
                            <router-link to="/settings">Settings</router-link>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarContent>
        </Sidebar>

        <SidebarInset class="bg-gray-100">
            <header
                class="flex h-16 shrink-0 items-center bg-white gap-2 border-b px-4 sticky top-0 z-50"
                v-show="!isSettingsPage"
            >
                <Separator orientation="vertical" class="mr-2 h-4" />
            </header>

            <div class="flex flex-1 flex-col gap-4 p-4">
                <router-view></router-view>
            </div>
        </SidebarInset>
    </SidebarProvider>
</template>
