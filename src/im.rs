#![allow(non_snake_case)]

use failure::{bail, Fallible};

use core_foundation::{
    base::{OSStatus, TCFType},
    string::{CFString, CFStringRef},
};

// Opaque C struct.
enum TISInputSourceRef {}

#[allow(dead_code)]
#[link(name = "Carbon", kind = "framework")]
extern "C" {
    static kTISPropertyInputSourceID: CFStringRef;

    fn TISCopyCurrentKeyboardInputSource() -> *const TISInputSourceRef;
    fn TISCopyInputSourceForLanguage(language: CFStringRef) -> *const TISInputSourceRef;
    fn TISSelectInputSource(input_source_ref: *const TISInputSourceRef) -> OSStatus;
    fn TISGetInputSourceProperty(
        input_source_ref: *const TISInputSourceRef,
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

pub fn set_input_source(input_source_id: &str) -> Fallible<()> {
    unsafe {
        let input_source_id = CFString::new(input_source_id).as_concrete_TypeRef();
        let input_source_ref = TISCopyInputSourceForLanguage(input_source_id);
        if TISSelectInputSource(input_source_ref) != 0 {
            bail!("Failed to set input source!");
        }
        Ok(())
    }
}
