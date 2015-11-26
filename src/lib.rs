extern crate libc;

use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use libc::c_char;

#[no_mangle]
pub extern "C" fn get_data(arg1: *const c_char, arg2: *const c_char) -> *const c_char {
    let s1 = unsafe { CStr::from_ptr(arg1) };
    let s2 = unsafe { CStr::from_ptr(arg2) };
	
	let str1 = s1.to_str().unwrap();
	let str2 = s2.to_str().unwrap();
	println!("str1: {}", str1);
	println!("str2: {}", str2);

	let content = format!(r#"
	{{
		"message": "Hello,
		"arg1": {},
		"arg2": {}, 
	}}"#, str1, str2);

	println!("content: \n{}", content);
	let ret = CString::new(content.as_bytes()).unwrap();
	ret.into_raw()
}

fn rust_substrings(value: &str, substr: &str) -> i32 {
    value.matches(substr).count() as i32
}
