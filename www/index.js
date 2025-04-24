import * as wasm from "mask-my-text";

export function maskText(text, maskWords) {
  const maskMode = localStorage.getItem("maskMode") || "asterisks";
  if (maskMode === "asterisks") {
    return wasm.mask_text(text, new Set(maskWords));
  } else {
    return wasm.mask_text_with_fields(text, new Set(maskWords));
  }
}

export function decodeObfuscatedText(text, maskWords) {
  return wasm.decode_obfuscated_text(text, new Set(maskWords));
}

if (typeof window !== "undefined") {
  window.maskText = maskText;
  window.decodeObfuscatedText = decodeObfuscatedText;

  // Listen for messages from the service worker
  navigator.serviceWorker.addEventListener("message", (event) => {
    if (event.data && event.data.type === "CACHE_UPDATED") {
      console.log("New version detected:", event.data.version);
      // Reload the page to use the latest assets
      window.location.reload();
    }
  });
}
