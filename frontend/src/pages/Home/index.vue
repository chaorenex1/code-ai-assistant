<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { ElButton, ElCard } from 'element-plus'
import {Setting, Document, ChatDotRound } from '@element-plus/icons-vue'

const router = useRouter()

const features = [
  {
    icon: ChatDotRound,
    title: 'AI 聊天助手',
    description: '与 AI 对话，获取代码建议和帮助',
  },
  {
    icon: Document,
    title: '智能代码编辑',
    description: '使用 AI 辅助编写和优化代码',
  },
  {
    icon: Setting,
    title: '灵活配置',
    description: '自定义设置，适应不同开发场景',
  },
]

function startCoding() {
  router.push('/dashboard')
}

function goToSettings() {
  router.push('/settings')
}
</script>

<template>
  <div class="home-page min-h-screen bg-gradient-to-br from-primary-50 to-primary-100 dark:from-gray-900 dark:to-gray-800">
    <div class="container mx-auto px-4 py-16">
      <!-- Hero Section -->
      <div class="text-center mb-16">
        <h1 class="text-5xl font-bold text-primary-600 dark:text-primary-400 mb-4">
          Code AI Assistant
        </h1>
        <p class="text-xl text-gray-600 dark:text-gray-300 mb-8">
          您的智能代码开发助手，让编程更高效、更简单
        </p>
        <div class="flex justify-center gap-4">
          <ElButton type="primary" size="large" @click="startCoding">
            <el-icon class="mr-2"><Rocket /></el-icon>
            开始使用
          </ElButton>
          <ElButton size="large" @click="goToSettings">
            <el-icon class="mr-2"><Setting /></el-icon>
            设置
          </ElButton>
        </div>
      </div>

      <!-- Features Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <ElCard v-for="(feature, index) in features" :key="index" shadow="hover" class="text-center">
          <template #header>
            <div class="flex justify-center">
              <el-icon :size="40" class="text-primary-500">
                <component :is="feature.icon" />
              </el-icon>
            </div>
          </template>
          <h3 class="text-lg font-semibold mb-2">{{ feature.title }}</h3>
          <p class="text-gray-600 dark:text-gray-400">{{ feature.description }}</p>
        </ElCard>
      </div>

      <!-- Quick Start Guide -->
      <div class="mt-16 max-w-3xl mx-auto">
        <ElCard>
          <template #header>
            <h2 class="text-2xl font-bold text-center">快速开始</h2>
          </template>
          <div class="space-y-4">
            <div class="flex items-start gap-4">
              <div class="flex-shrink-0 w-8 h-8 bg-primary-500 text-white rounded-full flex items-center justify-center font-bold">
                1
              </div>
              <div>
                <h4 class="font-semibold mb-1">选择工作目录</h4>
                <p class="text-gray-600 dark:text-gray-400">打开您的项目文件夹开始工作</p>
              </div>
            </div>
            <div class="flex items-start gap-4">
              <div class="flex-shrink-0 w-8 h-8 bg-primary-500 text-white rounded-full flex items-center justify-center font-bold">
                2
              </div>
              <div>
                <h4 class="font-semibold mb-1">与 AI 对话</h4>
                <p class="text-gray-600 dark:text-gray-400">描述您的需求，AI 将帮助您生成代码</p>
              </div>
            </div>
            <div class="flex items-start gap-4">
              <div class="flex-shrink-0 w-8 h-8 bg-primary-500 text-white rounded-full flex items-center justify-center font-bold">
                3
              </div>
              <div>
                <h4 class="font-semibold mb-1">编辑和运行</h4>
                <p class="text-gray-600 dark:text-gray-400">在编辑器中修改代码并在终端中运行</p>
              </div>
            </div>
          </div>
        </ElCard>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home-page {
  min-height: 100vh;
}
</style>
