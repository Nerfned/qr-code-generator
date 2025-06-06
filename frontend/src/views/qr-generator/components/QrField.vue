<script setup lang="ts">

import { NCard, NFlex, useThemeVars } from 'naive-ui';
import DownloadButton from './DownloadButton.vue';
import SavingButton from './SavingButton.vue';

const themeVars = useThemeVars();

defineProps({
    qrSvg: {
        type: String,
        required: true,
    },
    qrData: {
        type: String,
        required: true,
    },
    qrLogo: {
        type: Blob,
        required: false,
    },
});
</script>

<template>
    <n-card class="right-field">
        <n-flex class="qr-field"
                justify="center"
                :style="{
                    '--rv-base-design': themeVars.primaryColor,
                    '--rv-darkmode-transition': themeVars.cubicBezierEaseInOut
                }">
            <div class="qr"
                 v-html="qrSvg" />
            <download-button class="download"
                             :qrSvg="qrSvg" />
            <saving-button class="download"
                           :qrcode="qrData"
                           :qrLogo="qrLogo" />
        </n-flex>
    </n-card>
</template>

<style lang="sass" scoped>
.right-field
    justify-content: center
    display: flex
    width: 270px
    box-shadow: 22px 10px 5px 12px #0202024f

.qr
    width: 200px
    height: 200px
    text-align: center
    border: 10px dashed var(--rv-base-design)

.download
    margin-left: 0px
    color: #fff
    justify-content: center
    display: flex
</style>