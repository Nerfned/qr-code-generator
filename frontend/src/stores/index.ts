import { useLayout } from './layout';
import { useUserInfo } from './user';

export const store = {
    get layout() {
        return useLayout();
    },
    get useUserInfo() {
        return useUserInfo();
    },
};
