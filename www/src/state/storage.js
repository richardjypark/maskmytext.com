const STORAGE_KEYS = {
  MASK_WORDS: "maskWords",
  THEME: "theme",
  MASK_MODE: "maskMode",
};

const MAX_WORDS = 500;
const MAX_WORD_LENGTH = 256;

function safeGetItem(key) {
  try {
    return localStorage.getItem(key);
  } catch (error) {
    console.warn(`Unable to read localStorage key '${key}':`, error);
    return null;
  }
}

function safeSetItem(key, value) {
  try {
    localStorage.setItem(key, value);
  } catch (error) {
    console.warn(`Unable to write localStorage key '${key}':`, error);
  }
}

function safeRemoveItem(key) {
  try {
    localStorage.removeItem(key);
  } catch (error) {
    console.warn(`Unable to remove localStorage key '${key}':`, error);
  }
}

function sanitizeMaskWords(value) {
  if (!Array.isArray(value)) {
    return [];
  }

  const deduped = new Set();
  const words = [];

  for (const entry of value) {
    if (typeof entry !== "string") {
      continue;
    }

    const word = entry.trim();
    if (!word || word.length > MAX_WORD_LENGTH) {
      continue;
    }

    if (deduped.has(word)) {
      continue;
    }

    deduped.add(word);
    words.push(word);

    if (words.length >= MAX_WORDS) {
      break;
    }
  }

  return words;
}

export function loadMaskWords() {
  const raw = safeGetItem(STORAGE_KEYS.MASK_WORDS);
  if (raw === null) {
    return [];
  }

  try {
    const parsed = JSON.parse(raw);
    const sanitized = sanitizeMaskWords(parsed);

    if (JSON.stringify(parsed) !== JSON.stringify(sanitized)) {
      safeSetItem(STORAGE_KEYS.MASK_WORDS, JSON.stringify(sanitized));
    }

    return sanitized;
  } catch (error) {
    console.warn("Invalid maskWords payload. Resetting localStorage key.", error);
    safeRemoveItem(STORAGE_KEYS.MASK_WORDS);
    return [];
  }
}

export function saveMaskWords(words) {
  const sanitized = sanitizeMaskWords(words);
  safeSetItem(STORAGE_KEYS.MASK_WORDS, JSON.stringify(sanitized));
}

export function clearMaskWords() {
  safeRemoveItem(STORAGE_KEYS.MASK_WORDS);
}

export function loadTheme(prefersDark) {
  const stored = safeGetItem(STORAGE_KEYS.THEME);
  if (stored === "dark" || stored === "light") {
    return stored;
  }

  return prefersDark ? "dark" : "light";
}

export function saveTheme(theme) {
  safeSetItem(STORAGE_KEYS.THEME, theme);
}

export function clearTheme() {
  safeRemoveItem(STORAGE_KEYS.THEME);
}

export function loadMaskMode() {
  const stored = safeGetItem(STORAGE_KEYS.MASK_MODE);
  if (stored === "asterisks" || stored === "field_numbers") {
    return stored;
  }

  return "asterisks";
}

export function saveMaskMode(mode) {
  safeSetItem(STORAGE_KEYS.MASK_MODE, mode);
}
