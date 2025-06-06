<script setup lang="ts">
import { NCard, useThemeVars } from 'naive-ui';
import HubQrContent from '@/views/hub/components/HubQrContent.vue';

const themeVars = useThemeVars();
</script>

<template>
    <div class="nav"
         :style="{
             '--rv--value-text-color': themeVars.textColor1,
             '--rv-darkmode-transition': themeVars.cubicBezierEaseInOut
         }">
        <n-card class="field"
                color="#0fa5bd">
            <hub-qr-content />
        </n-card>

    </div>
</template>

<style scoped lang="sass">
.field
    height: 200vh
    width: max-content
    
.content
    margin-top: 20px
</style>

