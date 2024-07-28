backend:
	wasm-pack build --target web

frontend:
	rm -rf dist
	mkdir dist
	npm run build
	cp static/* dist/
	cp index.html licence.html style.css pkg/passphoto_bg.wasm dist/
	mkdir dist/assets
	cp assets/mask2.png dist/assets/
	mkdir dist/assets/favicon
	cp assets/favicon/site.webmanifest assets/favicon/favicon-32x32.png assets/favicon/favicon-16x16.png dist/assets/favicon
	cp assets/favicon/android-chrome-192x192.png assets/favicon/android-chrome-512x512.png dist/assets/favicon

licences:
	cargo about generate about.hbs > public/licence.html

bundle:
	cd dist && zip -r bundle.zip *
