import { invoke } from '@tauri-apps/api/tauri'

function nextLed(awakeEnable, sleepEnable) {
    // awakeEnable: set the keyboard LED to enabled while the device is awake
    // sleepEnable: set the keyboard LED suspend animation to enabled while the device is suspended
    let shellCommand = `asusctl led-mode -n${awakeEnable? ' -a':''}${sleepEnable? ' -s':''}`;

    console.log(`executing shell command: ${shellCommand}`);

    invoke(shellCommand);
}

function prevLed(awakeEnable, sleepEnable) {
    // awakeEnable: set the keyboard LED to enabled while the device is awake
    // sleepEnable: set the keyboard LED suspend animation to enabled while the device is suspended
    let shellCommand = `asusctl led-mode -p${awakeEnable? ' -a':''}${sleepEnable? ' -s':''}`;

    console.log(`executing shell command: ${shellCommand}`);

    invoke(shellCommand);
}

function setLed(ledMode, awakeEnable, sleepEnable) {
    // ledMode: name of led pattern
    // awakeEnable: set the keyboard LED to enabled while the device is awake
    // sleepEnable: set the keyboard LED suspend animation to enabled while the device is suspended
    let shellCommand = `asusctl led-mode ${ledMode} -p${awakeEnable? ' -a':''}${sleepEnable? ' -s':''}`;

    console.log(`executing shell command: ${shellCommand}`);

    invoke(shellCommand);
}

export { nextLed, prevLed, setLed };