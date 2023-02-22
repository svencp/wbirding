const {
  invoke
} = window.__TAURI__.tauri;

import {
  appendText
} from './footer.js';


let returnString;

async function incoming_button_bar() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  returnString = await invoke("incoming_button_bar", {
    command: returnString
  });
  
  console.log(returnString);
  // do the command
  switch (returnString) {
    case "button-bar-save":
      appendText("Not coded yet!");
      break;
      
      case "button-bar-increase":
      appendText("Reloading!");
      location.reload();
      break;

    case "button-bar-decrease":
      appendText("Not coded yet!");
      break;

    default:
      console.log("This is not a valid fruit.");
      break;
  }
}





const buttons = document.querySelectorAll('.button-bar-button');

buttons.forEach(button => {
  button.addEventListener('click', event => {
    const buttonId = event.target.id;
    returnString = buttonId;
    incoming_button_bar()
  });
});






window.addEventListener("DOMContentLoaded", () => {
  document
});