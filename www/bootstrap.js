// A dependency graph that contains wasm must be imported asynchronously.
// This bootstrap keeps the asynchronous entrypoint in one place.
import(/* webpackChunkName: "app-init" */ "./app-init.js")
  .then(({ initApp }) => initApp())
  .catch((error) => console.error("Error initializing app:", error));
