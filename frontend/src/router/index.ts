import { createRouter, createWebHistory } from 'vue-router';

import Dashboard from '../pages/Dashboard/index.vue';
import Home from '../pages/Home/index.vue';
import Login from '../pages/Login/index.vue';
import Settings from '../pages/Settings/index.vue';
import CliPaths from '../pages/Settings/CliPaths.vue';
import Environment from '../pages/Settings/Environment.vue';
import AiModels from '../pages/Settings/AiModels.vue';
import CodeCli from '../pages/Settings/CodeCli.vue';

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
      redirect: '/settings/cli-paths',
      meta: {
        title: '设置',
      },
      children: [
        {
          path: 'cli-paths',
          name: 'settings-cli-paths',
          component: CliPaths,
          meta: {
            title: 'CLI 工具路径设置',
          },
        },
        {
          path: 'environment',
          name: 'settings-environment',
          component: Environment,
          meta: {
            title: '环境变量设置',
          },
        },
        {
          path: 'ai-models',
          name: 'settings-ai-models',
          component: AiModels,
          meta: {
            title: 'AI 模型管理',
          },
        },
        {
          path: 'code-cli',
          name: 'settings-code-cli',
          component: CodeCli,
          meta: {
            title: 'Code CLI 管理',
          },
        },
      ],
    },
    {
      // Redirect unknown routes to home
      path: '/:pathMatch(.*)*',
      redirect: '/',
    },
  ],
});

// Navigation guard for route title
router.beforeEach((to, from, next) => {
  // Set page title
  if (to.meta.title) {
    document.title = `${to.meta.title} - Code AI Assistant`;
  }

  // Check authentication (if needed)
  // if (to.meta.requiresAuth) {
  //   // Check if user is authenticated
  //   // If not, redirect to login
  // }

  next();
});

export default router;
