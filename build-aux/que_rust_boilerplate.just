[group('rust')]
[group('lint')]
lint-rust: rustfmt
	echo from rust lint

lints := lints + "lint-rust"

[group('rust')]
rustfmt:

