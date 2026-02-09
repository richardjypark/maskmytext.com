export const SW_MESSAGES = {
  SKIP_WAITING: "skipWaiting",
  CACHE_UPDATED: "CACHE_UPDATED",
};

export const SW_EVENTS = {
  CONTROLLER_CHANGE: "controllerchange",
  UPDATE_FOUND: "updatefound",
  STATE_CHANGE: "statechange",
  MESSAGE: "message",
};

export const SW_STATES = {
  INSTALLED: "installed",
};

export const SW_PATHS = {
  DEVELOPMENT: "./service-worker.js",
  PRODUCTION: "/service-worker.js",
  GITHUB_PAGES: "/maskmytext.com/service-worker.js",
};

export const SW_LOGS = {
  REGISTRATION_START: "Registering service worker with path:",
  REGISTRATION_SUCCESS: "Service worker registration successful",
  REGISTRATION_FAILED: "Service worker registration failed:",
  CONTROLLER_CHANGED: "Service worker controller changed",
  UPDATE_AVAILABLE: "New service worker available",
};
