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
        type: 'phonenumber',
        number: '',
    },
});

const rules: FormRules = {
    data: {
        number: [{
            validator() {
                return true;
            },
        }],
    },
};

watch(formValue, () => emit('update:data', JSON.stringify(formValue)));
</script>

<template>
    <n-form ref="formRef"
            :model="formValue"
            :rules="rules">
        <n-form-item class="number"
                     size="medium"
                     id="demo"
                     label="Telefonnummer"
                     path="data.number">
            <n-input v-model:value="formValue.data.number"
                     placeholder="Telefonnummer" />
        </n-form-item>
    </n-form>
</template>

<style lang="sass" scoped>
.n-form-item-label__text 
    color: #083549
    font-size: 18px
</style>
