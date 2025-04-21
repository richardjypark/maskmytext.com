import * as wasm from "mask-my-text";

wasm.greet();

export function maskText(text, maskWords) {
  const maskMode = localStorage.getItem("maskMode") || "asterisks";
  if (maskMode === "asterisks") {
    return wasm.mask_text(text, new Set(maskWords));
  } else {
    return wasm.mask_text_with_fields(text, new Set(maskWords));
  }
}

if (typeof window !== "undefined") {
  window.maskText = maskText;
}
