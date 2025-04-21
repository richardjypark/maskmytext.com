const CACHE_NAME = "mask-my-text-v2";
const BASE_PATH = (() => {
  const hostname = location.hostname;
  const pathname = self.location.pathname;

  if (hostname === "maskmytext.com" || hostname === "www.maskmytext.com") {
    return ""; // Root path for production domain
  }

  // For GitHub Pages or other hosting with path prefix, extract just one instance of the path
  const match = pathname.match(/\/maskmytext\.com(?!\/.+\/maskmytext\.com)/);
  if (match) {
    return "/maskmytext.com";
  }

  return ""; // Default to empty path
})();
const ASSETS_TO_CACHE = [
  `${BASE_PATH}/`,
  `${BASE_PATH}/index.html`,
  `${BASE_PATH}/bootstrap.js`,
  `${BASE_PATH}/index.js`,
  `${BASE_PATH}/manifest.json`,
  `${BASE_PATH}/icons/icon-192x192.png`,
  `${BASE_PATH}/icons/icon-512x512.png`,
  // WebAssembly and generated files
  `${BASE_PATH}/pkg/mask_my_text_bg.wasm`,
  `${BASE_PATH}/pkg/mask_my_text.js`,
  `${BASE_PATH}/pkg/mask_my_text_bg.js`,
];

// Install event - cache assets
self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) =>
      cache.addAll(ASSETS_TO_CACHE).catch((error) => {
        console.error("Cache addAll failed:", error);
        // Attempt to cache files individually
        return Promise.all(
          ASSETS_TO_CACHE.map((url) =>
            cache
              .add(url)
              .catch((err) => console.error("Failed to cache:", url, err))
          )
        );
      })
    )
  );
  // Activate the new service worker immediately
  self.skipWaiting();
});

// Activate event - clean up old caches and take control
self.addEventListener("activate", (event) => {
  event.waitUntil(
    Promise.all([
      // Clean up old caches
      caches.keys().then((cacheNames) => {
        return Promise.all(
          cacheNames
            .filter((name) => name !== CACHE_NAME)
            .map((name) => caches.delete(name))
        );
      }),
      // Clear fetch cache for problematic URLs
      caches.open(CACHE_NAME).then((cache) => {
        return cache.keys().then((requests) => {
          return Promise.all(
            requests
              .filter((request) => {
                // Identify problematic URLs with duplicated paths
                const url = request.url;
                return url.includes("/maskmytext.com/maskmytext.com/");
              })
              .map((request) => {
                return cache.delete(request);
              })
          );
        });
      }),
      // Force update all clients
      self.clients.matchAll().then((clients) => {
        return Promise.all(
          clients.map((client) => {
            // Notify clients to refresh
            return client.postMessage({
              type: "CACHE_UPDATED",
              version: CACHE_NAME,
            });
          })
        );
      }),
      // Take control of all clients
      clients.claim(),
    ])
  );
});

// Fetch event - serve from cache or network with improved error handling
self.addEventListener("fetch", (event) => {
  // Navigation fallback: serve cached index page for navigation requests when offline
  if (event.request.mode === "navigate") {
    event.respondWith(fetch(event.request).catch(() => caches.match("/")));
    return;
  }

  event.respondWith(
    caches.match(event.request).then((cachedResponse) => {
      if (cachedResponse) {
        return cachedResponse;
      }

      return fetch(event.request)
        .then((response) => {
          // Don't cache if not a valid response
          if (
            !response ||
            response.status !== 200 ||
            response.type !== "basic"
          ) {
            return response;
          }

          // Clone the response as it can only be consumed once
          const responseToCache = response.clone();

          caches
            .open(CACHE_NAME)
            .then((cache) => {
              cache.put(event.request, responseToCache);
            })
            .catch((error) => {
              console.error("Failed to cache response:", error);
            });

          return response;
        })
        .catch((error) => {
          console.error("Fetch failed:", error);
          // You might want to return a custom offline page here
          return new Response("Offline - Resource not available");
        });
    })
  );
});
