<script setup lang="ts">
import type { FormInst, FormItemRule, FormRules, FormValidationError } from 'naive-ui';
import { NAvatar, NButton, NDivider, NFlex, NForm, NFormItem, NGrid, NGridItem, NInput, NPopover, useNotification, useThemeVars } from 'naive-ui';
import { onMounted, ref } from 'vue';
import Cookies from 'js-cookie';
import ProfilIconUpload from './ProfilIconUpload.vue';
import { store } from '@/stores';
import logo from '@/assets/logo.svg'

const themeVars = useThemeVars();
const alert = useNotification();

const emailChange = ref<boolean>(false);
const loadingElementsAnimation = ref<boolean>(false);
const passChange = ref<boolean>(false);
const errorHandler = ref<boolean>(false);

const token = ref(Cookies.get('login'));

const formRef = ref<FormInst | null>(null);
const formValue = ref({
    user: {
        name: store.useUserInfo.username,
        email: store.useUserInfo.usermail,
        password: '',
    },
    newPassword: '',
    checkPassword: '',
});

const rules: FormRules = {
    user: {
        name: [
            {
                required: true,
            }],
        email: [{
            required: true,
            validator(rule: FormItemRule, value: string) {
                loadingElementsAnimation.value = false;
                if (passChange.value) return true;
                const emailRegex = new RegExp(/^[A-Za-z0-9_!#$%&'*+/=?`{|}~^.-]+@[A-Za-z0-9.-]+\.[a-zA-Z0-9_.+-]+/, 'gm');
                if (!emailRegex.test(value)) return new Error('Keine gültige Email-Adresse');
                if (errorHandler.value) {
                    errorHandler.value = false;
                    return new Error('E-Mail bereits vorhanden');
                }
                return true;
            },
        }],
        password: [{
            required: true,
            validator(rule: FormItemRule, value: string) {
                if (emailChange.value) { return true; }
                const passRegex = new RegExp(/^(?=.*\d)(?=.*[a-z])(?=.*[!@#$%^&*=()€+*{}/])(?=.*[A-Z])(?=.*[a-zA-Z]).{8,}$/gm);
                if (!passRegex.test(value)) { return new Error('Überprüfen Sie ihr Passwort'); }
                if (errorHandler.value) {
                    errorHandler.value = false;
                    return new Error('Falsches Passwort');
                }
                return true;
            },
        }],
    },
    newPassword: [{
        required: true,
        validator(rule: FormItemRule, value: string) {
            if (emailChange.value) { return true; }
            const passRegex = new RegExp(/^(?=.*\d)(?=.*[a-z])(?=.*[!@#$%^&*=()€+*{}/])(?=.*[A-Z])(?=.*[a-zA-Z]).{8,}$/gm);
            if (!passRegex.test(value)) {
                return new Error('Überprüfen Sie ihr Passwort');
            }
            return true;
        },
    }],

    checkPassword: [{
        required: true,
        validator(rule: FormItemRule, value: string) {
            if (emailChange.value) { return true; }
            if (value == '' || formValue.value.newPassword == '') {
                return new Error('Bitte Passwort eingeben');
            } else if (value !== formValue.value.newPassword) {
                return new Error('Passwort stimmt nicht überein');
            }
            return true;

        },
    }],
};

onMounted(() => {
    emailChange.value = false;
    passChange.value = false;
});

function validatorInator() {
    formRef.value?.validate(
        async (errors: Array<FormValidationError> | undefined) => {
            if (errors) return;
            if (!errorHandler.value) userInfoChanger();
        },
    );
}

const handleValidate = async (e: KeyboardEvent | MouseEvent) => {
    e.preventDefault();
    loadingElementsAnimation.value = true;
    validatorInator();
};


function userInfoChanger() {
    if (token.value != undefined) {
        const request = new XMLHttpRequest();

        request.responseType = 'json';
        emailChange.value && request.open('PUT', '/api/change-email', true);
        passChange.value && request.open('PUT', '/api/change-password', true);

        request.setRequestHeader('Content-Type', 'application/json');
        request.setRequestHeader('Authorization', token.value);


        request.onload = () => {
            loadingElementsAnimation.value = false;

            if (request.readyState !== request.DONE) {
                return alert.create({
                    title: 'Verbindungsversuch fehlgescheitert',
                    duration: 5000,
                });
            }

            if (request.status === 409) {
                errorHandler.value = true;
                alert.create({
                    title: 'Funktioniert nicht',
                    duration: 5000,
                });
                validatorInator();
            }

            if (request.status === 200) {
                emailChange.value = false;
                passChange.value = false;
                formValue.value.newPassword = '';
                formValue.value.user.password = '';
                formValue.value.checkPassword = '';
                formValue.value.user.email && store.useUserInfo.updateEmail(formValue.value.user.email);

                alert.create({
                    title: 'Erfolgreich geändert',
                    duration: 5000,
                });
            }
        };

        request.onerror = () => {
            alert.create({
                title: 'Verbindungsversuch fehlgescheitert',
                duration: 5000,
            });
        };

        if (emailChange.value) {
            request.send(JSON.stringify({ 'new-email': formValue.value.user.email }));

        } else if (passChange.value) {
            request.send(JSON.stringify({ 'new-password': formValue.value.newPassword, 'old-password': formValue.value.user.password }));
        }
    } else {
        return alert.create({
            title: 'Verbindungsversuch fehlgescheitert',
            duration: 5000,
        });
    }
}

function newPassword() {
    passChange.value = true;
    emailChange.value = false;
}

function newEmail() {
    emailChange.value = true;
    passChange.value = false;
}
</script>

<template >
    <transition name="fade">
        <n-flex vertical>
            <n-grid responsive="screen"
                    x-gap="20"
                    y-gap="20"
                    cols="1 s:1 m:2 l:2">
                <n-grid-item span="1">
                    <n-flex vertical>
                        <div class="icon-change">
                            <n-avatar round
                                      :size="150"
                                      :fallback-src="logo"
                                      :src="store.useUserInfo.usericon" />
                        </div>
                        <profil-icon-upload class="icon-change" />
                    </n-flex>
                </n-grid-item>

                <n-grid-item span="1">
                    <n-form ref="formRef"
                            :model="formValue"
                            :rules="rules"
                            class="regi-form">
                        <n-form-item class="user-field"
                                     label="Ihr Benutzername"
                                     path="user.name">
                            <n-input disabled
                                     v-model:value="formValue.user.name"
                                     :maxlength="20"
                                     placeholder="Benutzername"
                                     @keydown.enter="handleValidate" />
                        </n-form-item>
                        <n-divider />
                        <n-form-item class="email-field"
                                     label="Ihre E-Mail"
                                     path="user.email">
                            <n-input :disabled="!emailChange"
                                     v-model:value="formValue.user.email"
                                     placeholder="E-Mail"
                                     @keydown.enter="handleValidate" />
                        </n-form-item>
                        <div class="icon-change"
                             v-if="!emailChange">
                            <n-button :color="themeVars.primaryColor"
                                      class="button"
                                      @click="newEmail()"> E-Mail
                                ändern</n-button>
                        </div>
                        <div class="icon-change"
                             v-else>
                            <n-button class="button"
                                      :loading="loadingElementsAnimation"
                                      @click="handleValidate"
                                      :color="themeVars.primaryColor"> Neue E-Mail
                                speichern</n-button>
                        </div>

                        <n-divider />
                        <transition name="fade">
                            <div v-show="!passChange">
                                <n-form-item class="password-field"
                                             label="Ihr Passwort"
                                             path="user.password">
                                    <n-input disabled
                                             value="Password"
                                             type="password"
                                             show-password-on="click"
                                             placeholder="Passwort" />
                                </n-form-item>

                                <div class="icon-change">

                                    <n-button class="button"
                                              @click="newPassword"
                                              :color="themeVars.primaryColor"> Passwort
                                        ändern</n-button>
                                </div>

                            </div>
                        </transition>
                        <transition name="fade">
                            <div v-show="passChange">
                                <n-form-item class="password-field"
                                             label="Altes Passwort"
                                             path="user.password">
                                    <n-input v-model:value="formValue.user.password"
                                             type="password"
                                             show-password-on="click"
                                             placeholder="Passwort"
                                             @keydown.enter="handleValidate" />
                                </n-form-item>
                                <n-popover trigger="hover"
                                           class="pass-info"
                                           :keep-alive-on-hover="false">
                                    <template #trigger>
                                        <n-form-item class="password-field"
                                                     label="Neues Passwort"
                                                     path="newPassword">
                                            <n-input v-model:value="formValue.newPassword"
                                                     type="password"
                                                     show-password-on="click"
                                                     placeholder="Passwort"
                                                     @keydown.enter="handleValidate" />
                                        </n-form-item>
                                    </template>
                                    <span>
                                        Ihr Passwort muss folgendes beinhalten: <br />
                                        - 8 Zeichen mindestens, <br />
                                        - 1 Großbuchstaben, <br />
                                        - 1 Zahl, <br />
                                        - 1 Sonderzeichen
                                    </span>
                                </n-popover>
                                <n-form-item class="password-check-field"
                                             label="Neues Passwort erneut eingeben"
                                             path="checkPassword">
                                    <n-input type="password"
                                             v-model:value="formValue.checkPassword"
                                             show-password-on="click"
                                             placeholder="Passwort wiederholen"
                                             @keydown.enter="handleValidate" />
                                </n-form-item>
                                <div class="icon-change">
                                    <n-button @click="handleValidate"
                                              :color="themeVars.primaryColor"
                                              class="button"> Neues
                                        Passwort
                                        speichern</n-button>
                                </div>
                            </div>
                        </transition>
                    </n-form>
                </n-grid-item>
            </n-grid>
        </n-flex>
    </transition>
</template>

<style lang="sass" scoped>
.button
    color: #FFF
.icon-change
    display: flex
    justify-content: center
    align-items: center
</style>