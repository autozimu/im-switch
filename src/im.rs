#![allow(non_snake_case)]

use core_foundation::{
    base::{OSStatus, TCFType},
    string::{CFString, CFStringRef},
};

#[repr(C)]
pub struct TISInputSource {
    _private: [u8; 0],
}

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    static kTISPropertyInputSourceID: CFStringRef;

    fn TISCopyCurrentKeyboardInputSource() -> *const TISInputSource;
    fn TISCopyInputSourceForLanguage(language: CFStringRef) -> *const TISInputSource;
    fn TISSelectInputSource(input_source_ref: *const TISInputSource) -> OSStatus;
    fn TISGetInputSourceProperty(
        input_source_ref: *const TISInputSource,
        key: CFStringRef,
    ) -> CFStringRef;
}

pub fn get_input_source() -> String {
    unsafe {
        let input_source = TISCopyCurrentKeyboardInputSource();
        let input_source_id = TISGetInputSourceProperty(input_source, kTISPropertyInputSourceID);
        CFString::wrap_under_get_rule(input_source_id).to_string()
    }
}

pub fn set_input_source(language: &str) -> () {
    unsafe {
        let language = CFString::new(language);
        let input_source_ref = TISCopyInputSourceForLanguage(language.as_concrete_TypeRef());
        TISSelectInputSource(input_source_ref);
    }
}
