<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElButton, ElCard, ElForm, ElFormItem, ElInput, ElMessage } from 'element-plus'
import { User, Lock } from '@element-plus/icons-vue'

const router = useRouter()

const loginForm = ref({
  username: '',
  password: '',
})

const isLoading = ref(false)

async function handleLogin() {
  if (!loginForm.value.username || !loginForm.value.password) {
    ElMessage.warning('请输入用户名和密码')
    return
  }

  isLoading.value = true
  try {
    // TODO: Implement actual login logic
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    ElMessage.success('登录成功')
    router.push('/dashboard')
  } catch (error) {
    ElMessage.error('登录失败')
  } finally {
    isLoading.value = false
  }
}

function goToHome() {
  router.push('/')
}
</script>

<template>
  <div class="login-page min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800">
    <ElCard class="w-full max-w-md" shadow="always">
      <template #header>
        <div class="text-center">
          <h1 class="text-3xl font-bold text-primary-600 dark:text-primary-400 mb-2">
            Code AI Assistant
          </h1>
          <p class="text-gray-600 dark:text-gray-400">登录以继续</p>
        </div>
      </template>

      <ElForm :model="loginForm" @submit.prevent="handleLogin">
        <ElFormItem>
          <ElInput
            v-model="loginForm.username"
            size="large"
            placeholder="用户名"
            :prefix-icon="User"
            clearable
          />
        </ElFormItem>
        <ElFormItem>
          <ElInput
            v-model="loginForm.password"
            type="password"
            size="large"
            placeholder="密码"
            :prefix-icon="Lock"
            show-password
            @keyup.enter="handleLogin"
          />
        </ElFormItem>
        <ElFormItem>
          <ElButton
            type="primary"
            size="large"
            class="w-full"
            :loading="isLoading"
            @click="handleLogin"
          >
            登录
          </ElButton>
        </ElFormItem>
      </ElForm>

      <div class="text-center mt-4">
        <ElButton text @click="goToHome">
          返回首页
        </ElButton>
      </div>
    </ElCard>
  </div>
</template>

<style scoped>
.login-page {
  min-height: 100vh;
}
</style>
