<script setup lang="ts">
import { NLayout, NLayoutContent, NLayoutHeader } from 'naive-ui';
import Cookies from 'js-cookie';
import { RouterView } from 'vue-router';
import WebsiteHeaderHub from './components/WebsiteHeaderHub.vue';
import { onMounted } from 'vue';
import router from '@/router';
import { store } from '@/stores';

onMounted(() => {
    sessionReader(Cookies.get('login'));
});

function sessionReader(sessionToken: string | undefined) {
    if (sessionToken === undefined) return router.push('/');

    const request = new XMLHttpRequest();
    request.responseType = 'json';
    request.open('PUT', '/api/send-data', true);
    request.setRequestHeader('Content-Type', 'application/json');

    sessionToken && request.setRequestHeader('Authorization', sessionToken);

    request.onload = () => {
        if (request.readyState !== request.DONE || request.status !== 200) return router.push('/');
        const answer = request.response;

        if (answer.email === undefined || answer.username === undefined) return router.push('/');
        store.useUserInfo.updateName(answer.username);
        store.useUserInfo.updateEmail(answer.email);
        store.useUserInfo.updateIcon(answer.icon);

        if (store.useUserInfo.username != router.currentRoute.value.params.user) router.replace(`/hub/${store.useUserInfo.username}`);
    };

    request.send();
}
</script>

<template>
    <n-layout class="layout">
        <n-layout-header class="head">
            <website-header-hub />
        </n-layout-header>

        <n-layout-content class="body">
            <router-view name="default"
                         class="form" />
        </n-layout-content>
    </n-layout>
</template>

<style lang="sass">

</style>