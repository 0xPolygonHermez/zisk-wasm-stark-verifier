# ZisK Wasm Stark Verifier

WebAssembly bindings for the ZisK STARK verifier.

## Overview

This module builds the `verify_stark` function from `pil2-proofman` into WebAssembly, enabling STARK proof verification to run directly in both web browsers and Node.js environments.

## Usage

### Installation

```bash
npm install @ethproofs/zisk-wasm-stark-verifier
```

### React Integration

```typescript
import init, { main, verify_stark } from '@ethproofs/zisk-wasm-stark-verifier';

await init(); // Initialize WASM (if needed)
main(); // Initialize panic hook

// Verify a proof
const isValid = verify_stark(proofBytes, vkBytes);
```

### Node.js Usage

```javascript
const { main, verify_stark } = require('@ethproofs/zisk-wasm-stark-verifier');

// The Node.js version initializes automatically

main(); // Initialize panic hook
const result = verify_stark(proofBytes, vkBytes);
```

## Testing

### Installation

```bash
npm install
```

### Prerequisites

- [Rust](https://0xpolygonhermez.github.io/zisk/getting_started/quickstart.html)
- [wasm-pack](https://github.com/drager/wasm-pack)

### Building

```bash
# Build for all targets
npm run build:all
```

### Node.js Example

```bash
npm run test:node
```

This runs the Node.js example that loads proof and verification key files from the filesystem and verifies them.

### Browser Example

```bash
npm run test
```

This starts a local HTTP server at `http://localhost:8080` with a browser example that demonstrates:

- Loading the WASM module in a browser environment
- File upload interface for proof and verification key files
- Interactive STARK proof verification
- Performance metrics and detailed logging
- Error handling and user feedback

The browser example provides a complete UI for testing the WASM verifier with drag-and-drop file selection and real-time verification results.

**Note:** The browser example requires files to be served over HTTP due to WASM CORS restrictions. The included server script handles this automatically.
