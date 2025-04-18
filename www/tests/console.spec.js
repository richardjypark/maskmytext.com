const { test, expect } = require("@playwright/test");

test("should output the expected log message", async ({ page }) => {
  // Create a promise that will resolve with the next console message
  const consoleMessagePromise = new Promise((resolve) => {
    page.on("console", (msg) => {
      if (msg.type() === "log") {
        resolve(msg.text());
      }
    });
  });

  // Navigate to the page
  await page.goto("/");

  // Wait for and get the console message
  const logMessage = await consoleMessagePromise;

  // Verify the exact message from the Rust code
  expect(logMessage).toBe("Hello, console log message mask-my-text from Rust!");
});
