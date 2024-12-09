#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn NtDeviceIoControlFile(filehandle: super::super::super::Win32::Foundation::HANDLE, event: Option<super::super::super::Win32::Foundation::HANDLE>, apcroutine: Option<super::super::super::Win32::System::IO::PIO_APC_ROUTINE>, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::super::super::Win32::System::IO::IO_STATUS_BLOCK, iocontrolcode: u32, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtDeviceIoControlFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, iocontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    NtDeviceIoControlFile(
        core::mem::transmute(filehandle),
        core::mem::transmute(event.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(apcroutine.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(apccontext.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(iostatusblock),
        core::mem::transmute(iocontrolcode),
        core::mem::transmute(inputbuffer.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(inputbufferlength),
        core::mem::transmute(outputbuffer.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(outputbufferlength),
    )
}
