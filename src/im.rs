#![allow(non_snake_case)]

use failure::{bail, Fallible};

use cocoa::{base::id, foundation::NSString};
use core_foundation_sys::string::CFStringRef;
use std::ffi::CStr;

// Opaque C struct.
enum TISInputSourceRef {}

#[allow(dead_code)]
#[link(name = "Carbon", kind = "framework")]
extern "C" {
    fn TISCopyCurrentKeyboardInputSource() -> *mut TISInputSourceRef;
    fn TISGetInputSourceProperty(
        inputSource: *mut TISInputSourceRef,
        key: CFStringRef,
    ) -> CFStringRef;
    fn TISCopyInputSourceForLanguage(CFStringRef: CFStringRef) -> *mut TISInputSourceRef;
    fn TISSelectInputSource(source: *mut TISInputSourceRef) -> i64;
    static kTISPropertyLocalizedName: CFStringRef;
    static kTISPropertyInputSourceID: CFStringRef;
}

trait ToStr {
    fn to_str(&self) -> Fallible<&str>;
}

impl ToStr for CFStringRef {
    fn to_str(&self) -> Fallible<&str> {
        unsafe {
            let ptr = (*self as id).UTF8String();
            Ok(CStr::from_ptr(ptr).to_str()?)
        }
    }
}

trait TOCFStringRef {
    fn to_CFStringRef(&self) -> CFStringRef;
}

impl TOCFStringRef for str {
    fn to_CFStringRef(&self) -> CFStringRef {
        use cocoa::base::nil;

        unsafe { NSString::alloc(nil).init_str(self) as CFStringRef }
    }
}

pub fn get_input_source() -> Fallible<String> {
    let input_source = unsafe { TISCopyCurrentKeyboardInputSource() };
    let local_name = unsafe { TISGetInputSourceProperty(input_source, kTISPropertyInputSourceID) };
    Ok(local_name.to_str()?.into())
}

pub fn set_input_source(input_source_name: String) -> Fallible<()> {
    let name = input_source_name.as_str().to_CFStringRef();

    let input_source = unsafe { TISCopyInputSourceForLanguage(name) };
    let ret = unsafe { TISSelectInputSource(input_source) };
    if ret == 0 {
        Ok(())
    } else {
        bail!("Failed to set input source!: {}", ret)
    }
}
