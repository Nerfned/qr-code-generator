<script setup lang="ts">
import { NButton, NFlex, NIcon, NUpload, NUploadDragger, NUploadTrigger } from 'naive-ui';
import { ref, watch } from 'vue';
import type { FileInfo } from 'naive-ui/es/upload/src/interface';

const props = defineProps({
    file: {
        type: String,
        required: false,
    },
});


const emit = defineEmits<{

    'update:file': [value?: String],
}>();

const upload = ref<InstanceType<typeof NUpload> | null>(null);
const fileList = ref<FileInfo[]>([]);
const logo = ref(props.file);

const sendTimeout: number | undefined = void 0;
clearTimeout(sendTimeout);

function deleteLogo() {
    logo.value = undefined;
    fileList.value = [];
    emit('update:file', logo.value);
}

watch(() => fileList.value[0]?.file, () => {
    if (fileList.value[0]?.file) {
        // logo.value = fileList.value[0].file;

        const reader = new FileReader();
        reader.onloadend = () => {
            logo.value = reader.result as string;
            emit('update:file', logo.value);
        };

        reader.readAsDataURL(fileList.value[0]?.file);
    }
});
</script>

<template>
    <n-flex class="upload"
            vertical>
        <n-upload abstract
                  ref="upload"
                  :default-upload="false"
                  list-type="image"
                  directory-dnd
                  :max="1"
                  v-model:file-list="fileList">
            <n-flex justify="center">

                <n-upload-trigger>
                    <n-button class="remove"
                              @click="deleteLogo">Logo einfügen</n-button>
                </n-upload-trigger>

                <n-button class="remove"
                          @click="deleteLogo">Logo entfernen</n-button>
            </n-flex>

            <n-upload-trigger>
                <n-upload-dragger>
                    <div style="margin-bottom: 12px">
                        <n-icon />
                    </div>
                    <n-text style="font-size: 16px">
                        Klicken oder ziehen Sie ihr Bild in das Feld zum Hochladen
                    </n-text>
                </n-upload-dragger>
            </n-upload-trigger>

            <template #header-extra>
                + &nbsp;
            </template>
        </n-upload>
    </n-flex>
</template>

<style scoped lang="sass">
.remove
    margin-top: 20px

.upload
    margin-left: 19px

</style>