<script setup lang="ts">
import { NLayout, NLayoutContent, NLayoutHeader } from 'naive-ui';
import Cookies from 'js-cookie';
import MainBody from './components/MainBody.vue';
import WebsiteHeader from './components/WebsiteHeader.vue';
import { onMounted } from 'vue';
import { store } from '@/stores';


// Function when element load 
onMounted(() => {
    sessionReader(Cookies.get('login'));
});

function sessionReader(sessionToken: string | undefined) {
    if (sessionToken === undefined) return false;
    const request = new XMLHttpRequest();
    request.responseType = 'json';
    request.open('PUT', '/api/send-data', true);
    request.setRequestHeader('Content-Type', 'application/json');

    sessionToken && request.setRequestHeader('Authorization', sessionToken);

    request.onload = () => {
        if (request.status === 409) return false;
        if (request.readyState !== request.DONE || request.status !== 200) return;
        const answer = request.response;
        store.useUserInfo.updateName(answer.username);
        store.useUserInfo.updateEmail(answer.email);
        store.useUserInfo.updateIcon(answer.icon);
        store.useUserInfo.updateStatusLogin(true);
    };
    request.send();
}
</script>

<template>
    <n-layout class="layout">
        <n-layout-header class="head">
            <website-header />
        </n-layout-header>
        <transition name="fade">
            <n-layout-content class="body">
                <main-body />
            </n-layout-content>
        </transition>
    </n-layout>
</template>

<style lang="sass">

</style>