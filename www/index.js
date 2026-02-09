import * as wasm from "mask-my-text";

export function maskText(text, maskWords, maskMode = "asterisks") {
  if (maskMode === "asterisks") {
    return wasm.mask_text(text, new Set(maskWords));
  }

  return wasm.mask_text_with_fields(text, new Set(maskWords));
}

export function decodeObfuscatedText(text, maskWords) {
  return wasm.decode_obfuscated_text(text, new Set(maskWords));
}
