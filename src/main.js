const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;


async function try_parse() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // greetMsgEl.textContent = await invoke("start_download", { url: greetInputEl.value });
  await invoke("start_download", { url: greetInputEl.value });
  resetState();
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    try_parse();
    playFirstVideo();
  });
});

function resetState(){
  document.getElementById('image-container').style.display = 'flex';
  document.getElementById('video-container').style.display = 'none';
}

function playFirstVideo() {
  var firstVideo = document.getElementById('firstVideo');
  var secondVideo = document.getElementById('secondVideo');

  document.getElementById('image-container').style.display = 'none';
  document.getElementById('video-container').style.display = 'block';

  firstVideo.play();
  secondVideo.play();

  setTimeout(function(){
    firstVideo.style.display = 'none';
    secondVideo.currentTime = 0;
  }, 7000);
}