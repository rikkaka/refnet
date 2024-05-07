<template>
    <el-dialog v-model="dialogVisible" title="Tips" width="500">
        <span>This is a message</span>
        <template #footer>
            <div class="dialog-footer">
                <el-button @click="closeDialog">Cancel</el-button>
                <el-button type="primary" @click="closeDialog">
                    Confirm
                </el-button>
            </div>
        </template>
    </el-dialog>
</template>

<script setup>
import { ref } from 'vue'

const dialogVisible = ref(false)

const emit = defineEmits(['closeDialog'])


const props = defineProps({
    nodes: Array,
    dialogVisible: Boolean
});

function closeDialog() {
    emit('closeDialog')
}
</script>