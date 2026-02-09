import { decodeObfuscatedText, maskText } from "./index.js";
import { registerServiceWorker } from "./sw-register.js";
import {
  clearMaskWords,
  clearTheme,
  loadMaskMode,
  loadMaskWords,
  loadTheme,
  saveMaskMode,
  saveMaskWords,
  saveTheme,
} from "./src/state/storage.js";
import { renderWordChips } from "./src/ui/word-chips.js";

function parseWordInput(input) {
  return input
    .split(",")
    .map((word) => word.trim())
    .filter((word) => word.length > 0);
}

function showTemporaryButtonMessage(button, message, timeoutMs = 2000) {
  const originalText = button.textContent;
  button.textContent = message;
  setTimeout(() => {
    button.textContent = originalText;
  }, timeoutMs);
}

function isDarkPreference() {
  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

function hasSavedTheme() {
  try {
    return localStorage.getItem("theme") !== null;
  } catch (error) {
    console.warn("Unable to check theme preference in localStorage:", error);
    return false;
  }
}

function setTheme(theme, themeToggle) {
  document.documentElement.setAttribute("data-theme", theme);
  themeToggle.textContent = theme === "dark" ? "☀️ Light Mode" : "🌙 Dark Mode";
}

function setMaskMode(mode) {
  document.documentElement.setAttribute("data-mask-mode", mode);
  document.documentElement.setAttribute(
    "data-mask-words-heading",
    mode === "asterisks" ? "Words to Mask" : "Words to Obfuscate"
  );
  document.documentElement.setAttribute(
    "data-output-heading",
    mode === "asterisks" ? "Masked Output" : "Obfuscated Output"
  );
}

function updateModeText(mode, wordsHeading, outputHeading, decodeSection) {
  wordsHeading.textContent = mode === "asterisks" ? "Words to Mask" : "Words to Obfuscate";
  outputHeading.textContent = mode === "asterisks" ? "Masked Output" : "Obfuscated Output";
  decodeSection.style.display = mode === "field_numbers" ? "block" : "none";
}

export function initApp() {
  const inputTextarea = document.getElementById("input");
  const outputDiv = document.getElementById("output");
  const clearBtn = document.getElementById("clear-btn");
  const copyBtn = document.getElementById("copy-btn");
  const themeToggle = document.getElementById("theme-toggle");
  const maskWordsInput = document.getElementById("mask-words-input");
  const wordChipsContainer = document.getElementById("word-chips");
  const clearWordsBtn = document.getElementById("clear-words-btn");
  const settingsBtn = document.getElementById("settings-btn");
  const settingsPopup = document.getElementById("settings-popup");
  const clearDataBtn = document.getElementById("clear-data-btn");
  const closeToastBtn = document.getElementById("close-toast");
  const laterUpdateBtn = document.getElementById("later-update");
  const updateNowBtn = document.getElementById("update-now");
  const decodeInput = document.getElementById("decode-input");
  const decodeOutput = document.getElementById("decode-output");
  const clearDecodeBtn = document.getElementById("clear-decode-btn");
  const copyDecodeBtn = document.getElementById("copy-decode-btn");
  const wordsHeading = document.getElementById("words-heading");
  const outputHeading = document.getElementById("output-heading");
  const decodeSection = document.getElementById("decode-section");
  const toastNotification = document.getElementById("toast-notification");
  const updateToast = document.getElementById("update-toast");

  const requiredElements = [
    inputTextarea,
    outputDiv,
    clearBtn,
    copyBtn,
    themeToggle,
    maskWordsInput,
    wordChipsContainer,
    clearWordsBtn,
    settingsBtn,
    settingsPopup,
    clearDataBtn,
    closeToastBtn,
    laterUpdateBtn,
    updateNowBtn,
    decodeInput,
    decodeOutput,
    clearDecodeBtn,
    copyDecodeBtn,
    wordsHeading,
    outputHeading,
    decodeSection,
    toastNotification,
    updateToast,
  ];

  if (requiredElements.some((element) => element === null)) {
    throw new Error("One or more required DOM elements are missing.");
  }

  let maskWords = new Set(loadMaskWords());
  let maskMode = loadMaskMode();
  let registrationHandle = null;

  const updateMaskedText = () => {
    try {
      outputDiv.textContent = maskText(inputTextarea.value, maskWords, maskMode);
    } catch (error) {
      console.error("Error masking text:", error);
      outputDiv.textContent = "An error occurred while masking text.";
    }
  };

  const removeWord = (word) => {
    maskWords.delete(word);
    saveMaskWords([...maskWords]);
    renderWordChips(wordChipsContainer, maskWords, removeWord);
    updateMaskedText();
  };

  const applyWords = (words) => {
    let added = false;
    for (const word of words) {
      if (!maskWords.has(word)) {
        maskWords.add(word);
        added = true;
      }
    }

    if (added) {
      saveMaskWords([...maskWords]);
      renderWordChips(wordChipsContainer, maskWords, removeWord);
      updateMaskedText();
    }
  };

  const processAndAddWords = () => {
    const words = parseWordInput(maskWordsInput.value);
    if (words.length === 0) {
      return;
    }

    applyWords(words);
    maskWordsInput.value = "";
  };

  const syncDecodeOutput = () => {
    if (maskMode !== "field_numbers") {
      decodeOutput.textContent = "";
      return;
    }

    decodeOutput.textContent = decodeObfuscatedText(decodeInput.value, maskWords);
  };

  const setMode = (mode) => {
    maskMode = mode;
    setMaskMode(mode);
    saveMaskMode(mode);
    updateModeText(mode, wordsHeading, outputHeading, decodeSection);
    updateMaskedText();
    syncDecodeOutput();
  };

  const prefersDark = isDarkPreference();
  const theme = loadTheme(prefersDark);
  setTheme(theme, themeToggle);
  setMode(maskMode);
  renderWordChips(wordChipsContainer, maskWords, removeWord);
  syncDecodeOutput();

  document.querySelectorAll(".mode-toggle").forEach((button) => {
    button.addEventListener("click", () => {
      setMode(button.dataset.mode || "asterisks");
    });
  });

  inputTextarea.addEventListener("input", updateMaskedText);

  maskWordsInput.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
      event.preventDefault();
      processAndAddWords();
    }
  });

  maskWordsInput.addEventListener("blur", processAndAddWords);

  clearWordsBtn.addEventListener("click", () => {
    maskWords.clear();
    saveMaskWords([]);
    renderWordChips(wordChipsContainer, maskWords, removeWord);
    updateMaskedText();
    syncDecodeOutput();
  });

  clearBtn.addEventListener("click", () => {
    inputTextarea.value = "";
    outputDiv.textContent = "";
  });

  copyBtn.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(outputDiv.textContent);
      showTemporaryButtonMessage(copyBtn, "✅ Copied!");
      toastNotification.classList.add("show");
    } catch (error) {
      console.error("Failed to copy text:", error);
      showTemporaryButtonMessage(copyBtn, "❌ Failed to copy");
    }
  });

  closeToastBtn.addEventListener("click", () => {
    toastNotification.classList.remove("show");
  });

  settingsBtn.addEventListener("click", (event) => {
    event.stopPropagation();
    settingsPopup.classList.toggle("show");
  });

  document.addEventListener("click", (event) => {
    if (!settingsPopup.contains(event.target) && !settingsBtn.contains(event.target)) {
      settingsPopup.classList.remove("show");
    }
  });

  clearDataBtn.addEventListener("click", () => {
    maskWords.clear();
    clearMaskWords();
    clearTheme();
    renderWordChips(wordChipsContainer, maskWords, removeWord);
    updateMaskedText();
    syncDecodeOutput();

    const resetTheme = loadTheme(isDarkPreference());
    setTheme(resetTheme, themeToggle);

    settingsPopup.classList.remove("show");
    showTemporaryButtonMessage(clearDataBtn, "✓ Cleared");
  });

  const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
  mediaQuery.addEventListener("change", (event) => {
    const hasExplicitTheme = hasSavedTheme();
    if (!hasExplicitTheme) {
      setTheme(event.matches ? "dark" : "light", themeToggle);
    }
  });

  themeToggle.addEventListener("click", () => {
    const current = document.documentElement.getAttribute("data-theme");
    const next = current === "dark" ? "light" : "dark";
    setTheme(next, themeToggle);
    saveTheme(next);
  });

  decodeInput.addEventListener("input", syncDecodeOutput);

  clearDecodeBtn.addEventListener("click", () => {
    decodeInput.value = "";
    decodeOutput.textContent = "";
  });

  copyDecodeBtn.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(decodeOutput.textContent);
      showTemporaryButtonMessage(copyDecodeBtn, "✅ Copied!");
    } catch (error) {
      console.error("Failed to copy decoded text:", error);
      showTemporaryButtonMessage(copyDecodeBtn, "❌ Failed to copy");
    }
  });

  laterUpdateBtn.addEventListener("click", () => {
    updateToast.classList.remove("show");
  });

  updateNowBtn.addEventListener("click", () => {
    updateToast.classList.remove("show");
    registrationHandle?.applyUpdate();
  });

  document.documentElement.classList.remove("loading");
  document.documentElement.classList.add("js-loaded");

  updateMaskedText();

  registerServiceWorker({
    onUpdateAvailable: () => {
      updateToast.classList.add("show");
    },
    onControllerChange: () => {
      window.location.reload();
    },
  })
    .then((handle) => {
      registrationHandle = handle;
    })
    .catch((error) => {
      console.error("Service worker registration setup failed:", error);
    });
}
