<script setup lang="ts">
import { NButton, NCard, useThemeVars } from 'naive-ui';
import { ref, watch } from 'vue';
import { RouterView } from 'vue-router';
import router from '@/router';
import { store } from '@/stores';

const themeVars = useThemeVars();
const travelguide = ref(false);
const activUserName = ref<string | undefined>(undefined);

function backToLogin() {
    travelguide.value = true;
    router.replace('/login');
}

function backToGenerator() {
    router.replace('/');
}

watch(() => activUserName.value, () => { activUserName.value != undefined ? travelguide.value = true : travelguide.value = false; });
</script>

<template>
    <n-card class="field"
            :style="{
                '--rv-text-color': themeVars.textColor1,
            }">
        <div class="head">
            <transition name="slide-fade">
                <div v-if="travelguide">
                    <n-button class="back-port"
                              text
                              @click="backToLogin">
                        <b>&#9664; Zurück zum Login</b>
                    </n-button>
                </div>
                <div v-else>
                    <n-button class="back-port"
                              text
                              @click="backToGenerator">
                        <b>&#9664; Zurück zum QR-Code Generator</b>
                    </n-button>
                </div>
            </transition>

            <div v-if="store.layout.darkMode">
                <img class="title"
                     src="@/assets/logo-mw.svg"
                     width="200"
                     height="30" />
            </div>
            <div v-else>
                <img class="title"
                     src="@/assets/logo-mw-color.svg"
                     width="200"
                     height="30" />
            </div>

            <div class="greeting"><b>Anmeldung</b></div>
        </div>

        <transition name="slide-fade">
            <router-view name="default"
                         class="form"
                         v-model:userName="activUserName" />
        </transition>

    </n-card>
</template>

<style scoped lang="sass">
.field
    width: 450px
    position: absolute
    top: 50%
    left: 50%
    transform: translate(-50%, -50%)
    box-shadow: 10px 10px 5px 12px #0202024f
    min-height: 471px

.title
    margin-top: 20px
.head
    text-align: center
    font-size: 20px
    margin-bottom: 50px
.qr
    text-decoration: none
    color: var(--rv-text-color)
    margin-top: 20px
.back-port
    font-size: 24px
</style>

