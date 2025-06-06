<script setup lang="ts">
import { NNotificationProvider, useThemeVars } from 'naive-ui';
import AppView from './AppView.vue';

const themeVars = useThemeVars();
</script>

<template>
    <div :style="{
        '--rv-transition': themeVars.cubicBezierEaseInOut,
    }">
        <n-notification-provider placement="top"
                                 :max=1>
            <app-view />
        </n-notification-provider>
    </div>
</template>




