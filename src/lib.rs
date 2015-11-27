extern crate libc;

use std::ffi::CStr;
use std::ffi::CString;
use std::str;
use libc::c_char;

static mut initialized: bool = false;

#[no_mangle]
pub extern "C" fn get_data(arg1: *const c_char, arg2: *const c_char) -> *const c_char {
    
    unsafe {if !initialized{
        init();
        initialized = true;
    }}

    let s1 = unsafe { CStr::from_ptr(arg1) };
    let s2 = unsafe { CStr::from_ptr(arg2) };
	
	let str1 = s1.to_str().unwrap();
	let str2 = s2.to_str().unwrap();
    
    let content = retrieve_data(str1, str2);

	let ret = CString::new(content.as_bytes()).unwrap();
	ret.into_raw()
}

fn retrieve_data(arg1: &str, arg2: &str)->String{
 	println!("arg1: {}", arg1);
	println!("arg2: {}", arg2);

	let content = format!(r#"
	{{
		"message": "Hello,
		"arg1": {},
		"arg2": {}, 
	}}"#, arg1, arg2);

	println!("content: \n{}", content); 
    content
}

fn init(){
    println!("Initializing....")
}
