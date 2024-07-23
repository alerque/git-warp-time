import 'build-aux/que.just'
import 'build-aux/que_rust_boilerplate.just'

[group('test')]
test:
	make test

[group('lint')]
lint: lint-rust
	echo from parent lint

# vim: set ft=just
