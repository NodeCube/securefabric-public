// SPDX-License-Identifier: Apache-2.0

// Simple gRPC-web client using @improbable-eng/grpc-web or grpc-web generated client.
// This is a placeholder showing how to call the node using fetch and gRPC-web framing.

export async function sendMessage(url, to, topic, payload) {
 // For demo, use a simple HTTP POST to a proxy that converts to gRPC, or implement grpc-web client.
 const body = { to: arrayBufferToBase64(to), topic: arrayBufferToBase64(topic), payload: arrayBufferToBase64(payload) };
 const res = await fetch(url + '/send', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body) });
 return res.ok;
}

function arrayBufferToBase64(buf) {
 return btoa(String.fromCharCode(...new Uint8Array(buf)));
}
