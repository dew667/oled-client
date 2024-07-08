<!-- app.vue -->

<template>
  <!-- Main Framework7 App component where we pass Framework7 params -->
  <f7-app v-bind="f7params">
  <!-- Your main view,
    <!-- Initial Page -->
    <f7-page>
      <!-- Top Navbar-->
      <f7-navbar>
        <f7-link tab-link="#tab-3" href="/settings">
          <f7-icon class="gear" f7="gear_alt"></f7-icon>
        </f7-link>
      </f7-navbar>
      <f7-views tabs>
        <f7-view id="tab-1" tab tab-active url="/greet"></f7-view>  
        <f7-view id="tab-2" tab url="/raspSysInfo"></f7-view>   
        <f7-view id="tab-3" tab url="/settings"></f7-view>
        <f7-view id="tab-4" tab url="/networkConfig"></f7-view>
        <f7-view id="tab-5" tab url="/about"></f7-view>
      </f7-views>
      <f7-toolbar position="bottom" tabbar icons>   
        <f7-link
          tab-link="#tab-1"
          tab-link-active
          href="/greet"
        ><f7-icon f7="chat_bubble_text_fill"></f7-icon></f7-link>
        <f7-link
          tab-link="#tab-2"
          href="/raspSysInfo"
        ><f7-icon f7="info_circle_fil"></f7-icon></f7-link>
        <f7-link
          tab-link="#tab-3"
          href="/settings"
        ><f7-icon f7="wrench_fill"></f7-icon></f7-link>
    </f7-toolbar>
    </f7-page>
  </f7-app>
</template>
<script>  
  import routes from './router/index.js';  
  import {f7Page, f7Panel, f7App, f7Block, f7Link, f7Button, f7View, f7Navbar, f7Toolbar, f7Views, f7Icon} from "framework7-vue";
  import 'framework7-icons';

  export default {
    components: {
      f7Page,
      f7Block,
      f7Button,
      f7Link,
      f7Panel,
      f7App,
      f7View,
      f7Navbar,
      f7Toolbar,
      f7Views,
      f7Icon
    },
    data() {
      return {
        // Framework7 parameters that we pass to <f7-app> component
        f7params: {
          // Array with app routes
          routes,         
          // App Name
          name: 'My App',    
          // ...  
        } 
      }
    }
  }
</script>

<style scoped>
  .gear {
      margin-left: 20px; /* 可选：增加左外边距 */
  }
</style>