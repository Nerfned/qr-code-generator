<script lang="ts" setup>
import { NButton, useThemeVars } from 'naive-ui';
import { ref, watch } from 'vue';
const themeVars = useThemeVars();

const props = defineProps({
    value: {
        required: true,
        type: String,
    },
    title: {
        required: true,
        type: String,
    },
});

const qrcode = ref(props.value);
const title = ref(props.title);

watch(() => props.value, () => qrcode.value = props.value);
watch(() => props.title, () => title.value = props.title);

const download = () => {
    downloadFile(qrcode.value);
};

function downloadFile(content: string) {
    const blob = new Blob([content], { type: 'image/svg+xml' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = title.value;
    link.click();
}
</script>

<template>
    <n-button @click="download"
              :color="themeVars.primaryColor"
              class="download">Download</n-button>
</template>

<style lang="sass" scoped>
.download
  color: #fff
  min-width: 115px
  height: 40px
</style> 