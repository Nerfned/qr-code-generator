<script setup lang="ts">
import { NResult } from 'naive-ui';
</script>

<template>
    <transition name="fade">
        <n-result class="error"
                  status="404"
                  title="404 Not Found"
                  description="The Cake is a Lie" />
    </transition>
</template>

<style lang="sass">
.error
    position: absolute
    top: 50%
    left: 50%
    transform: translate(-50%, -50%)
</style>