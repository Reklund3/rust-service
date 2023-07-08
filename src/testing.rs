use std::ffi::c_char;
use jni::sys::jbyte;

pub unsafe fn things(mut str: *const c_char, mut bytes: *const jbyte) -> () {
    while str.read() != 0 {
        bytes = str;
        println!("read {}", str.read());
        bytes = bytes.offset(1);
        str = str.offset(1);
    }
}