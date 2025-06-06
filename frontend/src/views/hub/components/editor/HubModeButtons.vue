<script lang="ts" setup>
import { NButton, NGrid, NGridItem, NIcon } from 'naive-ui';

</script>

<template>
    <n-grid class="hub-mode-select"
            y-gap="5"
            x-gap="3"
            cols="5 s:5 m:10 l:10">

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'url')">
                <n-icon>
                    <img src="@/assets/mode-buttons/UrlButton.svg"
                         alt="URL" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'email')">
                <n-icon>
                    <img src="@/assets/mode-buttons/EmailButton.svg"
                         alt="Email" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'phonenumber')">
                <n-icon>
                    <img src="@/assets/mode-buttons/CallButton.svg"
                         alt="Call" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'sms')">
                <n-icon>
                    <img src="@/assets/mode-buttons/SmsButton.svg"
                         alt="SMS" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'vcard')">
                <n-icon>
                    <img src="@/assets/mode-buttons/VcardButton.svg"
                         alt="vCard" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'paypal')">
                <n-icon>
                    <img src="@/assets/mode-buttons/PaypalButton.svg"
                         alt="Paypal" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'whatsapp')">
                <n-icon>
                    <img src="@/assets/mode-buttons/WaButton.svg"
                         alt="WhatsApp" />
                </n-icon>
            </n-button>
        </n-grid-item>
        <n-grid-item class="grid">

            <n-button class="button"
                      size="medium"
                      @click="$emit('update:modeSelector', 'event')">
                <n-icon>
                    <img src="@/assets/mode-buttons/EventButton.svg"
                         alt="Event" />
                </n-icon>
            </n-button>
        </n-grid-item>

        <n-grid-item class="grid">
            <n-button class="button"
                      size="medium"
                      disabled
                      @click="$emit('update:modeSelector', 'App')">
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
                      @click="$emit('update:modeSelector', 'Social Media')">
                <n-icon>
                    <img src="@/assets/mode-buttons/SocialButton.svg"
                         alt="Social Media" />
                </n-icon>
            </n-button>
        </n-grid-item>
    </n-grid>
</template>

<style scoped lang="sass">
.hub-mode-select
  margin-top: 5px
  margin-bottom: 20px
  max-width:  370px
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