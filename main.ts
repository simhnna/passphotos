import init, { gen } from "./pkg/passphoto.js";

import Panzoom from "@panzoom/panzoom";

const cropperElement = document.getElementById("cropper") as HTMLImageElement;
const zoomRange = document.getElementById("zoom") as HTMLInputElement;
const chinlineRange = document.getElementById("chinline") as HTMLInputElement;
const resultElement = document.getElementById("img") as HTMLImageElement;
const downloadLink = document.getElementById(
  "download-btn",
) as HTMLAnchorElement;
const fileInput = document.getElementById("file") as HTMLInputElement;

const faceHeightContainer = document.getElementById("face-height-container");

const chinlineToggle = document.getElementById(
  "chinline-toggle",
) as HTMLInputElement;

const maskToggle = document.getElementById("mask-toggle");
const imageWrapper = document.getElementById("image-wrapper");

const kidsMaskToggle = document.getElementById("kids-mask");

const panzoom = Panzoom(cropperElement, {
  maxScale: 10,
});

zoomRange.addEventListener("input", (e) => {
  panzoom.zoom((e.target as HTMLInputElement).valueAsNumber);
});

chinlineRange.addEventListener("input", (e) => {
  faceHeightContainer.style.translate = `0 ${-(e.target as HTMLInputElement).valueAsNumber}px`;
});

chinlineToggle.addEventListener("input", (e) => {
  if (e.target.checked) {
    faceHeightContainer.classList.add("hidden");
    chinlineRange.classList.add("hidden");
  } else {
    faceHeightContainer.classList.remove("hidden");
    chinlineRange.classList.remove("hidden");
  }
});

maskToggle.addEventListener("input", (e) => {
  if (e.target.checked) {
    imageWrapper.style.backgroundSize = "0 0";
  } else {
    imageWrapper.style.backgroundSize = "";
  }
});

kidsMaskToggle.addEventListener("input", (e) => {
  if (e.target.checked) {
    imageWrapper.style.background = "url(./assets/face-mask-kids.png)";
    faceHeightContainer.style.background = "url(./assets/face-height-kids.png)";
  } else {
    imageWrapper.style.background = "url(./assets/face-mask.png)";
    faceHeightContainer.style.background = "url(./assets/face-height.png)";
  }
});

let offsetX = 0;
let offsetY = 0;
let imageContents;

document.getElementById("generate-button")!.addEventListener("click", () => {
  showLoading();
  zoomRange.value = panzoom.getScale();
  setTimeout(() => {
    try {
      const res = gen(new Int8Array(imageContents));
      const resultBlob = new Blob([res], { type: "image/png" });
      const url = URL.createObjectURL(resultBlob);
      resultElement.src = url;
      downloadLink.href = url;
      downloadLink.download = "passbilder.png";
      showResult();
    } catch (e) {
      reportError(e);
    }
  });
});
fileInput.addEventListener("change", function () {
  var file = this.files[0];
  var reader = new FileReader();
  reader.onload = function (e) {
    imageContents = e.target.result;
    const img = new Blob([imageContents]);
    cropperElement.src = URL.createObjectURL(img);
    panzoom.pan(0, -300);
    panzoom.zoom(1.5);
    showCropper();
  };
  reader.readAsArrayBuffer(file);
});

const selectView = document.getElementById("select-view") as HTMLDivElement;
const cropView = document.getElementById("crop-view") as HTMLDivElement;
const resultView = document.getElementById("result-view") as HTMLDivElement;
const spinner = document.getElementById("spinner") as HTMLDivElement;
const dialog = document.querySelector("dialog") as HTMLDialogElement;

function showCropper() {
  selectView.classList.add("hidden");
  cropView.classList.remove("hidden");
  resultView.classList.add("hidden");
}
function showResult() {
  selectView.classList.add("hidden");
  cropView.classList.add("hidden");
  resultView.classList.remove("hidden");
  spinner.classList.add("hidden");
}

function showSelect() {
  selectView.classList.remove("hidden");
  cropView.classList.add("hidden");
  resultView.classList.add("hidden");
}

function showLoading() {
  spinner.classList.remove("hidden");
}

for (const btn of document.getElementsByClassName("restart")) {
  btn.addEventListener("click", () => {
    fileInput.value = null;
    showSelect();
  });
}

document.querySelector("#close-dialog")!.addEventListener("click", () => {
  dialog.close();
});

function reportError(error) {
  spinner.classList.add("hidden");
  const details = dialog.querySelector("p")!;
  details.textContent =
    error instanceof Error ? error.stack || error.message : error.toString();
  dialog.showModal();
  const data = {
    error:
      error instanceof Error ? error.stack || error.message : error.toString(),
    image: {
      clientWidth: cropperElement.clientWidth,
      clientHeight: cropperElement.clientHeight,
      naturalWidth: cropperElement.naturalWidth,
      naturalHeight: cropperElement.naturalHeight,
      translate: {
        x: offsetX,
        y: offsetY,
      },
      scale: cropperElement.style.scale,
    },
  };
  fetch(
    "https://storage.hannaweb.eu/api/collections/passphoto_errors/records",
    {
      method: "POST",
      body: JSON.stringify({ data }),
      headers: {
        "Content-Type": "application/json",
      },
    },
  ).catch((e) => {
    console.error("Error reporting error", e);
  });
}

async function run() {
  await init();
}
run();
