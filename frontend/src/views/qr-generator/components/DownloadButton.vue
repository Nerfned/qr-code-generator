<script lang="ts" setup>
import { NButton, useThemeVars } from 'naive-ui';

const themeVars = useThemeVars();

const props = defineProps({
    qrSvg: {
        required: true,
        type: String,
    },
});

function download() {
    downloadFile(props.qrSvg);
}

function downloadFile(content: string) {
    const blob = new Blob([content], { type: 'image/svg+xml' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = 'QR-Code';
    link.click();
}
</script>

<template>
    <n-button @click="download"
              size="large"
              :color="themeVars.primaryColor"
              class="download">Download
    </n-button>
</template>

<style lang="sass" scoped>
.n-button 
    width: 200px 
    height: 75px 
    margin-top: 25px 
    color: #fff
</style>
