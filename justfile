
watch:
	cargo leptos watch
serve:
	cargo leptos serve --release
container:
	nix build "./#container" && docker load -i result && docker run --rm --network host site-server
trace:
	cargo leptos serve --bin-features chrome-tracing
