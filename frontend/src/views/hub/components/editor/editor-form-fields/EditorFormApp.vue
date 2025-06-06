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
        type: 'app',
        appstore: props.currentValue.data.appstore || '',
        playstore: props.currentValue.data.playstore || '',
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

        <n-form-item class="appstore"
                     size="medium"
                     label="Appstore"
                     path="data.appstore">
            <n-input v-model:value="formValue.data.appstore"
                     placeholder="Link zum Appstore" />
        </n-form-item>
        <n-form-item class="playstore"
                     size="medium"
                     label="Playstore"
                     path="data.playstore">
            <n-input v-model:value="formValue.data.playstore"
                     placeholder="Link zum Playstore" />
        </n-form-item>

    </n-form>
</template>

<style scoped lang="sass" >
.n-form-item-label__text 
    color: #083549
    font-size: 18px
</style>
