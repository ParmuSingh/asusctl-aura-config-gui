<template>

  <div class='main-app-container'>
    <div class='inside-main-container'>
      <div class='logo'>
        <img alt="ROG logo" src="./assets/asus-rog-logo.jpg" width="200" height="113">
      </div>

      <h1>ASUS ROG Aura Config Panel {{ availableLedModes }}</h1>



      <div class='led-buttons-list'>
        <button v-on:click="nextLed('true', 'true')">next led mode</button>
        <button v-on:click="prevLed('true', 'true')">prev led mode</button>
        <button v-on:click="setLed('Static', 'true', 'true')">set static led mode</button>
      </div>

      <!-- <ButtonComponent action='next-led-mode' /> -->
    </div>
    
  </div>
  
</template>

<script>

import { invoke } from '@tauri-apps/api/tauri'

const nextLed = function (awakeEnable, sleepEnable) {
  // console.log(`executing nextLed(${awakeEnable}, ${sleepEnable})`)
  // awakeEnable: set the keyboard LED to enabled while the device is awake
  // sleepEnable: set the keyboard LED suspend animation to enabled while the device is suspended
  invoke('next_led', { awakeEnable: awakeEnable, sleepEnable: sleepEnable });
}

const prevLed = function (awakeEnable, sleepEnable) {
  // awakeEnable: set the keyboard LED to enabled while the device is awake
  // sleepEnable: set the keyboard LED suspend animation to enabled while the device is suspended
  invoke('prev_led', { awakeEnable: awakeEnable, sleepEnable: sleepEnable });
}

const setLed = function (ledMode, awakeEnable, sleepEnable) {
  // ledMode: name of led pattern
  // awakeEnable: set the keyboard LED to enabled while the device is awake
  // sleepEnable: set the keyboard LED suspend animation to enabled while the device is suspended
  invoke('set_led', { ledMode: ledMode, awakeEnable: awakeEnable, sleepEnable: sleepEnable });
}


let availableLedModes;
(async ()=>{
  availableLedModes = await invoke('get_led_modes');
})();

// import HelloWorld from './components/HelloWorld.vue'
export default {
  name: 'App',
  components: {
    // HelloWorld
  },
  methods: {
    invoke,
    nextLed,
    prevLed,
    setLed
  },
  data() {
    return {
      availableLedModes: availableLedModes
    }
  }
}
</script>

<style>
.main-app-container {
  background-color: #202020;
  display: flex;
  flex-direction: row;
  justify-content: center;
  height: 100%;
}

.inside-main-container {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.logo {
  display: flex;
  flex-direction: row;
  justify-content: center;
}

h1 {
  color: rgb(209, 183, 183);
}

p {
  color: white;
}

.led-buttons-list {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  width: 100%;
}

button {
  widows: 200px;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
