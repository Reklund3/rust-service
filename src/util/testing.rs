use std::ffi::c_char;
use jni::sys::jbyte;

/// Implementing the shift call on a *const c_char
pub unsafe fn loop_char(str: *const c_char, bytes: &mut [jbyte]) -> () {
    let mut i: usize = 0;
    let mut char_ptr: *mut c_char = str as *mut c_char;

    // check that the pointed to char isn't 0 aka \0 or the terminating char for c strings
    // see https://faculty.cs.niu.edu/~winans/CS501/Notes/cstrings.html
    while *char_ptr != 0 {
        bytes[i] = char_ptr.read();
        i+=1;
        char_ptr = char_ptr.offset(1);
    }
}

pub unsafe fn working(str: *const c_char,bytes: &mut [jbyte]) -> () {
    let mut i: isize = 0;
    while str.offset(i).read() != 0 {
        bytes[i as usize] = str.offset(i).read() as jbyte;
        i += 1;
    }
}

