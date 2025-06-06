<script setup lang="ts">
import type { FormInst, FormRules } from 'naive-ui';
import { NForm, NFormItem, NGrid, NGridItem, NInput } from 'naive-ui';
import { reactive, ref, watch } from 'vue';

const emit = defineEmits<{
    'update:data': [value: string]
}>();

const formRef = ref<FormInst | null>();
const formValue = reactive({
    data: {
        type: 'vcard',
        name: '',
        surname: '',
        tnumber: '',
        mnumber: '',
        email: '',
        url: '',
        company: '',
        jobtitle: '',
        fax: '',
        address: '',
        city: '',
        postcode: '',
        country: '',
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
        <n-grid class="test"
                :x-gap="10"
                cols="1 s:1 m:2 l:2 xl:2 2xl:2"
                responsive="screen">
            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="Vorname"
                             path="data.name">
                    <n-input v-model:value="formValue.data.name"
                             placeholder="Ihr Vorname" />
                </n-form-item>
            </n-grid-item>

            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="Nachname"
                             path="data.surname">
                    <n-input v-model:value="formValue.data.surname"
                             placeholder="Ihr Nachname" />
                </n-form-item>
            </n-grid-item>



            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="Telefonnummer"
                             path="data.tnumber">
                    <n-input v-model:value="formValue.data.tnumber"
                             placeholder="Ihre Telefonnummer" />
                </n-form-item>
            </n-grid-item>

            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="Handynummer"
                             path="data.mnumber">
                    <n-input v-model:value="formValue.data.mnumber"
                             placeholder="Ihre Handynummer" />
                </n-form-item>
            </n-grid-item>


            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="E-Mail"
                             path="data.email">
                    <n-input v-model:value="formValue.data.email"
                             placeholder="Ihre E-Mail" />
                </n-form-item>
            </n-grid-item>

            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="Webseite"
                             path="data.url">
                    <n-input v-model:value="formValue.data.url"
                             placeholder="https://" />
                </n-form-item>
            </n-grid-item>

            <n-grid-item span="2">
                <n-form-item class="v-card"
                             label="Firma"
                             path="data.company">
                    <n-input v-model:value="formValue.data.company"
                             placeholder="Firma" />
                </n-form-item>
            </n-grid-item>

            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="Jobtitel"
                             path="nudata.jobtitlember">
                    <n-input v-model:value="formValue.data.jobtitle"
                             placeholder="Jobtitel" />
                </n-form-item>
            </n-grid-item>

            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="Fax"
                             path="data.fax">
                    <n-input v-model:value="formValue.data.fax"
                             placeholder="Ihre Faxnummer" />
                </n-form-item>
            </n-grid-item>

            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="Adresse"
                             path="data.address">
                    <n-input v-model:value="formValue.data.address"
                             placeholder="Straße + Hausnummer" />
                </n-form-item>
            </n-grid-item>


            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="Stadt"
                             path="data.city">
                    <n-input v-model:value="formValue.data.city"
                             placeholder="Stadt" />
                </n-form-item>
            </n-grid-item>

            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="Postleitzahl"
                             path="data.postcode">
                    <n-input v-model:value="formValue.data.postcode"
                             placeholder="Postleitzahl" />
                </n-form-item>
            </n-grid-item>

            <n-grid-item span="1">
                <n-form-item class="v-card"
                             label="Land"
                             path="data.country">
                    <n-input v-model:value="formValue.data.country"
                             placeholder="Land" />
                </n-form-item>
            </n-grid-item>
        </n-grid>
    </n-form>
</template>

<style lang="sass" scoped>
.n-form-item-label__text 
    color: #083549
    font-size: 18px
</style>
