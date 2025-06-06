<script lang="ts" setup>
import { ref, watch } from 'vue';
import EditorFormApp from './editor-form-fields/EditorFormApp.vue';
import EditorFormCall from './editor-form-fields/EditorFormCall.vue';
import EditorFormEmail from './editor-form-fields/EditorFormEmail.vue';
import EditorFormEvent from './editor-form-fields/EditorFormEvent.vue';
import EditorFormPaypal from './editor-form-fields/EditorFormPaypal.vue';
import EditorFormSms from './editor-form-fields/EditorFormSms.vue';
import EditorFormSocial from './editor-form-fields/EditorFormSocial.vue';
import EditorFormURL from './editor-form-fields/EditorFormURL.vue';
import EditorFormVcard from './editor-form-fields/EditorFormVcard.vue';
import EditorFormWa from './editor-form-fields/EditorFormWa.vue';

const props = defineProps({
    Mode: {
        type: String,
    },
    Form: {
        type: String,
    },
    currentValue: {
        required: true,
        type: Object,
    },
});

const emit = defineEmits<{
    'update:formData': [value: string]
}>();

const activeMode = ref(props.Mode);
const form = ref(props.Form as string);

watch(() => props.Mode, () => activeMode.value = props.Mode);
watch(() => props.currentValue.data.type, () => activeMode.value = props.currentValue.data.type);
watch(() => form.value, () => emit('update:formData', form.value));
</script>

<template>
    <transition name="slide-fade">
        <div v-if="activeMode === 'url'">
            <editor-form-u-r-l :currentValue="props.currentValue"
                               v-model:data="form" />
        </div>

        <div v-else-if="activeMode == 'email'">
            <editor-form-email :currentValue="props.currentValue"
                               v-model:data="form" />
        </div>

        <div v-else-if="activeMode === 'phonenumber'">
            <editor-form-call :currentValue="props.currentValue"
                              v-model:data="form" />
        </div>

        <div v-else-if="activeMode === 'sms'">
            <editor-form-sms :currentValue="props.currentValue"
                             v-model:data="form" />
        </div>

        <div v-else-if="activeMode === 'vcard'">
            <editor-form-vcard :currentValue="props.currentValue"
                               v-model:data="form" />
        </div>

        <div v-else-if="activeMode === 'whatsapp'">
            <editor-form-wa :currentValue="props.currentValue"
                            v-model:data="form" />
        </div>

        <div v-else-if="activeMode === 'paypal'">
            <editor-form-paypal :currentValue="props.currentValue"
                                v-model:data="form" />
        </div>

        <div v-else-if="activeMode === 'event'">
            <editor-form-event :currentValue="props.currentValue"
                               v-model:data="form" />
        </div>

        <div v-else-if="activeMode === 'App'">
            <editor-form-app :currentValue="props.currentValue"
                             v-model:data="form" />
        </div>

        <div v-else-if="activeMode === 'Social Media'">
            <editor-form-social v-model:data="form" />
        </div>

        <div v-else>
            <editor-form-u-r-l :currentValue="props.currentValue"
                               v-model:data="form" />
        </div>
    </transition>
</template>