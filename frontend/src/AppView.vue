<script setup lang="ts">
import { NConfigProvider, NDialogProvider, NFlex, useThemeVars } from 'naive-ui';
import { computed, reactive } from 'vue';
import type { GlobalThemeOverrides } from 'naive-ui';
import { RouterView } from 'vue-router';
import { store } from './stores';

const themeVars = useThemeVars();

const background = computed(() => store.layout.darkMode ? `linear-gradient(45deg,#083549 0%,#05554a 100%)` : 'linear-gradient(45deg, #d53369 0%, #daae51 100%)');

const style = reactive({
    background,
});

const lightThemeOverrides: GlobalThemeOverrides = {
    common: {
        primaryColor: '#EAA342',
    },
};

const darkThemeOverrides: GlobalThemeOverrides = {
    common: {
        primaryColor: '#0fa5bd',
    },
    Button: {
        color: '#0fa5bd',
        colorHover: '#00e65ab8',
        colorFocus: '#00e65ab8',
    },
};
</script>

<template>
    <div :style="{
        '--rv-transition': themeVars.cubicBezierEaseInOut,
    }">
        <n-config-provider :theme-overrides="store.layout.darkMode === null ? lightThemeOverrides : darkThemeOverrides"
                           :theme="store.layout.darkMode">
            <n-dialog-provider>
                <n-flex class="page"
                        :style="style"
                        justify="space-between">
                    <transition name="fade">
                        <router-view name="main" />
                    </transition>
                </n-flex>
            </n-dialog-provider>
        </n-config-provider>
    </div>
</template>

<style lang="sass">
.page
    height: 100vh
    width: 100vw

.layout
    background-color: #00000000
    height: 100vh

.head  
    font-family: system-ui, -pple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif

.body 
    width: 100vw
    height: 91.5vh
    background-color: #00000000
    transition: color .5s var(--rv-transition)

.fade-enter-active, .fade-leave-active .slide-fade-leave-active 
    transition: opacity 0.8s var(--rv-transition)

.fade-enter-from 
    opacity: 0

.slide-fade-enter-active 
    transition: all 0.5s ease-out
 
.slide-fade-enter-from, .slide-fade-leave-to
    transform: translateX(20px)
    opacity: 0
</style>



