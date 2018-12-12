#![allow(non_snake_case)]

use failure::{bail, Fallible};

use cocoa::foundation::NSString;
use core_foundation_sys::string::CFStringRef;
use std::ffi::CStr;

trait ToStr {
    fn to_str(&self) -> Fallible<&str>;
}

impl ToStr for CFStringRef {
    fn to_str(&self) -> Fallible<&str> {
        use cocoa::base::id;

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

// Opaque C struct.
enum TISInputSourceRef {}
type OSStatus = i64;

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    fn TISCopyCurrentKeyboardInputSource() -> *mut TISInputSourceRef;
    fn TISGetInputSourceProperty(
        inputSource: *mut TISInputSourceRef,
        key: CFStringRef,
    ) -> CFStringRef;
    fn TISCopyInputSourceForLanguage(CFStringRef: CFStringRef) -> *mut TISInputSourceRef;
    fn TISSelectInputSource(source: *mut TISInputSourceRef) -> OSStatus;
    static kTISPropertyLocalizedName: CFStringRef;
}

extern crate structopt;
use structopt::StructOpt;
#[macro_use]
extern crate structopt_derive;

#[derive(Debug, StructOpt)]
struct Arguments {
    /// Set up the input method to use inputmethodname
    #[structopt(short = "s")]
    inputmethodname: Option<String>,
}

fn main() -> Fallible<()> {
    let args = Arguments::from_args();

    if let Some(inputmethodname) = args.inputmethodname {
        // Set IM.
        let name = inputmethodname.as_str().to_CFStringRef();

        let input_source = unsafe { TISCopyInputSourceForLanguage(name) };
        let local_name =
            unsafe { TISGetInputSourceProperty(input_source, kTISPropertyLocalizedName) };
        let ret = unsafe { TISSelectInputSource(input_source) };
        if ret == 0 {
            println!("Switched to input source: {}", local_name.to_str()?);
        } else {
            bail!("Failed to switch to input source: {}")
        }
    } else {
        // Get IM.
        let input_source = unsafe { TISCopyCurrentKeyboardInputSource() };
        let local_name =
            unsafe { TISGetInputSourceProperty(input_source, kTISPropertyLocalizedName) };
        println!("Current input source: {}", local_name.to_str()?);
    }

    Ok(())
}
