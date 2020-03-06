use std::ffi::CStr;
use std::os::raw::c_char;
use std::str;

mod memorytracking;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref COMMAND_PROCESSOR: memorytracking::CommandProcessor =
        memorytracking::CommandProcessor::new();
}

#[no_mangle]
pub extern "C" fn pymemprofile_add_allocation(
    address: usize,
    size: libc::size_t,
    line_number: u16,
) {
    COMMAND_PROCESSOR.add_allocation(address, size, line_number);
}

#[no_mangle]
pub extern "C" fn pymemprofile_free_allocation(address: usize) {
    COMMAND_PROCESSOR.free_allocation(address);
}

/// # Safety
/// Intended for use from C APIs, what can I say.
#[no_mangle]
pub unsafe extern "C" fn pymemprofile_start_call(
    parent_line_number: u16,
    file_name: *const c_char,
    func_name: *const c_char,
    line_number: u16,
) {
    let function_name = str::from_utf8_unchecked(CStr::from_ptr(func_name).to_bytes());
    let module_name = str::from_utf8_unchecked(CStr::from_ptr(file_name).to_bytes());
    let call_site = memorytracking::Function::new(module_name, function_name);
    COMMAND_PROCESSOR.start_call(call_site, parent_line_number, line_number);
}

#[no_mangle]
pub extern "C" fn pymemprofile_finish_call() {
    COMMAND_PROCESSOR.finish_call();
}

#[no_mangle]
pub extern "C" fn pymemprofile_reset() {
    COMMAND_PROCESSOR.reset();
}

/// # Safety
/// Intended for use from C APIs, what can I say.
#[no_mangle]
pub unsafe extern "C" fn pymemprofile_dump_peak_to_flamegraph(path: *const c_char) {
    let path = CStr::from_ptr(path)
        .to_str()
        .expect("Path wasn't UTF-8")
        .to_string();
    COMMAND_PROCESSOR.dump_peak_to_flamegraph(&path);
}

#[cfg(test)]
mod tests {}
