# Mask My Text

A WebAssembly module for text masking and obfuscation, built with Rust.

## Features

- Mask sensitive words with asterisks
- Replace sensitive words with field placeholders (FIELD_N)
- Decode obfuscated text with field placeholders
- Support for compound words:
  - Handles camelCase: `mySecretKey` → `my******Key`
  - Handles snake\*case: `user_password_123` → `user***\*\*\*\***\_123`
  - Handles hyphen-case: `user-password-123` → `user-********-123`
  - Handles uppercase words: `UserPassword` → `User********`
  - Preserves case information when using field placeholders

## Usage

```javascript
import {
  mask_text,
  mask_text_with_fields,
  decode_obfuscated_text,
} from "mask-my-text";

// Define sensitive words
const sensitiveWords = new Set(["password", "secret", "user"]);

// 1) Mask with asterisks
const masked = mask_text("My password is secret123", sensitiveWords);
// => "My ******** is ******123"

// 2) Mask with field placeholders
const obfuscated = mask_text_with_fields(
  "My UserName is admin, password is top-secret!",
  sensitiveWords
);
// => "My FIELD_1_F is admin, FIELD_2 is top-FIELD_3!"

// 3) Compound words
const maskedCompound = mask_text(
  "my user_password_123 and user-password-123",
  sensitiveWords
);
// => "my user_********_123 and user-********-123"

// 4) Decode back to original text
const decoded = decode_obfuscated_text(
  "My FIELD_1_F is admin, FIELD_2 is top-FIELD_3!",
  sensitiveWords
);
// => "My UserName is admin, password is top-secret!"
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
