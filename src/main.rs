extern crate dlopen;

#[macro_use]
extern crate dlopen_derive;

use dlopen::wrapper::{ Container, WrapperApi };

const BASE_PATH: &str = "./test";
const MODULE_NAME: &str = "Add";

extern "C" fn test_sub_fn (a: i32, b: i32) -> i32 { a - b }

#[repr(C)]
pub struct TestInData {
	pub sub: Option<unsafe extern "C" fn (a: i32, b: i32) -> i32>,
}

#[derive(WrapperApi)]
pub struct TestDllApi {
	Add: fn (a: i32, b: i32) -> i32,
	CallSub: fn (in_data: &TestInData) -> i32,
}

fn call_test_dll() {
	let mut module_path: String = String::from("");  // default format is windows dll
	let os = std::env::consts::OS;

	match os {
		"windows" => { module_path = format!("{}/{}.dll", BASE_PATH, MODULE_NAME); },
		"macos" => { module_path = format!("{}/{}.plugin/Contents/MacOS/{}", BASE_PATH, MODULE_NAME, MODULE_NAME); },
		_ => { eprintln!("Cannot detect os") },
	};

	println!("<aexlo> [INFO]  - Detected OS: {}", os);
	println! ("<aexlo> [INFO]  - Loading library: {} from {}", MODULE_NAME, module_path);

	//* Load DLL
	let container: Container<TestDllApi> = unsafe {
		Container::load (module_path)
	}.expect ("Cannot load library");


	//* Call Function
	let test_add_result: i32 = container.Add (1, 2);  // expect 3
	println! ("Add result: {}", test_add_result);


	//* Apply Callback and Call
	let in_data: TestInData = TestInData { sub: Some(test_sub_fn) };
	let test_sub_result: i32 = container.CallSub (&in_data);

	println! ("Call Sub result: {}", test_sub_result);
}

fn main() {
	call_test_dll();

	println! ("Hello, world!");
}
