
extern int ugo_builtin_println(int x);
extern int ugo_builtin_exit(int x);

int main() {
	ugo_builtin_println(42);
	return 0;
}
