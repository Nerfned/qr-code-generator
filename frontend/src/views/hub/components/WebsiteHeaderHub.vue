<script setup lang="ts">
import { NAvatar, NButton, NDivider, NDrawer, NDrawerContent, NDropdown, NFlex, NGrid, NGridItem, NIcon, darkTheme, useThemeVars } from 'naive-ui';
import Cookies from 'js-cookie';
import type { GlobalTheme } from 'naive-ui';
import HubQrContent from './HubQrContent.vue';
import { RouterLink } from 'vue-router';
import { ref } from 'vue';
import router from '@/router';
import { store } from '@/stores';
import usericon from '@/assets/thumb-126898.jpg'

const themeVars = useThemeVars();

const options = ref(false);
const optionsQr = ref(false);

const content = ref([
    {
        label: 'Hub',
        key: 'hub;',
        props: {
            onClick: () => {
                router.push(`/hub/${store.useUserInfo.username}`);
            },
        },
    }, {
        label: 'Profile',
        key: 'profile;',
        props: {
            onClick: () => {
                profilView();
            },
        },

    }, {
        label: 'Logout',
        key: 'Logout',
        props: {
            onClick: () => {
                logOut();
            },
        },
    }, {
        type: 'divider',
        key: 'd1',
    }, {
        label: 'Design ändern',
        key: 'darkmode',
        props: {
            onClick: () => {
                if (store.layout.darkMode) {
                    templateChange(null);
                } else {
                    templateChange(darkTheme);
                }
            },
        },
    }],
);

function templateChange(value: GlobalTheme | null) {
    store.layout.update(value);
}

function logOut() {
    Cookies.remove('login');
    store.useUserInfo.remove();
    router.replace('/login');
}

function profilView() {
    router.push({ name: 'profil' });
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
                width="100vw"
                item-responsive
                responsive="screen">
            <n-grid-item span="1 m:1 l:0">
                <n-button @click="optionsQr = !optionsQr"
                          text
                          class="options">
                    <n-icon>
                        <div v-if="store.layout.darkMode">
                            <img class="qr-options"
                                 src="@/assets/OptionsQrCode_weiss.svg" />
                        </div>
                        <div v-else>
                            <img class="qr-options"
                                 src="@/assets/OptionsQrCode.svg" />
                        </div>
                    </n-icon></n-button>
                <n-drawer class="options-pile"
                          placement="left"
                          default-width="320"
                          resizable
                          v-model:show="optionsQr">
                    <n-drawer-content>
                        <hub-qr-content />
                    </n-drawer-content>
                </n-drawer>
            </n-grid-item>

            <n-grid-item offset="0 m:0 l:1"
                         span="3">
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
                    <h1>Dashboard</h1>
                </div>
            </n-grid-item>

            <n-grid-item span="1">

                <n-grid class="grid-login"
                        cols="1"
                        item-responsive
                        responsive="screen">
                    <n-grid-item span="0 m:0 l:1">
                        <n-flex class="login-user"
                                justify="end">

                            <n-avatar round
                                      :size="32"
                                      :fallback-src="usericon"
                                      :src="store.useUserInfo.usericon" />

                            <n-dropdown trigger="click"
                                        :options="content">
                                <n-button class="login"
                                          text>{{ store.useUserInfo.username }}&#11033;</n-button>
                            </n-dropdown>

                        </n-flex>
                    </n-grid-item>

                    <n-grid-item span="1 m:1 l:0">
                        <n-button @click="options = !options"
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
                        <n-drawer v-model:show="options">
                            <n-drawer-content>
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

                                <n-divider />
                                <n-flex vertical>

                                    <n-button class="options-pile"
                                              @click="profilView"
                                              text>Profil</n-button>
                                    <n-button class="options-pile"
                                              @click="logOut"
                                              text>Logout</n-button>
                                </n-flex>

                                <n-divider />
                                <div class="darkmode-button"
                                     v-if="store.layout.darkMode">
                                    <n-button class="options-pile"
                                              text
                                              @click="templateChange(null)">Light Mode</n-button>
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
    margin-top: 6px
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

.qr-options
    font-size: 40px
    margin-left: 5px

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