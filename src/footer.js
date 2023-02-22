const { invoke } = window.__TAURI__.tauri;



export function appendText(textToAppend) {
  var textarea = document.getElementById("statusArea");
  var currentText = textarea.value;
  var newText = currentText + textToAppend + "\n";
  textarea.value = newText;
  textarea.scrollTop = textarea.scrollHeight - textarea.clientHeight;
}





