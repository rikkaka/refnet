<template>
    <div ref="chartContainer" style="width: 100%; height: 700px;"></div>
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

    nodes.forEach(node => {
        let author = node.author;
        if (author === " " || author === "" || author === null) {
            author = "Organization";
        }
        data.push({
            name: node.doi, 
            draggable: true, 
            symbolSize: 60,
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
            title: node.title,
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
          data,
          links,
          force: {
            repulsion: 400,
            edgeLength: 100
          }
        }
      ]
    };

    myChart.setOption(option);

    myChart.on('click', function (params) {
      if (params.data && params.data.name) {
        window.open(`https://doi.org/${params.data.name}`);
      }
    console.log(`https://doi.org/${params.data.name}`);
    });
    }
  }
</script>
  