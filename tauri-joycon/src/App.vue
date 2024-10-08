<script setup>
import nipplejs from 'nipplejs';
import Slider from '@vueform/slider';
import { onMounted, reactive, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const conn = reactive({
  isConnected: false,
  ip: '192.168.4.1',
  port: 10000
})

async function    connectToServer() {
  try {
    const addr = `${conn.ip}:${conn.port}`;
    console.log('conn>>', addr)
    const response = await invoke('connect_tcp', { addr });
    console.log('response>>', response)
    conn.isConnected = true
  } catch (error) {
    alert('连接失败：' + error)
  }
}

async function disConnServer() {
  await invoke('disconnect_tcp');
  await checkConnectionStatus();
}
async function checkConnectionStatus() {
  try {
    conn.isConnected = await invoke('get_connection_status');
  } catch (error) {
    console.error('Failed to check connection status:', error);
    conn.isConnected = false;
  }
}
async function sendDataToServer(data) {
  try {
    console.log('send >>', data)
    const bytesSent = await invoke('send_data', { data }); // 打印发送的字节数
  } catch (error) {
    console.error('Failed to send data:', error);
  }
}

function handlerConnect() {
  if (conn.isConnected) {
    disConnServer()
  }else{
    connectToServer()
  }
}

const state = reactive({
  ym: 0, ym_out: 0, ym_min: 0, ym_max: 180,
  ud: 0, ud_out: 0, ud_min: 30, ud_max: 150,
  lr: 0, lr_out: 0, lr_min: 30, lr_max: 150,
})
onMounted(() => {
  var joystick_right = nipplejs.create({
    zone: document.getElementById('joystick_right'),
    mode: 'static',
    position: {left: '70%', top: '50%'},
    size: 200,
    color: 'blue',
    restJoystick: true,
    restOpacity: true,
  });
  joystick_right.on('dir:up', (evt, data) => {
    state.ud = parseInt(data.distance)
  })
  joystick_right.on('dir:down', (evt, data) => {
    state.ud = parseInt(-data.distance)
  })
  joystick_right.on('dir:left', (evt, data) => {
    state.lr = parseInt(-data.distance)
  })
  joystick_right.on('dir:right', (evt, data) => {
    state.lr = parseInt(data.distance)
  })
  joystick_right.on('end', (evt, data) => {
    state.lr = 0;
    state.ud = 0;
  })

  setInterval(() => {
    checkConnectionStatus()
  }, 1000)
})

watch(() => [state.ym, state.lr, state.ud], (newVal, oldVal) => {
  state.ym_out = state.ym;
  state.ud_out = mapRange(state.ud, state.ud_min, state.ud_max);
  state.lr_out = mapRange(state.lr, state.lr_min, state.lr_max);
  const data = state.ym_out + ',' + state.ud_out + ',' + state.lr_out + ';';
  if (conn.isConnected) {
    sendDataToServer(data);
  }
})
function mapRange(input, min, max) {
  return parseInt(
    min + (input + 100) / 200 * (max - min)
  )
}

</script>

<template>
  <div class="container">
    <div class="state">
      <div>油门：{{ state.ym_out }}
      </div>
      <div>俯仰：{{ state.ud_out }}
        <div>min<input v-model="state.ud_min"></div>
        <div>max<input v-model="state.ud_max"></div>
      </div>
      <div>左右：{{ state.lr_out }}
        <div>min<input v-model="state.lr_min"></div>
        <div>max<input v-model="state.lr_max"></div>
      </div>

      <div style="margin-top: 10px;">
        <div>IP<input v-model="conn.ip"></div>
      </div>
      <div style="margin-top: 10px;">
        <button @click="handlerConnect">{{ conn.isConnected ? '已连接，点击断开' : '建立连接'}}</button>
      </div>
    </div>
    <div id="joystick_left" class="joystick_left">
      <Slider v-model="state.ym" v-bind="{
        orientation: 'vertical',
        tooltips: false,
        direction: 'rtl',
        min: 0,
        max: 180,
        lazy: false,
      }"/>
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
.state {
  position: fixed;
  left: 0;
  top: 0;
}
.state input {
  width: 5rem;
}
.joystick_left {
  margin-left: 25%;
  .slider-base {
    height: 80vh;
    width: 30px;
  }
  .slider-target {
    height: 80vh;
  }
  .slider-touch-area{
    width: 30px;
    height: 30px;
  }
  .slider-handle{
    left: -31px;
    height: 30px;
    width: 30px;
  }
}
</style>
