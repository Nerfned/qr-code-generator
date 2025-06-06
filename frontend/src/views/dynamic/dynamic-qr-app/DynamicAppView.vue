<script setup lang="ts">
import { NCard, NFlex, useNotification, useThemeVars } from 'naive-ui';
import { onMounted } from 'vue';
import router from '@/router';


const themeVars = useThemeVars();
const notification = useNotification();
const qrid = router.currentRoute.value.params.code;


const title = 'Google GmbH';
const description = 'Laden Sie sich die App herunterladen';
// const logo = '';
const appstoreLink = 'www.google.com';
const playstoreLink = 'https://apps.apple.com/de/app/pok%C3%A9mon-go/id1094591345';

onMounted(() => {
    dynamicQR();
});
function dynamicQR() {
    const request = new XMLHttpRequest();
    request.responseType = 'json';
    request.open('PUT', '/api/change-qr', true);
    request.setRequestHeader('Content-Type', 'application/json');

    request.onload = () => {
        if (request.readyState !== request.DONE) {
            return notification.create({
                title: 'Da hat was nicht funktioniert beim Verbinden',
                duration: 5000,
            });
        }

        if (request.status === 200) {
            notification.create({
                title: 'QR-Code erfolgreich geändert',
                duration: 5000,
            });
        }

        if (request.status === 409) {
            notification.create({
                title: 'Da hat was nicht funktioniert bei der Datenbank',
                duration: 5000,
            });
        }
    };

    request.onerror = () => {
        notification.create({
            title: 'Da hat was nicht funktioniert weil kaputt',
            duration: 5000,
        });
    };
    request.send(JSON.stringify({ 'dynamicqr': qrid }));
}
</script>

<template>
    <n-card class="field"
            :style="{
                '--rv-text-color': themeVars.textColor1,
            }">
        <transition name="slide-fade">
            <n-flex vertical>

                <div>
                    <img class="logo"
                         src="@/assets/logo-mw.svg" />
                </div>

                <div class="title">
                    {{ title }}
                </div>
                <div class="description">
                    {{ description }}
                </div>
                <n-flex justify="center">
                    <a :href="appstoreLink">
                        <img class="store"
                             src="@/assets/AppstoreLogo.svg"
                             alt="Appstore" />
                    </a>

                    <a :href="playstoreLink">
                        <img class="store"
                             src="@/assets/PlaystoreLogo.svg"
                             alt="Playstore" />
                    </a>
                </n-flex>


            </n-flex>
        </transition>
    </n-card>
</template>

<style scoped lang="sass">
.field
    max-width: 450px
    width: fit-content
    position: absolute
    top: 50%
    left: 50%
    transform: translate(-50%, -50%)
    min-height: 350px
    text-align: center
    box-shadow: 10px 5px 5px 5px #0202024f
.store
    width: 150px
    height: 150px

.title
    font-weight: bold
    font-size: 28px

.description
    font-size: 18px


.logo
    height: 40px

</style>

