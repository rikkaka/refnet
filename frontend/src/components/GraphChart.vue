<template>
  <div ref="chartContainer" class="chart-container"></div>
</template>

<script setup>
import { onMounted, ref, watchEffect } from 'vue';
import * as echarts from 'echarts';

const props = defineProps({
  nodes: Array
});

const chartContainer = ref(null);
let myChart = null;


onMounted(() => {
  myChart = echarts.init(chartContainer.value);
  createChart();
});

watchEffect(() => {
  createChart();
});

function prepareGraphData(nodes) {
  let data = [];
  let links = [];
  let score_sum = nodes.reduce((acc, node) => acc + node.score, 0);

  nodes.forEach(node => {
    let author = node.author;
    if (author === " " || author === "" || author === null) {
      author = "Organization";
    }
    data.push({
      name: node.doi,
      draggable: true,
      symbolSize: node.score/score_sum * nodes.length * 60,
      // symbolSize: 60,
      itemStyle: {
        color: 'lightblue',
        borderColor: 'black',
        borderWidth: 1
      },
      label: {
        show: true,
        fontSize: 12,
        formatter: `${author}\n${node.year}`,
      },
      title: node.title
    });
  });

  nodes.forEach(node => {
    node.refs.forEach(source => {
      links.push({ source: source, target: node.doi, symbol: ['none', 'arrow'] });
    });
  })

  return { data, links };
}

function createChart() {
  if (myChart && props.nodes) {
    const { data, links } = prepareGraphData(props.nodes);
    const option = {
      tooltip: {
        show: true,
        backgroundColor: 'rgba(255, 255, 255, 0.7)',
        formatter: function (params) {
          if (params.dataType === 'node') {
            let text = params.data.title;

            // 若到某个单词结尾的空格处累计字符数达到40，换行
            let count = 0;
            for (let i = 0; i < text.length; i++) {
              count++;
              if (text[i] === ' ' && count >= 40) {
                text = text.slice(0, i) + '<br>' + text.slice(i);
                count = 0;
              }
            }

            return text;
          }
        }
      },
      series: [
        {
          type: 'graph',
          layout: 'force',
          roam: true,
          data,
          links,
          force: {
            repulsion: 400,
            gravity: 0.1,
            edgeLength: 105
          },
          lineStyle: {
            color: 'grey',
            opacity: 0.5, 
            width: 2,
            // curveness: 0.
          },
          emphasis: {
            focus: 'adjacency', 
            lineStyle: {
              opacity: 1
            }
          },
          selectedMode: 'sigle'
        }
      ]
    };

    myChart.setOption(option);

    myChart.on('click', function (params) {
      if (params.dataType === 'node') {
        window.open(`https://doi.org/${params.data.name}`);
        // openDialog(params.data);
      }
    });
  }
}
</script>

<style scoped>
.chart-container {
  /* 居中 */
  display: flex;
  justify-content: center;
  align-items: center;
  width: 90vw;
  height: 700px;
  border: grey solid 1px;
}
</style>
