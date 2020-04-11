// not quite DJB2, this is for old EA games.
// TODO: change this for 1.0.0 and extract EA specific things into a specific crate
const DEFAULT_LEVEL: u32 = 0xFFFFFFFF;

pub fn djb2(input: &str) -> u32 {
	djb2_level(input, DEFAULT_LEVEL)
}

pub fn djb2_level(input: &str, level: u32) -> u32 {
	let mut hash: u32 = level;

	for c in input.chars() {
		hash = (c as u32).wrapping_add(33_u32.wrapping_mul(hash));
	}

	hash
}

pub fn djb2_upper(input: &str) -> u32 {
	djb2(input.to_ascii_uppercase().as_str())
}

pub fn djb2_level_upper(input: &str, level: u32) -> u32 {
	djb2_level(input.to_ascii_uppercase().as_str(), level)
}

pub mod bindings {
	use super::djb2 as safe_djb2;
	use super::djb2_level as safe_djb2_level;
	use super::djb2_upper as safe_djb2_upper;
	use super::djb2_level_upper as safe_djb2_level_upper;

	use std::os::raw::{c_char, c_uint};
	use std::ffi::CStr;

	#[no_mangle]
	pub extern "C" fn djb2(input: *const c_char) -> u32 {
		let c_str = unsafe { CStr::from_ptr(input).to_string_lossy().into_owned() };

		safe_djb2(&c_str)
	}

	#[no_mangle]
	pub extern "C" fn djb2_level(input: *const c_char, level: c_uint) -> u32 {
		let c_str = unsafe { CStr::from_ptr(input).to_string_lossy().into_owned() };

		safe_djb2_level(&c_str, level)
	}

	#[no_mangle]
	pub extern "C" fn djb2_upper(input: *const c_char) -> u32 {
		let c_str = unsafe { CStr::from_ptr(input).to_string_lossy().into_owned() };

		safe_djb2_upper(&c_str)
	}

	#[no_mangle]
	pub extern "C" fn djb2_level_upper(input: *const c_char, level: c_uint) -> u32 {
		let c_str = unsafe { CStr::from_ptr(input).to_string_lossy().into_owned() };

		safe_djb2_level_upper(&c_str, level)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_djb2() {
		use super::{djb2, djb2_upper};

		// U2 hashes, verified
		assert_eq!(djb2("UNDIM_COMPLETE"), 0x0153E234);
		assert_eq!(djb2("DISABLE_INPUTS"), 0x05922615);
		assert_eq!(djb2("ENABLE_INPUTS"), 0x7E4D1288);
		assert_eq!(djb2_upper("Pad_Button1"), 0xC519BFC0);
		assert_eq!(djb2_upper("Pad_Button2"), 0xC519BFC1);
		assert_eq!(djb2_upper("Pad_Button3"), 0xC519BFC2);
		assert_eq!(djb2_upper("Pad_Button4"), 0xC519BFC3);
		assert_eq!(djb2("EXIT_COMPLETE"), 0xE1FDE1D1);
		assert_eq!(djb2("FENG_CHALLENGE_MSG"), 0xF764CBE7);
	}
}