# Mask My Text

[![Build, Test, and Deploy](https://github.com/richardjypark/maskmytext.com/actions/workflows/deploy.yml/badge.svg?branch=master)](https://github.com/richardjypark/maskmytext.com/actions/workflows/deploy.yml)

A privacy-focused text masking tool that works entirely in your browser. Uses WebAssembly and Rust.

![Mask My Text](maskmytext.gif)

## Development Setup

1. **Clone the repository:**

   ```bash
   git clone https://github.com/richardjypark/maskmytext.com.git
   cd maskmytext.com
   ```

2. **Install dependencies:**

   ```bash
   wasm-pack build
   ```

   ```bash
   cd www
   pnpm install
   ```

3. **Run the project:**

   ```bash
   pnpm run dev
   ```

## Testing

To run tests, use the following command:

```bash
wasm-pack test --headless --chrome
```

```bash
cd www
pnpm test
```

## License

MIT
