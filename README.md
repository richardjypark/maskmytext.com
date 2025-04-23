# Mask My Text

A WebAssembly module for text masking and obfuscation, built with Rust.

## Features

- Mask sensitive words with asterisks
- Replace sensitive words with field placeholders (FIELD_N)
- Decode obfuscated text with field placeholders
- Support for compound words:
  - Handles words in camelCase: `mySecretKey` becomes `my******Key`
  - Handles words in snake\*case: `user_password_123` becomes `user***\*\*\*\***\_123`
  - Handles words with uppercase: `UserPassword` becomes `User********`
  - Preserves case information when replacing with field placeholders

## Usage

```javascript
import {
  mask_text,
  mask_text_with_fields,
  decode_obfuscated_text,
} from "mask-my-text";

// Create a set of sensitive words
const sensitiveWords = new Set(["password", "secret", "user"]);

// Mask text with asterisks
const masked = mask_text("My password is secret123", sensitiveWords);
// Result: "My ******** is ******123"

// Mask text with field placeholders
const obfuscated = mask_text_with_fields(
  "My UserName is admin, password is top-secret!",
  sensitiveWords
);
// Result: "My FIELD_1_F is admin, FIELD_2 is top-FIELD_3!"

// Decode obfuscated text
const decoded = decode_obfuscated_text(
  "My FIELD_1_F is admin, FIELD_2 is top-FIELD_3!",
  sensitiveWords
);
// Result: "My UserName is admin, password is top-secret!"
```

## Building

```bash
# Build the project
wasm-pack build

# Run tests
wasm-pack test --chrome --headless
```

## License

MIT
