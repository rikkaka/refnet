<template>
    <el-button @click="dialogVisible = true" :icon = "Setting" circle></el-button>
    <el-dialog v-model="dialogVisible" title="Tips" width="500">
        <div class="params-container">
      <span>扩展次数：</span>
      <el-input v-model="params.extend_num" class="param-input">{{ extend_num }}</el-input>

      <span>关键文献个数：</span>
      <el-input v-model="params.best_num" class="param-input">{{ best_num }}</el-input>

      <span>平衡因子：</span>
      <el-input v-model="params.alpha" class="param-input">{{ alpha }}</el-input>

      <span>时间衰减速率：</span>
      <el-input v-model="params.decay_factor" class="param-input">{{ decay_factor }}</el-input>
    </div>
    <template #footer>
            <div class="dialog-footer">
                <el-button type="primary" @click="search" :icon="Search" width=100px>
        检索
      </el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { ref } from 'vue'
import { Setting, Search } from '@element-plus/icons-vue'

const dialogVisible = ref(false)

const emit = defineEmits(['search'])

const params = defineModel({required: true})

function search() {
    dialogVisible.value = false
    emit('search')
}
</script>