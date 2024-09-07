<template>
    <div class="v-container text-center form-container">
        <form class="v-row mb-4" @submit.prevent="handleSubmit">
            <div class="v-col-12">
                <v-text-field clearable label="File Path" variant="outlined" @click="openFileDialog"
                    v-model="file_path"></v-text-field>
            </div>
        </form>
        <div class="canvas-container mt-4">
            <canvas id="canvas"></canvas>
        </div>
        <div v-if="file_url" class="path-info">
            URL: <a class="text-body-2 " style="color: lawngreen;" @click="writeToClip">{{ file_url }}</a>
        </div>
        <v-snackbar v-model="snackbar" :timeout="timeout" >
            {{ text }}
            <template v-slot:actions>
                <v-btn  class="ma-1 primary" rounded="pill" @click="snackbar = false">X</v-btn>
            </template>
        </v-snackbar>
    </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { dialog } from '@tauri-apps/api';
import { writeText, readText } from '@tauri-apps/api/clipboard';
import { open } from '@tauri-apps/api/dialog';
import QRCode from 'qrcode';
const file_path = ref('');
const file_url = ref('');
const port = ref(0);
const snackbar = ref(false);
const text = ref('');
const timeout = ref(1000);

onMounted(async () => {
    port.value = await invoke("get_port");
});

async function showAlert(message) {
    await dialog.message(message);
}
async function writeToClip() {
    text.value = '已复制到剪贴板';
    if (file_url.value === await readText()) {
        snackbar.value = true;
        return;
    }
    await writeText(file_url.value);
    snackbar.value = true;
}
async function openFileDialog() {
    if (file_path.value) {
        return;
    }

    const selected = await open({ directory: false, multiple: false });
    if (selected === null) {
        await dialog.message("No file selected.");
    } else {
        file_path.value = selected;

    }
}

async function handleSubmit() {
    // Open a selection dialog for image files
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