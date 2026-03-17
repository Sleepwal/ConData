import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import ConnectionView from '../views/ConnectionView.vue'
import QueryView from '../views/QueryView.vue'

const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView,
    meta: { title: '首页' }
  },
  {
    path: '/connections',
    name: 'connections',
    component: ConnectionView,
    meta: { title: '连接管理' }
  },
  {
    path: '/query',
    name: 'query',
    component: QueryView,
    meta: { title: '查询执行' }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
