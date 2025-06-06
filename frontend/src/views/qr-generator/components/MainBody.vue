<script setup lang="ts">
import { NCard, NFlex, useNotification } from 'naive-ui';
import { reactive, ref, watch } from 'vue';
import FormBasicSettings from './FormBasicSettings.vue';
import FormMode from './FormMode.vue';
import ImageUpload from './ImageUpload.vue';
import ModeButtons from './ModeButtons.vue';
import QrField from './QrField.vue';

const notification = useNotification();
let sendTimeout: number | undefined = void 0;
clearTimeout(sendTimeout);

const activForm = ref();

interface Qr {
    form: string,
    basicSettings: string,
    logo: Blob | undefined
}

const qrCodeSvg = ref('<svg viewBox="0 0 33 33" xmlns="http://www.w3.org/2000/svg"><rect width="33px" height="33px" fill="#ffffff00"/><path d="M5,12.5a.5,.5 0 1,1 0,-.1M5,13.5a.5,.5 0 1,1 0,-.1M5,14.5a.5,.5 0 1,1 0,-.1M5,15.5a.5,.5 0 1,1 0,-.1M5,17.5a.5,.5 0 1,1 0,-.1M5,18.5a.5,.5 0 1,1 0,-.1M5,19.5a.5,.5 0 1,1 0,-.1M6,16.5a.5,.5 0 1,1 0,-.1M6,17.5a.5,.5 0 1,1 0,-.1M7,13.5a.5,.5 0 1,1 0,-.1M7,15.5a.5,.5 0 1,1 0,-.1M7,17.5a.5,.5 0 1,1 0,-.1M7,19.5a.5,.5 0 1,1 0,-.1M8,12.5a.5,.5 0 1,1 0,-.1M8,16.5a.5,.5 0 1,1 0,-.1M8,18.5a.5,.5 0 1,1 0,-.1M9,13.5a.5,.5 0 1,1 0,-.1M9,14.5a.5,.5 0 1,1 0,-.1M9,17.5a.5,.5 0 1,1 0,-.1M9,20.5a.5,.5 0 1,1 0,-.1M10,13.5a.5,.5 0 1,1 0,-.1M10,16.5a.5,.5 0 1,1 0,-.1M10,17.5a.5,.5 0 1,1 0,-.1M11,12.5a.5,.5 0 1,1 0,-.1M11,14.5a.5,.5 0 1,1 0,-.1M11,16.5a.5,.5 0 1,1 0,-.1M11,18.5a.5,.5 0 1,1 0,-.1M11,20.5a.5,.5 0 1,1 0,-.1M12,13.5a.5,.5 0 1,1 0,-.1M12,14.5a.5,.5 0 1,1 0,-.1M12,15.5a.5,.5 0 1,1 0,-.1M12,16.5a.5,.5 0 1,1 0,-.1M12,17.5a.5,.5 0 1,1 0,-.1M12,18.5a.5,.5 0 1,1 0,-.1M13,6.5a.5,.5 0 1,1 0,-.1M13,8.5a.5,.5 0 1,1 0,-.1M13,9.5a.5,.5 0 1,1 0,-.1M13,10.5a.5,.5 0 1,1 0,-.1M13,12.5a.5,.5 0 1,1 0,-.1M13,13.5a.5,.5 0 1,1 0,-.1M13,15.5a.5,.5 0 1,1 0,-.1M13,18.5a.5,.5 0 1,1 0,-.1M13,21.5a.5,.5 0 1,1 0,-.1M13,25.5a.5,.5 0 1,1 0,-.1M13,28.5a.5,.5 0 1,1 0,-.1M14,5.5a.5,.5 0 1,1 0,-.1M14,11.5a.5,.5 0 1,1 0,-.1M14,12.5a.5,.5 0 1,1 0,-.1M14,13.5a.5,.5 0 1,1 0,-.1M14,14.5a.5,.5 0 1,1 0,-.1M14,16.5a.5,.5 0 1,1 0,-.1M14,18.5a.5,.5 0 1,1 0,-.1M14,19.5a.5,.5 0 1,1 0,-.1M14,21.5a.5,.5 0 1,1 0,-.1M14,22.5a.5,.5 0 1,1 0,-.1M14,26.5a.5,.5 0 1,1 0,-.1M14,27.5a.5,.5 0 1,1 0,-.1M14,28.5a.5,.5 0 1,1 0,-.1M15,4.5a.5,.5 0 1,1 0,-.1M15,7.5a.5,.5 0 1,1 0,-.1M15,8.5a.5,.5 0 1,1 0,-.1M15,10.5a.5,.5 0 1,1 0,-.1M15,12.5a.5,.5 0 1,1 0,-.1M15,13.5a.5,.5 0 1,1 0,-.1M15,15.5a.5,.5 0 1,1 0,-.1M15,17.5a.5,.5 0 1,1 0,-.1M15,18.5a.5,.5 0 1,1 0,-.1M15,21.5a.5,.5 0 1,1 0,-.1M15,23.5a.5,.5 0 1,1 0,-.1M15,27.5a.5,.5 0 1,1 0,-.1M15,28.5a.5,.5 0 1,1 0,-.1M16,6.5a.5,.5 0 1,1 0,-.1M16,9.5a.5,.5 0 1,1 0,-.1M16,14.5a.5,.5 0 1,1 0,-.1M16,15.5a.5,.5 0 1,1 0,-.1M16,17.5a.5,.5 0 1,1 0,-.1M16,18.5a.5,.5 0 1,1 0,-.1M16,20.5a.5,.5 0 1,1 0,-.1M16,22.5a.5,.5 0 1,1 0,-.1M16,23.5a.5,.5 0 1,1 0,-.1M17,5.5a.5,.5 0 1,1 0,-.1M17,6.5a.5,.5 0 1,1 0,-.1M17,9.5a.5,.5 0 1,1 0,-.1M17,10.5a.5,.5 0 1,1 0,-.1M17,12.5a.5,.5 0 1,1 0,-.1M17,13.5a.5,.5 0 1,1 0,-.1M17,17.5a.5,.5 0 1,1 0,-.1M17,19.5a.5,.5 0 1,1 0,-.1M17,20.5a.5,.5 0 1,1 0,-.1M17,21.5a.5,.5 0 1,1 0,-.1M17,22.5a.5,.5 0 1,1 0,-.1M17,25.5a.5,.5 0 1,1 0,-.1M17,27.5a.5,.5 0 1,1 0,-.1M17,28.5a.5,.5 0 1,1 0,-.1M18,6.5a.5,.5 0 1,1 0,-.1M18,7.5a.5,.5 0 1,1 0,-.1M18,9.5a.5,.5 0 1,1 0,-.1M18,11.5a.5,.5 0 1,1 0,-.1M18,13.5a.5,.5 0 1,1 0,-.1M18,14.5a.5,.5 0 1,1 0,-.1M18,15.5a.5,.5 0 1,1 0,-.1M18,16.5a.5,.5 0 1,1 0,-.1M18,19.5a.5,.5 0 1,1 0,-.1M18,20.5a.5,.5 0 1,1 0,-.1M18,21.5a.5,.5 0 1,1 0,-.1M18,22.5a.5,.5 0 1,1 0,-.1M18,27.5a.5,.5 0 1,1 0,-.1M18,28.5a.5,.5 0 1,1 0,-.1M19,4.5a.5,.5 0 1,1 0,-.1M19,6.5a.5,.5 0 1,1 0,-.1M19,8.5a.5,.5 0 1,1 0,-.1M19,9.5a.5,.5 0 1,1 0,-.1M19,10.5a.5,.5 0 1,1 0,-.1M19,11.5a.5,.5 0 1,1 0,-.1M19,15.5a.5,.5 0 1,1 0,-.1M19,17.5a.5,.5 0 1,1 0,-.1M19,19.5a.5,.5 0 1,1 0,-.1M19,20.5a.5,.5 0 1,1 0,-.1M19,21.5a.5,.5 0 1,1 0,-.1M19,27.5a.5,.5 0 1,1 0,-.1M19,28.5a.5,.5 0 1,1 0,-.1M20,5.5a.5,.5 0 1,1 0,-.1M20,6.5a.5,.5 0 1,1 0,-.1M20,8.5a.5,.5 0 1,1 0,-.1M20,11.5a.5,.5 0 1,1 0,-.1M20,12.5a.5,.5 0 1,1 0,-.1M20,13.5a.5,.5 0 1,1 0,-.1M20,15.5a.5,.5 0 1,1 0,-.1M20,16.5a.5,.5 0 1,1 0,-.1M20,17.5a.5,.5 0 1,1 0,-.1M20,20.5a.5,.5 0 1,1 0,-.1M20,24.5a.5,.5 0 1,1 0,-.1M20,27.5a.5,.5 0 1,1 0,-.1M20,28.5a.5,.5 0 1,1 0,-.1M21,4.5a.5,.5 0 1,1 0,-.1M21,6.5a.5,.5 0 1,1 0,-.1M21,10.5a.5,.5 0 1,1 0,-.1M21,11.5a.5,.5 0 1,1 0,-.1M21,12.5a.5,.5 0 1,1 0,-.1M21,13.5a.5,.5 0 1,1 0,-.1M21,14.5a.5,.5 0 1,1 0,-.1M21,19.5a.5,.5 0 1,1 0,-.1M21,28.5a.5,.5 0 1,1 0,-.1M22,12.5a.5,.5 0 1,1 0,-.1M22,13.5a.5,.5 0 1,1 0,-.1M22,16.5a.5,.5 0 1,1 0,-.1M22,17.5a.5,.5 0 1,1 0,-.1M22,25.5a.5,.5 0 1,1 0,-.1M22,26.5a.5,.5 0 1,1 0,-.1M22,28.5a.5,.5 0 1,1 0,-.1M23,13.5a.5,.5 0 1,1 0,-.1M23,14.5a.5,.5 0 1,1 0,-.1M23,25.5a.5,.5 0 1,1 0,-.1M23,26.5a.5,.5 0 1,1 0,-.1M23,27.5a.5,.5 0 1,1 0,-.1M23,28.5a.5,.5 0 1,1 0,-.1M24,12.5a.5,.5 0 1,1 0,-.1M24,13.5a.5,.5 0 1,1 0,-.1M24,14.5a.5,.5 0 1,1 0,-.1M24,16.5a.5,.5 0 1,1 0,-.1M24,18.5a.5,.5 0 1,1 0,-.1M24,19.5a.5,.5 0 1,1 0,-.1M24,28.5a.5,.5 0 1,1 0,-.1M25,12.5a.5,.5 0 1,1 0,-.1M25,16.5a.5,.5 0 1,1 0,-.1M25,17.5a.5,.5 0 1,1 0,-.1M26,14.5a.5,.5 0 1,1 0,-.1M26,15.5a.5,.5 0 1,1 0,-.1M26,16.5a.5,.5 0 1,1 0,-.1M26,17.5a.5,.5 0 1,1 0,-.1M26,18.5a.5,.5 0 1,1 0,-.1M26,21.5a.5,.5 0 1,1 0,-.1M26,23.5a.5,.5 0 1,1 0,-.1M26,24.5a.5,.5 0 1,1 0,-.1M26,26.5a.5,.5 0 1,1 0,-.1M26,27.5a.5,.5 0 1,1 0,-.1M27,12.5a.5,.5 0 1,1 0,-.1M27,15.5a.5,.5 0 1,1 0,-.1M27,17.5a.5,.5 0 1,1 0,-.1M27,20.5a.5,.5 0 1,1 0,-.1M27,22.5a.5,.5 0 1,1 0,-.1M27,24.5a.5,.5 0 1,1 0,-.1M27,25.5a.5,.5 0 1,1 0,-.1M27,28.5a.5,.5 0 1,1 0,-.1M28,13.5a.5,.5 0 1,1 0,-.1M28,15.5a.5,.5 0 1,1 0,-.1M28,16.5a.5,.5 0 1,1 0,-.1M28,17.5a.5,.5 0 1,1 0,-.1M28,18.5a.5,.5 0 1,1 0,-.1M28,19.5a.5,.5 0 1,1 0,-.1M28,22.5a.5,.5 0 1,1 0,-.1M28,23.5a.5,.5 0 1,1 0,-.1M28,27.5a.5,.5 0 1,1 0,-.1M29,14.5a.5,.5 0 1,1 0,-.1M29,16.5a.5,.5 0 1,1 0,-.1M29,18.5a.5,.5 0 1,1 0,-.1M29,19.5a.5,.5 0 1,1 0,-.1M29,23.5a.5,.5 0 1,1 0,-.1M29,27.5a.5,.5 0 1,1 0,-.1M29,28.5a.5,.5 0 1,1 0,-.1" fill="#000000"/><path d="M4,4h1v1h-1M4,5h1v1h-1M4,6h1v1h-1M4,7h1v1h-1M4,8h1v1h-1M4,9h1v1h-1M4,10h1v1h-1M4,22h1v1h-1M4,23h1v1h-1M4,24h1v1h-1M4,25h1v1h-1M4,26h1v1h-1M4,27h1v1h-1M4,28h1v1h-1M5,4h1v1h-1M5,10h1v1h-1M5,22h1v1h-1M5,28h1v1h-1M6,4h1v1h-1M6,6h1v1h-1M6,7h1v1h-1M6,8h1v1h-1M6,10h1v1h-1M6,22h1v1h-1M6,24h1v1h-1M6,25h1v1h-1M6,26h1v1h-1M6,28h1v1h-1M7,4h1v1h-1M7,6h1v1h-1M7,7h1v1h-1M7,8h1v1h-1M7,10h1v1h-1M7,22h1v1h-1M7,24h1v1h-1M7,25h1v1h-1M7,26h1v1h-1M7,28h1v1h-1M8,4h1v1h-1M8,6h1v1h-1M8,7h1v1h-1M8,8h1v1h-1M8,10h1v1h-1M8,22h1v1h-1M8,24h1v1h-1M8,25h1v1h-1M8,26h1v1h-1M8,28h1v1h-1M9,4h1v1h-1M9,10h1v1h-1M9,22h1v1h-1M9,28h1v1h-1M10,4h1v1h-1M10,5h1v1h-1M10,6h1v1h-1M10,7h1v1h-1M10,8h1v1h-1M10,9h1v1h-1M10,10h1v1h-1M10,22h1v1h-1M10,23h1v1h-1M10,24h1v1h-1M10,25h1v1h-1M10,26h1v1h-1M10,27h1v1h-1M10,28h1v1h-1M20,20h1v1h-1M20,21h1v1h-1M20,22h1v1h-1M20,23h1v1h-1M20,24h1v1h-1M21,20h1v1h-1M21,24h1v1h-1M22,4h1v1h-1M22,5h1v1h-1M22,6h1v1h-1M22,7h1v1h-1M22,8h1v1h-1M22,9h1v1h-1M22,10h1v1h-1M22,20h1v1h-1M22,22h1v1h-1M22,24h1v1h-1M23,4h1v1h-1M23,10h1v1h-1M23,20h1v1h-1M23,24h1v1h-1M24,4h1v1h-1M24,6h1v1h-1M24,7h1v1h-1M24,8h1v1h-1M24,10h1v1h-1M24,20h1v1h-1M24,21h1v1h-1M24,22h1v1h-1M24,23h1v1h-1M24,24h1v1h-1M25,4h1v1h-1M25,6h1v1h-1M25,7h1v1h-1M25,8h1v1h-1M25,10h1v1h-1M26,4h1v1h-1M26,6h1v1h-1M26,7h1v1h-1M26,8h1v1h-1M26,10h1v1h-1M27,4h1v1h-1M27,10h1v1h-1M28,4h1v1h-1M28,5h1v1h-1M28,6h1v1h-1M28,7h1v1h-1M28,8h1v1h-1M28,9h1v1h-1M28,10h1v1h-1" fill="#000000"/></svg>');

