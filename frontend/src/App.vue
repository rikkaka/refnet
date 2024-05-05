<template>
  <el-config-provider namespace="ep">
    <div class="search-container">
      <el-input placeholder="请输入文献DOI" v-model="doi" class="search-input">{{ doi }}</el-input>
      <el-button type="primary" @click="search" class="search-button">
        检索
      </el-button>
    </div>

    <div class="params-container">
      <span>扩展次数：</span>
      <el-input v-model="extend_num" class="param-input">{{ extend_num }}</el-input>

      <span>关键文献个数：</span>
      <el-input v-model="best_num" class="param-input">{{ best_num }}</el-input>

      <span>平衡因子：</span>
      <el-input v-model="alpha" class="param-input">{{ alpha }}</el-input>

      <span>时间衰减速率：</span>
      <el-input v-model="decay_factor" class="param-input">{{ decay_factor }}</el-input>
    </div>

    <div class="graph-container">
      <GraphChart :nodes="searchResult" :key="searchResult"/>
    </div>
  </el-config-provider>
</template>

<script setup>
import { ref } from 'vue';
const apiURL = import.meta.env.VITE_API_URL;

const doi = ref('10.1038/s41586-024-07336-w');
const alpha = ref(0.9);
const decay_factor = ref(0.08);
const extend_num = ref(500);
const best_num = ref(20);

const searchResult = ref([]);

async function search() {
  const url = apiURL + '/refnet/doi';
  const params = {
    doi: doi.value,
    extend_num: extend_num.value,
    best_num: best_num.value,
    decay_factor: decay_factor.value,
    alpha: alpha.value,
  };

  const queryString = new URLSearchParams(params).toString();

  try {
    const response = await fetch(`${url}?${queryString}`);
    const data = await response.json();
    searchResult.value = data;
  } catch (error) {
    console.error('Error:', error);
  }
};

import GraphChart from './components/GraphChart.vue';

</script>


<style scoped>
.search-container {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 30px;
  margin-bottom: 18px;
}

.params-container {
  display: grid;
  grid-template-columns: 100px 100px 120px 100px;
  grid-template-rows: 50px 50px;
  align-items: center;
  justify-content: center;
  margin-top: 10px;
}

.param-input {
  width: 50px;
  margin-right: 30px;
  margin-left: 5px;
}

.search-input {
  width: 320px;
}

.search-button {
  width: 100px;
  transition: background-color 0.3s ease;
}

.search-button:hover {
  background-color: #1e5d14;
}

.search-result {
  font-size: 15px;
  line-height: 1.2;
}

.graph-container {
  display: flex;
  justify-content: center;
  align-items: center;
  border: #1e5d14 solid 1px;
}
</style>
