const { test, expect } = require("@playwright/test");

test("recovers from malformed persisted mask words", async ({ page }) => {
  await page.addInitScript(() => {
    localStorage.setItem("maskWords", "{bad-json");
  });

  await page.goto("/");

  await expect(page.locator("#mask-words-input")).toBeVisible();
  await expect(page.locator("#word-chips")).toHaveText("");

  const storedPayload = await page.evaluate(() => localStorage.getItem("maskWords"));
  expect([null, "[]"]).toContain(storedPayload);

  await page.fill("#mask-words-input", "token");
  await page.keyboard.press("Enter");

  await expect(page.locator("#word-chips")).toContainText("token");
});
