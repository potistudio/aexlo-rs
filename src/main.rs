extern crate dlopen;

#[macro_use]
extern crate dlopen_derive;

use dlopen::wrapper::{ Container, WrapperApi };

const BASE_PATH: &str = "./test";
const MODULE_NAME: &str = "SDK_Noise";

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


#[derive(WrapperApi)]
pub struct EffectMain {
	EffectMain: unsafe extern "C" fn (
		cmd: after_effects_sys::PF_Cmd,
		in_data: *mut after_effects_sys::PF_InData,
		out_data: *mut after_effects_sys::PF_OutData,
		params: after_effects_sys::PF_ParamList,
		output: *mut after_effects_sys::PF_LayerDef,
		extra: *mut ::std::os::raw::c_void,
	) -> after_effects_sys::PF_Err,
}

//* Call the plugin entry point *//
fn call_plugin() -> Result<(), dlopen::Error> {

	// Detect OS
	let mut module_path: String = String::from("");  // default format is windows dll
	let os = std::env::consts::OS;

	match os {
		"windows" => { module_path = format!("{}/{}.aex", BASE_PATH, MODULE_NAME); },
		"macos" => { module_path = format!("{}/{}.plugin/Contents/MacOS/{}", BASE_PATH, MODULE_NAME, MODULE_NAME); },
		_ => { eprintln!("Cannot detect os") },
	};

	println!("[INFO] - Detected OS: {}", os);
	println! ("[INFO] - Loading library: {} from {}", MODULE_NAME, module_path);

	// Initialize Cmd
	let cmd = after_effects_sys::PF_Cmd_GLOBAL_SETUP;

	// Initialize Interact Callbacks
	let interact_callbacks = after_effects_sys::PF_InteractCallbacks {
		checkout_param: None,
		checkin_param: None,
		add_param: None,
		abort: None,
		progress: None,
		register_ui: None,
		checkout_layer_audio: None,
		checkin_layer_audio: None,
		get_audio_data: None,
		reserved_str: [std::ptr::null_mut(); 3],
		reserved: [std::ptr::null_mut(); 10],
	};

	// Initialize InData
	let mut in_data = after_effects_sys::PF_InData {
		inter:           interact_callbacks,
		utils:           std::ptr::null_mut(),
		effect_ref:      std::ptr::null_mut(),
		quality:         after_effects_sys::PF_Quality_HI,
		version:         after_effects_sys::PF_SpecVersion { major: 13, minor: 28 },
		serial_num:      -2147483648,
		appl_id:         1180193859,
		num_params:      0,
		reserved:        0,
		what_cpu:        3,
		what_fpu:        0,
		current_time:    0,
		time_step:       1024,
		total_time:      0,
		local_time_step: 0,
		time_scale:      0,
		field:           after_effects_sys::PF_Field_UPPER,
		shutter_angle:   0,
		width:           1920,
		height:          1080,
		extent_hint:     after_effects_sys::PF_UnionableRect { left: 0, top: 0, right: 1920, bottom: 1080 },
		output_origin_x: 0,
		output_origin_y: 0,
		downsample_x:    after_effects_sys::PF_RationalScale { num: 1, den: 0 },
		downsample_y:    after_effects_sys::PF_RationalScale { num: 1, den: 0 },
		pixel_aspect_ratio: after_effects_sys::PF_RationalScale { num: 1, den: 0 },
		in_flags:        after_effects_sys::PF_InFlag_NONE,
		global_data :    std::ptr::null_mut(),
		sequence_data:   std::ptr::null_mut(),
		frame_data:      std::ptr::null_mut(),
		start_sampL:     0,
		dur_sampL:       0,
		total_sampL:     0,
		src_snd:         after_effects_sys::PF_SoundWorld { fi: after_effects_sys::PF_SoundFormatInfo { rateF: 1.0, num_channels: 2, format: 16, sample_size: 1024 }, num_samples: 1024, dataP: std::ptr::null_mut() },
		pica_basicP:     std::ptr::null_mut(),
		pre_effect_source_origin_x: 0,
		pre_effect_source_origin_y: 0,
		shutter_phase:   0
	};

	// Initialize OutData
	let mut out_data = after_effects_sys::PF_OutData {
		my_version: 0,
		name: [0; 32],
		global_data: std::ptr::null_mut(),
		num_params: 0,
		sequence_data: std::ptr::null_mut(),
		flat_sdata_size: 0,
		frame_data: std::ptr::null_mut(),
		width: 0,
		height: 0,
		origin: after_effects_sys::PF_Point { h: 0, v: 0 },
		out_flags: after_effects_sys::PF_OutFlag_NONE,
		return_msg: [0; 256],
		start_sampL: 0,
		dur_sampL: 0,
		dest_snd: after_effects_sys::PF_SoundWorld { fi: after_effects_sys::PF_SoundFormatInfo { rateF: 1024.0, num_channels: 0, format: 16, sample_size: 1024 }, num_samples: 1024, dataP: std::ptr::null_mut() },
		out_flags2: after_effects_sys::PF_OutFlag2_NONE,
	};

	// Initialize Params
	let raw_params: Vec<after_effects_sys::PF_ParamDef> = Vec::new();
	let mut params: after_effects_sys::PF_ParamList = raw_params.as_ptr() as after_effects_sys::PF_ParamList;

	// Initialize Layer
	let mut layer = after_effects_sys::PF_LayerDef {
		reserved0: std::ptr::null_mut(),
		reserved1: std::ptr::null_mut(),
		world_flags: after_effects_sys::PF_NewWorldFlag_NONE,
		data: std::ptr::null_mut(),
		rowbytes: 0,
		width: 0,
		height: 0,
		extent_hint: after_effects_sys::PF_UnionableRect { left: 0, top: 0, right: 0, bottom: 0 },
		platform_ref: std::ptr::null_mut(),
		reserved_long1: 0,
		reserved_long4: std::ptr::null_mut(),
		pix_aspect_ratio: after_effects_sys::PF_RationalScale { num: 1, den: 0 },
		reserved_long2: std::ptr::null_mut(),
		origin_x: 0,
		origin_y: 0,
		reserved_long3: 0,
		dephault: 0,
	};
	println! ("[INFO] - Plugin parameters initialized");

	// Load DLL
	let container: Container<EffectMain> = unsafe {
		Container::load (module_path)
	}.expect ("Cannot load library");
	println! ("[INFO] - Plugin loaded successfully");

	// Call Entry Point
	println! ("[DEBUG] - OutData::my_version (before): {}", out_data.my_version);
	println! ("[INFO] - Calling EffectMain with cmd: {:?}", cmd);
	unsafe {
		let result: after_effects_sys::PF_Err = container.EffectMain(cmd, &mut in_data, &mut out_data, params, &mut layer, std::ptr::null_mut());  // expect 0
		println! ("[DEBUG] - Result: {}", result);
	}
	println! ("[DEBUG] - OutData::my_version (after): {}", out_data.my_version);

	Ok(())
}


fn main() {
	// call_test_dll();
	call_plugin().expect("Failed to call plugin");

	println! ("Hello, World!");
}
