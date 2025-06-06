<script lang="ts" setup>
import { NButton, NCard, NFlex, NInput, NModal, NTabPane, NTabs, useDialog, useNotification, useThemeVars } from 'naive-ui';
import { reactive, ref, watch } from 'vue';
import Cookies from 'js-cookie';
import HubEditorQrField from './editor/HubEditorQrField.vue';
import HubFormBasicSettings from '@/views/hub/components/editor/HubFormBasicSettings.vue';
import HubFormMode from './editor/HubFormMode.vue';
import HubImageUpload from './editor/HubImageUpload.vue';
import HubModeButtons from './editor/HubModeButtons.vue';

const themeVars = useThemeVars();
const notification = useNotification();
const dialog = useDialog();

const props = defineProps({
    qrcodeId: {
        required: true,
        type: Number,
    },
    qrcodeData: {
        required: true,
        type: Object,
    },
    qrcodeImage: {
        required: true,
        type: String,
    },
});

const emit = defineEmits<{
    'update:reloadQr': [value: boolean]
}>();

let sendTimeout: number | undefined = void 0;
clearTimeout(sendTimeout);

const token = ref(Cookies.get('login'));
const showModal = ref(false);

const qrCodeId = ref(props.qrcodeId);
const qrCodeData = ref(props.qrcodeData);
const qrCodePreview = ref(props.qrcodeImage);
const qrCodeTitle = ref(props.qrcodeData.qrtitle);
const formSelect = ref(qrCodeData.value.data.type);

const qrCode = reactive({
    form: JSON.stringify({ 'data': qrCodeData.value.data }),
    basicSettings: JSON.stringify({ 'size': qrCodeData.value.size.toString(), 'color': qrCodeData.value.color, 'layout': qrCodeData.value.layout }),
    logo: qrCodeData.value.logo,
});

function dataCollector() {
    const dataForAPI = { ...JSON.parse(qrCode.form), ...JSON.parse(qrCode.basicSettings) };
    qrCodeData.value = dataForAPI;
    return JSON.stringify(dataForAPI);
}

function sendingDataToAPI() {
    sendTimeout = window.setTimeout(() => {
        const formData = new FormData();
        formData.append('qr', dataCollector());
        qrCode.logo && formData.append('base64', JSON.stringify({ 'logo': qrCode.logo }));

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
            qrCodePreview.value = request.response;
        };
        request.send(formData);
    }, 500);
}

function QRSaving() {

    token.value = Cookies.get('login');

    if (token.value === undefined) {
        return notification.create({
            title: 'Seite neuladen',
            duration: 5000,
        });
    }


    const request = new XMLHttpRequest();
    request.responseType = 'json';
    request.open('PUT', '/api/change-qr', true);
    request.setRequestHeader('Content-Type', 'application/json');
    request.setRequestHeader('Authorization', token.value);

    request.onload = () => {
        if (request.readyState !== request.DONE) {
            return notification.create({
                title: 'Da hat was nicht funktioniert beim Verbinden',
                duration: 5000,
            });
        }

        showModal.value = false;
        if (request.status === 200) {
            notification.create({
                title: 'QR-Code erfolgreich geändert',
                duration: 5000,
            });
            emit('update:reloadQr', true);
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
    request.send(JSON.stringify({
        qrid: qrCodeId.value, loadqr: { ...JSON.parse(dataCollector()), qrtitle: qrCodeTitle.value, logo: qrCode.logo },
    }));
}

function ClosingModal() {
    dialog.error({
        title: 'Abbrechen',
        content: 'Nicht gespeicherte Änderungen werden verworfen',
        positiveText: 'Ok!',
        onPositiveClick: () => {
            showModal.value = false;
            qrCodeId.value = props.qrcodeId;
            qrCodeData.value = props.qrcodeData;
            qrCodePreview.value = props.qrcodeImage;
            qrCodeTitle.value = props.qrcodeData.qrtitle;
            formSelect.value = qrCodeData.value.data.type;
        },
    });
}


watch(qrCode, () => sendingDataToAPI());
</script>

<template>
    <n-button @click="showModal = true"
              :color="themeVars.primaryColor"
              class="edit">Bearbeiten</n-button>
    <n-modal :show="showModal"
             :auto-focus="false">

        <n-card class="editor"
                title="QR-Code bearbeiten"
                style="width: 900px;"
                :bordered="false"
                aria-modal="true"
                header-style="display: flex; justify-content: center;"
                role="dialog">
            <n-tabs type="card"
                    animated>
                <n-tab-pane name="data"
                            tab="Daten">
                    <hub-mode-buttons class="form"
                                      @update:modeSelector="formSelect = $event" />
                    <hub-form-mode class="form"
                                   :currentValue="qrCodeData"
                                   v-model:Mode="formSelect"
                                   v-model:formData="qrCode.form" />
                </n-tab-pane>

                <n-tab-pane name="design"
                            tab="Design">
                    <hub-form-basic-settings class="form"
                                             :generalSetting="qrCodeData"
                                             @update:basic-data="qrCode.basicSettings = $event" />
                </n-tab-pane>

                <n-tab-pane name="bild"
                            tab="Bild">
                    <hub-image-upload class="form"
                                      v-model:file="qrCode.logo" />
                </n-tab-pane>

                <n-tab-pane name="title"
                            tab="Titel">
                    <h2 class="form">Name des QR-Codes ändern</h2>
                    <n-input maxlength="12"
                             v-model:value="qrCodeTitle"
                             placeholder="QR-Code" />
                </n-tab-pane>
            </n-tabs>

            <n-flex class="qr-preview"
                    vertical>
                <h2 class="preview">QR-Code Preview: </h2>
                <hub-editor-qr-field v-model:mode="qrCode.form"
                                     v-model:qrPreview="qrCodePreview" />
            </n-flex>
            <n-flex class="qr-preview"
                    justify="center">
                <n-button class="navigator"
                          style="background: gray;"
                          @click="ClosingModal">Abbrechen</n-button>
                <n-button class="navigator"
                          :color="themeVars.primaryColor"
                          @click="QRSaving">QR-Code speichern</n-button>
            </n-flex>
        </n-card>
    </n-modal>
</template>

<style lang="sass" scoped>
.form
    margin-top: 20px
.editor
    height: fit-content
.edit
    min-width: 115px
    height: 40px
    color: #fff
.preview
    justify-content: center
    display: flex
.navigator
    height: 50px
    width: 150px
    color: #fff
</style> 