<script lang="ts" setup>
import { NButton, useThemeVars } from 'naive-ui';

// import { NTabs, NFlex, NTabPane, NCard, NButton, NModal, NSplit, useThemeVars, useNotification } from 'naive-ui'
// import { ref, watch, } from 'vue';

const themeVars = useThemeVars();
// const notification = useNotification();

// const props = defineProps({
//     qrcodeId: {
//         required: true,
//         type: Number
//     },
// });

// const emit = defineEmits<{
//     'update:archivedStatus': [value: boolean]
// }>();
</script>

<template>
    <n-button disabled
              :color="themeVars.primaryColor"
              class="archiv">Archivieren</n-button>
</template>

<style lang="sass" scoped>

.editor
    height: fit-content
.archiv
  color: #fff
  min-width: 115px
  height: 40px
.tabs
    padding-right: 14px

.preview
    justify-content: center
    display: flex

.qr-preview
    margin-left: 19px

</style> 