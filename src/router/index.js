import { createRouter, createWebHistory } from 'vue-router'
import Greet from '../components/Greet.vue'
import Settings from '../components/Settings.vue'
import RaspSysInfo from '../components/RaspSysInfo.vue'
import NetworkConfig from '../components/NetworkConfig.vue'
import About from '../components/About.vue'


const routes = [
  { path: '/greet', component: Greet },
  { path: '/settings', component: Settings },
  { path: '/raspSysInfo', component: RaspSysInfo },
  { path: '/networkConfig', component: NetworkConfig },
  { path: '/about', component: About }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})  

export default routes