import type { GlobalTheme } from 'naive-ui';
import { darkTheme } from 'naive-ui';
import { defineStore } from 'pinia';

export interface Layout {
    darkMode: GlobalTheme | null;
}

export const useLayout = defineStore('Layout', {
    state: () => ({
        darkMode: darkTheme,
    } as Layout),

    actions: {
        update(darkMode: GlobalTheme | null) {
            this.darkMode = darkMode;
        },
    },
});
