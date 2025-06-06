<script setup lang="ts">
import { ref, watch } from 'vue';
import { useThemeVars } from 'naive-ui';

const themeVars = useThemeVars();
const props = defineProps({
    qrPreview: {
        type: String,
        required: true,
    },
    mode: {
        type: String,
        required: true,
    },
});

const emit = defineEmits(['update:value']);

const qrMode = ref<string | undefined>(props.mode);
const qrCode = ref(props.qrPreview);

watch(() => props.mode, () => qrMode.value = props.mode);
watch(() => props.qrPreview, () => qrCode.value = props.qrPreview);
watch(() => qrCode.value, () => emit('update:value', qrCode.value));
</script>

<template>
    <div class="container"
         :style="{
             '--rv-base-design': themeVars.primaryColor,
             '--rv-darkmode-transition': themeVars.cubicBezierEaseInOut
         }">
        <div class="qr"
             v-html="qrCode" />
    </div>
</template>

<style lang="sass" scoped>
.container
    justify-content: center
    display: flex
.qr
    justify-content: center
    display: flex
    width: 200px
    height: 200px
    text-align: center
    // border: 10px dashed var(--rv-base-design)

</style>