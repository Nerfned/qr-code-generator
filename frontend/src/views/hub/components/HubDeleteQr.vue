<script lang="ts" setup>
import { NButton, NModal, useNotification, useThemeVars } from 'naive-ui';
import Cookies from 'js-cookie';
import { ref } from 'vue';

const themeVars = useThemeVars();
const alert = useNotification();

const props = defineProps({
    qrcodeId: {
        required: true,
        type: Number,
    },
});

const emit = defineEmits<{
    'update:deleteStatus': [value: boolean]
}>();

const qrcode = ref(props.qrcodeId);
const options = ref(false);
const qrcodeDeleteSignal = ref(false);
const token = ref(Cookies.get('login'));

function deleteQR() {
    token.value = Cookies.get('login');
    if (token.value === undefined) {
        return alert.create({
            title: 'Fehler beim Lesen',
            duration: 5000,
        });
    }

    const request = new XMLHttpRequest;
    request.responseType = 'json';
    request.open('PUT', '/api/delete-qr', true);
    request.setRequestHeader('Content-Type', 'application/json');

    request.setRequestHeader('Authorization', token.value);

    request.onload = () => {
        if (request.DONE !== request.readyState || request.status !== 200) return alert.create({
            title: 'Verbindungsversuch abgebrochen',
            duration: 5000,
        });

        qrcodeDeleteSignal.value = false;

        if (request.response) {
            qrcodeDeleteSignal.value = true;
            alert.create({
                title: 'QR-Code konnte nicht gelöscht werden',
                duration: 5000,
            });

            emit('update:deleteStatus', qrcodeDeleteSignal.value);
        } else {
            qrcodeDeleteSignal.value = true;
            emit('update:deleteStatus', qrcodeDeleteSignal.value);
        }

    };

    request.ontimeout = () => {
        alert.create({
            title: 'Verbindungsversuch abgebrochen wegen Timeout',
            duration: 5000,
        });
    };

    request.onerror = () => {
        alert.create({
            title: 'Verbindungsversuch fehlgescheitert',
            duration: 5000,
        });
    };

    request.send(JSON.stringify({ 'qrid': qrcode.value }));
}
</script>

<template>
    <n-button @click="options = true"
              :color="themeVars.primaryColor"
              class="delete">Löschen</n-button>
    <n-modal preset="dialog"
             title="QR-Code löschen"
             content="Wollen sie den QR-Code löschen?"
             v-model:show="options"
             positive-text="Löschen"
             negative-text="Abbrechen"
             @positive-click="deleteQR"
             @negative-click="options = false" />
</template>

<style lang="sass" scoped>
.delete
  color: #fff
  min-width: 115px
  height: 40px
</style> 