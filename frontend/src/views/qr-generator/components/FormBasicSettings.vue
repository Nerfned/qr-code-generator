<script setup lang="ts">
import type { FormInst, FormRules } from 'naive-ui';
import { NButton, NCollapse, NCollapseItem, NColorPicker, NForm, NFormItem, NGrid, NGridItem, NIcon, NInputNumber, useThemeVars } from 'naive-ui';
import { reactive, ref, watch } from 'vue';
const themeVars = useThemeVars();

const emit = defineEmits<{
    'update:basicSettings': [value: string]
}>();

const formRef = ref<FormInst | null>(null);

const formValue = reactive({
    size: 15,
    color: '#000000',
    layout: 'circles',
});

const rules: FormRules = {};

watch(formValue, () => emit('update:basicSettings', JSON.stringify({ 'size': formValue.size.toString(), 'color': formValue.color, 'layout': formValue.layout })));

function shapeSwitch(selected: string) {
    formValue.layout = selected;
}
</script>

<template>
    <n-form ref="formRef"
            :model="formValue"
            :rules="rules">
        <div class="basic-settings"
             :style="{
                 '--rv--value-text-color': themeVars.textColor1
             }">

            <n-grid class="grid"
                    x-gap="10"
                    cols="1 s:1 m:3 l:3 xl:3 2xl:3"
                    responsive="screen">

                <n-grid-item span="1">
                    <n-form-item class="mist"
                                 label="Größe">
                        <n-input-number style="width: 100%;"
                                        :min="10"
                                        :show-button="false"
                                        v-model:value="formValue.size"
                                        placeholder="15cm">
                            <template #suffix>
                                cm
                            </template>
                        </n-input-number>
                    </n-form-item>
                </n-grid-item>

                <n-grid-item class="color"
                             span="2">
                    <n-form-item label="Farbe">
                        <n-color-picker size="small"
                                        :modes="['hex']"
                                        v-model:value="formValue.color"
                                        :show-alpha="false" />
                    </n-form-item>
                </n-grid-item>

                <n-grid-item span="3">
                    <n-collapse>
                        <n-collapse-item class="inner-formen"
                                         title="Formen"
                                         name="1">
                            <n-grid :cols="4"
                                    :y-gap="5">
                                <n-grid-item>
                                    <n-button class="shape"
                                              @click="shapeSwitch('squares')">
                                        <n-icon>
                                            <img class="inner-form-icon"
                                                 src="@/assets/inner-forms/square.svg"
                                                 alt="square" />
                                        </n-icon>
                                    </n-button>
                                </n-grid-item>

                                <n-grid-item>
                                    <n-button class="shape"
                                              @click="shapeSwitch('circles')">
                                        <n-icon>
                                            <img class="inner-form-icon"
                                                 src="@/assets/inner-forms/circle.svg"
                                                 alt="circe" />
                                        </n-icon>
                                    </n-button>
                                </n-grid-item>

                                <n-grid-item>
                                    <n-button class="shape"
                                              @click="shapeSwitch('rounded-squares')">
                                        <n-icon>
                                            <img class="inner-form-icon"
                                                 src="@/assets/inner-forms/squareSmall.svg"
                                                 alt="squareSmall" />
                                        </n-icon>
                                    </n-button>
                                </n-grid-item>

                                <n-grid-item>
                                    <n-button class="shape"
                                              @click="shapeSwitch('rectangles')">
                                        <n-icon>
                                            <img class="inner-form-icon"
                                                 src="@/assets/inner-forms/rectangle.svg"
                                                 alt="rectangle" />
                                        </n-icon>
                                    </n-button>
                                </n-grid-item>
                            </n-grid>
                        </n-collapse-item>

                    </n-collapse>
                </n-grid-item>
            </n-grid>

        </div>
    </n-form>
</template>

<style scoped lang="sass">
.basic-settings
    text-align: center
    width: 100%

.n-form-item-label__text, .n-collapse-item__header-main
    font-size: 18px
    color: var(--rv--value-text-color)

.shape
    width: 50px
    height: 50px
    margin-bottom: 10px

.mist
    width: 100%
</style>


