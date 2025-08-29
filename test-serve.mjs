import http from 'http';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const PORT = process.env.PORT || 8080;

// MIME types for different file extensions
const mimeTypes = {
  '.html': 'text/html',
  '.js': 'application/javascript',
  '.wasm': 'application/wasm',
  '.json': 'application/json',
  '.bin': 'application/octet-stream',
  '.css': 'text/css',
  '.ts': 'application/typescript',
};

function getMimeType(filePath) {
  const ext = path.extname(filePath).toLowerCase();
  return mimeTypes[ext] || 'application/octet-stream';
}

function serveFile(res, filePath) {
  const fullPath = path.join(__dirname, filePath);

  fs.readFile(fullPath, (err, data) => {
    if (err) {
      // Only log file not found for legitimate requests, not for browser probes
      if (
        !filePath.includes('favicon') &&
        !filePath.includes('ws') &&
        !filePath.includes('socket')
      ) {
        console.error(`❌ File not found: ${filePath}`);
      }
      res.writeHead(404, { 'Content-Type': 'text/plain' });
      res.end('File not found');
      return;
    }

    const mimeType = getMimeType(filePath);
    const headers = {
      'Content-Type': mimeType,
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin',
    };

    // Ensure WASM files are served with correct MIME type for streaming
    if (filePath.endsWith('.wasm')) {
      headers['Content-Type'] = 'application/wasm';
    }

    res.writeHead(200, headers);
    res.end(data);

    console.log(`📄 Served: ${filePath} (${mimeType})`);
  });
}

const server = http.createServer((req, res) => {
  let url = req.url;

  // Handle root path
  if (url === '/') {
    url = '/test-browser.html';
  }

  // Remove query parameters and decode URL
  url = decodeURIComponent(url.split('?')[0]);

  // Security check - prevent directory traversal
  if (url.includes('..')) {
    res.writeHead(400, { 'Content-Type': 'text/plain' });
    res.end('Bad Request');
    return;
  }

  // Handle WebSocket upgrade attempts (reject them gracefully)
  if (req.url === '/ws' || req.headers.upgrade === 'websocket') {
    res.writeHead(400, { 'Content-Type': 'text/plain' });
    res.end('WebSocket connections not supported by this server');
    return;
  }

  // Remove leading slash for path.join
  const filePath = url.startsWith('/') ? url.slice(1) : url;

  console.log(`🌐 Request: ${req.method} ${req.url} -> ${filePath}`);

  serveFile(res, filePath);
});

server.listen(PORT, () => {
  console.log('🚀 ZisK WASM STARK Verifier Example Server');
  console.log(`📡 Server running at http://localhost:${PORT}`);
  console.log('');
  console.log('📝 Available endpoints:');
  console.log(
    `   http://localhost:${PORT}/                     - Browser example`
  );
  console.log(
    `   http://localhost:${PORT}/pkg/                 - WASM package files`
  );
  console.log(`   http://localhost:${PORT}/proofs/              - Proof files`);
  console.log(
    `   http://localhost:${PORT}/vks/           - Verification keys`
  );
  console.log('');
  console.log(
    '💡 Open your browser and navigate to the server URL to test the WASM verifier!'
  );
  console.log('⏹️  Press Ctrl+C to stop the server');
});

// Graceful shutdown
process.on('SIGINT', () => {
  console.log('\n🛑 Shutting down server...');
  server.close(() => {
    console.log('✅ Server shut down gracefully');
    process.exit(0);
  });
});