<script setup lang="ts">
import type { FormInst, FormRules } from 'naive-ui';
import { NForm, NFormItem, NGrid, NGridItem, NInput } from 'naive-ui';
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
        type: 'url',
        url: props.currentValue.data.url || '',
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
        <n-grid cols="2"
                item-responsive>
            <n-grid-item span="2">
                <n-form-item class="url"
                             size="medium"
                             id="demo"
                             label="URL"
                             path="data.url">
                    <n-input v-model:value="formValue.data.url"
                             placeholder="google.com" />
                </n-form-item>
            </n-grid-item>
        </n-grid>
    </n-form>
</template>

<style scoped lang="sass" >

    

.n-form-item-label__text 
    color: #083549
    font-size: 18px

</style>
