import type { Struct } from './';

export const de: Struct = {
    test: `test deutsch`,
    test2: (val) => `test ${val} deutsch`,
    test3: {
        test4: `test4 deutsch`,
    },
};
