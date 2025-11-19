backend:
	wasm-pack build --target web

frontend:
	rm -rf dist
	mkdir dist
	npm run build
	cp pkg/passphoto_bg.wasm public/


licences:
	cargo about generate about.hbs > public/licence.html

bundle:
	cd dist && zip -r bundle.zip *
