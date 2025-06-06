<script lang="ts" setup>
import FormApp from './form-fields/FormApp.vue';
import FormCall from './form-fields/FormCall.vue';
import FormEmail from './form-fields/FormEmail.vue';
import FormEvent from './form-fields/FormEvent.vue';
import FormPaypal from './form-fields/FormPaypal.vue';
import FormSms from './form-fields/FormSms.vue';
import FormURL from './form-fields/FormURL.vue';
import FormVcard from './form-fields/FormVcard.vue';
import FormWa from './form-fields/FormWa.vue';

defineProps({
    modeSelector: {
        type: String,
    },
});
</script>

<template>
    <transition name="slide-fade">
        <form-u-r-l v-if="modeSelector === 'Url'"
                    @update:data="$emit('update:formData', $event)" />
        <form-email v-else-if="modeSelector == 'Email'"
                    @update:data="$emit('update:formData', $event)" />
        <form-call v-else-if="modeSelector === 'Call'"
                   @update:data="$emit('update:formData', $event)" />
        <form-sms v-else-if="modeSelector === 'SMS'"
                  @update:data="$emit('update:formData', $event)" />
        <form-vcard v-else-if="modeSelector === 'vCard'"
                    @update:data="$emit('update:formData', $event)" />
        <form-wa v-else-if="modeSelector === 'WhatsApp'"
                 @update:data="$emit('update:formData', $event)" />
        <form-paypal v-else-if="modeSelector === 'Paypal'"
                     @update:data="$emit('update:formData', $event)" />
        <form-event v-else-if="modeSelector === 'Event'"
                    @update:data="$emit('update:formData', $event)" />
        <form-app v-else-if="modeSelector === 'App'"
                  @update:data="$emit('update:formData', $event)" />
        <!-- <form-social v-else-if="modeSelector === 'Social Media'" @update:data="$emit('update:formData', $event)" /> -->
        <form-u-r-l v-else
                    @update:data="$emit('update:formData', $event)" />
    </transition>
</template>