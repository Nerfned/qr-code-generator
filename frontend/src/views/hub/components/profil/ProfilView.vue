<script setup lang="ts">
import { NCard, NFlex, NTabPane, NTabs, useThemeVars } from 'naive-ui';
import Profil from './Profil.vue';
const themeVars = useThemeVars();

</script>

<template>
    <n-flex justify="center">
        <n-card class="field"
                color="#0fa5bd"
                :style="{
                    '--rv-text-color': themeVars.textColor1,
                }">
            <n-tabs type="segment">
                <n-tab-pane name="Profil"
                            tab="Profil">
                    <h1>Profil</h1>
                    <profil class="profil" />
                </n-tab-pane>

                <n-tab-pane name="Placeholder"
                            tab="Placeholder">
                    <h1>User2</h1>
                </n-tab-pane>

            </n-tabs>
        </n-card>
    </n-flex>
</template>

<style scoped lang="sass">
.field
    min-width: 300px
    max-width: 1110px
.content
    margin-top: 20px
.logout
    font-size: 20px 
    text-decoration: none
    color: var(--rv--value-text-color)
    transition: color .3s var(--rv-darkmode-transition)  

.field
    margin-top: 10px
</style>

