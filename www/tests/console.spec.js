const { test, expect } = require("@playwright/test");

test("initializes the UI and keeps masking functional", async ({ page }) => {
  await page.goto("/");

  await expect(page.locator("html")).toHaveClass(/js-loaded/);

  await page.fill("#input", "my secret value");
  await page.fill("#mask-words-input", "secret");
  await page.keyboard.press("Enter");

  await expect(page.locator("#output")).toContainText("my ****** value");
});
