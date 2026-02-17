const { test, expect } = require("@playwright/test");

test("core input UI is visible after initial load", async ({ page }) => {
  await page.goto("/");

  await expect(page.locator("#input")).toBeVisible();

  const state = await page.evaluate(() => {
    const rootClass = document.documentElement.className;
    const content = document.querySelector(".content-section");
    return {
      rootClass,
      display: content ? getComputedStyle(content).display : "missing",
    };
  });

  expect(state.rootClass.includes("loading")).toBe(false);
  expect(state.display).not.toBe("none");
});
