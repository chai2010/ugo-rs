use ugo_internal_macros::runtime_fn;

#[cfg(feature = "ugo_builtin_defs")]
pub fn print_api_list() {
	println!("TODO: print_api_list");
}

#[no_mangle]
#[runtime_fn]
pub extern "C" fn ugo_builtin_println(x: i32) -> i32 {
	println!("{}", x);
	0
}

#[no_mangle]
#[runtime_fn]
pub extern "C" fn ugo_builtin_exit(x: i32) -> i32 {
	std::process::exit(x);
}
