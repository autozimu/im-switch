#![allow(non_snake_case)]

#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}
use errors::*;

extern crate core_foundation_sys;
extern crate cocoa;

use std::ffi::CStr;
use core_foundation_sys::string::CFStringRef;
use cocoa::foundation::NSString;

trait ToStr {
    fn to_str(&self) -> &str;
}

impl ToStr for CFStringRef {
    fn to_str(&self) -> &str {
        use cocoa::base::id;

        unsafe {
            let ptr = (*self as id).UTF8String();
            CStr::from_ptr(ptr).to_str().expect(
                "Failed to convert to str",
            )
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
    static kTISPropertyInputSourceID: CFStringRef;
    static kTISPropertyLocalizedName: CFStringRef;
}

extern crate structopt;
use structopt::StructOpt;
#[macro_use]
extern crate structopt_derive;

#[derive(Debug, StructOpt)]
struct Arguments {
    #[structopt(short = "s", help = "Target input source name")]
    source: String;
}


fn main() {
    let args = Arguments::from_args();

    if args.is_present("source") {
        // Set IM.
        let name = args.value_of("source").unwrap().to_CFStringRef();

        let input_source = unsafe { TISCopyInputSourceForLanguage(name) };
        let input_source_name =
            unsafe { TISGetInputSourceProperty(input_source, kTISPropertyLocalizedName) };
        let input_source_name = input_source_name.to_str();
        let ret = unsafe { TISSelectInputSource(input_source) };
        if ret == 0 {
            println!("Switched to input source: {}", input_source_name);
        } else {
            println!("Failed to switch to input source: {}", input_source_name);
            std::process::exit(ret as i32);
        }
    } else {
        // Get IM.
        let input_source = unsafe { TISCopyCurrentKeyboardInputSource() };
        let input_source_name =
            unsafe { TISGetInputSourceProperty(input_source, kTISPropertyLocalizedName) };
        let input_source_name = input_source_name.to_str();
        println!("Current input source: {}", input_source_name);

    }
}
