<script setup lang="ts">
import { NButton, NDivider, NSpace, useThemeVars } from 'naive-ui';
import Cookies from 'js-cookie';
import router from '@/router';

const themeVars = useThemeVars();

const emit = defineEmits<{
    'update:token': [value: undefined | string]
}>();

function switchQR() {
    emit('update:token', Cookies.get('login'));
    router.push('/');
}
</script>

<template>
    <div class="nav"
         :style="{
             '--rv--value-text-color': themeVars.textColor1,
             '--rv-darkmode-transition': themeVars.cubicBezierEaseInOut
         }">

        <n-space class="content"
                 vertical>
            <n-button text
                      @click="switchQR"
                      class="logout">QR-Code Generator</n-button>
            <n-divider />
            <h2 class="logout">Gespeicherte QR-Codes</h2>
            <n-button text
                      class="logout">Aktive</n-button>
            <n-button text
                      class="logout">Archivierte</n-button>
            <n-button text
                      class="logout">Auswertung</n-button>
            <n-divider />
            <n-button text
                      class="logout">Placeholder</n-button>

        </n-space>
    </div>
</template>

<style scoped lang="sass">
.content
    margin-top: 20px

.logout
    font-size: 20px 
    text-decoration: none
    color: var(--rv--value-text-color)
    transition: color .3s var(--rv-darkmode-transition)  
</style>

