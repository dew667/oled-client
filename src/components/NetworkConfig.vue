<template>
    <f7-page name="networkConfig">
        <div class="container">
        <div class="input-group">
        <label for="ip">IP Address:</label>
        <f7-input
          type="text"
          id="ip"
          v-model="ipAddress"
          placeholder="Enter IP Address"
          required
        />
      </div>
      <div class="input-group">
        <label for="port">Port:</label>
        <f7-input
          type="text"
          id="port"
          v-model="port"    
          placeholder="Enter Port"
          required
        />
      </div>
      <div class="button-container">
        <f7-button small tonal round style="width: 100px;" @click="save()">Save</f7-button>
      </div>
    </div>
    </f7-page>
  </template>
  
  <script setup>
    import { onMounted, ref, reactive } from "vue";
    import { invoke } from "@tauri-apps/api/core";
    import {f7Page, f7Button, f7Input, f7List, f7ListItem, f7Link} from "framework7-vue"

    const ipAddress = ref("");
    const port = ref("");

    onMounted(() => {
      getDataFromStore();
    });

    async function getDataFromStore () {
       let result = await invoke('read_config');
       let jsonObject = JSON.parse(result);
       ipAddress.value = jsonObject.ip;
       port.value = jsonObject.port;
    }

    async function save() {
      let result = await invoke('save', { ipAddress: ipAddress.value, port: port.value});
      console.log("save: ", result);
    }
  </script>
  
  <style scoped>
        .container {
  position: relative;
  max-width: 400px;
  width: 100%;
  background: #fff;
  border-radius: 0;
  text-align: center;
}

h1 {
  font-size: 24px;
  margin-bottom: 0px;
  color: #333;
}

.input-group {
  margin-bottom: 20px;
  text-align: left;
  margin-left: 20px;
  margin-right: 20px;
}

label {
  display: block;
  font-size: 16px;
  margin-bottom: 5px;
  color: #555;
}


.button-container {
    display: flex;
    justify-content: center;
}

.title {
  display: flex;
  justify-content: flex-start;
  align-items: flex-start;
  margin-left: 20px;
}
  </style>