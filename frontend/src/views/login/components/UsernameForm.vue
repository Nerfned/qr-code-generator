<script setup lang="ts">
import type { FormInst, FormItemRule, FormRules, FormValidationError, InputInst } from 'naive-ui';
import { NButton, NForm, NFormItem, NInput, useNotification, useThemeVars } from 'naive-ui';
import { onMounted, ref } from 'vue';
import ResetModul from './ResetModul.vue';
import { RouterLink } from 'vue-router';
import router from '@/router';
const themeVars = useThemeVars();
const alert = useNotification();

const emit = defineEmits<{
    'update:userName': [value: string]
}>();

const userValid = ref<boolean>(false);
const loadingAnimation = ref<boolean>(false);
const InputInstRef = ref<InputInst | null>(null);
const formRef = ref<FormInst | null>(null);
const formValue = ref({
    user: {
        name: '',
    },
});

onMounted(() => InputInstRef.value?.focus());

const rules: FormRules = {
    user: {
        name: [
            {
                required: true,
                validator(rule: FormItemRule, value: string) {
                    if (!value) return new Error('Username ist erforderlich');
                    if (!userValid.value || value.length < 3) return new Error('Benutzer nicht bekannt');
                    return true;
                },
            },
        ],
    },
};

function validateInator() {
    formRef.value?.validate(
        (errors: Array<FormValidationError> | undefined) => {
            loadingAnimation.value = false;
            if (errors || !userValid.value) return;
            emit('update:userName', formValue.value.user.name);
            router.push({ name: 'login-sign-in' });
        },
    );
}

const handleValidate = (e: KeyboardEvent | MouseEvent) => {
    loadingAnimation.value = true;
    e.preventDefault();
    validateDataWithDatabase(formValue.value.user.name);
};

function validateDataWithDatabase(name: string) {
    if (name === undefined || name.length < 3) return validateInator();
    const request = new XMLHttpRequest();
    request.timeout = 10000;
    request.open('PUT', '/api/username', true);
    request.setRequestHeader('Content-Type', 'application/json');

    request.onload = () => {
        if (request.readyState !== request.DONE) {
            userValid.value = false;
            return alert.create({
                title: 'Verbindungsversuch abgebrochen',
                duration: 5000,
            });
        }
        if (request.status === 404) {
            userValid.value = false;
            validateInator();
        } else if (request.status === 200) {
            userValid.value = true;
            validateInator();
        }
    };

    request.ontimeout = () => {
        alert.create({
            title: 'Verbindungsversuch abgebrochen wegen Timeout',
            duration: 5000,
        });
    };

    request.onerror = () => {
        userValid.value = false;
        loadingAnimation.value = false;
        alert.create({
            title: 'Verbindungsversuch fehlgescheitert',
            duration: 5000,
        });
    };

    request.send(JSON.stringify({ 'username': name }));
}
</script>

<template>
    <n-form ref="formRef"
            :model="formValue"
            :rules="rules"
            class="main">
        <n-form-item class="login-field"
                     label="Benutzername"
                     path="user.name">
            <n-input ref="InputInstRef"
                     v-model:value="formValue.user.name"
                     placeholder="Benutzername"
                     @keydown.enter="handleValidate"
                     :maxlength="20" />
        </n-form-item>
        <reset-modul />

        <n-form-item class="utility">
            <router-link to="/login/sign-up">
                <n-button class="registration-button"
                          text>Konto erstellen</n-button>
            </router-link>
            <n-button :loading="loadingAnimation"
                      class="forward-button"
                      @click="handleValidate"
                      :color="themeVars.primaryColor">
                Weiter
            </n-button>
        </n-form-item>
    </n-form>
</template>

<style lang="sass">
.login-field
    width: 300px
    transition: height 2s 
    margin-left: 50px      

.registration-button
    margin-left: 50px

.forward-button
    height: 50px
    width: 90px
    margin-left: 110px  
    color: #fff

.utility
    height: 80px
    margin-top: 50px
    margin-bottom: 20px
</style>

