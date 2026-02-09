const { test, expect } = require("@playwright/test");

test("does not reload on first page load", async ({ page }) => {
  let mainFrameNavigations = 0;

  page.on("framenavigated", (frame) => {
    if (frame === page.mainFrame()) {
      mainFrameNavigations += 1;
    }
  });

  await page.goto("/");

  await expect
    .poll(() => mainFrameNavigations, {
      timeout: 3000,
      intervals: [100, 250, 500],
    })
    .toBe(1);
});
