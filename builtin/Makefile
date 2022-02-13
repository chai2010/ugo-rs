default:
	cargo run
	cargo run --features "ugo_builtin_defs"
	make c-run
c-run:
	cargo build --release
	clang ./main.c ./target/release/libugo_builtin.a
	./a.out

clean:
