import { defineStore } from 'pinia';

export interface UserInfo {
    statusLogin: boolean;
    username: string | undefined;
    usermail: string | undefined;
    usericon: string | undefined;
}

export const useUserInfo = defineStore('user', {
    state: () => ({
        statusLogin: false,
        username: undefined,
        usermail: undefined,
        usericon: undefined,
    } as UserInfo),

    actions: {
        updateStatusLogin(login: boolean) {
            this.statusLogin = login;
        },

        updateIcon(usericon: string) {
            this.usericon = usericon;
        },
        updateEmail(usermail: string) {
            this.usermail = usermail;
        },
        updateName(username: string) {
            this.username = username;
        },

        remove() {
            this.statusLogin = false;
            this.username = undefined;
            this.usermail = undefined;
            this.usericon = undefined;
        },
    },
});