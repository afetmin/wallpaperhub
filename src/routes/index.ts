import { createRouter, createWebHistory } from 'vue-router';
import Settings from '../pages/Settings.vue';
import Home from '../pages/Home.vue';

const routes = [
    {
        path: '/',
        name: 'Home',
        component: Home,
    },
    {
        path: '/settings',
        name: 'Settings',
        component: Settings,
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
