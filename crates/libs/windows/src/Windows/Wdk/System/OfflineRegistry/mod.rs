#[inline]
pub unsafe fn ORCloseHive(handle: ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORCloseHive(handle : ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORCloseHive(core::mem::transmute(handle))
}
#[inline]
pub unsafe fn ORCloseKey(keyhandle: ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORCloseKey(keyhandle : ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORCloseKey(core::mem::transmute(keyhandle))
}
#[inline]
pub unsafe fn ORCreateHive(horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORCreateHive(horkey : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORCreateHive(core::mem::transmute(horkey))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ORCreateKey<P1, P2>(keyhandle: ORHKEY, lpsubkey: P1, lpclass: P2, dwoptions: Option<u32>, psecuritydescriptor: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, phkresult: *mut ORHKEY, pdwdisposition: Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORCreateKey(keyhandle : ORHKEY, lpsubkey : windows_core::PCWSTR, lpclass : windows_core::PCWSTR, dwoptions : u32, psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, phkresult : *mut ORHKEY, pdwdisposition : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORCreateKey(core::mem::transmute(keyhandle), lpsubkey.param().abi(), lpclass.param().abi(), core::mem::transmute(dwoptions.unwrap_or(core::mem::zeroed())), core::mem::transmute(psecuritydescriptor.unwrap_or(core::mem::zeroed())), core::mem::transmute(phkresult), core::mem::transmute(pdwdisposition.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn ORDeleteKey<P1>(handle: ORHKEY, lpsubkey: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORDeleteKey(handle : ORHKEY, lpsubkey : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORDeleteKey(core::mem::transmute(handle), lpsubkey.param().abi())
}
#[inline]
pub unsafe fn ORDeleteValue<P1>(handle: ORHKEY, lpvaluename: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORDeleteValue(handle : ORHKEY, lpvaluename : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORDeleteValue(core::mem::transmute(handle), lpvaluename.param().abi())
}
#[inline]
pub unsafe fn OREnumKey(handle: ORHKEY, dwindex: u32, lpname: windows_core::PWSTR, lpcname: *mut u32, lpclass: Option<windows_core::PWSTR>, lpcclass: Option<*mut u32>, lpftlastwritetime: Option<*mut super::super::super::Win32::Foundation::FILETIME>) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn OREnumKey(handle : ORHKEY, dwindex : u32, lpname : windows_core::PWSTR, lpcname : *mut u32, lpclass : windows_core::PWSTR, lpcclass : *mut u32, lpftlastwritetime : *mut super::super::super::Win32::Foundation:: FILETIME) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    OREnumKey(core::mem::transmute(handle), core::mem::transmute(dwindex), core::mem::transmute(lpname), core::mem::transmute(lpcname), core::mem::transmute(lpclass.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpcclass.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpftlastwritetime.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn OREnumValue(handle: ORHKEY, dwindex: u32, lpvaluename: windows_core::PWSTR, lpcvaluename: *mut u32, lptype: Option<*mut u32>, lpdata: Option<*mut u8>, lpcbdata: Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn OREnumValue(handle : ORHKEY, dwindex : u32, lpvaluename : windows_core::PWSTR, lpcvaluename : *mut u32, lptype : *mut u32, lpdata : *mut u8, lpcbdata : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    OREnumValue(core::mem::transmute(handle), core::mem::transmute(dwindex), core::mem::transmute(lpvaluename), core::mem::transmute(lpcvaluename), core::mem::transmute(lptype.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpdata.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpcbdata.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ORGetKeySecurity(handle: ORHKEY, securityinformation: u32, psecuritydescriptor: Option<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>, lpcbsecuritydescriptor: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORGetKeySecurity(handle : ORHKEY, securityinformation : u32, psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORGetKeySecurity(core::mem::transmute(handle), core::mem::transmute(securityinformation), core::mem::transmute(psecuritydescriptor.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpcbsecuritydescriptor))
}
#[inline]
pub unsafe fn ORGetValue<P1, P2>(handle: ORHKEY, lpsubkey: P1, lpvalue: P2, pdwtype: Option<*mut u32>, pvdata: Option<*mut core::ffi::c_void>, pcbdata: Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORGetValue(handle : ORHKEY, lpsubkey : windows_core::PCWSTR, lpvalue : windows_core::PCWSTR, pdwtype : *mut u32, pvdata : *mut core::ffi::c_void, pcbdata : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORGetValue(core::mem::transmute(handle), lpsubkey.param().abi(), lpvalue.param().abi(), core::mem::transmute(pdwtype.unwrap_or(core::mem::zeroed())), core::mem::transmute(pvdata.unwrap_or(core::mem::zeroed())), core::mem::transmute(pcbdata.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn ORGetVersion(pdwmajorversion: *mut u32, pdwminorversion: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORGetVersion(pdwmajorversion : *mut u32, pdwminorversion : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORGetVersion(core::mem::transmute(pdwmajorversion), core::mem::transmute(pdwminorversion))
}
#[inline]
pub unsafe fn ORGetVirtualFlags(handle: ORHKEY, pdwflags: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORGetVirtualFlags(handle : ORHKEY, pdwflags : *mut u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORGetVirtualFlags(core::mem::transmute(handle), core::mem::transmute(pdwflags))
}
#[inline]
pub unsafe fn ORMergeHives(hivehandles: &[ORHKEY], phkresult: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORMergeHives(hivehandles : *const ORHKEY, hivecount : u32, phkresult : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORMergeHives(core::mem::transmute(hivehandles.as_ptr()), hivehandles.len().try_into().unwrap(), core::mem::transmute(phkresult))
}
#[inline]
pub unsafe fn OROpenHive<P0>(filepath: P0, horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn OROpenHive(filepath : windows_core::PCWSTR, horkey : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    OROpenHive(filepath.param().abi(), core::mem::transmute(horkey))
}
#[inline]
pub unsafe fn OROpenHiveByHandle(filehandle: super::super::super::Win32::Foundation::HANDLE, horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn OROpenHiveByHandle(filehandle : super::super::super::Win32::Foundation:: HANDLE, horkey : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    OROpenHiveByHandle(core::mem::transmute(filehandle), core::mem::transmute(horkey))
}
#[inline]
pub unsafe fn OROpenKey<P1>(handle: ORHKEY, lpsubkey: P1, phkresult: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn OROpenKey(handle : ORHKEY, lpsubkey : windows_core::PCWSTR, phkresult : *mut ORHKEY) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    OROpenKey(core::mem::transmute(handle), lpsubkey.param().abi(), core::mem::transmute(phkresult))
}
#[inline]
pub unsafe fn ORQueryInfoKey(handle: ORHKEY, lpclass: Option<windows_core::PWSTR>, lpcclass: Option<*mut u32>, lpcsubkeys: Option<*mut u32>, lpcmaxsubkeylen: Option<*mut u32>, lpcmaxclasslen: Option<*mut u32>, lpcvalues: Option<*mut u32>, lpcmaxvaluenamelen: Option<*mut u32>, lpcmaxvaluelen: Option<*mut u32>, lpcbsecuritydescriptor: Option<*mut u32>, lpftlastwritetime: Option<*mut super::super::super::Win32::Foundation::FILETIME>) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORQueryInfoKey(handle : ORHKEY, lpclass : windows_core::PWSTR, lpcclass : *mut u32, lpcsubkeys : *mut u32, lpcmaxsubkeylen : *mut u32, lpcmaxclasslen : *mut u32, lpcvalues : *mut u32, lpcmaxvaluenamelen : *mut u32, lpcmaxvaluelen : *mut u32, lpcbsecuritydescriptor : *mut u32, lpftlastwritetime : *mut super::super::super::Win32::Foundation:: FILETIME) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORQueryInfoKey(
        core::mem::transmute(handle),
        core::mem::transmute(lpclass.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcclass.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcsubkeys.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcmaxsubkeylen.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcmaxclasslen.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcvalues.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcmaxvaluenamelen.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcmaxvaluelen.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcbsecuritydescriptor.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpftlastwritetime.unwrap_or(core::mem::zeroed())),
    )
}
#[inline]
pub unsafe fn ORRenameKey<P1>(handle: ORHKEY, lpnewname: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORRenameKey(handle : ORHKEY, lpnewname : windows_core::PCWSTR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORRenameKey(core::mem::transmute(handle), lpnewname.param().abi())
}
#[inline]
pub unsafe fn ORSaveHive<P1>(horkey: ORHKEY, hivepath: P1, osmajorversion: u32, osminorversion: u32) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORSaveHive(horkey : ORHKEY, hivepath : windows_core::PCWSTR, osmajorversion : u32, osminorversion : u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORSaveHive(core::mem::transmute(horkey), hivepath.param().abi(), core::mem::transmute(osmajorversion), core::mem::transmute(osminorversion))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn ORSetKeySecurity(handle: ORHKEY, securityinformation: u32, psecuritydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORSetKeySecurity(handle : ORHKEY, securityinformation : u32, psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORSetKeySecurity(core::mem::transmute(handle), core::mem::transmute(securityinformation), core::mem::transmute(psecuritydescriptor))
}
#[inline]
pub unsafe fn ORSetValue<P1>(handle: ORHKEY, lpvaluename: P1, dwtype: u32, lpdata: Option<&[u8]>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("offreg.dll" "system" fn ORSetValue(handle : ORHKEY, lpvaluename : windows_core::PCWSTR, dwtype : u32, lpdata : *const u8, cbdata : u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORSetValue(core::mem::transmute(handle), lpvaluename.param().abi(), core::mem::transmute(dwtype), core::mem::transmute(lpdata.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn ORSetVirtualFlags(handle: ORHKEY, dwflags: u32) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORSetVirtualFlags(handle : ORHKEY, dwflags : u32) -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORSetVirtualFlags(core::mem::transmute(handle), core::mem::transmute(dwflags))
}
#[inline]
pub unsafe fn ORShutdown() -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORShutdown() -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORShutdown()
}
#[inline]
pub unsafe fn ORStart() -> super::super::super::Win32::Foundation::WIN32_ERROR {
    windows_targets::link!("offreg.dll" "system" fn ORStart() -> super::super::super::Win32::Foundation:: WIN32_ERROR);
    ORStart()
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ORHKEY(pub *mut core::ffi::c_void);
impl windows_core::TypeKind for ORHKEY {
    type TypeKind = windows_core::CopyType;
}
impl ORHKEY {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for ORHKEY {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("offreg.dll" "system" fn ORCloseKey(keyhandle : *mut core::ffi::c_void) -> u32);
            ORCloseKey(self.0);
        }
    }
}
impl Default for ORHKEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
