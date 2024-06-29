<script setup>
import { ref } from "vue";
import { fetch } from "@tauri-apps/plugin-http"

const message = ref("");
const toastMessage = ref("");

async function greet() {
  console.log("message: ", message.value);
  const url = 'http://118.25.85.152:8080/?msg=' + message.value;
  console.log("url: ", url);
  const response = await fetch(url);
  console.log("status: ", response.status);  // e.g. 200
  console.log("text: ", response.statusText); // e.g. "OK"
  if (response.status == "200") {
      const jsonData = await response.json();
      console.log("text11: ", jsonData.body);
      // 在这里可以对获取的数据进行处理
      toastMessage.value = "Hello. This is from raspiberry pi: " + jsonData.body + ".";
      showToast()
  } else {
      toastMessage.value = "Message sent failed.";
      showToast()
  }
}

function showToast() {
    var toast = document.getElementById("toast");
    toast.className = "toast show";
    setTimeout(function() {
        toast.className = toast.className.replace("show", "");
    }, 3000);
}    
</script>

<template>
  <div class="body">
    <div class="container">
        <input type="text" placeholder="Enter your message" v-model="message">
        <button @click="greet()">Send</button>
    </div>
    <div id="toast" class="toast">{{ toastMessage }}</div> 
  </div>  
</template>

<style scoped>
.body {
    font-family: Arial, sans-serif;
    background-color: #d3d3d3; /* 浅灰色背景 */
    margin: 0;
    padding: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    overflow: hidden; /* 防止滚动条出现 */
}

.container {
    width: 100%;
    height: 100vh; /* 占满视口高度 */
    background-color: #fff;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    padding: 20px;
    box-sizing: border-box;
}

input[type="text"] {
    width: 100%;
    max-width: 400px; /* 最大宽度 */
    padding: 10px;
    margin: 10px 0;
    border: 1px solid #ccc;
    border-radius: 5px;
    box-sizing: border-box;
}

button {
    width: 100%;
    max-width: 400px; /* 最大宽度 */
    padding: 10px;
    background-color: #007bff;
    color: #fff;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    font-size: 16px;
}

button:hover {
    background-color: #0056b3;
}

.toast {
    visibility: hidden;
    min-width: 250px;
    margin-left: -125px;
    background-color: #87ceeb; /* 浅蓝色背景 */
    color: #fff;
    text-align: center;
    border-radius: 12px; /* 圆角 */
    padding: 16px;
    position: fixed;
    z-index: 1;
    left: 50%;
    top: -50px;
    font-size: 17px;
    transition: top 0.5s, visibility 0.5s;
}

.toast.show {
    visibility: visible;
    top: 30px;
}

</style>
