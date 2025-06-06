<script setup lang="ts">
import { NButton, NUpload, useNotification, useThemeVars } from 'naive-ui';
import { ref, watch } from 'vue';
import Cookies from 'js-cookie';
import type { FileInfo } from 'naive-ui/es/upload/src/interface';
import { store } from '@/stores';

const alert = useNotification();
const themeVars = useThemeVars();


const upload = ref<InstanceType<typeof NUpload> | null>(null);
const fileList = ref<FileInfo[]>([]);
const token = ref(Cookies.get('login'));

const userIcon = ref(store.useUserInfo.usericon);

function sendingDataToAPI() {
    if (fileList.value[0]?.file === undefined) return alert.create({
        title: 'Benutzerbild konnte nicht geändert werden',
        duration: 5000,
    });

    const request = new XMLHttpRequest();
    request.responseType = 'json';
    request.open('PUT', '/api/change-icon', true);
    request.setRequestHeader('Content-Type', 'application/json');
    token.value && request.setRequestHeader('Authorization', token.value);

    request.onload = () => {
        if (request.readyState !== request.DONE || request.status !== 200) return alert.create({
            title: 'Benutzerbild konnte nicht geändert werden',
            duration: 5000,
        });
        fileList.value = [];
        userIcon.value && store.useUserInfo.updateIcon(userIcon.value);
    };

    request.onerror = () => {
        alert.create({
            title: 'Benutzerbild konnte nicht geändert werden',
            duration: 5000,
        });
    };
    request.send(JSON.stringify({ 'new-icon': userIcon.value }));
}


watch(() => fileList.value[0]?.file, () => {
    if (fileList.value[0]?.file) {
        const reader = new FileReader();
        reader.onloadend = () => {
            userIcon.value = reader.result as string;
            sendingDataToAPI();
        };
        reader.readAsDataURL(fileList.value[0]?.file);
    }
});
</script>

<template>
    <n-upload class="upload"
              ref="upload"
              :show-file-list="false"
              :default-upload="false"
              list-type="image"
              directory-dnd
              :max="1"
              v-model:file-list="fileList">
        <n-button class="button"
                  :color="themeVars.primaryColor">Benutzericon ändern</n-button>
    </n-upload>
</template>

<style scoped lang="sass">
.button
    color: #FFF
    display: flex
    justify-content: center
    align-items: center
</style>