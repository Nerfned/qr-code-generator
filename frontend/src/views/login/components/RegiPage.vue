<script lang="ts" setup>
import type { FormInst, FormItemRule, FormRules, FormValidationError } from 'naive-ui';
import { NButton, NForm, NFormItem, NInput, NPopover, NTabPane, NTabs, useThemeVars } from 'naive-ui';
import RegiImageUpload from './RegiImageUpload.vue';
import { ref } from 'vue';
import router from '@/router';

const themeVars = useThemeVars();

const nameValid = ref<boolean>(false);
const emailValid = ref<boolean>(false);
const regriValid = ref<boolean>(false);
const loadingElementAnimation = ref<boolean>(false);

const tab = ref('benutzer');
const formRef = ref<FormInst | null>(null);
const formValue = ref({
    user: {
        username: '',
        email: '',
        password: '',
        icon: '',
    },
    checkPassword: '',
});

const sendTimeout: number | undefined = void 0;
clearTimeout(sendTimeout);

const rules: FormRules = {
    user: {
        username: [
            {
                required: true,
                asyncValidator: async (rule: FormItemRule, value: string) => {
                    if (!value) throw new Error('Username ist erforderlich');
                    else if (value.length < 3) throw new Error('Name zu Kurz');
                    else {
                        nameValid.value = await validateData(JSON.stringify({ 'username': formValue.value.user.username }), 'username');
                        if (nameValid.value) {
                            throw new Error('Benutzer existiert bereits');
                        }
                    }
                },
                trigger: ['blur'],
            }],
        email: [{
            required: true,
            asyncValidator: async (rule: FormItemRule, value: string) => {
                const emailRegex = new RegExp(/^[A-Za-z0-9_!#$%&'*+/=?`{|}~^.-]+@[A-Za-z0-9.-]+\.[a-zA-Z0-9_.+-]+/, 'gm');
                if (!value) throw new Error('Bitte Email-Adresse angeben');
                else if (!emailRegex.test(value)) throw new Error('Keine gültige Email-Adresse');
                else {
                    emailValid.value = await validateData(JSON.stringify({ 'email': formValue.value.user.email }), 'email');
                    if (emailValid.value) throw new Error('Email-Adresse existiert bereits');
                }

            },
            validator(rule: FormItemRule, value: string) {
                if (value == '') return new Error('Bitte Email-Adresse angeben');
                const emailRegex = new RegExp(/^[A-Za-z0-9_!#$%&'*+/=?`{|}~^.-]+@[A-Za-z0-9.-]+\.[a-zA-Z0-9_.+-]+/, 'gm');
                if (!emailRegex.test(value)) return new Error('Keine gültige Email-Adresse');

                if (!emailValid.value) return new Error('Email-Adresse existiert bereits');
                return true;
            },
            trigger: ['blur'],
        }],
        password: [{
            required: true,
            validator(rule: FormItemRule, value: string) {
                if (value == '' || formValue.value.user.password == '') return new Error('Bitte Passwort eingeben');
                const passRegex = new RegExp(/^(?=.*\d)(?=.*[a-z])(?=.*[!@#$%^&*=()€+*{}/])(?=.*[A-Z])(?=.*[a-zA-Z]).{8,}$/gm);
                if (!passRegex.test(value)) return new Error('Überprüfen Sie ihr Passwort');
                return true;
            },
            trigger: ['blur'],
        }],
    },
    checkPassword: [{
        required: true,
        validator(rule: FormItemRule, value: string) {
            if (value !== formValue.value.user.password) return new Error('Passwort stimmt nicht überein');
            return true;
        },
        trigger: ['blur'],
    }],
};

const handleValidate = (e: KeyboardEvent | MouseEvent) => {
    e.preventDefault();
    formRef.value?.validate(
        async (errors: Array<FormValidationError> | undefined) => {
            if (errors) return;
            loadingElementAnimation.value = false;
            tab.value = 'icon';
        },
    );
};

async function savingUser() {
    regriValid.value = await sendingDataToAPI() as boolean;
    regriValid.value && router.push('/login');
}

function validateData(value: string, type: string) {
    return new Promise<boolean>((resolve) => {
        const request = new XMLHttpRequest();
        if (type == 'username') {
            request.open('PUT', '/api/check-user', true);
        } else if (type == 'email') {
            request.open('PUT', '/api/check-email', true);
        }
        request.setRequestHeader('Content-Type', 'application/json');

        request.onload = () => {
            if (request.readyState !== request.DONE) return resolve(false);
            if (request.status === 200) {
                resolve(true);
            } else if (request.status === 404) {
                resolve(false);
            }
        };

        request.onerror = () => {
            resolve(false);
        };

        request.send(value);
    });
}

function sendingDataToAPI() {
    return new Promise((resolve) => {
        const request = new XMLHttpRequest();
        request.responseType = 'json';
        request.open('PUT', '/api/registry', true);
        request.setRequestHeader('Content-Type', 'application/json');


        request.onload = () => {
            if (request.readyState !== request.DONE || request.status !== 200) { resolve(false); }


            loadingElementAnimation.value = false;
            if (request.status === 200) {
                resolve(true);
            } else if (request.status === 409) {
                resolve(false);
            }

        };

        request.onerror = () => {
            loadingElementAnimation.value = false;
            resolve(false);
        };

        request.send(JSON.stringify(formValue.value.user));
    });
}

function back() {
    tab.value = 'benutzer';
}

function stopNavigation() {
    return false;
}
</script>

<template>
    <n-tabs justify-content="space-evenly"
            type="line"
            v-model:value="tab"
            @before-leave="stopNavigation"
            animated>
        <n-tab-pane name="benutzer"
                    tab="Benutzerdaten">
            <n-form ref="formRef"
                    :model="formValue"
                    :rules="rules"
                    class="regi-form">
                <n-form-item class="user-field"
                             label="Ihr Benutzername"
                             path="user.username">
                    <n-input v-model:value="formValue.user.username"
                             :maxlength="20"
                             placeholder="Benutzername"
                             @keydown.enter="handleValidate" />
                </n-form-item>
                <n-form-item class="email-field"
                             label="Ihre E-Mail"
                             path="user.email">
                    <n-input v-model:value="formValue.user.email"
                             placeholder="E-Mail"
                             @keydown.enter="handleValidate" />
                </n-form-item>


                <n-popover trigger="hover"
                           class="pass-info"
                           :keep-alive-on-hover="false">
                    <template #trigger>
                        <n-form-item class="password-field"
                                     label="Ihr Passwort"
                                     path="user.password">
                            <n-input v-model:value="formValue.user.password"
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
                             label="Passwort erneut eingeben"
                             path="checkPassword">
                    <n-input type="password"
                             v-model:value="formValue.checkPassword"
                             show-password-on="click"
                             placeholder="Passwort wiederholen"
                             @keydown.enter="handleValidate" />
                </n-form-item>
                <n-form-item>
                    <router-link to="/login">
                        <n-button class="login-button"
                                  text>Zurück zum Login</n-button>
                    </router-link>
                    <n-button class="register-button"
                              :loading="loadingElementAnimation"
                              @click="handleValidate"
                              :color="themeVars.primaryColor">
                        Weiter
                    </n-button>
                </n-form-item>
            </n-form>
        </n-tab-pane>

        <n-tab-pane name="icon"
                    tab="Profilbild">
            <regi-image-upload class="image"
                               v-model:icon="formValue.user.icon" />

            <n-form-item style="justify-content: center;">
                <n-button class="login-button-next"
                          @click="back"
                          text>Zurück</n-button>

                <n-button class="register-button-next"
                          :loading="loadingElementAnimation"
                          @click="savingUser"
                          :color="themeVars.primaryColor">
                    Registrieren
                </n-button>
            </n-form-item>
        </n-tab-pane>
    </n-tabs>
</template>



<style scoped lang="sass">
.email-field, .user-field, .password-field, .password-check-field
    width: 300px
    padding-left: 50px
    margin-top: 5px
.login-button-next
    margin-left: 90px
.login-button
    margin-left: 50px
.register-button
    height: 50px
    width: 120px
    margin-left: 80px  
    color: #fff
.register-button-next
    height: 50px
    width: 120px
    margin-left: 110px  
    color: #fff
.pass-info
    margin-left: 50px
</style>
