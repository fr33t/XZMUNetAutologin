<script setup lang="ts">
import { ref } from 'vue';

import router from '../router';
import { XZMUAccount } from '../structs';
import { invoke } from '@tauri-apps/api/core';

const sa = ref(false);
;
const account = ref<XZMUAccount>({
    username: '',
    password: '',
});

const save_xzmu_account = async () => {
    const status: boolean = await invoke('save_account', {
        account: account.value
    });
    console.log(status);
    sa.value = status;
}

</script>
<template>
    <v-sheet class="mx-auto mt-10" width="300">
        <v-form fast-fail @submit.prevent>
            <v-text-field v-model="account.username" label="学号"></v-text-field>

            <v-text-field v-model="account.password" type="password" label="密码"></v-text-field>

            <v-btn class="mt-2" type="submit" @click="save_xzmu_account()" block>保存</v-btn>
            <v-btn class="mt-2" type="reset" @click="router.push('/')" block>返回</v-btn>
        </v-form>
    </v-sheet>
    <div>{{ sa }}
    </div>
</template>
<style scoped></style>