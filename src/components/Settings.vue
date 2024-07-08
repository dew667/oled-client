<template>
    <f7-page name="settings">
      
      <div class="title">
        <h1>设置</h1>  
      </div>

      <f7-list menu-list media-list strong-ios outline-ios>
        <f7-list-item
          link="/networkConfig"
          title="网络连接"  
        >
          <template #media>
            <f7-icon f7="wifi" />
          </template>
        </f7-list-item>
        <f7-list-item
          link="/about"
          title="关于"
        >
          <template #media>
            <f7-icon f7="question_circle" />
          </template>
        </f7-list-item>
      </f7-list>
     
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