
watch:
	cargo leptos watch
serve:
	cargo leptos serve --release
container:
	nix build "./#container" && docker load -i result && docker run --rm --network host tikv-explorer
trace:
	cargo leptos serve --bin-features chrome-tracing
