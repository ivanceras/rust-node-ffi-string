extern crate libc;

use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use libc::c_char;

#[no_mangle]
pub extern "C" fn count_substrings(value: *const c_char, substr: *const c_char) -> *const c_char {
    let c_value = unsafe { CStr::from_ptr(value).to_bytes() };
    let c_substr = unsafe { CStr::from_ptr(substr).to_bytes() };
    println!("got: {:?} and {:?}:",c_value, c_substr);
	match str::from_utf8(c_value) {
        Ok(value) => match str::from_utf8(c_substr) {
            Ok(substr) => rust_substrings(value, substr),
            Err(_) => -1,
        },
        Err(_) => -1,
    };
	let ret = CString::new("Hello!").unwrap();
	println!("{:?}", ret);
	//ret.as_ptr()
	ret.into_raw()
}

fn rust_substrings(value: &str, substr: &str) -> i32 {
    value.matches(substr).count() as i32
}
