#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
windows_targets::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoActivateInstance(activatableclassid : * mut core::ffi::c_void, instance : *mut * mut core::ffi::c_void) -> HRESULT);
pub type HRESULT = i32;