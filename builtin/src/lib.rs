#[no_mangle]
pub extern "C" fn ugo_builtin_println(x: i32) -> i32 {
	println!("{}", x);
	0
}

#[no_mangle]
pub extern "C" fn ugo_builtin_exit(x: i32) -> i32 {
	std::process::exit(x);
}
