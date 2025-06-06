<script lang="ts" setup>
import { NButton, NGrid, NGridItem, NIcon, useDialog } from 'naive-ui';
import Cookies from 'js-cookie';
import { ref } from 'vue';
import router from '@/router';
import { store } from '@/stores';

const dialog = useDialog();

const token = ref(Cookies.get('login'));

function OpenDynamicEditor() {
    if (token.value == undefined || !store.useUserInfo.statusLogin) dialog.error({
        title: 'Anmeldung erforderlich!',
        content: 'Sie müssen angemeldet sein um dynamische QR-Codes zu erstellen',
        positiveText: 'Zum Login',
        negativeText: 'Zur Registrierung',
        onPositiveClick: () => {
            router.push('/login');
        },
        onNegativeClick: () => {
            router.push('/login/sign-up');
        },
    });
    else {
        dialog.success({
            title: 'Dynamische QR-Codes erstellen',
            content: 'Erstellen Sie hier ihren dynamischen QR-Code',
            positiveText: 'Zum Editor von Dynamischen QR-Codes',
            onPositiveClick: () => {
                router.push('/editor');
            },
        });
    }
}
</script>

<template>
    <n-grid class="mode-select"
            y-gap="5"
            x-gap="3"
            cols="5 s:5 m:10 l:10"
            responsive="screen">
        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'Url')">
                <n-icon>
                    <img src="@/assets/mode-buttons/UrlButton.svg"
                         alt="URL" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'Email')">
                <n-icon>
                    <img src="@/assets/mode-buttons/EmailButton.svg"
                         alt="Email" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      isPressed
                      @click="$emit('update:modeSelector', 'Call')">
                <n-icon>
                    <img src="@/assets/mode-buttons/CallButton.svg"
                         alt="Call" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'SMS')">
                <n-icon>
                    <img src="@/assets/mode-buttons/SmsButton.svg"
                         alt="SMS" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      active
                      @click="$emit('update:modeSelector', 'vCard')">
                <n-icon>
                    <img src="@/assets/mode-buttons/VcardButton.svg"
                         alt="vCard" />
                </n-icon>
            </n-button>
        </n-grid-item>


        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'Paypal')">
                <n-icon>
                    <img src="@/assets/mode-buttons/PaypalButton.svg"
                         alt="Paypal" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'WhatsApp')">
                <n-icon>
                    <img src="@/assets/mode-buttons/WaButton.svg"
                         alt="WhatsApp" />
                </n-icon>
            </n-button>
        </n-grid-item>
        <n-grid-item class="grid">

            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'Event')">
                <n-icon>
                    <img src="@/assets/mode-buttons/EventButton.svg"
                         alt="Event" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="OpenDynamicEditor()">
                <n-icon>
                    <img src="@/assets/mode-buttons/AppButton.svg"
                         alt="App" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      disabled
                      @click="OpenDynamicEditor()">
                <n-icon>
                    <img src="@/assets/mode-buttons/SocialButton.svg"
                         alt="Social Media" />
                </n-icon>
            </n-button>
        </n-grid-item>
    </n-grid>
</template>

<style scoped lang="sass">
.filter
    font-size: 28px
    margin-bottom: 10px
.mode-select
  margin-bottom: 25px
.n-icon 
  font-size: 35px
  fill: #fff
  color: #fff
.grid
  display: flex
  align-items: center
  justify-content: center
svg
  color: #fff
.button
  height: 60px
  width: 60px
</style>