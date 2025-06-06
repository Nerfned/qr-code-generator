<script setup lang="ts">
import { NGrid, NGridItem, useThemeVars } from 'naive-ui';
import HubShowQr from './HubShowQr.vue';
import HubSidebar from './HubSidebar.vue';

const themeVars = useThemeVars();
</script>

<template>
    <div class="nav"
         :style="{
             '--rv--value-text-color': themeVars.textColor1,
             '--rv-darkmode-transition': themeVars.cubicBezierEaseInOut
         }">

        <n-grid cols="5"
                item-responsive
                responsive="screen">
            <n-grid-item span="0 m:0 l:1">
                <hub-sidebar />
            </n-grid-item>

            <n-grid-item class="content"
                         span="5 m:5 l:4">
                <hub-show-qr class="content" />
            </n-grid-item>

        </n-grid>

    </div>
</template>

<style scoped lang="sass">
.n-card
    color: #1e1d1d
.logout
    font-size: 20px 
    text-decoration: none
    color: var(--rv--value-text-color)
    transition: color .3s var(--rv-darkmode-transition)  
</style>

