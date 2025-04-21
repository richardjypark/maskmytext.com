# Mask My Text

A privacy-focused text masking tool that works entirely in your browser.

[![Build and Test](https://github.com/richardjypark/maskmytext.com/actions/workflows/deploy.yml/badge.svg?branch=master)](https://github.com/richardjypark/maskmytext.com/actions/workflows/deploy.yml)

## Key Features

- **Client-side only** - Your data never leaves your device
- **Works offline** - Use anytime, even without internet
- **Powered by WebAssembly** - Fast and secure text processing

## User Guide

1. Enter or paste your text
2. Choose masking options (eg. mask with \*\*\* or obfuscate)
3. Copy masked result

## Development

### Build

1. `wasm-pack build`
2. `cd www`
3. `pnpm install`

### Dependencies

1. `pnpm update`
2. `pnpm prune`

### Run

#### Local Development

1. `pnpm run dev`

### Testing

#### E2E Tests

1. Install Playwright browsers (first time only):

   ```bash
   cd www
   pnpm exec playwright install chromium
   ```

2. Run tests:
   - Headless mode: `pnpm test`
   - UI mode: `pnpm test:ui`
   - Debug mode: `pnpm test:debug`

The E2E tests verify core functionality including the WebAssembly integration by checking console output from Rust code.
