<script setup lang="ts">
import { NLayout, NLayoutContent, NLayoutHeader } from 'naive-ui';
import Cookies from 'js-cookie';
import DynamicWebsiteHeader from '@/views/dynamic/DynamicWebsiteHeader.vue'
import { RouterView } from 'vue-router';
import { onMounted } from 'vue';
import router from '@/router';
import { store } from '@/stores';


</script>

<template>
    <n-layout class="layout">
        <n-layout-header class="head">
            <dynamic-website-header />
        </n-layout-header>

        <n-layout-content class="body">
            <router-view name="default"
                         class="form" />
        </n-layout-content>
    </n-layout>
</template>

<style lang="sass">

</style>