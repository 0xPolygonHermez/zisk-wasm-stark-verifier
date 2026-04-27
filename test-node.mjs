import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

import { main, verify_stark } from './pkg-node/zisk_wasm_stark_verifier.js';

async function testVerification() {
  console.log('🧪 Testing ZisK WASM STARK Verifier\n');

  try {
    main();
    console.log('✅ WASM module initialized\n');

    // Load eth block proof and verification key
    const kbProofPath = path.join(__dirname, 'proofs', 'zisk.proof.bin');
    const kbVkPath = path.join(__dirname, 'vks', 'zisk.vk.bin');

    console.log('\nLoading eth proof and verification key...');
    const kbProofBytes = fs.readFileSync(kbProofPath);
    const kbVkBytes = fs.readFileSync(kbVkPath);

    console.log(`  Proof size: ${kbProofBytes.length} bytes`);
    console.log(`  VK size: ${kbVkBytes.length} bytes`);

    // Test verification
    console.log('\n🔍 Verifying Zisk 18885301 block proof...');
    const start = performance.now();
    const kbResult = verify_stark(kbProofBytes, kbVkBytes);
    const end = performance.now();
    console.log(`✅ Verification result: ${kbResult}`);

    console.log('\n📊 Verification Summary:');
    console.log(`  STARK proof: ${kbResult ? '✅ VALID' : '❌ INVALID'}`);
    console.log(`  Time taken: ${end - start} milliseconds`);
  } catch (error) {
    console.error('❌ Error during verification:', error.message);
    console.error('Stack trace:', error.stack);
    process.exit(1);
  }
}

testVerification();