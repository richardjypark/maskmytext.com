const CACHE_NAME = "mask-my-text-v4";

const BASE_PATH = (() => {
  const hostname = location.hostname;
  const pathname = self.location.pathname;

  if (hostname === "maskmytext.com" || hostname === "www.maskmytext.com") {
    return ""; // Root path for production domain
  }

  // For GitHub Pages or other hosting with path prefix
  if (pathname.includes("/maskmytext.com/")) {
    return "/maskmytext.com";
  }

  return ""; // Local development or other scenarios
})();

const ASSETS_TO_CACHE = [
  `${BASE_PATH}/`,
  `${BASE_PATH}/index.html`,
  `${BASE_PATH}/bootstrap.js`,
  `${BASE_PATH}/index.js`,
  `${BASE_PATH}/manifest.json`,
  `${BASE_PATH}/icons/icon-192x192.png`,
  `${BASE_PATH}/icons/icon-512x512.png`,
  `${BASE_PATH}/pkg/mask_my_text_bg.wasm`,
  `${BASE_PATH}/pkg/mask_my_text.js`,
  `${BASE_PATH}/pkg/mask_my_text_bg.js`,
];

/**
 * Attempts to cache all assets, falling back to individual caching on failure
 * @param {Cache} cache
 * @returns {Promise<void>}
 */
async function cacheAssets(cache) {
  try {
    await cache.addAll(ASSETS_TO_CACHE);
  } catch (error) {
    console.error("Cache addAll failed:", error);
    // Attempt to cache files individually
    await Promise.allSettled(
      ASSETS_TO_CACHE.map((url) =>
        cache
          .add(url)
          .catch((err) => console.error("Failed to cache:", url, err))
      )
    );
  }
}

/**
 * Cleans up old caches and problematic URLs
 * @param {string} currentCache
 * @returns {Promise<void>}
 */
async function cleanupCaches(currentCache) {
  const cacheNames = await caches.keys();
  await Promise.all([
    // Delete old caches
    ...cacheNames
      .filter((name) => name !== currentCache)
      .map((name) => caches.delete(name)),

    // Clean problematic URLs from current cache
    caches.open(currentCache).then(async (cache) => {
      const requests = await cache.keys();
      return Promise.all(
        requests
          .filter((request) =>
            request.url.includes("/maskmytext.com/maskmytext.com/")
          )
          .map((request) => cache.delete(request))
      );
    }),
  ]);
}

/**
 * Notifies all clients about cache updates
 */
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

// Install event - cache assets
self.addEventListener("install", (event) => {
  event.waitUntil(caches.open(CACHE_NAME).then(cacheAssets));
});

// Activate event - cleanup and take control
self.addEventListener("activate", (event) => {
  event.waitUntil(
    Promise.all([cleanupCaches(CACHE_NAME), notifyClients(), clients.claim()])
  );
});

// Message event - handle skip waiting
self.addEventListener("message", (event) => {
  if (event.data?.action === "skipWaiting") {
    self.skipWaiting();
  }
});

// Fetch event - serve from cache or network
self.addEventListener("fetch", (event) => {
  // Navigation fallback
  if (event.request.mode === "navigate") {
    event.respondWith(fetch(event.request).catch(() => caches.match("/")));
    return;
  }

  event.respondWith(
    caches.match(event.request).then(async (cachedResponse) => {
      if (cachedResponse) {
        return cachedResponse;
      }

      try {
        const response = await fetch(event.request);

        // Only cache valid responses
        if (!response || response.status !== 200 || response.type !== "basic") {
          return response;
        }

        // Clone and cache the response
        const responseToCache = response.clone();
        const cache = await caches.open(CACHE_NAME);
        await cache.put(event.request, responseToCache);

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
