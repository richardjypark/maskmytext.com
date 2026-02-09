const CACHE_NAME = "mask-my-text-v4";
const MAX_RUNTIME_ENTRIES = 100;

const BASE_PATH = (() => {
  const hostname = location.hostname;
  const pathname = self.location.pathname;

  if (hostname === "maskmytext.com" || hostname === "www.maskmytext.com") {
    return "";
  }

  if (pathname.includes("/maskmytext.com/")) {
    return "/maskmytext.com";
  }

  return "";
})();

const APP_SHELL_PATHS = [
  `${BASE_PATH}/`,
  `${BASE_PATH}/index.html`,
  `${BASE_PATH}/bootstrap.js`,
  `${BASE_PATH}/app-init.js`,
  `${BASE_PATH}/manifest.json`,
  `${BASE_PATH}/styles/main.css`,
  `${BASE_PATH}/icons/icon-192x192.png`,
  `${BASE_PATH}/icons/icon-512x512.png`,
  `${BASE_PATH}/pkg/mask_my_text_bg.wasm`,
  `${BASE_PATH}/pkg/mask_my_text.js`,
  `${BASE_PATH}/pkg/mask_my_text_bg.js`,
];

const CACHEABLE_EXTENSIONS = new Set([
  ".html",
  ".js",
  ".css",
  ".wasm",
  ".json",
  ".png",
  ".svg",
]);

async function cacheAssets(cache) {
  try {
    await cache.addAll(APP_SHELL_PATHS);
  } catch (error) {
    console.error("Cache addAll failed:", error);
    await Promise.allSettled(
      APP_SHELL_PATHS.map((url) =>
        cache
          .add(url)
          .catch((err) => console.error("Failed to cache:", url, err))
      )
    );
  }
}

async function cleanupCaches(currentCache) {
  const cacheNames = await caches.keys();
  await Promise.all(
    cacheNames
      .filter((name) => name !== currentCache)
      .map((name) => caches.delete(name))
  );

  const cache = await caches.open(currentCache);
  const requests = await cache.keys();
  await Promise.all(
    requests
      .filter((request) =>
        request.url.includes("/maskmytext.com/maskmytext.com/")
      )
      .map((request) => cache.delete(request))
  );
}

async function notifyClients() {
  const clients = await self.clients.matchAll();
  await Promise.all(
    clients.map((client) =>
      client.postMessage({
        type: "CACHE_UPDATED",
        version: CACHE_NAME,
      })
    )
  );
}

function isCacheableRequest(request) {
  if (request.method !== "GET") {
    return false;
  }

  const url = new URL(request.url);

  if (url.origin !== self.location.origin) {
    return false;
  }

  if (url.pathname.endsWith("/")) {
    return true;
  }

  for (const extension of CACHEABLE_EXTENSIONS) {
    if (url.pathname.endsWith(extension)) {
      return true;
    }
  }

  return false;
}

async function pruneRuntimeCache(cache) {
  const requests = await cache.keys();
  if (requests.length <= MAX_RUNTIME_ENTRIES) {
    return;
  }

  const excessEntries = requests.length - MAX_RUNTIME_ENTRIES;
  const toDelete = requests.slice(0, excessEntries);
  await Promise.all(toDelete.map((request) => cache.delete(request)));
}

self.addEventListener("install", (event) => {
  event.waitUntil(caches.open(CACHE_NAME).then(cacheAssets));
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    Promise.all([cleanupCaches(CACHE_NAME), notifyClients(), self.clients.claim()])
  );
});

self.addEventListener("message", (event) => {
  if (event.data?.action === "skipWaiting") {
    self.skipWaiting();
  }
});

self.addEventListener("fetch", (event) => {
  if (event.request.mode === "navigate") {
    event.respondWith(
      fetch(event.request).catch(async () => {
        const prefixedFallback = await caches.match(`${BASE_PATH}/`);
        if (prefixedFallback) {
          return prefixedFallback;
        }

        return caches.match("/");
      })
    );
    return;
  }

  event.respondWith(
    caches.match(event.request).then(async (cachedResponse) => {
      if (cachedResponse) {
        return cachedResponse;
      }

      try {
        const response = await fetch(event.request);

        if (
          response &&
          response.status === 200 &&
          response.type === "basic" &&
          isCacheableRequest(event.request)
        ) {
          const cache = await caches.open(CACHE_NAME);
          await cache.put(event.request, response.clone());
          await pruneRuntimeCache(cache);
        }

        return response;
      } catch (error) {
        console.error("Fetch failed:", error);
        return new Response("Offline - Resource not available", {
          status: 503,
          statusText: "Service Unavailable",
          headers: new Headers({
            "Content-Type": "text/plain",
          }),
        });
      }
    })
  );
});
