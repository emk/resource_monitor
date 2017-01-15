//! Internal interface to our allocator, which attemps to get heap
//! usage stats.

use libc::{c_char, c_int, c_void, size_t};
use std::ffi::{CStr, CString};
use std::mem::size_of;
use std::ptr;

use errors::*;

type MallocStatsCallback =
    unsafe extern "C" fn(*mut c_void, *const c_char);

extern "C" {
    // Print out current jemalloc stats.
    fn malloc_stats_print(cb: MallocStatsCallback,
 	                      cbopaque: *mut c_void,
                          opts: *const c_char);

    /// Access the jemalloc API using the C FFI.
    fn mallctl(name: *const c_char,
               oldp: *mut c_void,
               oldlenp: *mut size_t,
               newp: *mut c_void,
               newlen: size_t)
               -> c_int;
}

/// Fetch a jemalloc internal value.
unsafe fn mallctl_read<T: Default>(name: &str) -> Result<T> {
    let key = CString::new(name).unwrap();
    let mut old: T = T::default();
    let mut oldlen: size_t = size_of::<T>();
    let err =
        mallctl(key.as_ptr(),
                ((&mut old) as *mut T) as *mut c_void,
                &mut oldlen as *mut _,
                ptr::null_mut(),
                0);
    if err != 0 {
        return Err("could not access jemalloc internal data".into());
    }
    Ok(old)
}

/// How much memory is the allocator currently using for actual user
/// data?
pub fn used() -> Result<usize> {
    // We might prefer "stats.cactive" (it's faster and more conservative),
    // but that requires messing around with an atomic pointer read.
    unsafe { mallctl_read::<size_t>("stats.active") }
}

/// How much total memory has the allocator reserved for user allocations?
pub fn reserved() -> Result<usize> {
    // TODO: See http://jemalloc.net/jemalloc.3.html, which lists some
    // other values we might want to check.  This is an underestimate
    // of RAM we have in use.
    unsafe { mallctl_read::<size_t>("stats.mapped") }
}

/// Are our allocator stats enabled?
pub fn allocator_stats_enabled() -> bool {
    let enabled = unsafe { mallctl_read::<u8>("config.stats") }.unwrap_or(0);
    enabled != 0
}

/// Callback used to dump statistics.
unsafe extern "C" fn dumpstat(_: *mut c_void, msg: *const c_char) {
    let msg = CStr::from_ptr(msg);
    print!("{}", msg.to_str().unwrap());
}

/// Dump our allocator stats to standard output.
pub fn print_allocator_stats() {
    let opts = CString::new("").unwrap();
    unsafe {
        malloc_stats_print(dumpstat, ptr::null_mut(), opts.as_ptr());
    }
}
