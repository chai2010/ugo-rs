run:
	echo 1+3-2 | cargo run

.PHONY: builtin
builtin:
	make -C ./builtin

clean:
	-rm -rf ./target
	-rm a.out*
