const { test, expect } = require("@playwright/test");

test("should output the expected service worker messages", async ({ page }) => {
  // Create an array to collect console messages
  const consoleMessages = [];

  // Listen for multiple console messages
  page.on("console", (msg) => {
    if (msg.type() === "log") {
      consoleMessages.push(msg.text());
    }
  });

  // Navigate to the page
  await page.goto("/");

  // Wait a moment for all console messages to be processed
  await page.waitForTimeout(1000);

  // Expected messages from service worker registration
  const expectedMessages = [
    "Registering service worker...",
    "Service worker registration successful",
  ];

  // Check if all expected messages were found
  const allMessagesFound = expectedMessages.every((expected) =>
    consoleMessages.some((msg) => msg.includes(expected))
  );

  // Output all collected messages for debugging
  console.log("Collected console messages:", consoleMessages);

  // Verify the service worker messages were output
  expect(allMessagesFound).toBe(true);
});
