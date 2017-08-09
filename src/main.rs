extern crate core_foundation_sys;
extern crate core_foundation;
extern crate cocoa;
extern crate libc;

use std::env;
use std::ffi::CStr;
use core_foundation_sys::string::{CFStringRef, CFStringGetCStringPtr, kCFStringEncodingUTF8, CFStringCreateWithBytes};
use core_foundation_sys::base::{Boolean, kCFAllocatorDefault, kCFAllocatorNull};
use core_foundation::base::{CFIndexConvertible};
use core_foundation::string::CFString;

trait ToStr {
    fn to_str(&self) -> &str;
}

impl ToStr for CFStringRef {
    fn to_str(&self) -> &str {
        let ptr = unsafe { CFStringGetCStringPtr(*self, kCFStringEncodingUTF8) };
        unsafe { CStr::from_ptr(ptr) }.to_str().expect("Failed to convert to str")
    }
}

trait TOCFStringRef {
    fn to_CFStringRef(&self) -> CFStringRef;
}

impl TOCFStringRef for str {
    fn to_CFStringRef(&self) -> CFStringRef {
        unsafe {
            CFStringCreateWithBytes(kCFAllocatorDefault,
                                    self.as_ptr(),
                                    self.len().to_CFIndex(),
                                    kCFStringEncodingUTF8,
                                    false as Boolean,
                                    kCFAllocatorNull)
        }
    }
}

enum TISInputSourceRef {}
#[link(name = "Carbon", kind = "framework")]
#[allow(non_snake_case)]
extern {
    fn TISCopyCurrentKeyboardInputSource() -> *mut TISInputSourceRef;
    fn TISGetInputSourceProperty(inputSource: *mut TISInputSourceRef, key: CFStringRef) -> CFStringRef;
    fn TISCopyInputSourceForLanguage(CFStringRef: CFStringRef) -> *mut TISInputSourceRef;
    static kTISPropertyInputSourceID: CFStringRef;
}


fn main() {
    // Get IM.
    let input_source = unsafe { TISCopyCurrentKeyboardInputSource() };
    let input_source_id = unsafe { TISGetInputSourceProperty(input_source, kTISPropertyInputSourceID) };
    println!("Current input method: {}", input_source_id.to_str());

    // Set IM.
    let name = env::args().nth(1).expect("Argument expected").to_CFStringRef();

    let input_source = unsafe { TISCopyInputSourceForLanguage(name) };
    let input_source_id = unsafe { TISGetInputSourceProperty(input_source, kTISPropertyInputSourceID) };
    println!("New input method: {}", input_source_id.to_str());
}
