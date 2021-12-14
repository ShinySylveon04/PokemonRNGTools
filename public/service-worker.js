import { manifest, version } from '@parcel/service-worker';
import 'regenerator-runtime/runtime';

async function install() {
  const cache = await caches.open(version);
  await cache.addAll(manifest);
}
addEventListener('install', e => e.waitUntil(install()));

async function activate() {
  const keys = await caches.keys();
  await Promise.all(keys.map(key => key !== version && caches.delete(key)));
}
addEventListener('activate', e => e.waitUntil(activate()));

addEventListener('fetch', event => {
  event.respondWith(
    caches.match(event.request).then(resp => {
      return (
        resp ||
        fetch(event.request).then(response => {
          return caches.open(version).then(cache => {
            cache.put(event.request, response.clone());
            return response;
          });
        })
      );
    }),
  );
});
