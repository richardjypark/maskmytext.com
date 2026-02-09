import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import {
  clearMaskWords,
  clearTheme,
  loadMaskMode,
  loadMaskWords,
  loadTheme,
  saveMaskMode,
  saveMaskWords,
  saveTheme,
} from "./storage.js";

describe("storage helpers", () => {
  beforeEach(() => {
    localStorage.clear();
    vi.spyOn(console, "warn").mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("recovers from malformed maskWords payload", () => {
    localStorage.setItem("maskWords", "{invalid-json");

    const words = loadMaskWords();

    expect(words).toEqual([]);
    expect(localStorage.getItem("maskWords")).toBeNull();
  });

  it("sanitizes and persists valid maskWords entries", () => {
    saveMaskWords(["alpha", "", "alpha", "beta", 3, "   gamma   "]);

    expect(loadMaskWords()).toEqual(["alpha", "beta", "gamma"]);
    expect(localStorage.getItem("maskWords")).toBe(
      JSON.stringify(["alpha", "beta", "gamma"])
    );

    clearMaskWords();
    expect(loadMaskWords()).toEqual([]);
  });

  it("loads and persists theme + mode with safe defaults", () => {
    expect(loadTheme(true)).toBe("dark");
    expect(loadTheme(false)).toBe("light");
    expect(loadMaskMode()).toBe("asterisks");

    saveTheme("light");
    saveMaskMode("field_numbers");

    expect(loadTheme(true)).toBe("light");
    expect(loadMaskMode()).toBe("field_numbers");

    clearTheme();
    expect(loadTheme(false)).toBe("light");
  });
});
