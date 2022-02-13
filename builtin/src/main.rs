fn main() {
	#[cfg(not(feature = "ugo_builtin_defs"))]
	println!("hello ugo-builtin"); // TODO: print api list

	#[cfg(feature = "ugo_builtin_defs")]
	ugo_builtin::print_api_list();
}
