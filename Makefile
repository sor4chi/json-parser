build:
	cargo build --release
	@echo "Build complete!"
	mv target/release/cli format_json
	chmod +x format

