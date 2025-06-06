<script setup lang="ts">
import { NButton, NCard, NFlex, useNotification, useThemeVars } from 'naive-ui';
import { onMounted, ref, watch } from 'vue';
import Cookies from 'js-cookie';
import HubArchivButton from './HubArchivButton.vue';
import HubDeleteQr from './HubDeleteQr.vue';
import HubDownloadButton from './HubDownloadButton.vue';
import HubEditButton from './HubEditButton.vue';
import router from '@/router';
import { store } from '@/stores';

const themeVars = useThemeVars();
const alert = useNotification();

const reload = ref(false);
const qrcodes = ref();
const token = ref(Cookies.get('login'));

onMounted(() => {
    token.value !== undefined && GettingQrcodesFromDatabase();
});

watch(() => reload.value, () => {
    reload.value && GettingQrcodesFromDatabase();
    reload.value = false;
});

function GettingQrcodesFromDatabase() {
    if (token.value === undefined) {
        return alert.create({
            title: 'Fehler beim Lesen',
            duration: 5000,
        });
    }

    const request = new XMLHttpRequest();
    request.responseType = 'json';
    request.timeout = 10000;
    request.open('PUT', '/api/load-qr', true);
    request.setRequestHeader('Content-Type', 'application/json');
    request.setRequestHeader('Authorization', token.value);

    request.onload = () => {
        if (request.readyState !== request.DONE || request.status !== 200) {
            return alert.create({
                title: 'Verbindungsversuch abgebrochen',
                duration: 5000,
            });
        }
        const answer = request.response;

        qrcodes.value = answer.qrcodes;
    };

    request.onerror = () => {
        alert.create({
            title: 'Verbindungsversuch fehlgescheitert',
            duration: 5000,
        });
    };

    request.send();
}

function switchQR() {
    router.push('/');
}
</script>

<template>
    <div :style="{
        '--rv-text-color': themeVars.textColor1,
        '--rv-transition': themeVars.cubicBezierEaseInOut,
    }">
        <n-flex justify="center"
                class="qr"
                style="margin-bottom: 10px;">
            <div class="codes"
                 v-for="qrcode in qrcodes"
                 :key="qrcode.id">
                <n-card class="savedQR"
                        hoverable>
                    <h1>
                        {{ qrcode.loadqr.qrtitle }}
                    </h1>
                    <p>
                        QR-Code für: {{ qrcode.loadqr.data.type }}

                    </p>
                    <div class="qr"
                         v-html="qrcode.svg" />

                    <n-flex justify="center"
                            vertical>

                        <n-flex justify="space-between">
                            <hub-edit-button :qrcodeData="qrcode.loadqr"
                                             :qrcodeId="qrcode.qrid"
                                             :qrcodeImage="qrcode.svg"
                                             @update:reload-qr="reload = $event" />
                            <hub-download-button v-model:title="qrcode.loadqr.qrtitle"
                                                 v-model:value="qrcode.svg" />
                        </n-flex>
                        <n-flex justify="space-between">
                            <hub-archiv-button v-model:qrcodeId="qrcode.qrid" />
                            <hub-delete-qr v-model:qrcodeId="qrcode.qrid"
                                           v-model:deleteStatus="reload" />
                        </n-flex>
                    </n-flex>
                </n-card>
            </div>

            <n-card class="more-qr"
                    hoverable>
                <div v-if="store.layout.darkMode">
                    <n-button @click="switchQR"
                              text>
                        <img src="@/assets/addQrCodes.svg"
                             width="200"
                             height="200" />
                    </n-button>
                </div>
                <div v-else>
                    <n-button @click="switchQR"
                              text>
                        <img src="@/assets/addQrCodesW.svg"
                             width="200"
                             height="200" />
                    </n-button>
                </div>
            </n-card>
        </n-flex>

    </div>
</template>
<style scoped lang="sass">
.savedQR, .more-qr
    color: var(--rv--value-text-color)
    height: fit-content
    text-align: center
    margin-top: 10px

.savedQR
    min-height: 485px

.more-qr
    height: 245px
    max-width: 263.23px
    width: 263.23px
</style>
