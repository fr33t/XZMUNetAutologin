<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onBeforeUnmount, onMounted, ref } from 'vue';
import router from '../router';
import { XZMUAccount } from '../structs';
import { info } from '@tauri-apps/plugin-log';
import fr33t_qr from '../assets/fr33t_qr.png';
import { enable, isEnabled } from '@tauri-apps/plugin-autostart';



const internet_status = ref(0);
const app_conf = ref('');
const account = ref<XZMUAccount>();
const init = async () => {
    info("init");
    const app_conf_data: string = await invoke('get_conf');
    app_conf.value = app_conf_data;
    info(app_conf.value.toString());

    const account_data: XZMUAccount = await invoke('get_account');
    account.value = account_data;
    info(account.value.toString());

    internet_status.value = await invoke("login", { account: account.value })
    info(internet_status.value.toString());
}
let timer: number | null;

onMounted(async () => {
    info('from Home.vue');
    timer = null;
    await init(); // 第一次

    if (internet_status.value !== 1) {
        timer = setInterval(async () => {
            await init();
            info("尝试连接")
        }, 5000)
    } else {
        timer = setInterval(async () => {
            await init();
            info("保持不掉线")
        }, 60000 * 60)
    }

    // 启用 autostart
    if (!await invoke('is_android')) {
        await enable();
        // 检查 enable 状态
        console.log(`registered for autostart? ${await isEnabled()}`);
    }


})
onBeforeUnmount(() => {
    if (timer) {
        clearInterval(timer);
        info("销毁定时器")
        timer = null;
    }

})
</script>

<template>

    <div>
        <div v-if="internet_status === 1">
            <v-alert class="mt-3 ml-3 mr-3 mb-3" text="欢迎使用西藏民族大学校园网络" title="您已登陆" type="success"></v-alert>
            <v-alert v-if="!account" class="mt-3 ml-3 mr-3 mb-3" text="检测到您未保存凭证！" title="未保存凭证" type="warning"
                variant="tonal"></v-alert>
            <v-alert v-else class="mt-3 ml-3 mr-3 mb-3" :text='account.username.toString()' title="当前用户" type="info"
                variant="tonal"></v-alert>
        </div>

        <div v-else-if="internet_status === 2">
            <v-alert class="mt-3 ml-3 mr-3 mb-3" text="检测到您已与互联网断开连接，但未登录校园网，可能是认证错误" title="已连接WIFI"
                type="warning"></v-alert>
        </div>

        <div v-else-if="internet_status === 3">
            <v-alert class="mt-3 ml-3 mr-3 mb-3" text="您已连接互联网，但未接入校园网" title="未接入" type="success"></v-alert>
        </div>

        <div v-else-if="internet_status === 4">
            <v-alert class="mt-3 ml-3 mr-3 mb-3" text="检测到您已与互联网断开连接，并未连接校园WIFI" title="未连接" type="error"></v-alert>
        </div>

        <div v-else-if="internet_status === -1">
            <v-alert class="mt-3 ml-3 mr-3 mb-3" text="请检查您的凭证或设备数量是否限制" title="认证失败" type="error"
                variant="tonal"></v-alert>
        </div>

        <div class="mt-3 ml-3 mr-3 mb-3">
            <v-btn rounded="lg" size="x-large" @click="router.push('/login')" block>重置凭证</v-btn>
        </div>


    </div>
    <div class="text-center"> <!-- 使内容居中 -->
        <v-img class="mb-0" cover :src="fr33t_qr" aspect-ratio="1.0"></v-img>
        <div class="mt-0 small-text">欢迎关注公众号</div>
    </div>



    <!-- <div>
        {{ internet_status }}
    </div>
    <div>
        {{ app_conf }}
    </div>
    <div>
        {{ account }}
    </div> -->
</template>
