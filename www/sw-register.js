import {
  SW_MESSAGES,
  SW_EVENTS,
  SW_STATES,
  SW_PATHS,
  SW_LOGS,
} from "./sw-constants.js";

/**
 * Handles the service worker update process
 * @param {ServiceWorkerRegistration} registration
 */
function handleServiceWorkerUpdate(registration) {
  registration.addEventListener(SW_EVENTS.UPDATE_FOUND, () => {
    const newWorker = registration.installing;

    newWorker?.addEventListener(SW_EVENTS.STATE_CHANGE, () => {
      // When the new service worker is installed and waiting
      if (
        newWorker.state === SW_STATES.INSTALLED &&
        navigator.serviceWorker.controller
      ) {
        // Immediately tell it to skip waiting
        newWorker.postMessage({ action: SW_MESSAGES.SKIP_WAITING });
      }
    });
  });
}

/**
 * Handles page refresh when service worker takes control
 */
function handleControllerChange() {
  let refreshing = false;
  navigator.serviceWorker.addEventListener(SW_EVENTS.CONTROLLER_CHANGE, () => {
    if (!refreshing) {
      console.log(SW_LOGS.CONTROLLER_CHANGED);
      window.location.reload();
      refreshing = true;
    }
  });
}

/**
 * Registers the service worker and sets up update handling
 * @returns {Promise<ServiceWorkerRegistration|null>}
 */
export async function registerServiceWorker() {
  if (!("serviceWorker" in navigator)) {
    return null;
  }

  try {
    console.log(SW_LOGS.REGISTRATION_START);

    const registration = await navigator.serviceWorker.register(
      SW_PATHS.SCRIPT
    );
    console.log(SW_LOGS.REGISTRATION_SUCCESS);

    handleServiceWorkerUpdate(registration);
    handleControllerChange();

    return registration;
  } catch (error) {
    console.error(SW_LOGS.REGISTRATION_FAILED, error);
    return null;
  }
}
