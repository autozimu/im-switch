extern crate core_foundation_sys;
extern crate core_foundation;
extern crate cocoa;
extern crate libc;

use std::ffi::CStr;
use core_foundation_sys::string::{CFStringRef, CFStringGetCStringPtr, kCFStringEncodingUTF8};

trait ToStr {
    fn to_str(&self) -> &str;
}

impl ToStr for CFStringRef {
    fn to_str(&self) -> &str {
        let ptr = unsafe { CFStringGetCStringPtr(*self, kCFStringEncodingUTF8) };
        unsafe { CStr::from_ptr(ptr) }.to_str().expect("Failed to convert to str")
    }
}

enum TISInputSourceRef {}

#[link(name = "Carbon", kind = "framework")]
#[allow(non_snake_case)]
extern {
    fn TISCopyCurrentKeyboardInputSource() -> *mut TISInputSourceRef;
    fn TISGetInputSourceProperty(inputSource: *mut TISInputSourceRef, key: CFStringRef) -> CFStringRef;
    static kTISPropertyInputSourceID: CFStringRef;
}


fn main() {
    let input_source = unsafe { TISCopyCurrentKeyboardInputSource() };
    let input_source_id = unsafe { TISGetInputSourceProperty(input_source, kTISPropertyInputSourceID) };
    println!("{:?}", input_source_id.to_str());
}
