const { test, expect } = require("@playwright/test");

/**
 * Ensures the page does not trigger a second navigation (reload)
 * on the very first visit when the Service Worker is installed.
 */
test("should not reload on first page load", async ({ page }) => {
  let mainFrameNavigations = 0;

  // Count navigations of the main frame
  page.on("framenavigated", (frame) => {
    if (frame === page.mainFrame()) {
      mainFrameNavigations += 1;
    }
  });

  // Initial navigation
  await page.goto("/");

  // Wait a bit longer than the SW installation + potential reload timeout
  await page.waitForTimeout(1500);

  // Expect only the initial navigation (no extra reload)
  expect(mainFrameNavigations).toBe(1);
});