const qrCodeData: Qr = reactive({
    form: JSON.stringify({ data: { type: 'url', url: 'google.com' } }),
    basicSettings: JSON.stringify({ 'size': '15', 'color': '#000000', 'layout': 'circles' }),
    logo: undefined,
});
const qrCodeForBackend = ref(dataCollector());

function dataCollector() {
    const dataForAPI = JSON.stringify({ ...JSON.parse(qrCodeData.form), ...JSON.parse(qrCodeData.basicSettings) });
    sendingDataToAPI(dataForAPI);
    return dataForAPI;
}

function sendingDataToAPI(value: string) {
    sendTimeout = window.setTimeout(() => {
        const formData = new FormData();
        formData.append('qr', value);
        qrCodeData.logo && formData.append('logo', qrCodeData.logo);

        const request = new XMLHttpRequest();
        request.open('POST', '/api', true);
        request.responseType = 'text';

        request.onload = () => {
            if (request.readyState !== request.DONE || request.status !== 200) {
                notification.create({
                    title: 'Da hat etwas nicht funktioniert',
                    duration: 5000,
                });
            }
            qrCodeSvg.value = request.response;
        };
        request.send(formData);
    }, 300);
}

watch(qrCodeData, () => { qrCodeForBackend.value = dataCollector(); });
</script>

<template>
    <n-flex class="space"
            justify="center"
            style="margin-top: 20px; gap: 0px 0px;">
        <n-card class="field"
                style="gap: 0px 0px;">

            <div>
                <mode-buttons @update:mode-selector="activForm = $event" />
                <form-mode :modeSelector="activForm"
                           @update:form-data="qrCodeData.form = $event" />
                <form-basic-settings @update:basic-settings="qrCodeData.basicSettings = $event" />
                <image-upload @update:file="qrCodeData.logo = $event" />
            </div>

        </n-card>
        <qr-field :qrSvg="qrCodeSvg"
                  :qrData="qrCodeForBackend"
                  :qrLogo="qrCodeData.logo" />
    </n-flex>
</template>

<style scoped lang="sass">
.field 
  width: fit-content
  justify-content: right
  display: flex
  box-shadow: 10px 10px 5px 12px #0202024f

img 
  margin-left: 65%
</style>
