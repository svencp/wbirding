const {   invoke } = window.__TAURI__.tauri;

import {   appendText } from './footer.js';


// function restoreWindowPositionAndSize() {
//   const windowX = 100;
//   const windowY = 100;
//   const windowWidth = 100;
//   const windowHeight = 100;;
  
//   if (windowX !== null && windowY !== null && windowWidth !== null && windowHeight !== null) {
//     const newWindowFeatures = `left=${windowX},top=${windowY},width=${windowWidth},height=${windowHeight}`;
//     window.open(window.location.href, '_self', newWindowFeatures);
//   }
//   location.reload();
// }


//@@@@@@@@@@@@@@@@ Functiosn @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

// Variables & Functions
let returnString;
let pString;

async function incoming_button_bar() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  pString = await invoke("incoming_button_bar", {
    command: returnString
  });

  console.log(pString);
  // do the command
  switch (pString) {
    case "button-bar-save":
      appendText("Not coded yet!");
      break;

    case "button-bar-increase":
      appendText("Reloading!");
      // location.reload();
      break;

    case "button-bar-decrease":
      appendText("Not coded yet!");
      break;

    case "button-bar-clear":
      appendText("The Clearies!");
      break;

    case "button-bar-exit":
      // console.log(window.screenX);

      appendText(window.screenX);
      break;

    default:
      console.log("This is not a valid fruit.");
      break;
    }
  }
  
  async function init() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    pString = await invoke("init", {   });
    console.log(pString);
}



//@@@@@@@@@@@@@@@@ Buttons @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
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


// restoreWindowPositionAndSize();

