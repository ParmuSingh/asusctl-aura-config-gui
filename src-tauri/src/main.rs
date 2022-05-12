#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::process::Command;
#[tauri::command]
fn next_led(awake_enable: String, sleep_enable: String) {
  let output = Command::new("asusctl")
      .arg("led-mode")
      .arg("-n")
      .arg("-a")
      .arg(awake_enable)
      .arg("-s")
      .arg(sleep_enable)
      .output()
      .expect("Failed to execute command");
 
}

#[tauri::command]
fn prev_led(awake_enable: String, sleep_enable: String) {
  let output = Command::new("asusctl")
      .arg("led-mode")
      .arg("-p")
      .arg("-a")
      .arg(awake_enable)
      .arg("-s")
      .arg(sleep_enable)
      .output()
      .expect("Failed to execute command");
}

#[tauri::command]
fn set_led(led_mode: String, awake_enable: String, sleep_enable: String) {
  let output = Command::new("asusctl")
      .arg("led-mode")
      .arg(led_mode)
      .arg("-a")
      .arg(awake_enable)
      .arg("-s")
      .arg(sleep_enable)
      .output()
      .expect("Failed to execute command");
}

#[tauri::command]
fn get_led_modes() -> String {
  let output = Command::new("cat")
      .arg("/etc/asusd/aura.conf")
      .output()
      .expect("Failed to execute command");

  let supportedLedModes = String::from_utf8(output.stdout).unwrap();

  return supportedLedModes;
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![next_led, prev_led, set_led, get_led_modes])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
