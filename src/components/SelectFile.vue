<template>
    <div class="container text-center form-container">
        <form class="row mb-4" @submit.prevent="handleSubmit">
            <div class="col-12">
                <input id="path-input" class="form-control mx-auto my-auto" v-model="file_path"
                    placeholder="输入文件路径..." />
            </div>
        </form>
        <div class="canvas-container mt-4">
            <canvas id="canvas"></canvas>
        </div>
        <div v-if="file_url" class="path-info">当前文件路径: {{ file_path }}</div>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { dialog } from '@tauri-apps/api';
import QRCode from 'qrcode';

const file_path = ref('');
const file_url = ref('');
const port = ref(0);

onMounted(async () => {
    port.value = await invoke("get_port");
});

async function showAlert(message) {
    await dialog.message(message);
}

async function handleSubmit() {
    if (!file_path.value) {
        showAlert('请先选择一个文件');
        return;
    }
    const url = `http://127.0.0.1:${port.value}/generate_url`;
    const data = { "path": file_path.value };

    try {
        const response = await fetch(url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        });

        if (!response.ok) {
            throw new Error('网络响应错误');
        }

        // 处理响应数据, 生成QRCode
        const responseData = await response.json();
        console.log(responseData);
        file_url.value = responseData.url;

        // 画二维码
        QRCode.toCanvas(document.getElementById('canvas'), responseData.url, { width: 300 });

    } catch (error) {
        showAlert('上传出错:', error);
    }
}
</script>

<style scoped>
.form-container {
    margin-top: 50px;
    /* 顶部留出一些空间 */
}
.canvas-container {
    display: flex;
    justify-content: center;
    align-items: center;
}

</style>