<script lang="ts" setup>
import { NButton, NCard, NInput, NModal, useNotification, useThemeVars } from 'naive-ui';
import Cookies from 'js-cookie';
import { ref } from 'vue';
import { store } from '@/stores';

const themeVars = useThemeVars();
const notification = useNotification();

const props = defineProps({
    qrcode: {
        type: String,
        required: true,
    },
    qrLogo: {
        type: Blob,
        required: false,
    },
});

const modalView = ref(false);
const token = ref(Cookies.get('login'));
const qrtitle = ref('QR-Code');

function dataCollector() {
    if (props.qrLogo) {
        const reader = new FileReader();
        reader.onloadend = () => {
            const logoConverted = reader.result as string;
            QRSaving(JSON.stringify({ qrtitle: qrtitle.value, ...JSON.parse(props.qrcode), logo: logoConverted }));
        };
        reader.readAsDataURL(props.qrLogo);
    } else {
        QRSaving(JSON.stringify({ qrtitle: qrtitle.value, ...JSON.parse(props.qrcode) }));
    }
}

function CheckLogin() {
    !store.useUserInfo.statusLogin || token.value === undefined ?
        notification.create({
            title: 'Sie sind nicht angemeldet um QR-Codes zu speichern',
            duration: 5000,
        })
        : modalView.value = true;
}

function QRSaving(value: string) {
    if (token.value === undefined || !store.useUserInfo.statusLogin) {
        return notification.create({
            title: 'Sie sind nicht angemeldet um QR-Codes zu speichern',
            duration: 5000,
        });
    }

    const request = new XMLHttpRequest();
    request.responseType = 'json';
    request.open('PUT', '/api/save-qr', true);
    request.setRequestHeader('Content-Type', 'application/json');
    request.setRequestHeader('Authorization', token.value);

    request.onload = () => {
        if (request.readyState !== request.DONE) {
            return notification.create({
                title: 'Da hat was nicht funktioniert beim Verbinden',
                duration: 5000,
            });
        }

        modalView.value = false;
        if (request.status === 200) {
            notification.create({
                title: 'QR-Code erfolgreich gespeichert',
                duration: 5000,
            });
        }

        if (request.status === 409) {
            notification.create({
                title: 'Da hat was nicht funktioniert bei der Datenbank',
                duration: 5000,
            });
        }
    };

    request.onerror = () => {
        notification.create({
            title: 'Da hat was nicht funktioniert weil kaputt',
            duration: 5000,
        });
    };

    request.send(value);
}
</script>
    
<template>
    <div class="nav"
         :style="{
             '--rv--value-text-color': themeVars.textColor1,
             '--rv-darkmode-transition': themeVars.cubicBezierEaseInOut
         }">
        <n-button @click="CheckLogin"
                  size="large"
                  :color="themeVars.primaryColor">QR-Code speichern
        </n-button>
        <n-modal preset="dialog"
                 v-model:show="modalView"
                 positive-text="Speichern"
                 negative-text="Abbrechen"
                 @positive-click="dataCollector"
                 @negative-click="modalView = false">

            <n-card title="Geben Sie dem Qr Code einen Namen"
                    :bordered="false"
                    size="medium">

                <n-input v-model:value="qrtitle"
                         placeholder="QR-Code" />
            </n-card>

        </n-modal>
    </div>
</template>

<style lang="sass" scoped>
.n-button
  width: 200px
  height: 75px
  margin-top: 25px
  color: #fff
</style> 