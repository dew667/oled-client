<script setup>
import { onMounted, ref, reactive } from "vue";
import { fetch } from "@tauri-apps/plugin-http";
import { invoke } from "@tauri-apps/api/core";

import {f7Page, f7Icon, f7Button, f7Input, f7Block, f7} from "framework7-vue"

const message = ref("");

const ipAddress = ref("");
const port = ref("");

const list = ref([]);
const itemSelected = ref("");  

import routes from '../router/index.js';

const f7params= reactive({
    // Array with app routes
    routes: routes,         
    // App Name
    name: 'Greet', 
})

onMounted(async () => {  
    getDataFromStore();
}) 

async function getDataFromStore () {
    let result = await invoke('read_config');
    let jsonObject = JSON.parse(result);
    ipAddress.value = jsonObject.ip;
    port.value = jsonObject.port;
    
    console.log(ipAddress.value)
    console.log(port.value)

    let messageList = await invoke('list_message');
    let jsonList = JSON.parse(messageList);
    list.value = jsonList;
    console.log("list: ", list.value);
}

async function greet() {
  // 此处url写死，可考虑采用配置文件
  const url = 'http://' + ipAddress.value + ':' + port.value + '/?msg=' + message.value;
  console.log("url: ", url);
  try {
    const response = await fetch(url);
    if (response.status == "200") {
      const jsonData = await response.json();
      // 在这里可以对获取的数据进行处理
      let text = "Hello. This is from raspiberry pi: " + jsonData.body + ".";
      showToastCenter(text);
      await invoke('save_message', {message: message.value});
    } else {
      let text = "Message sent failed.";
      showToastCenter(text);
    }
  } catch (e) {
      console.log("fetch error: ", e);
      let text = "Message sent failed.";
      showToastCenter(text);
  }
  
} 

  async function showToastCenter(text) {
    let toastCenter = f7.toast.create({
      text: text,
      position: 'center',
      closeTimeout: 2000,
    });
    // Open it
    console.log("toast: ", toastCenter);
    toastCenter.open();
  }

  function select() {
    console.log("itemSelected: ", itemSelected.value);
    message.value = itemSelected.value;
  }

</script>

<template>
    <f7-page name="greet">
      <div class="title">
        <h1>Send Message</h1>
      </div>
      <f7-card class="card"> 
        <div class="input-container">
          <f7-input type="text" placeholder="输入要发送的消息" v-model:value="message" clear-button inputStyle="width: 300px; height: 2em;"></f7-input> 
        </div>
        <div class="button-container">
          <f7-button @click="greet()" small tonal round style="width: 100px;">发送</f7-button> 
        </div>
      </f7-card>
    </f7-page>  
</template>  
   
<style scoped> 
  .input-container {
      display: flex;
      justify-content: center;
      margin-top: 0px;
      margin-bottom: 20px;
  }
  .button-container {
      display: flex;
      justify-content: center;
  }

  h1 {
    font-size: 30px;
    margin-bottom: 0px;  
    color: #333;
    padding: 50px; /* 可选：增加内边距 */
  }

  .title {
    display: flex;
    justify-content: flex-start;
    align-items: flex-start;
  }

  .card {
    height: 100px;
    padding-top: 50px;
  }
</style>
