<template>
  <div class="search-container">
    <img src="./assets/logo.png" alt="logo" style="width: 100px; height: 80px"/>
    <Params v-model="params" @search="search" />
    <el-input placeholder="请输入文献DOI" v-model="doi" class="search-input">{{
      doi
    }}</el-input>
    <el-button
      type="primary"
      @click="search"
      :icon="Search"
      :loading="isLoading"
    >
      检索
    </el-button>
  </div>

  <el-tabs type="border-card" class="body-container">
    <el-tab-pane label="关系网络">
      <GraphChart :nodes="searchResult" :key="searchResult" />
    </el-tab-pane>
    <el-tab-pane label="文献综述">
      <div class="body-container">
        <el-button type="primary" style="margin-bottom: 12px" @click="get_review" round>
          获取文献综述
        </el-button>
        <el-text
          size="large"
          style="width: 60%"
        >
          {{ review }}</el-text
        >
      </div>
    </el-tab-pane>
  </el-tabs>
</template>

<script setup>
import { ref } from "vue";
import { Search } from "@element-plus/icons-vue";

import GraphChart from "./components/GraphChart.vue";
import Params from "./components/Params.vue";

const apiURL = import.meta.env.VITE_API_URL;
const wsURL = import.meta.env.VITE_WS_URL;

const doi = ref("10.1038/s41586-024-07336-w");
const alpha = ref(0.9);
const decay_factor = ref(0.08);
const extend_num = ref(500);
const best_num = ref(20);

const params = ref({
  alpha: alpha,
  decay_factor: decay_factor,
  extend_num: extend_num,
  best_num: best_num,
});

const searchResult = ref([]);
const review = ref("");

const isLoading = ref(false);

async function search() {
  isLoading.value = true;
  review.value = "";
  const url = apiURL + "/refnet/doi";
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
    console.error("Error:", error);
  } finally {
    isLoading.value = false;
  }
}

async function get_review() {
  review.value = "";
  const dois = searchResult.value.map((node) => node.doi);
  const url = wsURL + "/refnet/review";
  const params = {
    dois: dois,
  };
  const queryString = new URLSearchParams(params).toString();

  const ws = new WebSocket(`${url}?${queryString}`);

  ws.onopen = function () {
    console.log("WebSocket Client Connected");
  };

  ws.onmessage = function (e) {
    console.log("Received: '" + e.data + "'");
    review.value += e.data;
  };

  ws.onerror = function (e) {
    console.log("Error: '" + e.data + "'");
  };
}
</script>

<style>
.search-container {
  display: grid;
  grid-template-columns: 150px 50px 320px 100px 40px;
  align-items: center;
  justify-content: center;
  /* padding-left: 10px; */
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
}

.search-input {
  width: 320px;
}

.search-button {
  width: 80px;
}

.search-result {
  font-size: 15px;
  line-height: 1.2;
}

.body-container {
  display: flex;
  flex-direction: column;
  /* justify-content: center; */
  align-items: center;
  height: 100vh;
}
</style>
