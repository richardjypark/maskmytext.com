import { describe, expect, it, vi } from "vitest";
import { createWordChip, renderWordChips } from "./word-chips.js";

describe("word chip rendering", () => {
  it("renders chip content safely as text", () => {
    const removeHandler = vi.fn();
    const chip = createWordChip("<script>alert(1)</script>", removeHandler);

    expect(chip.textContent).toContain("<script>alert(1)</script>");
    expect(chip.querySelector("script")).toBeNull();
  });

  it("binds remove handler to chip button", () => {
    const removeHandler = vi.fn();
    const chip = createWordChip("secret", removeHandler);

    chip.querySelector("button")?.click();

    expect(removeHandler).toHaveBeenCalledTimes(1);
    expect(removeHandler).toHaveBeenCalledWith("secret");
  });

  it("renders all chips and clears stale nodes", () => {
    const container = document.createElement("div");
    const removeHandler = vi.fn();

    renderWordChips(container, ["alpha", "beta"], removeHandler);
    expect(container.children).toHaveLength(2);

    renderWordChips(container, ["gamma"], removeHandler);
    expect(container.children).toHaveLength(1);
    expect(container.textContent).toContain("gamma");
    expect(container.textContent).not.toContain("alpha");
  });
});
