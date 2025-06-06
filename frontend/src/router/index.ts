import { createRouter, createWebHistory } from 'vue-router';
import DynamicMainView from '@/views/dynamic/DynamicMainView.vue';
import Hub from '@/views/hub/components/Hub.vue';
import HubView from '@/views/hub/HubView.vue';
import LoginView from '@/views/login/LoginView.vue';
import MainView from '@/views/qr-generator/MainView.vue';
import NotFound from '@/views/NotFound.vue';
import PasswordForm from '@/views/login/components/PasswordForm.vue';
import ProfilView from '@/views/hub/components/profil/ProfilView.vue';
import RegiPage from '@/views/login/components/RegiPage.vue';
import UsernameForm from '@/views/login/components/UsernameForm.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      components: {
        main: MainView,
      },

    },
    {
      path: '/login',
      name: 'login',
      components: {
        main: LoginView,
      },
      children: [
        {
          path: '',
          name: 'login-username',
          components: {
            default: UsernameForm,
          },
        },
        {
          path: 'sign-in',
          name: 'login-sign-in',
          components: {
            default: PasswordForm,
          },
        },
        {
          path: 'sign-up',
          name: 'login-sign-up',
          components: {
            default: RegiPage,
          },
        }],
    },
    {
      path: '/hub/:user',
      name: 'hub',
      components: {
        main: HubView,
      },
      children: [
        {
          path: 'profil',
          name: 'profil',
          components: {
            default: ProfilView,
          },

        }, {
          path: '',
          name: 'hubfield',
          components: {
            default: Hub,
          },
        },
      ],
    },
    {
      path: '/editor',
      name: 'dynamic',
      components: {
        main: DynamicMainView,
      },
    },


    {
      path: '/:catchAll(.*)',
      name: 'NotFound',
      components: {
        main: NotFound,
      },
    },

  ],
});

export default router;