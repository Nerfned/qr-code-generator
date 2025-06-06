<script setup lang="ts">
import { type FormInst, type FormItemRule, type FormRules, type FormValidationError, NButton, NCard, NForm, NFormItem, NInput, NModal } from 'naive-ui';
import { ref } from 'vue';

const modal = ref<boolean>(false);

const formRef = ref<FormInst | null>(null);
const formValue = ref({
    user: {
        email: '',
    },
});

const rules: FormRules = {
    user: {
        email: [{
            required: true,
            validator(rule: FormItemRule, value: string) {
                const emailRegex = new RegExp(/^[A-Za-z0-9_!#$%&'*+/=?`{|}~^.-]+@[A-Za-z0-9.-]+\.[a-zA-Z0-9_.+-]+/, 'gm');
                if (!emailRegex.test(value)) {
                    return new Error('Keine gültige Email-Adresse');
                }
                return true;
            },
        }],
    },
};

const handleValidateEnter = (e: KeyboardEvent) => {
    e.preventDefault();
    formRef.value?.validate(
        (errors: Array<FormValidationError> | undefined) => {
            if (errors) return;
        },
    );
};

const handleValidateButtonClick = (e: MouseEvent) => {
    e.preventDefault();
    formRef.value?.validate(
        (errors: Array<FormValidationError> | undefined) => {
            if (errors) return;
            modal.value = false;
        },
    );
};
</script>

<template>
    <n-form ref="formRef"
            :model="formValue"
            :rules="rules"
            class="regi-form">
        <n-button class="reset-button"
                  @click="modal = true"
                  text>Anmeldedaten vergessen?</n-button>
        <n-modal class="popup"
                 v-model:show="modal"
                 :mask-closable="false"
                 transform-origin="center"
                 :preset="undefined">

            <n-card>
                <h1 class="header">Haben Sie Ihr Passwort vergessen? Kein Problem!</h1>
                <p class="info">Zur Kontowiederherstellung geben Sie Ihre registrierte E-Mail-Adresse an.</p>
                <n-form-item class="email-field"
                             label="Ihre E-Mail"
                             path="user.email">
                    <n-input v-model:value="formValue.user.email"
                             placeholder="E-Mail"
                             @keydown.enter="handleValidateEnter" />
                </n-form-item>
                <n-button class="button"
                          @click="modal = false">Zurück</n-button>
                <n-button class="button"
                          @click="handleValidateButtonClick">Senden</n-button>
            </n-card>
        </n-modal>
    </n-form>
</template>

<style scoped lang="sass">
.reset-button
    margin-left: 50px
    height: fit-content
.popup
    width: 540px

.header
    margin-bottom: 30px
.info
    margin-bottom: 30px
.button
    height: 50px
    width: 100px
    margin-top: 25px
    margin-right: 25px

</style>

