extern crate dlopen;

#[macro_use]
extern crate dlopen_derive;

use dlopen::wrapper::{ Container, WrapperApi };

const MODULE_NAME: &str = "../Add.dll";

extern "C" fn test_sub_fn (a: i32, b: i32) -> i32 { a - b }

pub struct TestInData {
	pub sub: Option<unsafe extern "C" fn (a: i32, b: i32) -> i32>,
}

#[derive(WrapperApi)]
pub struct TestDllApi<> {
	Add: fn (a: i32, b: i32) -> i32,
	CallSub: fn (in_data: &TestInData) -> i32,
}

fn call_test_dll() {
	//* Load DLL
	let container: Container<TestDllApi> = unsafe {
		Container::load (MODULE_NAME)
	}.expect ("Cannot load library");


	//* Call Function
	let test_add_result: i32 = container.Add (1, 2);  // expect 3
	println! ("Add result: {}", test_add_result);


	//* Apply Callback and Call
	// let test_sub_result: i32 = container.CallSub (&TestInData { Sub: None });  // expect "STATUS_ACCESS_VIOLATION"
	let test_sub_result: i32 = container.CallSub (&TestInData {  // expect 1
		sub: Some (test_sub_fn)
	});

	println! ("Call Sub result: {}", test_sub_result);
}

fn main() {
	call_test_dll();

	println! ("Hello, world!");
}
