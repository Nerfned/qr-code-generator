<script setup lang="ts">
import { NAvatar, NFlex, NUpload, type UploadFileInfo } from 'naive-ui';
import { ref, watch } from 'vue';
import logo from '@/assets/logo.svg';
import user from '@/assets/user.svg';

const emit = defineEmits<{
    'update:icon': [value: string]
}>();

const upload = ref<InstanceType<typeof NUpload> | null>(null);
const fileList = ref<UploadFileInfo[]>([]);
const placeholderIcon = ref(user);

const userIcon = ref();

watch(() => fileList.value[0]?.file, () => {
    if (fileList.value[0]?.file) {
        userIcon.value = fileList.value[0].file;
        emit('update:icon', userIcon.value);
        const reader = new FileReader();
        reader.onloadend = () => {
            const icon = reader.result as string;
            placeholderIcon.value = icon;
            emit('update:icon', placeholderIcon.value);
            fileList.value = [];
        };

        reader.readAsDataURL(fileList.value[0]?.file);
    }
});
</script>

<template>
    <div class="text">
        <h1>Legen Sie Ihr Profilbild fest</h1>
        <h4>Profilbild kann jederzeit geändert werden</h4>
    </div>
    <n-flex justify="center">

        <n-upload class="upload"
                  justify-content="center"
                  accept="image/*"
                  ref="upload"
                  :show-file-list="false"
                  :default-upload="false"
                  list-type="image"
                  directory-dnd
                  :max="2"
                  v-model:file-list="fileList">

            <n-avatar round
                      :size="150"
                      :src="placeholderIcon"
                      :fallback-src="logo" />
        </n-upload>
    </n-flex>
</template>

<style scoped lang="sass">
.text
    text-align: center
    margin-bottom: 25px

.upload
    margin-bottom: 25px
    display: flex
</style>