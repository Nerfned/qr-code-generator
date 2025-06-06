<script setup lang="ts">
import type { FormInst, FormRules } from 'naive-ui';
import { NForm, NFormItem, NInput } from 'naive-ui';
import { reactive, ref, watch } from 'vue';

const props = defineProps({
    currentValue: {
        required: true,
        type: Object,
    },
});
const emit = defineEmits<{
    'update:data': [value: string]
}>();

const formRef = ref<FormInst | null>(null);
const formValue = reactive({
    data: {
        type: 'email',
        email: props.currentValue.data.email || '',
        subject: props.currentValue.data.subject || '',
        message: props.currentValue.data.message || '',
    },
});

const rules: FormRules = {
    data: {
        // email: [{
        //     validator(rule: FormItemRule, value: string) {
        //         // const emailRegex = new RegExp(/^[A-Za-z0-9_!#$%&'*+\/=?`{|}~^.-]+@[A-Za-z0-9.-]+\.[a-zA-Z0-9_.+-]+/, "gm")

        //         // if (!emailRegex.test(value)) {
        //         //     return new Error('Keine gültige Email-Adresse')
        //         // }
        //         return true;
        //     },
        //     trigger: ['blur'],
        // }],
    },
};


watch(formValue, () => emit('update:data', JSON.stringify(formValue)));

</script>

<template>
    <n-form ref="formRef"
            :model="formValue"
            :rules="rules">
        <n-form-item class="Email"
                     size="medium"
                     id="E-Mail"
                     label="E-Mail des Empfängers"
                     path="data.email">
            <n-input v-model:value="formValue.data.email"
                     placeholder="E-Mail des Empfängers" />
        </n-form-item>
        <n-form-item class="Email"
                     size="medium"
                     id="E-Mail"
                     label="Betreff"
                     path="data.subject">
            <n-input v-model:value="formValue.data.subject"
                     placeholder="Betreff der E-Mail" />
        </n-form-item>
        <n-form-item class="Email"
                     size="medium"
                     id="E-Mail"
                     label="Nachricht"
                     path="data.message">
            <n-input v-model:value="formValue.data.message"
                     type="textarea"
                     placeholder="Ihre Nachricht" />
        </n-form-item>
    </n-form>
</template>

<style lang="sass" scoped>
.n-form-item-label__text 
    color: #083549
    font-size: 18px
</style>
