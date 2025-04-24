// Service Worker Constants
export const SW_MESSAGES = {
  SKIP_WAITING: "skipWaiting",
  CACHE_UPDATED: "CACHE_UPDATED",
};

export const SW_EVENTS = {
  CONTROLLER_CHANGE: "controllerchange",
  UPDATE_FOUND: "updatefound",
  STATE_CHANGE: "statechange",
};

export const SW_STATES = {
  INSTALLED: "installed",
};

export const SW_PATHS = {
  SCRIPT: "./service-worker.js",
};

export const SW_LOGS = {
  REGISTRATION_START: "Registering service worker...",
  REGISTRATION_SUCCESS: "Service worker registration successful",
  REGISTRATION_FAILED: "Service worker registration failed:",
  CONTROLLER_CHANGED: "Service worker controller changed",
  CACHE_FAILED: "Cache addAll failed:",
  CACHE_ITEM_FAILED: "Failed to cache:",
  FETCH_FAILED: "Fetch failed:",
  CACHE_RESPONSE_FAILED: "Failed to cache response:",
};
