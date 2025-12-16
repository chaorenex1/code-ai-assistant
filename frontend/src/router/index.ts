import { createRouter, createWebHistory } from 'vue-router'
import Home from '../pages/Home/index.vue'
import Dashboard from '../pages/Dashboard/index.vue'
import Login from '../pages/Login/index.vue'
import Settings from '../pages/Settings/index.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
      meta: {
        title: '首页',
      },
    },
    {
      path: '/dashboard',
      name: 'dashboard',
      component: Dashboard,
      meta: {
        title: '工作台',
        requiresAuth: false, // Set to true if authentication is needed
      },
    },
    {
      path: '/login',
      name: 'login',
      component: Login,
      meta: {
        title: '登录',
      },
    },
    {
      path: '/settings',
      name: 'settings',
      component: Settings,
      meta: {
        title: '设置',
      },
    },
    {
      // Redirect unknown routes to home
      path: '/:pathMatch(.*)*',
      redirect: '/',
    },
  ],
})

// Navigation guard for route title
router.beforeEach((to, from, next) => {
  // Set page title
  if (to.meta.title) {
    document.title = `${to.meta.title} - Code AI Assistant`
  }
  
  // Check authentication (if needed)
  // if (to.meta.requiresAuth) {
  //   // Check if user is authenticated
  //   // If not, redirect to login
  // }
  
  next()
})

export default router