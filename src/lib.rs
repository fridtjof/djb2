use std::os::raw::c_char;
use std::ffi::CStr;

pub fn djb2(input: &str) -> u32 {
	let mut hash: u32 = 0xFFFFFFFF;

	for c in input.chars() {
		hash = (c as u32).wrapping_add(33_u32.wrapping_mul(hash));
	}

	hash
}

pub fn djb2_iter(input: &str) -> u32 {
	let initial: u32 = 0xFFFFFFFF;

	input.chars().into_iter().fold(initial, |acc, n| {
		(n as u32).wrapping_add(33_u32.wrapping_mul(acc))
	})
}
pub fn djb2u8(input: &[u8]) -> u32 {
	let mut hash: u32 = 0xFFFFFFFF;

	for c in input {
		hash = (*c as u32).wrapping_add(33_u32.wrapping_mul(hash));
	}

	hash
}


pub fn djb2_upper(input: &str) -> u32 {
	djb2(input.to_ascii_uppercase().as_str())
}

#[no_mangle]
pub extern "C" fn djb2_unsafe(input: *const c_char) -> u32 {
	let c_str = unsafe { CStr::from_ptr(input).to_string_lossy().into_owned() };

	djb2(&c_str)
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_djb2() {
		use crate::lib::{djb2, djb2_upper};

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