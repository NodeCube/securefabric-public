// SPDX-License-Identifier: Apache-2.0

import React, { useState, useEffect } from 'react';
import init, { encrypt, decrypt } from 'securefabric-js';
import { sendMessage } from '../js/send_client';

export default function Home() {
 const [ready, setReady] = useState(false);
 const [key, setKey] = useState('');
 const [nonce, setNonce] = useState('');
 const [plaintext, setPlaintext] = useState('hello world');
 const [ciphertext, setCiphertext] = useState('');

 useEffect(() => {
 init().then(() => setReady(true));
 }, []);

 async function doEncrypt() {
 const k = new Uint8Array(32);
 window.crypto.getRandomValues(k);
 setKey(Buffer.from(k).toString('hex'));
 const n = new Uint8Array(12);
 window.crypto.getRandomValues(n);
 setNonce(Buffer.from(n).toString('hex'));
 const ct = encrypt(k, n, new Uint8Array(), new TextEncoder().encode(plaintext));
 setCiphertext(Buffer.from(ct).toString('base64'));
 }

 async function doDecrypt() {
 const k = hexToUint8Array(key);
 const n = hexToUint8Array(nonce);
 const pt = decrypt(k, n, new Uint8Array(), Uint8Array.from(atob(ciphertext), c=>c.charCodeAt(0)));
 setPlaintext(new TextDecoder().decode(pt));
 }

 return (
 <div>
 <h1>SecureFabric JS Demo</h1>
 <p>WASM ready: {ready ? 'yes' : 'no'}</p>
 <div>
 <textarea value={plaintext} onChange={e => setPlaintext(e.target.value)} />
 <button onClick={doEncrypt}>Encrypt</button>
 <button onClick={doDecrypt}>Decrypt</button>
 <div>Ciphertext: <pre>{ciphertext}</pre></div>
 </div>
 </div>
 );
}

function hexToUint8Array(hex) {
 if (!hex) return new Uint8Array();
 const pairs = hex.match(/.{1,2}/g) || [];
 return new Uint8Array(pairs.map(p => parseInt(p,16)));
}
