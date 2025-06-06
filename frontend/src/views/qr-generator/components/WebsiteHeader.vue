<script setup lang="ts">
import { NAvatar, NButton, NDivider, NDrawer, NDrawerContent, NFlex, NGrid, NGridItem, NIcon, darkTheme, useThemeVars } from 'naive-ui';
import type { GlobalTheme } from 'naive-ui';
import { ref } from 'vue';
import { store } from '@/stores';
import usericon from '@/assets/thumb-126898.jpg'
const themeVars = useThemeVars();

const drawerView = ref(false);
function templateChange(value: GlobalTheme | null) {
    store.layout.update(value);
}
</script>
        
<template>
    <div class="nav"
         :style="{
             '--rv--value-text-color': themeVars.textColor1,
             '--rv-darkmode-transition': themeVars.cubicBezierEaseInOut
         }">

        <n-grid class="head"
                cols="5"
                width="100vw">

            <n-grid-item span="1">
                <n-grid cols="1"
                        item-responsive
                        responsive="screen">
                    <n-grid-item span="0 m:0 l:1">
                        <div class="darkmode-button"
                             v-if="store.layout.darkMode">
                            <n-button @click="templateChange(null)">Light Mode</n-button>
                        </div>
                        <div class="darkmode-button"
                             v-else>
                            <n-button @click="templateChange(darkTheme)">Dark Mode</n-button>
                        </div>
                    </n-grid-item>
                </n-grid>
            </n-grid-item>

            <n-grid-item span="3">
                <div class="title">
                    <div v-if="store.layout.darkMode">
                        <img class="logo"
                             src="@/assets/logo-mw.svg" />
                    </div>
                    <div v-else>
                        <img class="logo"
                             src="@/assets/logo-mw-color.svg"
                             width="200"
                             height="30" />
                    </div>
                    <h1>QR-Code Generator</h1>
                </div>
            </n-grid-item>

            <n-grid-item span="1">
                <n-grid class="grid-login"
                        cols="1"
                        item-responsive
                        responsive="screen">
                    <n-grid-item span="0 m:0 l:1">
                        <transition name="fade">
                            <div v-if="store.useUserInfo.statusLogin === false">
                                <RouterLink class="login"
                                            to="/login">Zum Login</RouterLink>
                            </div>
                            <div v-else>
                                <n-flex class="login-user"
                                        justify="end">
                                    <n-avatar round
                                              :size="32"
                                              fallback-src="usericon"
                                              :src="store.useUserInfo.usericon" />
                                    <RouterLink class="login"
                                                :to="`/hub/${store.useUserInfo.username}`">{{
                                                    store.useUserInfo.username }}&#11033;
                                    </RouterLink>
                                </n-flex>
                            </div>
                        </transition>
                    </n-grid-item>

                    <n-grid-item span="1 m:1 l:0">
                        <n-button @click="drawerView = !drawerView"
                                  text
                                  class="options">
                            <n-icon>
                                <div v-if="store.layout.darkMode">
                                    <img class="options"
                                         src="@/assets/options_weiss.svg" />
                                </div>
                                <div v-else>
                                    <img class="options"
                                         src="@/assets/options.svg" />
                                </div>
                            </n-icon></n-button>
                        <n-drawer v-model:show="drawerView">
                            <n-drawer-content>
                                <div v-if="store.useUserInfo.statusLogin === false">
                                    <RouterLink class="login"
                                                to="/login">Zum Login</RouterLink>
                                </div>
                                <div v-else>
                                    <n-flex class="login-user-drawer">
                                        <n-avatar round
                                                  :size="32"
                                                  :fallback-src="usericon"
                                                  :src="store.useUserInfo.usericon" />

                                        <RouterLink class="login"
                                                    :to="`/hub/${store.useUserInfo.username}`">{{
                                                        store.useUserInfo.username }}
                                        </RouterLink>
                                    </n-flex>
                                </div>

                                <n-divider />
                                <div class="darkmode-button"
                                     v-if="store.layout.darkMode">
                                    <n-button class="options-pile"
                                              text
                                              @click="templateChange(null)">Light
                                        Mode</n-button>
                                </div>
                                <div class="darkmode-button"
                                     v-else>
                                    <n-button class="options-pile"
                                              text
                                              @click="templateChange(darkTheme)">Dark
                                        Mode</n-button>
                                </div>
                            </n-drawer-content>
                        </n-drawer>
                    </n-grid-item>
                </n-grid>
            </n-grid-item>
        </n-grid>
    </div>
</template>

<style scoped lang="sass" >
.nav
    height: 10%
    width: 98vw
    margin-left: 1vw
.title 
    text-align: center
    margin-top: 20px
h1 
    margin-top: -8px
    font-size: 20px
    color: var(--rv--value-text-color)
.head
    justify-content: center
    align-items: center

.login
    font-size: 20px 
    text-decoration: none
    color: var(--rv--value-text-color)
    transition: color .3s var(--rv-darkmode-transition)
.avatar
    margin-top: 1px
.grid-login
    text-align: right  
.logo
    height: 20px

.options
    font-size: 40px
    color: #fff
    fill: #fff
    margin-right: 5px

.login-user
    margin-top: 10px
.login-user-drawer
    margin-top: 5px

.options-pile
    font-size: 20px 
    text-decoration: none
    color: var(--rv--value-text-color)
    transition: color .3s var(--rv-darkmode-transition)  
</style>