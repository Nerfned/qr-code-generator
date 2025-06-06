<script setup lang="ts">
import type { FormInst, FormItemRule, FormRules, FormValidationError, InputInst } from 'naive-ui';
import { NButton, NForm, NFormItem, NInput, useNotification, useThemeVars } from 'naive-ui';
import { onBeforeMount, onMounted, ref } from 'vue';
import Cookies from 'js-cookie';
import ResetModul from './ResetModul.vue';
import router from '@/router';
import { store } from '@/stores';

const themeVars = useThemeVars();
const alert = useNotification();

const props = defineProps({
    userName: {
        type: String,
        required: true,
    },
});

const passValid = ref<boolean>(false);
const loadingAnimation = ref<boolean>(false);
const inputInstRef = ref<InputInst | null>(null);
const formRef = ref<FormInst | null>(null);
const formValue = ref({
    user: {
        username: props.userName,
        password: '',
    },
});

onBeforeMount(() => {
    if (formValue.value.user.username === undefined) return router.replace('/login');
});

onMounted(() => inputInstRef.value?.focus());

const rules: FormRules = {
    user: {
        password: [
            {
                validator(rule: FormItemRule, value: string) {
                    if (!value) return new Error('Passwort erforderlich');
                    if (!passValid.value) return new Error('Falsches Passwort');
                    return true;
                },
            },
        ],
    },
};

function validateInator(token: string | undefined) {
    formRef.value?.validate(
        (errors: Array<FormValidationError> | undefined) => {
            loadingAnimation.value = false;
            if (errors || !passValid.value || !token) return;
            Cookies.set('login', token, { expires: 7 });
            router.replace({ path: `/hub/${formValue.value.user.username}` });
        },
    );
}

const handleValidate = (e: MouseEvent | KeyboardEvent) => {
    loadingAnimation.value = true;
    e.preventDefault();
    validatePassWithDatabase();
};

function validatePassWithDatabase() {
    const pass = formValue.value.user.password;
    const passRegex = new RegExp(/^(?=.*\d)(?=.*[a-z])(?=.*[!@#$%^&*=()€+*{}/])(?=.*[A-Z])(?=.*[a-zA-Z]).{8,}$/gm);
    if (pass === undefined || !passRegex.test(pass)) return validateInator(undefined);
    const request = new XMLHttpRequest();
    request.responseType = 'json';
    request.timeout = 10000;
    request.open('PUT', '/api/login', true);
    request.setRequestHeader('Content-Type', 'application/json');

    request.onload = () => {
        if (request.readyState !== request.DONE) {
            loadingAnimation.value = false;
            return alert.create({
                title: 'Verbindungsversuch abgebrochen',
                duration: 5000,
            });
        }

        if (request.status === 409) {
            passValid.value = false;
            validateInator(undefined);
        } else if (request.status === 200) {
            passValid.value = true;
            const answer = request.response;
            store.useUserInfo.updateName(formValue.value.user.username);
            store.useUserInfo.updateEmail(answer.email);
            store.useUserInfo.updateIcon(answer.icon);
            validateInator(answer.id);
        }

        loadingAnimation.value = false;
    };

    request.ontimeout = () => {
        passValid.value = false;
        loadingAnimation.value = false;
        alert.create({
            title: 'Verbindungsversuch abgebrochen wegen Timeout',
            duration: 5000,
        });
    };
    request.onerror = () => {
        passValid.value = false;
        loadingAnimation.value = false;
        alert.create({
            title: 'Verbindungsversuch fehlgescheitert',
            duration: 5000,
        });
    };
    request.send(JSON.stringify(formValue.value.user));
}
</script>

<template>
    <n-form ref="formRef"
            :model="formValue"
            :rules="rules"
            class="main">

        <n-form-item class="password-field"
                     label="Passwort"
                     path="user.password"
                     autofocus>
            <n-input ref="inputInstRef"
                     :disabled="loadingAnimation"
                     type="password"
                     v-model:value="formValue.user.password"
                     show-password-on="mousedown"
                     @keydown.enter="handleValidate"
                     placeholder="Passwort" />
        </n-form-item>
        <reset-modul />

        <n-form-item>
            <n-button class="login-button"
                      :color="themeVars.primaryColor"
                      :loading="loadingAnimation"
                      @click="handleValidate">Login</n-button>
        </n-form-item>
    </n-form>
</template>

<style scoped lang="sass">
.password-field
    width: 300px
    padding-left: 50px

.login-button
    width: 150px
    font-size: large
    position: relative
    left: 50%
    transform: translate(-50%, 0%)
    margin-left: 0px
    color: #fff
    height: 80px
    margin-top: 10px
</style>