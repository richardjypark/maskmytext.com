import * as wasm from "mask-my-text";

wasm.greet();

export function maskText(text, maskWords) {
  return wasm.mask_text(text, new Set(maskWords));
}

if (typeof window !== "undefined") {
  window.maskText = maskText;
}
