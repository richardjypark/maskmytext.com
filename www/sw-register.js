import {
  SW_EVENTS,
  SW_MESSAGES,
  SW_PATHS,
  SW_STATES,
  SW_LOGS,
} from "./sw-constants.js";

function resolveServiceWorkerPath() {
  const hostname = window.location.hostname;
  const pathname = window.location.pathname;

  if (hostname === "maskmytext.com" || hostname === "www.maskmytext.com") {
    return SW_PATHS.PRODUCTION;
  }

  if (hostname.includes("github.io") && pathname.includes("/maskmytext.com/")) {
    return SW_PATHS.GITHUB_PAGES;
  }

  return SW_PATHS.DEVELOPMENT;
}

/**
 * Registers service worker and returns a handle for update control.
 * @param {{
 *   onUpdateAvailable?: () => void,
 *   onControllerChange?: () => void,
 *   onCacheUpdated?: (version: string) => void,
 * }} options
 */
export async function registerServiceWorker(options = {}) {
  if (!("serviceWorker" in navigator)) {
    return null;
  }

  const {
    onUpdateAvailable = () => {},
    onControllerChange = () => {},
    onCacheUpdated = () => {},
  } = options;

  let currentRegistration = null;
  let waitingWorker = null;
  const hadController = Boolean(navigator.serviceWorker.controller);

  let controllerChanged = false;
  navigator.serviceWorker.addEventListener(SW_EVENTS.CONTROLLER_CHANGE, () => {
    if (controllerChanged) {
      return;
    }

    controllerChanged = true;
    console.log(SW_LOGS.CONTROLLER_CHANGED);
    if (hadController) {
      onControllerChange();
    }
  });

  navigator.serviceWorker.addEventListener(SW_EVENTS.MESSAGE, (event) => {
    if (event.data?.type === SW_MESSAGES.CACHE_UPDATED) {
      onCacheUpdated(event.data.version || "unknown");
    }
  });

  function trackWorker(worker) {
    if (!worker) {
      return;
    }

    worker.addEventListener(SW_EVENTS.STATE_CHANGE, () => {
      if (
        worker.state === SW_STATES.INSTALLED &&
        navigator.serviceWorker.controller
      ) {
        waitingWorker = currentRegistration?.waiting || worker;
        console.log(SW_LOGS.UPDATE_AVAILABLE);
        onUpdateAvailable();
      }
    });
  }

  try {
    const scriptPath = resolveServiceWorkerPath();
    console.log(SW_LOGS.REGISTRATION_START, scriptPath);

    currentRegistration = await navigator.serviceWorker.register(scriptPath);
    console.log(SW_LOGS.REGISTRATION_SUCCESS);

    if (currentRegistration.waiting && navigator.serviceWorker.controller) {
      waitingWorker = currentRegistration.waiting;
      onUpdateAvailable();
    }

    trackWorker(currentRegistration.installing);

    currentRegistration.addEventListener(SW_EVENTS.UPDATE_FOUND, () => {
      trackWorker(currentRegistration.installing);
    });

    return {
      applyUpdate() {
        const worker = currentRegistration?.waiting || waitingWorker;
        if (worker) {
          worker.postMessage({ action: SW_MESSAGES.SKIP_WAITING });
        }
      },
    };
  } catch (error) {
    console.error(SW_LOGS.REGISTRATION_FAILED, error);
    return null;
  }
}
