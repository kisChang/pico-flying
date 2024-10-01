<script setup>
import nipplejs from 'nipplejs';
import Slider from '@vueform/slider';
import { onMounted, reactive } from 'vue';

const state = reactive({
  ym: 0,
  config: {
    orientation: 'vertical',
    direction: 'rtl',
    min: 0,
    max: 180,
    lazy: false,
  },
  ud: 0,
  lr: 0,
})
onMounted(() => {
  var joystick_right = nipplejs.create({
    zone: document.getElementById('joystick_right'),
    mode: 'static',
    position: {left: '66%', top: '50%'},
    size: 150,
    color: 'blue',
  });
  joystick_right.on('plain:up plain:down plain:left plain:right',
      function (evt, data) {
          if (evt.type.indexOf('up')) {
            state.ud = parseInt(data.distance)
          }
          if (evt.type.indexOf('down')) {
            state.ud = parseInt(- data.distance)
          }
          if (evt.type.indexOf('left')) {
            state.lr = parseInt(- data.distance)
          }
          if (evt.type.indexOf('right')) {
            state.lr = parseInt(data.distance)
          }
      }
  )
})

</script>

<template>
  <div class="container">
    <div class="info">油门：{{ state.ym }}  浮仰：{{ state.ud }}  左右：{{ state.lr }}</div>
    <div id="joystick_left" class="joystick_left">
      <Slider v-model="state.ym" v-bind="state.config"/>
    </div>
    <div id="joystick_right"></div>
  </div>
</template>  

<style src="@vueform/slider/themes/default.css"></style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
  height: 100vh !important;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}
.joystick_left {
  margin-left: 20%;
  .slider-base {
    height: 80vh;
  }
  .slider-target {
    height: 80vh;
  }
}
</style>
