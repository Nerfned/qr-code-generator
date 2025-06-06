<script setup lang="ts">
import type { FormInst, FormRules } from 'naive-ui';
import { NDatePicker, NForm, NFormItem, NGrid, NGridItem, NInput } from 'naive-ui';
import { reactive, ref, watch } from 'vue';

const emit = defineEmits<{
    'update:data': [value: string]
}>();
const timeNow = new Date();
const time = ref<[number, number]>([timeNow.getTime(), timeNow.getTime()]);

const formRef = ref<FormInst | null>(null);
const formValue = reactive({
    data: {
        type: 'event',
        name: '',
        place: '',
        start: time.value[0],
        finish: time.value[1],
    },
});

const rules: FormRules = {
};

watch(() => time.value[0], (value) => formValue.data.start = value);
watch(() => time.value[1], (value) => formValue.data.finish = value);
watch(formValue, () => emit('update:data', JSON.stringify(formValue)));
</script>

<template>
    <n-form ref="formRef"
            :model="formValue"
            :rules="rules">
        <div class="form-event">
            <n-grid class="grid"
                    :x-gap="10"
                    cols="1 s:1 m:2 l:2 xl:2 2xl:2"
                    responsive="screen">
                <n-grid-item span="1">
                    <n-form-item class="event"
                                 size="medium"
                                 label="Name"
                                 path="data.name">
                        <n-input v-model:value="formValue.data.name"
                                 placeholder="Name des Events" />
                    </n-form-item>
                </n-grid-item>

                <n-grid-item span="1">
                    <n-form-item class="event"
                                 size="medium"
                                 label="Ort"
                                 path="data.place">
                        <n-input v-model:value="formValue.data.place"
                                 placeholder="Ort des Events" />
                    </n-form-item>
                </n-grid-item>
            </n-grid>
            <n-date-picker class="date-picker"
                           size="medium"
                           v-model:value="time"
                           type="datetimerange" />
        </div>
    </n-form>
</template>

<style lang="sass" scoped>
.n-form-item-label__text 
    color: #083549
    font-size: 18px

.date-picker
    margin-bottom: 15px
</style>
