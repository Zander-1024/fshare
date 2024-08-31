<template>
    <form class="row" @submit.prevent="handleSubmit">
        <input id="path-input" v-model="file_path" placeholder="Enter file path..." />
        <button type="submit">提交</button>
    </form>
    <div class="canvas-container">
        <canvas id="canvas"></canvas>
    </div>

</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { dialog } from '@tauri-apps/api';
import QRCode from 'qrcode';
const file_path = ref('');
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
            throw new Error('Network response was not ok');
        }

        // 处理响应数据,将响应数据转为qrcode
        const responseData = await response.json();
        console.log(responseData);

        QRCode.toCanvas(document.getElementById('canvas'), responseData.url, { width: 300 });

    } catch (error) {
        showAlert('上传出错:', error);
    }
}


</script>

<style scoped>
.canvas-container {
    display: flex;
    justify-content: center;
    align-items: center;
    /* height: 100vh; */
}
</style>