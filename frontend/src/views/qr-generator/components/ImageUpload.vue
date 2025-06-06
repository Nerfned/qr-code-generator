<script setup lang="ts">
import { NDivider, NFlex, NIcon, NUpload, NUploadDragger } from 'naive-ui';
import type { FileInfo } from 'naive-ui/es/upload/src/interface';
import { ref } from 'vue';

const emit = defineEmits<{
    'update:file': [value?: Blob]
}>();

const upload = ref<InstanceType<typeof NUpload> | null>(null);
const fileList = ref<FileInfo[]>([]);

let sendTimeout: number | undefined = void 0;
clearTimeout(sendTimeout);

function sendingDataToAPI() {
    sendTimeout = window.setTimeout(() => {
        emit('update:file', fileList.value[0]?.file ?? void 0);
    }, 300);
}
</script>

<template>
    <n-flex vertical>
        <n-divider title-placement="left">
            Logo einfügen
        </n-divider>
        <n-upload class="upload"
                  ref="upload"
                  @change="sendingDataToAPI"
                  :default-upload="false"
                  list-type="image"
                  directory-dnd
                  :max="1"
                  v-model:file-list="fileList">

            <n-upload-dragger>
                <div style="margin-bottom: 12px">
                    <n-icon />
                </div>
                <n-text style="font-size: 16px">
                    Klicken oder ziehen Sie ihr Bild in das Feld zum Hochladen
                </n-text>
            </n-upload-dragger>

        </n-upload>
        <template #header-extra>
            + &nbsp;
        </template>
    </n-flex>
</template>

<style scoped lang="sass">
</style>