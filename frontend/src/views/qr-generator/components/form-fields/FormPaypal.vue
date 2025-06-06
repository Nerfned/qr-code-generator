<script setup lang="ts">
import type { FormInst, FormRules } from 'naive-ui';
import { NForm, NFormItem, NInput } from 'naive-ui';
import { reactive, ref, watch } from 'vue';

const emit = defineEmits<{
    'update:data': [value: string]
}>();

const formRef = ref<FormInst | null>(null);
const formValue = reactive({
    data: {
        type: 'paypal',
        user: '',
    },
});

const rules: FormRules = {
};

watch(formValue, () => emit('update:data', JSON.stringify(formValue)));
</script>

<template>
    <n-form ref="formRef"
            :model="formValue"
            :rules="rules">
        <n-form-item class="paypal"
                     id="demo"
                     label="Paypal-Username"
                     path="data.user">
            <n-input v-model:value="formValue.data.user"
                     placeholder="Paypal-Username" />
        </n-form-item>
    </n-form>
</template>

<style scoped lang="sass" >
.n-form-item-label__text 
    color: #083549
    font-size: 18px
</style>
