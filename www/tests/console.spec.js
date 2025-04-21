const { test, expect } = require("@playwright/test");

test("should output the expected log message", async ({ page }) => {
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

  // Check if any of the messages match our expected Rust greeting
  const expectedMessage = "Hello, console log message mask-my-text from Rust!";
  const messageFound = consoleMessages.some((msg) => msg === expectedMessage);

  // Output all collected messages for debugging
  console.log("Collected console messages:", consoleMessages);

  // Verify the Rust message was output
  expect(messageFound).toBe(true);
});
