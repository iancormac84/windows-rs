#[inline]
pub unsafe fn AllJoynAcceptBusConnection(serverbushandle: super::super::Foundation::HANDLE, abortevent: super::super::Foundation::HANDLE) -> u32 {
    windows_targets::link!("msajapi.dll" "system" fn AllJoynAcceptBusConnection(serverbushandle : super::super::Foundation:: HANDLE, abortevent : super::super::Foundation:: HANDLE) -> u32);
    AllJoynAcceptBusConnection(core::mem::transmute(serverbushandle), core::mem::transmute(abortevent))
}
#[inline]
pub unsafe fn AllJoynCloseBusHandle(bushandle: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("msajapi.dll" "system" fn AllJoynCloseBusHandle(bushandle : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    AllJoynCloseBusHandle(core::mem::transmute(bushandle)).ok()
}
#[inline]
pub unsafe fn AllJoynConnectToBus<P0>(connectionspec: P0) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn AllJoynConnectToBus(connectionspec : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = AllJoynConnectToBus(connectionspec.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn AllJoynCreateBus(outbuffersize: u32, inbuffersize: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::HANDLE {
    windows_targets::link!("msajapi.dll" "system" fn AllJoynCreateBus(outbuffersize : u32, inbuffersize : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    AllJoynCreateBus(core::mem::transmute(outbuffersize), core::mem::transmute(inbuffersize), core::mem::transmute(lpsecurityattributes.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn AllJoynEnumEvents(connectedbushandle: super::super::Foundation::HANDLE, eventtoreset: Option<super::super::Foundation::HANDLE>, eventtypes: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msajapi.dll" "system" fn AllJoynEnumEvents(connectedbushandle : super::super::Foundation:: HANDLE, eventtoreset : super::super::Foundation:: HANDLE, eventtypes : *mut u32) -> super::super::Foundation:: BOOL);
    AllJoynEnumEvents(core::mem::transmute(connectedbushandle), core::mem::transmute(eventtoreset.unwrap_or(core::mem::zeroed())), core::mem::transmute(eventtypes)).ok()
}
#[inline]
pub unsafe fn AllJoynEventSelect(connectedbushandle: super::super::Foundation::HANDLE, eventhandle: super::super::Foundation::HANDLE, eventtypes: u32) -> windows_core::Result<()> {
    windows_targets::link!("msajapi.dll" "system" fn AllJoynEventSelect(connectedbushandle : super::super::Foundation:: HANDLE, eventhandle : super::super::Foundation:: HANDLE, eventtypes : u32) -> super::super::Foundation:: BOOL);
    AllJoynEventSelect(core::mem::transmute(connectedbushandle), core::mem::transmute(eventhandle), core::mem::transmute(eventtypes)).ok()
}
#[inline]
pub unsafe fn AllJoynReceiveFromBus(connectedbushandle: super::super::Foundation::HANDLE, buffer: Option<*mut core::ffi::c_void>, bytestoread: u32, bytestransferred: Option<*mut u32>, reserved: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("msajapi.dll" "system" fn AllJoynReceiveFromBus(connectedbushandle : super::super::Foundation:: HANDLE, buffer : *mut core::ffi::c_void, bytestoread : u32, bytestransferred : *mut u32, reserved : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    AllJoynReceiveFromBus(core::mem::transmute(connectedbushandle), core::mem::transmute(buffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(bytestoread), core::mem::transmute(bytestransferred.unwrap_or(core::mem::zeroed())), core::mem::transmute(reserved)).ok()
}
#[inline]
pub unsafe fn AllJoynSendToBus(connectedbushandle: super::super::Foundation::HANDLE, buffer: Option<*const core::ffi::c_void>, bytestowrite: u32, bytestransferred: Option<*mut u32>, reserved: *mut core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("msajapi.dll" "system" fn AllJoynSendToBus(connectedbushandle : super::super::Foundation:: HANDLE, buffer : *const core::ffi::c_void, bytestowrite : u32, bytestransferred : *mut u32, reserved : *mut core::ffi::c_void) -> super::super::Foundation:: BOOL);
    AllJoynSendToBus(core::mem::transmute(connectedbushandle), core::mem::transmute(buffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(bytestowrite), core::mem::transmute(bytestransferred.unwrap_or(core::mem::zeroed())), core::mem::transmute(reserved)).ok()
}
#[inline]
pub unsafe fn QCC_StatusText(status: QStatus) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn QCC_StatusText(status : QStatus) -> windows_core::PCSTR);
    QCC_StatusText(core::mem::transmute(status))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_create<P0>(defaultlanguage: P0) -> alljoyn_aboutdata
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_create(defaultlanguage : windows_core::PCSTR) -> alljoyn_aboutdata);
    alljoyn_aboutdata_create(defaultlanguage.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_create_empty() -> alljoyn_aboutdata {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_create_empty() -> alljoyn_aboutdata);
    alljoyn_aboutdata_create_empty()
}
#[inline]
pub unsafe fn alljoyn_aboutdata_create_full<P1>(arg: alljoyn_msgarg, language: P1) -> alljoyn_aboutdata
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_create_full(arg : alljoyn_msgarg, language : windows_core::PCSTR) -> alljoyn_aboutdata);
    alljoyn_aboutdata_create_full(core::mem::transmute(arg), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_createfrommsgarg<P2>(data: alljoyn_aboutdata, arg: alljoyn_msgarg, language: P2) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_createfrommsgarg(data : alljoyn_aboutdata, arg : alljoyn_msgarg, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_createfrommsgarg(core::mem::transmute(data), core::mem::transmute(arg), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_createfromxml<P1>(data: alljoyn_aboutdata, aboutdataxml: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_createfromxml(data : alljoyn_aboutdata, aboutdataxml : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_createfromxml(core::mem::transmute(data), aboutdataxml.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_destroy(data: alljoyn_aboutdata) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_destroy(data : alljoyn_aboutdata));
    alljoyn_aboutdata_destroy(core::mem::transmute(data))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getaboutdata<P2>(data: alljoyn_aboutdata, msgarg: alljoyn_msgarg, language: P2) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getaboutdata(data : alljoyn_aboutdata, msgarg : alljoyn_msgarg, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_getaboutdata(core::mem::transmute(data), core::mem::transmute(msgarg), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getajsoftwareversion(data: alljoyn_aboutdata, ajsoftwareversion: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getajsoftwareversion(data : alljoyn_aboutdata, ajsoftwareversion : *mut *mut i8) -> QStatus);
    alljoyn_aboutdata_getajsoftwareversion(core::mem::transmute(data), core::mem::transmute(ajsoftwareversion))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getannouncedaboutdata(data: alljoyn_aboutdata, msgarg: alljoyn_msgarg) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getannouncedaboutdata(data : alljoyn_aboutdata, msgarg : alljoyn_msgarg) -> QStatus);
    alljoyn_aboutdata_getannouncedaboutdata(core::mem::transmute(data), core::mem::transmute(msgarg))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getappid(data: alljoyn_aboutdata, appid: *mut *mut u8, num: *mut usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getappid(data : alljoyn_aboutdata, appid : *mut *mut u8, num : *mut usize) -> QStatus);
    alljoyn_aboutdata_getappid(core::mem::transmute(data), core::mem::transmute(appid), core::mem::transmute(num))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getappname<P2>(data: alljoyn_aboutdata, appname: *mut *mut i8, language: P2) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getappname(data : alljoyn_aboutdata, appname : *mut *mut i8, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_getappname(core::mem::transmute(data), core::mem::transmute(appname), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getdateofmanufacture(data: alljoyn_aboutdata, dateofmanufacture: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getdateofmanufacture(data : alljoyn_aboutdata, dateofmanufacture : *mut *mut i8) -> QStatus);
    alljoyn_aboutdata_getdateofmanufacture(core::mem::transmute(data), core::mem::transmute(dateofmanufacture))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getdefaultlanguage(data: alljoyn_aboutdata, defaultlanguage: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getdefaultlanguage(data : alljoyn_aboutdata, defaultlanguage : *mut *mut i8) -> QStatus);
    alljoyn_aboutdata_getdefaultlanguage(core::mem::transmute(data), core::mem::transmute(defaultlanguage))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getdescription<P2>(data: alljoyn_aboutdata, description: *mut *mut i8, language: P2) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getdescription(data : alljoyn_aboutdata, description : *mut *mut i8, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_getdescription(core::mem::transmute(data), core::mem::transmute(description), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getdeviceid(data: alljoyn_aboutdata, deviceid: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getdeviceid(data : alljoyn_aboutdata, deviceid : *mut *mut i8) -> QStatus);
    alljoyn_aboutdata_getdeviceid(core::mem::transmute(data), core::mem::transmute(deviceid))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getdevicename<P2>(data: alljoyn_aboutdata, devicename: *mut *mut i8, language: P2) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getdevicename(data : alljoyn_aboutdata, devicename : *mut *mut i8, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_getdevicename(core::mem::transmute(data), core::mem::transmute(devicename), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getfield<P1, P3>(data: alljoyn_aboutdata, name: P1, value: *mut alljoyn_msgarg, language: P3) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getfield(data : alljoyn_aboutdata, name : windows_core::PCSTR, value : *mut alljoyn_msgarg, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_getfield(core::mem::transmute(data), name.param().abi(), core::mem::transmute(value), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getfields(data: alljoyn_aboutdata, fields: *const *const i8, num_fields: usize) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getfields(data : alljoyn_aboutdata, fields : *const *const i8, num_fields : usize) -> usize);
    alljoyn_aboutdata_getfields(core::mem::transmute(data), core::mem::transmute(fields), core::mem::transmute(num_fields))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getfieldsignature<P1>(data: alljoyn_aboutdata, fieldname: P1) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getfieldsignature(data : alljoyn_aboutdata, fieldname : windows_core::PCSTR) -> windows_core::PCSTR);
    alljoyn_aboutdata_getfieldsignature(core::mem::transmute(data), fieldname.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_gethardwareversion(data: alljoyn_aboutdata, hardwareversion: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_gethardwareversion(data : alljoyn_aboutdata, hardwareversion : *mut *mut i8) -> QStatus);
    alljoyn_aboutdata_gethardwareversion(core::mem::transmute(data), core::mem::transmute(hardwareversion))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getmanufacturer<P2>(data: alljoyn_aboutdata, manufacturer: *mut *mut i8, language: P2) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getmanufacturer(data : alljoyn_aboutdata, manufacturer : *mut *mut i8, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_getmanufacturer(core::mem::transmute(data), core::mem::transmute(manufacturer), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getmodelnumber(data: alljoyn_aboutdata, modelnumber: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getmodelnumber(data : alljoyn_aboutdata, modelnumber : *mut *mut i8) -> QStatus);
    alljoyn_aboutdata_getmodelnumber(core::mem::transmute(data), core::mem::transmute(modelnumber))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getsoftwareversion(data: alljoyn_aboutdata, softwareversion: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getsoftwareversion(data : alljoyn_aboutdata, softwareversion : *mut *mut i8) -> QStatus);
    alljoyn_aboutdata_getsoftwareversion(core::mem::transmute(data), core::mem::transmute(softwareversion))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getsupportedlanguages(data: alljoyn_aboutdata, languagetags: *const *const i8, num: usize) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getsupportedlanguages(data : alljoyn_aboutdata, languagetags : *const *const i8, num : usize) -> usize);
    alljoyn_aboutdata_getsupportedlanguages(core::mem::transmute(data), core::mem::transmute(languagetags), core::mem::transmute(num))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_getsupporturl(data: alljoyn_aboutdata, supporturl: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_getsupporturl(data : alljoyn_aboutdata, supporturl : *mut *mut i8) -> QStatus);
    alljoyn_aboutdata_getsupporturl(core::mem::transmute(data), core::mem::transmute(supporturl))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_isfieldannounced<P1>(data: alljoyn_aboutdata, fieldname: P1) -> u8
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_isfieldannounced(data : alljoyn_aboutdata, fieldname : windows_core::PCSTR) -> u8);
    alljoyn_aboutdata_isfieldannounced(core::mem::transmute(data), fieldname.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_isfieldlocalized<P1>(data: alljoyn_aboutdata, fieldname: P1) -> u8
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_isfieldlocalized(data : alljoyn_aboutdata, fieldname : windows_core::PCSTR) -> u8);
    alljoyn_aboutdata_isfieldlocalized(core::mem::transmute(data), fieldname.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_isfieldrequired<P1>(data: alljoyn_aboutdata, fieldname: P1) -> u8
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_isfieldrequired(data : alljoyn_aboutdata, fieldname : windows_core::PCSTR) -> u8);
    alljoyn_aboutdata_isfieldrequired(core::mem::transmute(data), fieldname.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_isvalid<P1>(data: alljoyn_aboutdata, language: P1) -> u8
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_isvalid(data : alljoyn_aboutdata, language : windows_core::PCSTR) -> u8);
    alljoyn_aboutdata_isvalid(core::mem::transmute(data), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setappid(data: alljoyn_aboutdata, appid: *const u8, num: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setappid(data : alljoyn_aboutdata, appid : *const u8, num : usize) -> QStatus);
    alljoyn_aboutdata_setappid(core::mem::transmute(data), core::mem::transmute(appid), core::mem::transmute(num))
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setappid_fromstring<P1>(data: alljoyn_aboutdata, appid: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setappid_fromstring(data : alljoyn_aboutdata, appid : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setappid_fromstring(core::mem::transmute(data), appid.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setappname<P1, P2>(data: alljoyn_aboutdata, appname: P1, language: P2) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setappname(data : alljoyn_aboutdata, appname : windows_core::PCSTR, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setappname(core::mem::transmute(data), appname.param().abi(), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setdateofmanufacture<P1>(data: alljoyn_aboutdata, dateofmanufacture: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setdateofmanufacture(data : alljoyn_aboutdata, dateofmanufacture : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setdateofmanufacture(core::mem::transmute(data), dateofmanufacture.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setdefaultlanguage<P1>(data: alljoyn_aboutdata, defaultlanguage: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setdefaultlanguage(data : alljoyn_aboutdata, defaultlanguage : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setdefaultlanguage(core::mem::transmute(data), defaultlanguage.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setdescription<P1, P2>(data: alljoyn_aboutdata, description: P1, language: P2) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setdescription(data : alljoyn_aboutdata, description : windows_core::PCSTR, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setdescription(core::mem::transmute(data), description.param().abi(), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setdeviceid<P1>(data: alljoyn_aboutdata, deviceid: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setdeviceid(data : alljoyn_aboutdata, deviceid : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setdeviceid(core::mem::transmute(data), deviceid.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setdevicename<P1, P2>(data: alljoyn_aboutdata, devicename: P1, language: P2) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setdevicename(data : alljoyn_aboutdata, devicename : windows_core::PCSTR, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setdevicename(core::mem::transmute(data), devicename.param().abi(), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setfield<P1, P3>(data: alljoyn_aboutdata, name: P1, value: alljoyn_msgarg, language: P3) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setfield(data : alljoyn_aboutdata, name : windows_core::PCSTR, value : alljoyn_msgarg, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setfield(core::mem::transmute(data), name.param().abi(), core::mem::transmute(value), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_sethardwareversion<P1>(data: alljoyn_aboutdata, hardwareversion: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_sethardwareversion(data : alljoyn_aboutdata, hardwareversion : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_sethardwareversion(core::mem::transmute(data), hardwareversion.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setmanufacturer<P1, P2>(data: alljoyn_aboutdata, manufacturer: P1, language: P2) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setmanufacturer(data : alljoyn_aboutdata, manufacturer : windows_core::PCSTR, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setmanufacturer(core::mem::transmute(data), manufacturer.param().abi(), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setmodelnumber<P1>(data: alljoyn_aboutdata, modelnumber: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setmodelnumber(data : alljoyn_aboutdata, modelnumber : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setmodelnumber(core::mem::transmute(data), modelnumber.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setsoftwareversion<P1>(data: alljoyn_aboutdata, softwareversion: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setsoftwareversion(data : alljoyn_aboutdata, softwareversion : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setsoftwareversion(core::mem::transmute(data), softwareversion.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setsupportedlanguage<P1>(data: alljoyn_aboutdata, language: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setsupportedlanguage(data : alljoyn_aboutdata, language : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setsupportedlanguage(core::mem::transmute(data), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdata_setsupporturl<P1>(data: alljoyn_aboutdata, supporturl: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdata_setsupporturl(data : alljoyn_aboutdata, supporturl : windows_core::PCSTR) -> QStatus);
    alljoyn_aboutdata_setsupporturl(core::mem::transmute(data), supporturl.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutdatalistener_create(callbacks: *const alljoyn_aboutdatalistener_callbacks, context: *const core::ffi::c_void) -> alljoyn_aboutdatalistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdatalistener_create(callbacks : *const alljoyn_aboutdatalistener_callbacks, context : *const core::ffi::c_void) -> alljoyn_aboutdatalistener);
    alljoyn_aboutdatalistener_create(core::mem::transmute(callbacks), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_aboutdatalistener_destroy(listener: alljoyn_aboutdatalistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutdatalistener_destroy(listener : alljoyn_aboutdatalistener));
    alljoyn_aboutdatalistener_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_abouticon_clear(icon: alljoyn_abouticon) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticon_clear(icon : alljoyn_abouticon));
    alljoyn_abouticon_clear(core::mem::transmute(icon))
}
#[inline]
pub unsafe fn alljoyn_abouticon_create() -> alljoyn_abouticon {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticon_create() -> alljoyn_abouticon);
    alljoyn_abouticon_create()
}
#[inline]
pub unsafe fn alljoyn_abouticon_destroy(icon: alljoyn_abouticon) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticon_destroy(icon : alljoyn_abouticon));
    alljoyn_abouticon_destroy(core::mem::transmute(icon))
}
#[inline]
pub unsafe fn alljoyn_abouticon_getcontent(icon: alljoyn_abouticon, data: *const *const u8, size: *mut usize) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticon_getcontent(icon : alljoyn_abouticon, data : *const *const u8, size : *mut usize));
    alljoyn_abouticon_getcontent(core::mem::transmute(icon), core::mem::transmute(data), core::mem::transmute(size))
}
#[inline]
pub unsafe fn alljoyn_abouticon_geturl(icon: alljoyn_abouticon, r#type: *const *const i8, url: *const *const i8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticon_geturl(icon : alljoyn_abouticon, r#type : *const *const i8, url : *const *const i8));
    alljoyn_abouticon_geturl(core::mem::transmute(icon), core::mem::transmute(r#type), core::mem::transmute(url))
}
#[inline]
pub unsafe fn alljoyn_abouticon_setcontent<P1>(icon: alljoyn_abouticon, r#type: P1, data: *mut u8, csize: usize, ownsdata: u8) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticon_setcontent(icon : alljoyn_abouticon, r#type : windows_core::PCSTR, data : *mut u8, csize : usize, ownsdata : u8) -> QStatus);
    alljoyn_abouticon_setcontent(core::mem::transmute(icon), r#type.param().abi(), core::mem::transmute(data), core::mem::transmute(csize), core::mem::transmute(ownsdata))
}
#[inline]
pub unsafe fn alljoyn_abouticon_setcontent_frommsgarg(icon: alljoyn_abouticon, arg: alljoyn_msgarg) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticon_setcontent_frommsgarg(icon : alljoyn_abouticon, arg : alljoyn_msgarg) -> QStatus);
    alljoyn_abouticon_setcontent_frommsgarg(core::mem::transmute(icon), core::mem::transmute(arg))
}
#[inline]
pub unsafe fn alljoyn_abouticon_seturl<P1, P2>(icon: alljoyn_abouticon, r#type: P1, url: P2) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticon_seturl(icon : alljoyn_abouticon, r#type : windows_core::PCSTR, url : windows_core::PCSTR) -> QStatus);
    alljoyn_abouticon_seturl(core::mem::transmute(icon), r#type.param().abi(), url.param().abi())
}
#[inline]
pub unsafe fn alljoyn_abouticonobj_create(bus: alljoyn_busattachment, icon: alljoyn_abouticon) -> alljoyn_abouticonobj {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticonobj_create(bus : alljoyn_busattachment, icon : alljoyn_abouticon) -> alljoyn_abouticonobj);
    alljoyn_abouticonobj_create(core::mem::transmute(bus), core::mem::transmute(icon))
}
#[inline]
pub unsafe fn alljoyn_abouticonobj_destroy(icon: alljoyn_abouticonobj) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticonobj_destroy(icon : alljoyn_abouticonobj));
    alljoyn_abouticonobj_destroy(core::mem::transmute(icon))
}
#[inline]
pub unsafe fn alljoyn_abouticonproxy_create<P1>(bus: alljoyn_busattachment, busname: P1, sessionid: u32) -> alljoyn_abouticonproxy
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticonproxy_create(bus : alljoyn_busattachment, busname : windows_core::PCSTR, sessionid : u32) -> alljoyn_abouticonproxy);
    alljoyn_abouticonproxy_create(core::mem::transmute(bus), busname.param().abi(), core::mem::transmute(sessionid))
}
#[inline]
pub unsafe fn alljoyn_abouticonproxy_destroy(proxy: alljoyn_abouticonproxy) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticonproxy_destroy(proxy : alljoyn_abouticonproxy));
    alljoyn_abouticonproxy_destroy(core::mem::transmute(proxy))
}
#[inline]
pub unsafe fn alljoyn_abouticonproxy_geticon(proxy: alljoyn_abouticonproxy, icon: alljoyn_abouticon) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticonproxy_geticon(proxy : alljoyn_abouticonproxy, icon : alljoyn_abouticon) -> QStatus);
    alljoyn_abouticonproxy_geticon(core::mem::transmute(proxy), core::mem::transmute(icon))
}
#[inline]
pub unsafe fn alljoyn_abouticonproxy_getversion(proxy: alljoyn_abouticonproxy, version: *mut u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_abouticonproxy_getversion(proxy : alljoyn_abouticonproxy, version : *mut u16) -> QStatus);
    alljoyn_abouticonproxy_getversion(core::mem::transmute(proxy), core::mem::transmute(version))
}
#[inline]
pub unsafe fn alljoyn_aboutlistener_create(callback: *const alljoyn_aboutlistener_callback, context: *const core::ffi::c_void) -> alljoyn_aboutlistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutlistener_create(callback : *const alljoyn_aboutlistener_callback, context : *const core::ffi::c_void) -> alljoyn_aboutlistener);
    alljoyn_aboutlistener_create(core::mem::transmute(callback), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_aboutlistener_destroy(listener: alljoyn_aboutlistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutlistener_destroy(listener : alljoyn_aboutlistener));
    alljoyn_aboutlistener_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_aboutobj_announce(obj: alljoyn_aboutobj, sessionport: u16, aboutdata: alljoyn_aboutdata) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobj_announce(obj : alljoyn_aboutobj, sessionport : u16, aboutdata : alljoyn_aboutdata) -> QStatus);
    alljoyn_aboutobj_announce(core::mem::transmute(obj), core::mem::transmute(sessionport), core::mem::transmute(aboutdata))
}
#[inline]
pub unsafe fn alljoyn_aboutobj_announce_using_datalistener(obj: alljoyn_aboutobj, sessionport: u16, aboutlistener: alljoyn_aboutdatalistener) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobj_announce_using_datalistener(obj : alljoyn_aboutobj, sessionport : u16, aboutlistener : alljoyn_aboutdatalistener) -> QStatus);
    alljoyn_aboutobj_announce_using_datalistener(core::mem::transmute(obj), core::mem::transmute(sessionport), core::mem::transmute(aboutlistener))
}
#[inline]
pub unsafe fn alljoyn_aboutobj_create(bus: alljoyn_busattachment, isannounced: alljoyn_about_announceflag) -> alljoyn_aboutobj {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobj_create(bus : alljoyn_busattachment, isannounced : alljoyn_about_announceflag) -> alljoyn_aboutobj);
    alljoyn_aboutobj_create(core::mem::transmute(bus), core::mem::transmute(isannounced))
}
#[inline]
pub unsafe fn alljoyn_aboutobj_destroy(obj: alljoyn_aboutobj) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobj_destroy(obj : alljoyn_aboutobj));
    alljoyn_aboutobj_destroy(core::mem::transmute(obj))
}
#[inline]
pub unsafe fn alljoyn_aboutobj_unannounce(obj: alljoyn_aboutobj) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobj_unannounce(obj : alljoyn_aboutobj) -> QStatus);
    alljoyn_aboutobj_unannounce(core::mem::transmute(obj))
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_clear(description: alljoyn_aboutobjectdescription) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_clear(description : alljoyn_aboutobjectdescription));
    alljoyn_aboutobjectdescription_clear(core::mem::transmute(description))
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_create() -> alljoyn_aboutobjectdescription {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_create() -> alljoyn_aboutobjectdescription);
    alljoyn_aboutobjectdescription_create()
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_create_full(arg: alljoyn_msgarg) -> alljoyn_aboutobjectdescription {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_create_full(arg : alljoyn_msgarg) -> alljoyn_aboutobjectdescription);
    alljoyn_aboutobjectdescription_create_full(core::mem::transmute(arg))
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_createfrommsgarg(description: alljoyn_aboutobjectdescription, arg: alljoyn_msgarg) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_createfrommsgarg(description : alljoyn_aboutobjectdescription, arg : alljoyn_msgarg) -> QStatus);
    alljoyn_aboutobjectdescription_createfrommsgarg(core::mem::transmute(description), core::mem::transmute(arg))
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_destroy(description: alljoyn_aboutobjectdescription) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_destroy(description : alljoyn_aboutobjectdescription));
    alljoyn_aboutobjectdescription_destroy(core::mem::transmute(description))
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_getinterfacepaths<P1>(description: alljoyn_aboutobjectdescription, interfacename: P1, paths: *const *const i8, numpaths: usize) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_getinterfacepaths(description : alljoyn_aboutobjectdescription, interfacename : windows_core::PCSTR, paths : *const *const i8, numpaths : usize) -> usize);
    alljoyn_aboutobjectdescription_getinterfacepaths(core::mem::transmute(description), interfacename.param().abi(), core::mem::transmute(paths), core::mem::transmute(numpaths))
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_getinterfaces<P1>(description: alljoyn_aboutobjectdescription, path: P1, interfaces: *const *const i8, numinterfaces: usize) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_getinterfaces(description : alljoyn_aboutobjectdescription, path : windows_core::PCSTR, interfaces : *const *const i8, numinterfaces : usize) -> usize);
    alljoyn_aboutobjectdescription_getinterfaces(core::mem::transmute(description), path.param().abi(), core::mem::transmute(interfaces), core::mem::transmute(numinterfaces))
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_getmsgarg(description: alljoyn_aboutobjectdescription, msgarg: alljoyn_msgarg) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_getmsgarg(description : alljoyn_aboutobjectdescription, msgarg : alljoyn_msgarg) -> QStatus);
    alljoyn_aboutobjectdescription_getmsgarg(core::mem::transmute(description), core::mem::transmute(msgarg))
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_getpaths(description: alljoyn_aboutobjectdescription, paths: *const *const i8, numpaths: usize) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_getpaths(description : alljoyn_aboutobjectdescription, paths : *const *const i8, numpaths : usize) -> usize);
    alljoyn_aboutobjectdescription_getpaths(core::mem::transmute(description), core::mem::transmute(paths), core::mem::transmute(numpaths))
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_hasinterface<P1>(description: alljoyn_aboutobjectdescription, interfacename: P1) -> u8
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_hasinterface(description : alljoyn_aboutobjectdescription, interfacename : windows_core::PCSTR) -> u8);
    alljoyn_aboutobjectdescription_hasinterface(core::mem::transmute(description), interfacename.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_hasinterfaceatpath<P1, P2>(description: alljoyn_aboutobjectdescription, path: P1, interfacename: P2) -> u8
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_hasinterfaceatpath(description : alljoyn_aboutobjectdescription, path : windows_core::PCSTR, interfacename : windows_core::PCSTR) -> u8);
    alljoyn_aboutobjectdescription_hasinterfaceatpath(core::mem::transmute(description), path.param().abi(), interfacename.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutobjectdescription_haspath<P1>(description: alljoyn_aboutobjectdescription, path: P1) -> u8
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutobjectdescription_haspath(description : alljoyn_aboutobjectdescription, path : windows_core::PCSTR) -> u8);
    alljoyn_aboutobjectdescription_haspath(core::mem::transmute(description), path.param().abi())
}
#[inline]
pub unsafe fn alljoyn_aboutproxy_create<P1>(bus: alljoyn_busattachment, busname: P1, sessionid: u32) -> alljoyn_aboutproxy
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutproxy_create(bus : alljoyn_busattachment, busname : windows_core::PCSTR, sessionid : u32) -> alljoyn_aboutproxy);
    alljoyn_aboutproxy_create(core::mem::transmute(bus), busname.param().abi(), core::mem::transmute(sessionid))
}
#[inline]
pub unsafe fn alljoyn_aboutproxy_destroy(proxy: alljoyn_aboutproxy) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutproxy_destroy(proxy : alljoyn_aboutproxy));
    alljoyn_aboutproxy_destroy(core::mem::transmute(proxy))
}
#[inline]
pub unsafe fn alljoyn_aboutproxy_getaboutdata<P1>(proxy: alljoyn_aboutproxy, language: P1, data: alljoyn_msgarg) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutproxy_getaboutdata(proxy : alljoyn_aboutproxy, language : windows_core::PCSTR, data : alljoyn_msgarg) -> QStatus);
    alljoyn_aboutproxy_getaboutdata(core::mem::transmute(proxy), language.param().abi(), core::mem::transmute(data))
}
#[inline]
pub unsafe fn alljoyn_aboutproxy_getobjectdescription(proxy: alljoyn_aboutproxy, objectdesc: alljoyn_msgarg) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutproxy_getobjectdescription(proxy : alljoyn_aboutproxy, objectdesc : alljoyn_msgarg) -> QStatus);
    alljoyn_aboutproxy_getobjectdescription(core::mem::transmute(proxy), core::mem::transmute(objectdesc))
}
#[inline]
pub unsafe fn alljoyn_aboutproxy_getversion(proxy: alljoyn_aboutproxy, version: *mut u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_aboutproxy_getversion(proxy : alljoyn_aboutproxy, version : *mut u16) -> QStatus);
    alljoyn_aboutproxy_getversion(core::mem::transmute(proxy), core::mem::transmute(version))
}
#[inline]
pub unsafe fn alljoyn_applicationstatelistener_create(callbacks: *const alljoyn_applicationstatelistener_callbacks, context: *mut core::ffi::c_void) -> alljoyn_applicationstatelistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_applicationstatelistener_create(callbacks : *const alljoyn_applicationstatelistener_callbacks, context : *mut core::ffi::c_void) -> alljoyn_applicationstatelistener);
    alljoyn_applicationstatelistener_create(core::mem::transmute(callbacks), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_applicationstatelistener_destroy(listener: alljoyn_applicationstatelistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_applicationstatelistener_destroy(listener : alljoyn_applicationstatelistener));
    alljoyn_applicationstatelistener_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_authlistener_create(callbacks: *const alljoyn_authlistener_callbacks, context: *const core::ffi::c_void) -> alljoyn_authlistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_authlistener_create(callbacks : *const alljoyn_authlistener_callbacks, context : *const core::ffi::c_void) -> alljoyn_authlistener);
    alljoyn_authlistener_create(core::mem::transmute(callbacks), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_authlistener_destroy(listener: alljoyn_authlistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_authlistener_destroy(listener : alljoyn_authlistener));
    alljoyn_authlistener_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_authlistener_requestcredentialsresponse(listener: alljoyn_authlistener, authcontext: *mut core::ffi::c_void, accept: i32, credentials: alljoyn_credentials) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_authlistener_requestcredentialsresponse(listener : alljoyn_authlistener, authcontext : *mut core::ffi::c_void, accept : i32, credentials : alljoyn_credentials) -> QStatus);
    alljoyn_authlistener_requestcredentialsresponse(core::mem::transmute(listener), core::mem::transmute(authcontext), core::mem::transmute(accept), core::mem::transmute(credentials))
}
#[inline]
pub unsafe fn alljoyn_authlistener_setsharedsecret(listener: alljoyn_authlistener, sharedsecret: *const u8, sharedsecretsize: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_authlistener_setsharedsecret(listener : alljoyn_authlistener, sharedsecret : *const u8, sharedsecretsize : usize) -> QStatus);
    alljoyn_authlistener_setsharedsecret(core::mem::transmute(listener), core::mem::transmute(sharedsecret), core::mem::transmute(sharedsecretsize))
}
#[inline]
pub unsafe fn alljoyn_authlistener_verifycredentialsresponse(listener: alljoyn_authlistener, authcontext: *mut core::ffi::c_void, accept: i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_authlistener_verifycredentialsresponse(listener : alljoyn_authlistener, authcontext : *mut core::ffi::c_void, accept : i32) -> QStatus);
    alljoyn_authlistener_verifycredentialsresponse(core::mem::transmute(listener), core::mem::transmute(authcontext), core::mem::transmute(accept))
}
#[inline]
pub unsafe fn alljoyn_authlistenerasync_create(callbacks: *const alljoyn_authlistenerasync_callbacks, context: *const core::ffi::c_void) -> alljoyn_authlistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_authlistenerasync_create(callbacks : *const alljoyn_authlistenerasync_callbacks, context : *const core::ffi::c_void) -> alljoyn_authlistener);
    alljoyn_authlistenerasync_create(core::mem::transmute(callbacks), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_authlistenerasync_destroy(listener: alljoyn_authlistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_authlistenerasync_destroy(listener : alljoyn_authlistener));
    alljoyn_authlistenerasync_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_autopinger_adddestination<P1, P2>(autopinger: alljoyn_autopinger, group: P1, destination: P2) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_autopinger_adddestination(autopinger : alljoyn_autopinger, group : windows_core::PCSTR, destination : windows_core::PCSTR) -> QStatus);
    alljoyn_autopinger_adddestination(core::mem::transmute(autopinger), group.param().abi(), destination.param().abi())
}
#[inline]
pub unsafe fn alljoyn_autopinger_addpinggroup<P1>(autopinger: alljoyn_autopinger, group: P1, listener: alljoyn_pinglistener, pinginterval: u32)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_autopinger_addpinggroup(autopinger : alljoyn_autopinger, group : windows_core::PCSTR, listener : alljoyn_pinglistener, pinginterval : u32));
    alljoyn_autopinger_addpinggroup(core::mem::transmute(autopinger), group.param().abi(), core::mem::transmute(listener), core::mem::transmute(pinginterval))
}
#[inline]
pub unsafe fn alljoyn_autopinger_create(bus: alljoyn_busattachment) -> alljoyn_autopinger {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_autopinger_create(bus : alljoyn_busattachment) -> alljoyn_autopinger);
    alljoyn_autopinger_create(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_autopinger_destroy(autopinger: alljoyn_autopinger) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_autopinger_destroy(autopinger : alljoyn_autopinger));
    alljoyn_autopinger_destroy(core::mem::transmute(autopinger))
}
#[inline]
pub unsafe fn alljoyn_autopinger_pause(autopinger: alljoyn_autopinger) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_autopinger_pause(autopinger : alljoyn_autopinger));
    alljoyn_autopinger_pause(core::mem::transmute(autopinger))
}
#[inline]
pub unsafe fn alljoyn_autopinger_removedestination<P1, P2>(autopinger: alljoyn_autopinger, group: P1, destination: P2, removeall: i32) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_autopinger_removedestination(autopinger : alljoyn_autopinger, group : windows_core::PCSTR, destination : windows_core::PCSTR, removeall : i32) -> QStatus);
    alljoyn_autopinger_removedestination(core::mem::transmute(autopinger), group.param().abi(), destination.param().abi(), core::mem::transmute(removeall))
}
#[inline]
pub unsafe fn alljoyn_autopinger_removepinggroup<P1>(autopinger: alljoyn_autopinger, group: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_autopinger_removepinggroup(autopinger : alljoyn_autopinger, group : windows_core::PCSTR));
    alljoyn_autopinger_removepinggroup(core::mem::transmute(autopinger), group.param().abi())
}
#[inline]
pub unsafe fn alljoyn_autopinger_resume(autopinger: alljoyn_autopinger) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_autopinger_resume(autopinger : alljoyn_autopinger));
    alljoyn_autopinger_resume(core::mem::transmute(autopinger))
}
#[inline]
pub unsafe fn alljoyn_autopinger_setpinginterval<P1>(autopinger: alljoyn_autopinger, group: P1, pinginterval: u32) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_autopinger_setpinginterval(autopinger : alljoyn_autopinger, group : windows_core::PCSTR, pinginterval : u32) -> QStatus);
    alljoyn_autopinger_setpinginterval(core::mem::transmute(autopinger), group.param().abi(), core::mem::transmute(pinginterval))
}
#[inline]
pub unsafe fn alljoyn_busattachment_addlogonentry<P1, P2, P3>(bus: alljoyn_busattachment, authmechanism: P1, username: P2, password: P3) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_addlogonentry(bus : alljoyn_busattachment, authmechanism : windows_core::PCSTR, username : windows_core::PCSTR, password : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_addlogonentry(core::mem::transmute(bus), authmechanism.param().abi(), username.param().abi(), password.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_addmatch<P1>(bus: alljoyn_busattachment, rule: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_addmatch(bus : alljoyn_busattachment, rule : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_addmatch(core::mem::transmute(bus), rule.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_advertisename<P1>(bus: alljoyn_busattachment, name: P1, transports: u16) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_advertisename(bus : alljoyn_busattachment, name : windows_core::PCSTR, transports : u16) -> QStatus);
    alljoyn_busattachment_advertisename(core::mem::transmute(bus), name.param().abi(), core::mem::transmute(transports))
}
#[inline]
pub unsafe fn alljoyn_busattachment_bindsessionport(bus: alljoyn_busattachment, sessionport: *mut u16, opts: alljoyn_sessionopts, listener: alljoyn_sessionportlistener) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_bindsessionport(bus : alljoyn_busattachment, sessionport : *mut u16, opts : alljoyn_sessionopts, listener : alljoyn_sessionportlistener) -> QStatus);
    alljoyn_busattachment_bindsessionport(core::mem::transmute(bus), core::mem::transmute(sessionport), core::mem::transmute(opts), core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_busattachment_canceladvertisename<P1>(bus: alljoyn_busattachment, name: P1, transports: u16) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_canceladvertisename(bus : alljoyn_busattachment, name : windows_core::PCSTR, transports : u16) -> QStatus);
    alljoyn_busattachment_canceladvertisename(core::mem::transmute(bus), name.param().abi(), core::mem::transmute(transports))
}
#[inline]
pub unsafe fn alljoyn_busattachment_cancelfindadvertisedname<P1>(bus: alljoyn_busattachment, nameprefix: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_cancelfindadvertisedname(bus : alljoyn_busattachment, nameprefix : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_cancelfindadvertisedname(core::mem::transmute(bus), nameprefix.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_cancelfindadvertisednamebytransport<P1>(bus: alljoyn_busattachment, nameprefix: P1, transports: u16) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_cancelfindadvertisednamebytransport(bus : alljoyn_busattachment, nameprefix : windows_core::PCSTR, transports : u16) -> QStatus);
    alljoyn_busattachment_cancelfindadvertisednamebytransport(core::mem::transmute(bus), nameprefix.param().abi(), core::mem::transmute(transports))
}
#[inline]
pub unsafe fn alljoyn_busattachment_cancelwhoimplements_interface<P1>(bus: alljoyn_busattachment, implementsinterface: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_cancelwhoimplements_interface(bus : alljoyn_busattachment, implementsinterface : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_cancelwhoimplements_interface(core::mem::transmute(bus), implementsinterface.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_cancelwhoimplements_interfaces(bus: alljoyn_busattachment, implementsinterfaces: *const *const i8, numberinterfaces: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_cancelwhoimplements_interfaces(bus : alljoyn_busattachment, implementsinterfaces : *const *const i8, numberinterfaces : usize) -> QStatus);
    alljoyn_busattachment_cancelwhoimplements_interfaces(core::mem::transmute(bus), core::mem::transmute(implementsinterfaces), core::mem::transmute(numberinterfaces))
}
#[inline]
pub unsafe fn alljoyn_busattachment_clearkeys<P1>(bus: alljoyn_busattachment, guid: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_clearkeys(bus : alljoyn_busattachment, guid : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_clearkeys(core::mem::transmute(bus), guid.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_clearkeystore(bus: alljoyn_busattachment) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_clearkeystore(bus : alljoyn_busattachment));
    alljoyn_busattachment_clearkeystore(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_connect<P1>(bus: alljoyn_busattachment, connectspec: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_connect(bus : alljoyn_busattachment, connectspec : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_connect(core::mem::transmute(bus), connectspec.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_create<P0>(applicationname: P0, allowremotemessages: i32) -> alljoyn_busattachment
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_create(applicationname : windows_core::PCSTR, allowremotemessages : i32) -> alljoyn_busattachment);
    alljoyn_busattachment_create(applicationname.param().abi(), core::mem::transmute(allowremotemessages))
}
#[inline]
pub unsafe fn alljoyn_busattachment_create_concurrency<P0>(applicationname: P0, allowremotemessages: i32, concurrency: u32) -> alljoyn_busattachment
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_create_concurrency(applicationname : windows_core::PCSTR, allowremotemessages : i32, concurrency : u32) -> alljoyn_busattachment);
    alljoyn_busattachment_create_concurrency(applicationname.param().abi(), core::mem::transmute(allowremotemessages), core::mem::transmute(concurrency))
}
#[inline]
pub unsafe fn alljoyn_busattachment_createinterface<P1>(bus: alljoyn_busattachment, name: P1, iface: *mut alljoyn_interfacedescription) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_createinterface(bus : alljoyn_busattachment, name : windows_core::PCSTR, iface : *mut alljoyn_interfacedescription) -> QStatus);
    alljoyn_busattachment_createinterface(core::mem::transmute(bus), name.param().abi(), core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_busattachment_createinterface_secure<P1>(bus: alljoyn_busattachment, name: P1, iface: *mut alljoyn_interfacedescription, secpolicy: alljoyn_interfacedescription_securitypolicy) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_createinterface_secure(bus : alljoyn_busattachment, name : windows_core::PCSTR, iface : *mut alljoyn_interfacedescription, secpolicy : alljoyn_interfacedescription_securitypolicy) -> QStatus);
    alljoyn_busattachment_createinterface_secure(core::mem::transmute(bus), name.param().abi(), core::mem::transmute(iface), core::mem::transmute(secpolicy))
}
#[inline]
pub unsafe fn alljoyn_busattachment_createinterfacesfromxml<P1>(bus: alljoyn_busattachment, xml: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_createinterfacesfromxml(bus : alljoyn_busattachment, xml : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_createinterfacesfromxml(core::mem::transmute(bus), xml.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_deletedefaultkeystore<P0>(applicationname: P0) -> QStatus
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_deletedefaultkeystore(applicationname : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_deletedefaultkeystore(applicationname.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_deleteinterface(bus: alljoyn_busattachment, iface: alljoyn_interfacedescription) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_deleteinterface(bus : alljoyn_busattachment, iface : alljoyn_interfacedescription) -> QStatus);
    alljoyn_busattachment_deleteinterface(core::mem::transmute(bus), core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_busattachment_destroy(bus: alljoyn_busattachment) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_destroy(bus : alljoyn_busattachment));
    alljoyn_busattachment_destroy(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_disconnect<P1>(bus: alljoyn_busattachment, unused: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_disconnect(bus : alljoyn_busattachment, unused : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_disconnect(core::mem::transmute(bus), unused.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_enableconcurrentcallbacks(bus: alljoyn_busattachment) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_enableconcurrentcallbacks(bus : alljoyn_busattachment));
    alljoyn_busattachment_enableconcurrentcallbacks(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_enablepeersecurity<P1, P3>(bus: alljoyn_busattachment, authmechanisms: P1, listener: alljoyn_authlistener, keystorefilename: P3, isshared: i32) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_enablepeersecurity(bus : alljoyn_busattachment, authmechanisms : windows_core::PCSTR, listener : alljoyn_authlistener, keystorefilename : windows_core::PCSTR, isshared : i32) -> QStatus);
    alljoyn_busattachment_enablepeersecurity(core::mem::transmute(bus), authmechanisms.param().abi(), core::mem::transmute(listener), keystorefilename.param().abi(), core::mem::transmute(isshared))
}
#[inline]
pub unsafe fn alljoyn_busattachment_enablepeersecuritywithpermissionconfigurationlistener<P1, P3>(bus: alljoyn_busattachment, authmechanisms: P1, authlistener: alljoyn_authlistener, keystorefilename: P3, isshared: i32, permissionconfigurationlistener: alljoyn_permissionconfigurationlistener) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_enablepeersecuritywithpermissionconfigurationlistener(bus : alljoyn_busattachment, authmechanisms : windows_core::PCSTR, authlistener : alljoyn_authlistener, keystorefilename : windows_core::PCSTR, isshared : i32, permissionconfigurationlistener : alljoyn_permissionconfigurationlistener) -> QStatus);
    alljoyn_busattachment_enablepeersecuritywithpermissionconfigurationlistener(core::mem::transmute(bus), authmechanisms.param().abi(), core::mem::transmute(authlistener), keystorefilename.param().abi(), core::mem::transmute(isshared), core::mem::transmute(permissionconfigurationlistener))
}
#[inline]
pub unsafe fn alljoyn_busattachment_findadvertisedname<P1>(bus: alljoyn_busattachment, nameprefix: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_findadvertisedname(bus : alljoyn_busattachment, nameprefix : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_findadvertisedname(core::mem::transmute(bus), nameprefix.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_findadvertisednamebytransport<P1>(bus: alljoyn_busattachment, nameprefix: P1, transports: u16) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_findadvertisednamebytransport(bus : alljoyn_busattachment, nameprefix : windows_core::PCSTR, transports : u16) -> QStatus);
    alljoyn_busattachment_findadvertisednamebytransport(core::mem::transmute(bus), nameprefix.param().abi(), core::mem::transmute(transports))
}
#[inline]
pub unsafe fn alljoyn_busattachment_getalljoyndebugobj(bus: alljoyn_busattachment) -> alljoyn_proxybusobject {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getalljoyndebugobj(bus : alljoyn_busattachment) -> alljoyn_proxybusobject);
    alljoyn_busattachment_getalljoyndebugobj(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_getalljoynproxyobj(bus: alljoyn_busattachment) -> alljoyn_proxybusobject {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getalljoynproxyobj(bus : alljoyn_busattachment) -> alljoyn_proxybusobject);
    alljoyn_busattachment_getalljoynproxyobj(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_getconcurrency(bus: alljoyn_busattachment) -> u32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getconcurrency(bus : alljoyn_busattachment) -> u32);
    alljoyn_busattachment_getconcurrency(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_getconnectspec(bus: alljoyn_busattachment) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getconnectspec(bus : alljoyn_busattachment) -> windows_core::PCSTR);
    alljoyn_busattachment_getconnectspec(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_getdbusproxyobj(bus: alljoyn_busattachment) -> alljoyn_proxybusobject {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getdbusproxyobj(bus : alljoyn_busattachment) -> alljoyn_proxybusobject);
    alljoyn_busattachment_getdbusproxyobj(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_getglobalguidstring(bus: alljoyn_busattachment) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getglobalguidstring(bus : alljoyn_busattachment) -> windows_core::PCSTR);
    alljoyn_busattachment_getglobalguidstring(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_getinterface<P1>(bus: alljoyn_busattachment, name: P1) -> alljoyn_interfacedescription
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getinterface(bus : alljoyn_busattachment, name : windows_core::PCSTR) -> alljoyn_interfacedescription);
    alljoyn_busattachment_getinterface(core::mem::transmute(bus), name.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_getinterfaces(bus: alljoyn_busattachment, ifaces: *const alljoyn_interfacedescription, numifaces: usize) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getinterfaces(bus : alljoyn_busattachment, ifaces : *const alljoyn_interfacedescription, numifaces : usize) -> usize);
    alljoyn_busattachment_getinterfaces(core::mem::transmute(bus), core::mem::transmute(ifaces), core::mem::transmute(numifaces))
}
#[inline]
pub unsafe fn alljoyn_busattachment_getkeyexpiration<P1>(bus: alljoyn_busattachment, guid: P1, timeout: *mut u32) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getkeyexpiration(bus : alljoyn_busattachment, guid : windows_core::PCSTR, timeout : *mut u32) -> QStatus);
    alljoyn_busattachment_getkeyexpiration(core::mem::transmute(bus), guid.param().abi(), core::mem::transmute(timeout))
}
#[inline]
pub unsafe fn alljoyn_busattachment_getpeerguid<P1, P2>(bus: alljoyn_busattachment, name: P1, guid: P2, guidsz: *mut usize) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getpeerguid(bus : alljoyn_busattachment, name : windows_core::PCSTR, guid : windows_core::PCSTR, guidsz : *mut usize) -> QStatus);
    alljoyn_busattachment_getpeerguid(core::mem::transmute(bus), name.param().abi(), guid.param().abi(), core::mem::transmute(guidsz))
}
#[inline]
pub unsafe fn alljoyn_busattachment_getpermissionconfigurator(bus: alljoyn_busattachment) -> alljoyn_permissionconfigurator {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getpermissionconfigurator(bus : alljoyn_busattachment) -> alljoyn_permissionconfigurator);
    alljoyn_busattachment_getpermissionconfigurator(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_gettimestamp() -> u32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_gettimestamp() -> u32);
    alljoyn_busattachment_gettimestamp()
}
#[inline]
pub unsafe fn alljoyn_busattachment_getuniquename(bus: alljoyn_busattachment) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_getuniquename(bus : alljoyn_busattachment) -> windows_core::PCSTR);
    alljoyn_busattachment_getuniquename(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_isconnected(bus: alljoyn_busattachment) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_isconnected(bus : alljoyn_busattachment) -> i32);
    alljoyn_busattachment_isconnected(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_ispeersecurityenabled(bus: alljoyn_busattachment) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_ispeersecurityenabled(bus : alljoyn_busattachment) -> i32);
    alljoyn_busattachment_ispeersecurityenabled(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_isstarted(bus: alljoyn_busattachment) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_isstarted(bus : alljoyn_busattachment) -> i32);
    alljoyn_busattachment_isstarted(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_isstopping(bus: alljoyn_busattachment) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_isstopping(bus : alljoyn_busattachment) -> i32);
    alljoyn_busattachment_isstopping(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_join(bus: alljoyn_busattachment) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_join(bus : alljoyn_busattachment) -> QStatus);
    alljoyn_busattachment_join(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_joinsession<P1>(bus: alljoyn_busattachment, sessionhost: P1, sessionport: u16, listener: alljoyn_sessionlistener, sessionid: *mut u32, opts: alljoyn_sessionopts) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_joinsession(bus : alljoyn_busattachment, sessionhost : windows_core::PCSTR, sessionport : u16, listener : alljoyn_sessionlistener, sessionid : *mut u32, opts : alljoyn_sessionopts) -> QStatus);
    alljoyn_busattachment_joinsession(core::mem::transmute(bus), sessionhost.param().abi(), core::mem::transmute(sessionport), core::mem::transmute(listener), core::mem::transmute(sessionid), core::mem::transmute(opts))
}
#[inline]
pub unsafe fn alljoyn_busattachment_joinsessionasync<P1>(bus: alljoyn_busattachment, sessionhost: P1, sessionport: u16, listener: alljoyn_sessionlistener, opts: alljoyn_sessionopts, callback: alljoyn_busattachment_joinsessioncb_ptr, context: *mut core::ffi::c_void) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_joinsessionasync(bus : alljoyn_busattachment, sessionhost : windows_core::PCSTR, sessionport : u16, listener : alljoyn_sessionlistener, opts : alljoyn_sessionopts, callback : alljoyn_busattachment_joinsessioncb_ptr, context : *mut core::ffi::c_void) -> QStatus);
    alljoyn_busattachment_joinsessionasync(core::mem::transmute(bus), sessionhost.param().abi(), core::mem::transmute(sessionport), core::mem::transmute(listener), core::mem::transmute(opts), core::mem::transmute(callback), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_busattachment_leavesession(bus: alljoyn_busattachment, sessionid: u32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_leavesession(bus : alljoyn_busattachment, sessionid : u32) -> QStatus);
    alljoyn_busattachment_leavesession(core::mem::transmute(bus), core::mem::transmute(sessionid))
}
#[inline]
pub unsafe fn alljoyn_busattachment_namehasowner<P1>(bus: alljoyn_busattachment, name: P1, hasowner: *mut i32) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_namehasowner(bus : alljoyn_busattachment, name : windows_core::PCSTR, hasowner : *mut i32) -> QStatus);
    alljoyn_busattachment_namehasowner(core::mem::transmute(bus), name.param().abi(), core::mem::transmute(hasowner))
}
#[inline]
pub unsafe fn alljoyn_busattachment_ping<P1>(bus: alljoyn_busattachment, name: P1, timeout: u32) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_ping(bus : alljoyn_busattachment, name : windows_core::PCSTR, timeout : u32) -> QStatus);
    alljoyn_busattachment_ping(core::mem::transmute(bus), name.param().abi(), core::mem::transmute(timeout))
}
#[inline]
pub unsafe fn alljoyn_busattachment_registeraboutlistener(bus: alljoyn_busattachment, aboutlistener: alljoyn_aboutlistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_registeraboutlistener(bus : alljoyn_busattachment, aboutlistener : alljoyn_aboutlistener));
    alljoyn_busattachment_registeraboutlistener(core::mem::transmute(bus), core::mem::transmute(aboutlistener))
}
#[inline]
pub unsafe fn alljoyn_busattachment_registerapplicationstatelistener(bus: alljoyn_busattachment, listener: alljoyn_applicationstatelistener) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_registerapplicationstatelistener(bus : alljoyn_busattachment, listener : alljoyn_applicationstatelistener) -> QStatus);
    alljoyn_busattachment_registerapplicationstatelistener(core::mem::transmute(bus), core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_busattachment_registerbuslistener(bus: alljoyn_busattachment, listener: alljoyn_buslistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_registerbuslistener(bus : alljoyn_busattachment, listener : alljoyn_buslistener));
    alljoyn_busattachment_registerbuslistener(core::mem::transmute(bus), core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_busattachment_registerbusobject(bus: alljoyn_busattachment, obj: alljoyn_busobject) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_registerbusobject(bus : alljoyn_busattachment, obj : alljoyn_busobject) -> QStatus);
    alljoyn_busattachment_registerbusobject(core::mem::transmute(bus), core::mem::transmute(obj))
}
#[inline]
pub unsafe fn alljoyn_busattachment_registerbusobject_secure(bus: alljoyn_busattachment, obj: alljoyn_busobject) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_registerbusobject_secure(bus : alljoyn_busattachment, obj : alljoyn_busobject) -> QStatus);
    alljoyn_busattachment_registerbusobject_secure(core::mem::transmute(bus), core::mem::transmute(obj))
}
#[inline]
pub unsafe fn alljoyn_busattachment_registerkeystorelistener(bus: alljoyn_busattachment, listener: alljoyn_keystorelistener) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_registerkeystorelistener(bus : alljoyn_busattachment, listener : alljoyn_keystorelistener) -> QStatus);
    alljoyn_busattachment_registerkeystorelistener(core::mem::transmute(bus), core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_busattachment_registersignalhandler<P3>(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, srcpath: P3) -> QStatus
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_registersignalhandler(bus : alljoyn_busattachment, signal_handler : alljoyn_messagereceiver_signalhandler_ptr, member : alljoyn_interfacedescription_member, srcpath : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_registersignalhandler(core::mem::transmute(bus), core::mem::transmute(signal_handler), core::mem::transmute(member), srcpath.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_registersignalhandlerwithrule<P3>(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, matchrule: P3) -> QStatus
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_registersignalhandlerwithrule(bus : alljoyn_busattachment, signal_handler : alljoyn_messagereceiver_signalhandler_ptr, member : alljoyn_interfacedescription_member, matchrule : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_registersignalhandlerwithrule(core::mem::transmute(bus), core::mem::transmute(signal_handler), core::mem::transmute(member), matchrule.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_releasename<P1>(bus: alljoyn_busattachment, name: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_releasename(bus : alljoyn_busattachment, name : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_releasename(core::mem::transmute(bus), name.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_reloadkeystore(bus: alljoyn_busattachment) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_reloadkeystore(bus : alljoyn_busattachment) -> QStatus);
    alljoyn_busattachment_reloadkeystore(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_removematch<P1>(bus: alljoyn_busattachment, rule: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_removematch(bus : alljoyn_busattachment, rule : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_removematch(core::mem::transmute(bus), rule.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_removesessionmember<P2>(bus: alljoyn_busattachment, sessionid: u32, membername: P2) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_removesessionmember(bus : alljoyn_busattachment, sessionid : u32, membername : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_removesessionmember(core::mem::transmute(bus), core::mem::transmute(sessionid), membername.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_requestname<P1>(bus: alljoyn_busattachment, requestedname: P1, flags: u32) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_requestname(bus : alljoyn_busattachment, requestedname : windows_core::PCSTR, flags : u32) -> QStatus);
    alljoyn_busattachment_requestname(core::mem::transmute(bus), requestedname.param().abi(), core::mem::transmute(flags))
}
#[inline]
pub unsafe fn alljoyn_busattachment_secureconnection<P1>(bus: alljoyn_busattachment, name: P1, forceauth: i32) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_secureconnection(bus : alljoyn_busattachment, name : windows_core::PCSTR, forceauth : i32) -> QStatus);
    alljoyn_busattachment_secureconnection(core::mem::transmute(bus), name.param().abi(), core::mem::transmute(forceauth))
}
#[inline]
pub unsafe fn alljoyn_busattachment_secureconnectionasync<P1>(bus: alljoyn_busattachment, name: P1, forceauth: i32) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_secureconnectionasync(bus : alljoyn_busattachment, name : windows_core::PCSTR, forceauth : i32) -> QStatus);
    alljoyn_busattachment_secureconnectionasync(core::mem::transmute(bus), name.param().abi(), core::mem::transmute(forceauth))
}
#[inline]
pub unsafe fn alljoyn_busattachment_setdaemondebug<P1>(bus: alljoyn_busattachment, module: P1, level: u32) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_setdaemondebug(bus : alljoyn_busattachment, module : windows_core::PCSTR, level : u32) -> QStatus);
    alljoyn_busattachment_setdaemondebug(core::mem::transmute(bus), module.param().abi(), core::mem::transmute(level))
}
#[inline]
pub unsafe fn alljoyn_busattachment_setkeyexpiration<P1>(bus: alljoyn_busattachment, guid: P1, timeout: u32) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_setkeyexpiration(bus : alljoyn_busattachment, guid : windows_core::PCSTR, timeout : u32) -> QStatus);
    alljoyn_busattachment_setkeyexpiration(core::mem::transmute(bus), guid.param().abi(), core::mem::transmute(timeout))
}
#[inline]
pub unsafe fn alljoyn_busattachment_setlinktimeout(bus: alljoyn_busattachment, sessionid: u32, linktimeout: *mut u32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_setlinktimeout(bus : alljoyn_busattachment, sessionid : u32, linktimeout : *mut u32) -> QStatus);
    alljoyn_busattachment_setlinktimeout(core::mem::transmute(bus), core::mem::transmute(sessionid), core::mem::transmute(linktimeout))
}
#[inline]
pub unsafe fn alljoyn_busattachment_setlinktimeoutasync(bus: alljoyn_busattachment, sessionid: u32, linktimeout: u32, callback: alljoyn_busattachment_setlinktimeoutcb_ptr, context: *mut core::ffi::c_void) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_setlinktimeoutasync(bus : alljoyn_busattachment, sessionid : u32, linktimeout : u32, callback : alljoyn_busattachment_setlinktimeoutcb_ptr, context : *mut core::ffi::c_void) -> QStatus);
    alljoyn_busattachment_setlinktimeoutasync(core::mem::transmute(bus), core::mem::transmute(sessionid), core::mem::transmute(linktimeout), core::mem::transmute(callback), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_busattachment_setsessionlistener(bus: alljoyn_busattachment, sessionid: u32, listener: alljoyn_sessionlistener) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_setsessionlistener(bus : alljoyn_busattachment, sessionid : u32, listener : alljoyn_sessionlistener) -> QStatus);
    alljoyn_busattachment_setsessionlistener(core::mem::transmute(bus), core::mem::transmute(sessionid), core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_busattachment_start(bus: alljoyn_busattachment) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_start(bus : alljoyn_busattachment) -> QStatus);
    alljoyn_busattachment_start(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_stop(bus: alljoyn_busattachment) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_stop(bus : alljoyn_busattachment) -> QStatus);
    alljoyn_busattachment_stop(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_unbindsessionport(bus: alljoyn_busattachment, sessionport: u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_unbindsessionport(bus : alljoyn_busattachment, sessionport : u16) -> QStatus);
    alljoyn_busattachment_unbindsessionport(core::mem::transmute(bus), core::mem::transmute(sessionport))
}
#[inline]
pub unsafe fn alljoyn_busattachment_unregisteraboutlistener(bus: alljoyn_busattachment, aboutlistener: alljoyn_aboutlistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_unregisteraboutlistener(bus : alljoyn_busattachment, aboutlistener : alljoyn_aboutlistener));
    alljoyn_busattachment_unregisteraboutlistener(core::mem::transmute(bus), core::mem::transmute(aboutlistener))
}
#[inline]
pub unsafe fn alljoyn_busattachment_unregisterallaboutlisteners(bus: alljoyn_busattachment) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_unregisterallaboutlisteners(bus : alljoyn_busattachment));
    alljoyn_busattachment_unregisterallaboutlisteners(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_unregisterallhandlers(bus: alljoyn_busattachment) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_unregisterallhandlers(bus : alljoyn_busattachment) -> QStatus);
    alljoyn_busattachment_unregisterallhandlers(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busattachment_unregisterapplicationstatelistener(bus: alljoyn_busattachment, listener: alljoyn_applicationstatelistener) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_unregisterapplicationstatelistener(bus : alljoyn_busattachment, listener : alljoyn_applicationstatelistener) -> QStatus);
    alljoyn_busattachment_unregisterapplicationstatelistener(core::mem::transmute(bus), core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_busattachment_unregisterbuslistener(bus: alljoyn_busattachment, listener: alljoyn_buslistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_unregisterbuslistener(bus : alljoyn_busattachment, listener : alljoyn_buslistener));
    alljoyn_busattachment_unregisterbuslistener(core::mem::transmute(bus), core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_busattachment_unregisterbusobject(bus: alljoyn_busattachment, object: alljoyn_busobject) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_unregisterbusobject(bus : alljoyn_busattachment, object : alljoyn_busobject));
    alljoyn_busattachment_unregisterbusobject(core::mem::transmute(bus), core::mem::transmute(object))
}
#[inline]
pub unsafe fn alljoyn_busattachment_unregistersignalhandler<P3>(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, srcpath: P3) -> QStatus
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_unregistersignalhandler(bus : alljoyn_busattachment, signal_handler : alljoyn_messagereceiver_signalhandler_ptr, member : alljoyn_interfacedescription_member, srcpath : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_unregistersignalhandler(core::mem::transmute(bus), core::mem::transmute(signal_handler), core::mem::transmute(member), srcpath.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_unregistersignalhandlerwithrule<P3>(bus: alljoyn_busattachment, signal_handler: alljoyn_messagereceiver_signalhandler_ptr, member: alljoyn_interfacedescription_member, matchrule: P3) -> QStatus
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_unregistersignalhandlerwithrule(bus : alljoyn_busattachment, signal_handler : alljoyn_messagereceiver_signalhandler_ptr, member : alljoyn_interfacedescription_member, matchrule : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_unregistersignalhandlerwithrule(core::mem::transmute(bus), core::mem::transmute(signal_handler), core::mem::transmute(member), matchrule.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_whoimplements_interface<P1>(bus: alljoyn_busattachment, implementsinterface: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_whoimplements_interface(bus : alljoyn_busattachment, implementsinterface : windows_core::PCSTR) -> QStatus);
    alljoyn_busattachment_whoimplements_interface(core::mem::transmute(bus), implementsinterface.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busattachment_whoimplements_interfaces(bus: alljoyn_busattachment, implementsinterfaces: *const *const i8, numberinterfaces: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busattachment_whoimplements_interfaces(bus : alljoyn_busattachment, implementsinterfaces : *const *const i8, numberinterfaces : usize) -> QStatus);
    alljoyn_busattachment_whoimplements_interfaces(core::mem::transmute(bus), core::mem::transmute(implementsinterfaces), core::mem::transmute(numberinterfaces))
}
#[inline]
pub unsafe fn alljoyn_buslistener_create(callbacks: *const alljoyn_buslistener_callbacks, context: *const core::ffi::c_void) -> alljoyn_buslistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_buslistener_create(callbacks : *const alljoyn_buslistener_callbacks, context : *const core::ffi::c_void) -> alljoyn_buslistener);
    alljoyn_buslistener_create(core::mem::transmute(callbacks), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_buslistener_destroy(listener: alljoyn_buslistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_buslistener_destroy(listener : alljoyn_buslistener));
    alljoyn_buslistener_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_busobject_addinterface(bus: alljoyn_busobject, iface: alljoyn_interfacedescription) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_addinterface(bus : alljoyn_busobject, iface : alljoyn_interfacedescription) -> QStatus);
    alljoyn_busobject_addinterface(core::mem::transmute(bus), core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_busobject_addinterface_announced(bus: alljoyn_busobject, iface: alljoyn_interfacedescription) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_addinterface_announced(bus : alljoyn_busobject, iface : alljoyn_interfacedescription) -> QStatus);
    alljoyn_busobject_addinterface_announced(core::mem::transmute(bus), core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_busobject_addmethodhandler(bus: alljoyn_busobject, member: alljoyn_interfacedescription_member, handler: alljoyn_messagereceiver_methodhandler_ptr, context: *mut core::ffi::c_void) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_addmethodhandler(bus : alljoyn_busobject, member : alljoyn_interfacedescription_member, handler : alljoyn_messagereceiver_methodhandler_ptr, context : *mut core::ffi::c_void) -> QStatus);
    alljoyn_busobject_addmethodhandler(core::mem::transmute(bus), core::mem::transmute(member), core::mem::transmute(handler), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_busobject_addmethodhandlers(bus: alljoyn_busobject, entries: *const alljoyn_busobject_methodentry, numentries: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_addmethodhandlers(bus : alljoyn_busobject, entries : *const alljoyn_busobject_methodentry, numentries : usize) -> QStatus);
    alljoyn_busobject_addmethodhandlers(core::mem::transmute(bus), core::mem::transmute(entries), core::mem::transmute(numentries))
}
#[inline]
pub unsafe fn alljoyn_busobject_cancelsessionlessmessage(bus: alljoyn_busobject, msg: alljoyn_message) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_cancelsessionlessmessage(bus : alljoyn_busobject, msg : alljoyn_message) -> QStatus);
    alljoyn_busobject_cancelsessionlessmessage(core::mem::transmute(bus), core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_busobject_cancelsessionlessmessage_serial(bus: alljoyn_busobject, serialnumber: u32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_cancelsessionlessmessage_serial(bus : alljoyn_busobject, serialnumber : u32) -> QStatus);
    alljoyn_busobject_cancelsessionlessmessage_serial(core::mem::transmute(bus), core::mem::transmute(serialnumber))
}
#[inline]
pub unsafe fn alljoyn_busobject_create<P0>(path: P0, isplaceholder: i32, callbacks_in: *const alljoyn_busobject_callbacks, context_in: *const core::ffi::c_void) -> alljoyn_busobject
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_create(path : windows_core::PCSTR, isplaceholder : i32, callbacks_in : *const alljoyn_busobject_callbacks, context_in : *const core::ffi::c_void) -> alljoyn_busobject);
    alljoyn_busobject_create(path.param().abi(), core::mem::transmute(isplaceholder), core::mem::transmute(callbacks_in), core::mem::transmute(context_in))
}
#[inline]
pub unsafe fn alljoyn_busobject_destroy(bus: alljoyn_busobject) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_destroy(bus : alljoyn_busobject));
    alljoyn_busobject_destroy(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busobject_emitpropertieschanged<P1>(bus: alljoyn_busobject, ifcname: P1, propnames: *const *const i8, numprops: usize, id: u32)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_emitpropertieschanged(bus : alljoyn_busobject, ifcname : windows_core::PCSTR, propnames : *const *const i8, numprops : usize, id : u32));
    alljoyn_busobject_emitpropertieschanged(core::mem::transmute(bus), ifcname.param().abi(), core::mem::transmute(propnames), core::mem::transmute(numprops), core::mem::transmute(id))
}
#[inline]
pub unsafe fn alljoyn_busobject_emitpropertychanged<P1, P2>(bus: alljoyn_busobject, ifcname: P1, propname: P2, val: alljoyn_msgarg, id: u32)
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_emitpropertychanged(bus : alljoyn_busobject, ifcname : windows_core::PCSTR, propname : windows_core::PCSTR, val : alljoyn_msgarg, id : u32));
    alljoyn_busobject_emitpropertychanged(core::mem::transmute(bus), ifcname.param().abi(), propname.param().abi(), core::mem::transmute(val), core::mem::transmute(id))
}
#[inline]
pub unsafe fn alljoyn_busobject_getannouncedinterfacenames(bus: alljoyn_busobject, interfaces: *const *const i8, numinterfaces: usize) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_getannouncedinterfacenames(bus : alljoyn_busobject, interfaces : *const *const i8, numinterfaces : usize) -> usize);
    alljoyn_busobject_getannouncedinterfacenames(core::mem::transmute(bus), core::mem::transmute(interfaces), core::mem::transmute(numinterfaces))
}
#[inline]
pub unsafe fn alljoyn_busobject_getbusattachment(bus: alljoyn_busobject) -> alljoyn_busattachment {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_getbusattachment(bus : alljoyn_busobject) -> alljoyn_busattachment);
    alljoyn_busobject_getbusattachment(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busobject_getname<P1>(bus: alljoyn_busobject, buffer: P1, buffersz: usize) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_getname(bus : alljoyn_busobject, buffer : windows_core::PCSTR, buffersz : usize) -> usize);
    alljoyn_busobject_getname(core::mem::transmute(bus), buffer.param().abi(), core::mem::transmute(buffersz))
}
#[inline]
pub unsafe fn alljoyn_busobject_getpath(bus: alljoyn_busobject) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_getpath(bus : alljoyn_busobject) -> windows_core::PCSTR);
    alljoyn_busobject_getpath(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busobject_issecure(bus: alljoyn_busobject) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_issecure(bus : alljoyn_busobject) -> i32);
    alljoyn_busobject_issecure(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_busobject_methodreply_args(bus: alljoyn_busobject, msg: alljoyn_message, args: alljoyn_msgarg, numargs: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_methodreply_args(bus : alljoyn_busobject, msg : alljoyn_message, args : alljoyn_msgarg, numargs : usize) -> QStatus);
    alljoyn_busobject_methodreply_args(core::mem::transmute(bus), core::mem::transmute(msg), core::mem::transmute(args), core::mem::transmute(numargs))
}
#[inline]
pub unsafe fn alljoyn_busobject_methodreply_err<P2, P3>(bus: alljoyn_busobject, msg: alljoyn_message, error: P2, errormessage: P3) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_methodreply_err(bus : alljoyn_busobject, msg : alljoyn_message, error : windows_core::PCSTR, errormessage : windows_core::PCSTR) -> QStatus);
    alljoyn_busobject_methodreply_err(core::mem::transmute(bus), core::mem::transmute(msg), error.param().abi(), errormessage.param().abi())
}
#[inline]
pub unsafe fn alljoyn_busobject_methodreply_status(bus: alljoyn_busobject, msg: alljoyn_message, status: QStatus) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_methodreply_status(bus : alljoyn_busobject, msg : alljoyn_message, status : QStatus) -> QStatus);
    alljoyn_busobject_methodreply_status(core::mem::transmute(bus), core::mem::transmute(msg), core::mem::transmute(status))
}
#[inline]
pub unsafe fn alljoyn_busobject_setannounceflag(bus: alljoyn_busobject, iface: alljoyn_interfacedescription, isannounced: alljoyn_about_announceflag) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_setannounceflag(bus : alljoyn_busobject, iface : alljoyn_interfacedescription, isannounced : alljoyn_about_announceflag) -> QStatus);
    alljoyn_busobject_setannounceflag(core::mem::transmute(bus), core::mem::transmute(iface), core::mem::transmute(isannounced))
}
#[inline]
pub unsafe fn alljoyn_busobject_signal<P1>(bus: alljoyn_busobject, destination: P1, sessionid: u32, signal: alljoyn_interfacedescription_member, args: alljoyn_msgarg, numargs: usize, timetolive: u16, flags: u8, msg: alljoyn_message) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_busobject_signal(bus : alljoyn_busobject, destination : windows_core::PCSTR, sessionid : u32, signal : alljoyn_interfacedescription_member, args : alljoyn_msgarg, numargs : usize, timetolive : u16, flags : u8, msg : alljoyn_message) -> QStatus);
    alljoyn_busobject_signal(core::mem::transmute(bus), destination.param().abi(), core::mem::transmute(sessionid), core::mem::transmute(signal), core::mem::transmute(args), core::mem::transmute(numargs), core::mem::transmute(timetolive), core::mem::transmute(flags), core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_credentials_clear(cred: alljoyn_credentials) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_clear(cred : alljoyn_credentials));
    alljoyn_credentials_clear(core::mem::transmute(cred))
}
#[inline]
pub unsafe fn alljoyn_credentials_create() -> alljoyn_credentials {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_create() -> alljoyn_credentials);
    alljoyn_credentials_create()
}
#[inline]
pub unsafe fn alljoyn_credentials_destroy(cred: alljoyn_credentials) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_destroy(cred : alljoyn_credentials));
    alljoyn_credentials_destroy(core::mem::transmute(cred))
}
#[inline]
pub unsafe fn alljoyn_credentials_getcertchain(cred: alljoyn_credentials) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_getcertchain(cred : alljoyn_credentials) -> windows_core::PCSTR);
    alljoyn_credentials_getcertchain(core::mem::transmute(cred))
}
#[inline]
pub unsafe fn alljoyn_credentials_getexpiration(cred: alljoyn_credentials) -> u32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_getexpiration(cred : alljoyn_credentials) -> u32);
    alljoyn_credentials_getexpiration(core::mem::transmute(cred))
}
#[inline]
pub unsafe fn alljoyn_credentials_getlogonentry(cred: alljoyn_credentials) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_getlogonentry(cred : alljoyn_credentials) -> windows_core::PCSTR);
    alljoyn_credentials_getlogonentry(core::mem::transmute(cred))
}
#[inline]
pub unsafe fn alljoyn_credentials_getpassword(cred: alljoyn_credentials) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_getpassword(cred : alljoyn_credentials) -> windows_core::PCSTR);
    alljoyn_credentials_getpassword(core::mem::transmute(cred))
}
#[inline]
pub unsafe fn alljoyn_credentials_getprivateKey(cred: alljoyn_credentials) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_getprivateKey(cred : alljoyn_credentials) -> windows_core::PCSTR);
    alljoyn_credentials_getprivateKey(core::mem::transmute(cred))
}
#[inline]
pub unsafe fn alljoyn_credentials_getusername(cred: alljoyn_credentials) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_getusername(cred : alljoyn_credentials) -> windows_core::PCSTR);
    alljoyn_credentials_getusername(core::mem::transmute(cred))
}
#[inline]
pub unsafe fn alljoyn_credentials_isset(cred: alljoyn_credentials, creds: u16) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_isset(cred : alljoyn_credentials, creds : u16) -> i32);
    alljoyn_credentials_isset(core::mem::transmute(cred), core::mem::transmute(creds))
}
#[inline]
pub unsafe fn alljoyn_credentials_setcertchain<P1>(cred: alljoyn_credentials, certchain: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_setcertchain(cred : alljoyn_credentials, certchain : windows_core::PCSTR));
    alljoyn_credentials_setcertchain(core::mem::transmute(cred), certchain.param().abi())
}
#[inline]
pub unsafe fn alljoyn_credentials_setexpiration(cred: alljoyn_credentials, expiration: u32) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_setexpiration(cred : alljoyn_credentials, expiration : u32));
    alljoyn_credentials_setexpiration(core::mem::transmute(cred), core::mem::transmute(expiration))
}
#[inline]
pub unsafe fn alljoyn_credentials_setlogonentry<P1>(cred: alljoyn_credentials, logonentry: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_setlogonentry(cred : alljoyn_credentials, logonentry : windows_core::PCSTR));
    alljoyn_credentials_setlogonentry(core::mem::transmute(cred), logonentry.param().abi())
}
#[inline]
pub unsafe fn alljoyn_credentials_setpassword<P1>(cred: alljoyn_credentials, pwd: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_setpassword(cred : alljoyn_credentials, pwd : windows_core::PCSTR));
    alljoyn_credentials_setpassword(core::mem::transmute(cred), pwd.param().abi())
}
#[inline]
pub unsafe fn alljoyn_credentials_setprivatekey<P1>(cred: alljoyn_credentials, pk: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_setprivatekey(cred : alljoyn_credentials, pk : windows_core::PCSTR));
    alljoyn_credentials_setprivatekey(core::mem::transmute(cred), pk.param().abi())
}
#[inline]
pub unsafe fn alljoyn_credentials_setusername<P1>(cred: alljoyn_credentials, username: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_credentials_setusername(cred : alljoyn_credentials, username : windows_core::PCSTR));
    alljoyn_credentials_setusername(core::mem::transmute(cred), username.param().abi())
}
#[inline]
pub unsafe fn alljoyn_getbuildinfo() -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_getbuildinfo() -> windows_core::PCSTR);
    alljoyn_getbuildinfo()
}
#[inline]
pub unsafe fn alljoyn_getnumericversion() -> u32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_getnumericversion() -> u32);
    alljoyn_getnumericversion()
}
#[inline]
pub unsafe fn alljoyn_getversion() -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_getversion() -> windows_core::PCSTR);
    alljoyn_getversion()
}
#[inline]
pub unsafe fn alljoyn_init() -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_init() -> QStatus);
    alljoyn_init()
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_activate(iface: alljoyn_interfacedescription) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_activate(iface : alljoyn_interfacedescription));
    alljoyn_interfacedescription_activate(core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_addannotation<P1, P2>(iface: alljoyn_interfacedescription, name: P1, value: P2) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_addannotation(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, value : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_addannotation(core::mem::transmute(iface), name.param().abi(), value.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_addargannotation<P1, P2, P3, P4>(iface: alljoyn_interfacedescription, member: P1, argname: P2, name: P3, value: P4) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_addargannotation(iface : alljoyn_interfacedescription, member : windows_core::PCSTR, argname : windows_core::PCSTR, name : windows_core::PCSTR, value : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_addargannotation(core::mem::transmute(iface), member.param().abi(), argname.param().abi(), name.param().abi(), value.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_addmember<P2, P3, P4, P5>(iface: alljoyn_interfacedescription, r#type: alljoyn_messagetype, name: P2, inputsig: P3, outsig: P4, argnames: P5, annotation: u8) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_addmember(iface : alljoyn_interfacedescription, r#type : alljoyn_messagetype, name : windows_core::PCSTR, inputsig : windows_core::PCSTR, outsig : windows_core::PCSTR, argnames : windows_core::PCSTR, annotation : u8) -> QStatus);
    alljoyn_interfacedescription_addmember(core::mem::transmute(iface), core::mem::transmute(r#type), name.param().abi(), inputsig.param().abi(), outsig.param().abi(), argnames.param().abi(), core::mem::transmute(annotation))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_addmemberannotation<P1, P2, P3>(iface: alljoyn_interfacedescription, member: P1, name: P2, value: P3) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_addmemberannotation(iface : alljoyn_interfacedescription, member : windows_core::PCSTR, name : windows_core::PCSTR, value : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_addmemberannotation(core::mem::transmute(iface), member.param().abi(), name.param().abi(), value.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_addmethod<P1, P2, P3, P4, P6>(iface: alljoyn_interfacedescription, name: P1, inputsig: P2, outsig: P3, argnames: P4, annotation: u8, accessperms: P6) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_addmethod(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, inputsig : windows_core::PCSTR, outsig : windows_core::PCSTR, argnames : windows_core::PCSTR, annotation : u8, accessperms : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_addmethod(core::mem::transmute(iface), name.param().abi(), inputsig.param().abi(), outsig.param().abi(), argnames.param().abi(), core::mem::transmute(annotation), accessperms.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_addproperty<P1, P2>(iface: alljoyn_interfacedescription, name: P1, signature: P2, access: u8) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_addproperty(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, signature : windows_core::PCSTR, access : u8) -> QStatus);
    alljoyn_interfacedescription_addproperty(core::mem::transmute(iface), name.param().abi(), signature.param().abi(), core::mem::transmute(access))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_addpropertyannotation<P1, P2, P3>(iface: alljoyn_interfacedescription, property: P1, name: P2, value: P3) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_addpropertyannotation(iface : alljoyn_interfacedescription, property : windows_core::PCSTR, name : windows_core::PCSTR, value : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_addpropertyannotation(core::mem::transmute(iface), property.param().abi(), name.param().abi(), value.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_addsignal<P1, P2, P3, P5>(iface: alljoyn_interfacedescription, name: P1, sig: P2, argnames: P3, annotation: u8, accessperms: P5) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_addsignal(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, sig : windows_core::PCSTR, argnames : windows_core::PCSTR, annotation : u8, accessperms : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_addsignal(core::mem::transmute(iface), name.param().abi(), sig.param().abi(), argnames.param().abi(), core::mem::transmute(annotation), accessperms.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_eql(one: alljoyn_interfacedescription, other: alljoyn_interfacedescription) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_eql(one : alljoyn_interfacedescription, other : alljoyn_interfacedescription) -> i32);
    alljoyn_interfacedescription_eql(core::mem::transmute(one), core::mem::transmute(other))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getannotation<P1, P2>(iface: alljoyn_interfacedescription, name: P1, value: P2, value_size: *mut usize) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getannotation(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, value : windows_core::PCSTR, value_size : *mut usize) -> i32);
    alljoyn_interfacedescription_getannotation(core::mem::transmute(iface), name.param().abi(), value.param().abi(), core::mem::transmute(value_size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getannotationatindex<P2, P4>(iface: alljoyn_interfacedescription, index: usize, name: P2, name_size: *mut usize, value: P4, value_size: *mut usize)
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getannotationatindex(iface : alljoyn_interfacedescription, index : usize, name : windows_core::PCSTR, name_size : *mut usize, value : windows_core::PCSTR, value_size : *mut usize));
    alljoyn_interfacedescription_getannotationatindex(core::mem::transmute(iface), core::mem::transmute(index), name.param().abi(), core::mem::transmute(name_size), value.param().abi(), core::mem::transmute(value_size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getannotationscount(iface: alljoyn_interfacedescription) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getannotationscount(iface : alljoyn_interfacedescription) -> usize);
    alljoyn_interfacedescription_getannotationscount(core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getargdescriptionforlanguage<P1, P2, P3, P5>(iface: alljoyn_interfacedescription, member: P1, arg: P2, description: P3, maxlanguagelength: usize, languagetag: P5) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getargdescriptionforlanguage(iface : alljoyn_interfacedescription, member : windows_core::PCSTR, arg : windows_core::PCSTR, description : windows_core::PCSTR, maxlanguagelength : usize, languagetag : windows_core::PCSTR) -> usize);
    alljoyn_interfacedescription_getargdescriptionforlanguage(core::mem::transmute(iface), member.param().abi(), arg.param().abi(), description.param().abi(), core::mem::transmute(maxlanguagelength), languagetag.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getdescriptionforlanguage<P1, P3>(iface: alljoyn_interfacedescription, description: P1, maxlanguagelength: usize, languagetag: P3) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getdescriptionforlanguage(iface : alljoyn_interfacedescription, description : windows_core::PCSTR, maxlanguagelength : usize, languagetag : windows_core::PCSTR) -> usize);
    alljoyn_interfacedescription_getdescriptionforlanguage(core::mem::transmute(iface), description.param().abi(), core::mem::transmute(maxlanguagelength), languagetag.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getdescriptionlanguages(iface: alljoyn_interfacedescription, languages: *const *const i8, size: usize) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getdescriptionlanguages(iface : alljoyn_interfacedescription, languages : *const *const i8, size : usize) -> usize);
    alljoyn_interfacedescription_getdescriptionlanguages(core::mem::transmute(iface), core::mem::transmute(languages), core::mem::transmute(size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getdescriptionlanguages2<P1>(iface: alljoyn_interfacedescription, languages: P1, languagessize: usize) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getdescriptionlanguages2(iface : alljoyn_interfacedescription, languages : windows_core::PCSTR, languagessize : usize) -> usize);
    alljoyn_interfacedescription_getdescriptionlanguages2(core::mem::transmute(iface), languages.param().abi(), core::mem::transmute(languagessize))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getdescriptiontranslationcallback(iface: alljoyn_interfacedescription) -> alljoyn_interfacedescription_translation_callback_ptr {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getdescriptiontranslationcallback(iface : alljoyn_interfacedescription) -> alljoyn_interfacedescription_translation_callback_ptr);
    alljoyn_interfacedescription_getdescriptiontranslationcallback(core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmember<P1>(iface: alljoyn_interfacedescription, name: P1, member: *mut alljoyn_interfacedescription_member) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getmember(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, member : *mut alljoyn_interfacedescription_member) -> i32);
    alljoyn_interfacedescription_getmember(core::mem::transmute(iface), name.param().abi(), core::mem::transmute(member))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmemberannotation<P1, P2, P3>(iface: alljoyn_interfacedescription, member: P1, name: P2, value: P3, value_size: *mut usize) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getmemberannotation(iface : alljoyn_interfacedescription, member : windows_core::PCSTR, name : windows_core::PCSTR, value : windows_core::PCSTR, value_size : *mut usize) -> i32);
    alljoyn_interfacedescription_getmemberannotation(core::mem::transmute(iface), member.param().abi(), name.param().abi(), value.param().abi(), core::mem::transmute(value_size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmemberargannotation<P1, P2, P3, P4>(iface: alljoyn_interfacedescription, member: P1, argname: P2, name: P3, value: P4, value_size: *mut usize) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getmemberargannotation(iface : alljoyn_interfacedescription, member : windows_core::PCSTR, argname : windows_core::PCSTR, name : windows_core::PCSTR, value : windows_core::PCSTR, value_size : *mut usize) -> i32);
    alljoyn_interfacedescription_getmemberargannotation(core::mem::transmute(iface), member.param().abi(), argname.param().abi(), name.param().abi(), value.param().abi(), core::mem::transmute(value_size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmemberdescriptionforlanguage<P1, P2, P4>(iface: alljoyn_interfacedescription, member: P1, description: P2, maxlanguagelength: usize, languagetag: P4) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getmemberdescriptionforlanguage(iface : alljoyn_interfacedescription, member : windows_core::PCSTR, description : windows_core::PCSTR, maxlanguagelength : usize, languagetag : windows_core::PCSTR) -> usize);
    alljoyn_interfacedescription_getmemberdescriptionforlanguage(core::mem::transmute(iface), member.param().abi(), description.param().abi(), core::mem::transmute(maxlanguagelength), languagetag.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmembers(iface: alljoyn_interfacedescription, members: *mut alljoyn_interfacedescription_member, nummembers: usize) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getmembers(iface : alljoyn_interfacedescription, members : *mut alljoyn_interfacedescription_member, nummembers : usize) -> usize);
    alljoyn_interfacedescription_getmembers(core::mem::transmute(iface), core::mem::transmute(members), core::mem::transmute(nummembers))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getmethod<P1>(iface: alljoyn_interfacedescription, name: P1, member: *mut alljoyn_interfacedescription_member) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getmethod(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, member : *mut alljoyn_interfacedescription_member) -> i32);
    alljoyn_interfacedescription_getmethod(core::mem::transmute(iface), name.param().abi(), core::mem::transmute(member))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getname(iface: alljoyn_interfacedescription) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getname(iface : alljoyn_interfacedescription) -> windows_core::PCSTR);
    alljoyn_interfacedescription_getname(core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getproperties(iface: alljoyn_interfacedescription, props: *mut alljoyn_interfacedescription_property, numprops: usize) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getproperties(iface : alljoyn_interfacedescription, props : *mut alljoyn_interfacedescription_property, numprops : usize) -> usize);
    alljoyn_interfacedescription_getproperties(core::mem::transmute(iface), core::mem::transmute(props), core::mem::transmute(numprops))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getproperty<P1>(iface: alljoyn_interfacedescription, name: P1, property: *mut alljoyn_interfacedescription_property) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getproperty(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, property : *mut alljoyn_interfacedescription_property) -> i32);
    alljoyn_interfacedescription_getproperty(core::mem::transmute(iface), name.param().abi(), core::mem::transmute(property))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getpropertyannotation<P1, P2, P3>(iface: alljoyn_interfacedescription, property: P1, name: P2, value: P3, str_size: *mut usize) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getpropertyannotation(iface : alljoyn_interfacedescription, property : windows_core::PCSTR, name : windows_core::PCSTR, value : windows_core::PCSTR, str_size : *mut usize) -> i32);
    alljoyn_interfacedescription_getpropertyannotation(core::mem::transmute(iface), property.param().abi(), name.param().abi(), value.param().abi(), core::mem::transmute(str_size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getpropertydescriptionforlanguage<P1, P2, P4>(iface: alljoyn_interfacedescription, property: P1, description: P2, maxlanguagelength: usize, languagetag: P4) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getpropertydescriptionforlanguage(iface : alljoyn_interfacedescription, property : windows_core::PCSTR, description : windows_core::PCSTR, maxlanguagelength : usize, languagetag : windows_core::PCSTR) -> usize);
    alljoyn_interfacedescription_getpropertydescriptionforlanguage(core::mem::transmute(iface), property.param().abi(), description.param().abi(), core::mem::transmute(maxlanguagelength), languagetag.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getsecuritypolicy(iface: alljoyn_interfacedescription) -> alljoyn_interfacedescription_securitypolicy {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getsecuritypolicy(iface : alljoyn_interfacedescription) -> alljoyn_interfacedescription_securitypolicy);
    alljoyn_interfacedescription_getsecuritypolicy(core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_getsignal<P1>(iface: alljoyn_interfacedescription, name: P1, member: *mut alljoyn_interfacedescription_member) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_getsignal(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, member : *mut alljoyn_interfacedescription_member) -> i32);
    alljoyn_interfacedescription_getsignal(core::mem::transmute(iface), name.param().abi(), core::mem::transmute(member))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_hasdescription(iface: alljoyn_interfacedescription) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_hasdescription(iface : alljoyn_interfacedescription) -> i32);
    alljoyn_interfacedescription_hasdescription(core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_hasmember<P1, P2, P3>(iface: alljoyn_interfacedescription, name: P1, insig: P2, outsig: P3) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_hasmember(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, insig : windows_core::PCSTR, outsig : windows_core::PCSTR) -> i32);
    alljoyn_interfacedescription_hasmember(core::mem::transmute(iface), name.param().abi(), insig.param().abi(), outsig.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_hasproperties(iface: alljoyn_interfacedescription) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_hasproperties(iface : alljoyn_interfacedescription) -> i32);
    alljoyn_interfacedescription_hasproperties(core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_hasproperty<P1>(iface: alljoyn_interfacedescription, name: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_hasproperty(iface : alljoyn_interfacedescription, name : windows_core::PCSTR) -> i32);
    alljoyn_interfacedescription_hasproperty(core::mem::transmute(iface), name.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_introspect<P1>(iface: alljoyn_interfacedescription, str: P1, buf: usize, indent: usize) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_introspect(iface : alljoyn_interfacedescription, str : windows_core::PCSTR, buf : usize, indent : usize) -> usize);
    alljoyn_interfacedescription_introspect(core::mem::transmute(iface), str.param().abi(), core::mem::transmute(buf), core::mem::transmute(indent))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_issecure(iface: alljoyn_interfacedescription) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_issecure(iface : alljoyn_interfacedescription) -> i32);
    alljoyn_interfacedescription_issecure(core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_eql(one: alljoyn_interfacedescription_member, other: alljoyn_interfacedescription_member) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_member_eql(one : alljoyn_interfacedescription_member, other : alljoyn_interfacedescription_member) -> i32);
    alljoyn_interfacedescription_member_eql(core::mem::transmute(one), core::mem::transmute(other))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getannotation<P1, P2>(member: alljoyn_interfacedescription_member, name: P1, value: P2, value_size: *mut usize) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_member_getannotation(member : alljoyn_interfacedescription_member, name : windows_core::PCSTR, value : windows_core::PCSTR, value_size : *mut usize) -> i32);
    alljoyn_interfacedescription_member_getannotation(core::mem::transmute(member), name.param().abi(), value.param().abi(), core::mem::transmute(value_size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getannotationatindex<P2, P4>(member: alljoyn_interfacedescription_member, index: usize, name: P2, name_size: *mut usize, value: P4, value_size: *mut usize)
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_member_getannotationatindex(member : alljoyn_interfacedescription_member, index : usize, name : windows_core::PCSTR, name_size : *mut usize, value : windows_core::PCSTR, value_size : *mut usize));
    alljoyn_interfacedescription_member_getannotationatindex(core::mem::transmute(member), core::mem::transmute(index), name.param().abi(), core::mem::transmute(name_size), value.param().abi(), core::mem::transmute(value_size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getannotationscount(member: alljoyn_interfacedescription_member) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_member_getannotationscount(member : alljoyn_interfacedescription_member) -> usize);
    alljoyn_interfacedescription_member_getannotationscount(core::mem::transmute(member))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getargannotation<P1, P2, P3>(member: alljoyn_interfacedescription_member, argname: P1, name: P2, value: P3, value_size: *mut usize) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_member_getargannotation(member : alljoyn_interfacedescription_member, argname : windows_core::PCSTR, name : windows_core::PCSTR, value : windows_core::PCSTR, value_size : *mut usize) -> i32);
    alljoyn_interfacedescription_member_getargannotation(core::mem::transmute(member), argname.param().abi(), name.param().abi(), value.param().abi(), core::mem::transmute(value_size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getargannotationatindex<P1, P3, P5>(member: alljoyn_interfacedescription_member, argname: P1, index: usize, name: P3, name_size: *mut usize, value: P5, value_size: *mut usize)
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_member_getargannotationatindex(member : alljoyn_interfacedescription_member, argname : windows_core::PCSTR, index : usize, name : windows_core::PCSTR, name_size : *mut usize, value : windows_core::PCSTR, value_size : *mut usize));
    alljoyn_interfacedescription_member_getargannotationatindex(core::mem::transmute(member), argname.param().abi(), core::mem::transmute(index), name.param().abi(), core::mem::transmute(name_size), value.param().abi(), core::mem::transmute(value_size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_member_getargannotationscount<P1>(member: alljoyn_interfacedescription_member, argname: P1) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_member_getargannotationscount(member : alljoyn_interfacedescription_member, argname : windows_core::PCSTR) -> usize);
    alljoyn_interfacedescription_member_getargannotationscount(core::mem::transmute(member), argname.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_property_eql(one: alljoyn_interfacedescription_property, other: alljoyn_interfacedescription_property) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_property_eql(one : alljoyn_interfacedescription_property, other : alljoyn_interfacedescription_property) -> i32);
    alljoyn_interfacedescription_property_eql(core::mem::transmute(one), core::mem::transmute(other))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_property_getannotation<P1, P2>(property: alljoyn_interfacedescription_property, name: P1, value: P2, value_size: *mut usize) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_property_getannotation(property : alljoyn_interfacedescription_property, name : windows_core::PCSTR, value : windows_core::PCSTR, value_size : *mut usize) -> i32);
    alljoyn_interfacedescription_property_getannotation(core::mem::transmute(property), name.param().abi(), value.param().abi(), core::mem::transmute(value_size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_property_getannotationatindex<P2, P4>(property: alljoyn_interfacedescription_property, index: usize, name: P2, name_size: *mut usize, value: P4, value_size: *mut usize)
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_property_getannotationatindex(property : alljoyn_interfacedescription_property, index : usize, name : windows_core::PCSTR, name_size : *mut usize, value : windows_core::PCSTR, value_size : *mut usize));
    alljoyn_interfacedescription_property_getannotationatindex(core::mem::transmute(property), core::mem::transmute(index), name.param().abi(), core::mem::transmute(name_size), value.param().abi(), core::mem::transmute(value_size))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_property_getannotationscount(property: alljoyn_interfacedescription_property) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_property_getannotationscount(property : alljoyn_interfacedescription_property) -> usize);
    alljoyn_interfacedescription_property_getannotationscount(core::mem::transmute(property))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_setargdescription<P1, P2, P3>(iface: alljoyn_interfacedescription, member: P1, argname: P2, description: P3) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_setargdescription(iface : alljoyn_interfacedescription, member : windows_core::PCSTR, argname : windows_core::PCSTR, description : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_setargdescription(core::mem::transmute(iface), member.param().abi(), argname.param().abi(), description.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_setargdescriptionforlanguage<P1, P2, P3, P4>(iface: alljoyn_interfacedescription, member: P1, arg: P2, description: P3, languagetag: P4) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_setargdescriptionforlanguage(iface : alljoyn_interfacedescription, member : windows_core::PCSTR, arg : windows_core::PCSTR, description : windows_core::PCSTR, languagetag : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_setargdescriptionforlanguage(core::mem::transmute(iface), member.param().abi(), arg.param().abi(), description.param().abi(), languagetag.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_setdescription<P1>(iface: alljoyn_interfacedescription, description: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_setdescription(iface : alljoyn_interfacedescription, description : windows_core::PCSTR));
    alljoyn_interfacedescription_setdescription(core::mem::transmute(iface), description.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_setdescriptionforlanguage<P1, P2>(iface: alljoyn_interfacedescription, description: P1, languagetag: P2) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_setdescriptionforlanguage(iface : alljoyn_interfacedescription, description : windows_core::PCSTR, languagetag : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_setdescriptionforlanguage(core::mem::transmute(iface), description.param().abi(), languagetag.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_setdescriptionlanguage<P1>(iface: alljoyn_interfacedescription, language: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_setdescriptionlanguage(iface : alljoyn_interfacedescription, language : windows_core::PCSTR));
    alljoyn_interfacedescription_setdescriptionlanguage(core::mem::transmute(iface), language.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_setdescriptiontranslationcallback(iface: alljoyn_interfacedescription, translationcallback: alljoyn_interfacedescription_translation_callback_ptr) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_setdescriptiontranslationcallback(iface : alljoyn_interfacedescription, translationcallback : alljoyn_interfacedescription_translation_callback_ptr));
    alljoyn_interfacedescription_setdescriptiontranslationcallback(core::mem::transmute(iface), core::mem::transmute(translationcallback))
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_setmemberdescription<P1, P2>(iface: alljoyn_interfacedescription, member: P1, description: P2) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_setmemberdescription(iface : alljoyn_interfacedescription, member : windows_core::PCSTR, description : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_setmemberdescription(core::mem::transmute(iface), member.param().abi(), description.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_setmemberdescriptionforlanguage<P1, P2, P3>(iface: alljoyn_interfacedescription, member: P1, description: P2, languagetag: P3) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_setmemberdescriptionforlanguage(iface : alljoyn_interfacedescription, member : windows_core::PCSTR, description : windows_core::PCSTR, languagetag : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_setmemberdescriptionforlanguage(core::mem::transmute(iface), member.param().abi(), description.param().abi(), languagetag.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_setpropertydescription<P1, P2>(iface: alljoyn_interfacedescription, name: P1, description: P2) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_setpropertydescription(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, description : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_setpropertydescription(core::mem::transmute(iface), name.param().abi(), description.param().abi())
}
#[inline]
pub unsafe fn alljoyn_interfacedescription_setpropertydescriptionforlanguage<P1, P2, P3>(iface: alljoyn_interfacedescription, name: P1, description: P2, languagetag: P3) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_interfacedescription_setpropertydescriptionforlanguage(iface : alljoyn_interfacedescription, name : windows_core::PCSTR, description : windows_core::PCSTR, languagetag : windows_core::PCSTR) -> QStatus);
    alljoyn_interfacedescription_setpropertydescriptionforlanguage(core::mem::transmute(iface), name.param().abi(), description.param().abi(), languagetag.param().abi())
}
#[inline]
pub unsafe fn alljoyn_keystorelistener_create(callbacks: *const alljoyn_keystorelistener_callbacks, context: *const core::ffi::c_void) -> alljoyn_keystorelistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_keystorelistener_create(callbacks : *const alljoyn_keystorelistener_callbacks, context : *const core::ffi::c_void) -> alljoyn_keystorelistener);
    alljoyn_keystorelistener_create(core::mem::transmute(callbacks), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_keystorelistener_destroy(listener: alljoyn_keystorelistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_keystorelistener_destroy(listener : alljoyn_keystorelistener));
    alljoyn_keystorelistener_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_keystorelistener_getkeys<P2>(listener: alljoyn_keystorelistener, keystore: alljoyn_keystore, sink: P2, sink_sz: *mut usize) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_keystorelistener_getkeys(listener : alljoyn_keystorelistener, keystore : alljoyn_keystore, sink : windows_core::PCSTR, sink_sz : *mut usize) -> QStatus);
    alljoyn_keystorelistener_getkeys(core::mem::transmute(listener), core::mem::transmute(keystore), sink.param().abi(), core::mem::transmute(sink_sz))
}
#[inline]
pub unsafe fn alljoyn_keystorelistener_putkeys<P2, P3>(listener: alljoyn_keystorelistener, keystore: alljoyn_keystore, source: P2, password: P3) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_keystorelistener_putkeys(listener : alljoyn_keystorelistener, keystore : alljoyn_keystore, source : windows_core::PCSTR, password : windows_core::PCSTR) -> QStatus);
    alljoyn_keystorelistener_putkeys(core::mem::transmute(listener), core::mem::transmute(keystore), source.param().abi(), password.param().abi())
}
#[inline]
pub unsafe fn alljoyn_keystorelistener_with_synchronization_create(callbacks: *const alljoyn_keystorelistener_with_synchronization_callbacks, context: *mut core::ffi::c_void) -> alljoyn_keystorelistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_keystorelistener_with_synchronization_create(callbacks : *const alljoyn_keystorelistener_with_synchronization_callbacks, context : *mut core::ffi::c_void) -> alljoyn_keystorelistener);
    alljoyn_keystorelistener_with_synchronization_create(core::mem::transmute(callbacks), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_message_create(bus: alljoyn_busattachment) -> alljoyn_message {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_create(bus : alljoyn_busattachment) -> alljoyn_message);
    alljoyn_message_create(core::mem::transmute(bus))
}
#[inline]
pub unsafe fn alljoyn_message_description<P1>(msg: alljoyn_message, str: P1, buf: usize) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_description(msg : alljoyn_message, str : windows_core::PCSTR, buf : usize) -> usize);
    alljoyn_message_description(core::mem::transmute(msg), str.param().abi(), core::mem::transmute(buf))
}
#[inline]
pub unsafe fn alljoyn_message_destroy(msg: alljoyn_message) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_destroy(msg : alljoyn_message));
    alljoyn_message_destroy(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_eql(one: alljoyn_message, other: alljoyn_message) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_eql(one : alljoyn_message, other : alljoyn_message) -> i32);
    alljoyn_message_eql(core::mem::transmute(one), core::mem::transmute(other))
}
#[inline]
pub unsafe fn alljoyn_message_getarg(msg: alljoyn_message, argn: usize) -> alljoyn_msgarg {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getarg(msg : alljoyn_message, argn : usize) -> alljoyn_msgarg);
    alljoyn_message_getarg(core::mem::transmute(msg), core::mem::transmute(argn))
}
#[inline]
pub unsafe fn alljoyn_message_getargs(msg: alljoyn_message, numargs: *mut usize, args: *mut alljoyn_msgarg) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getargs(msg : alljoyn_message, numargs : *mut usize, args : *mut alljoyn_msgarg));
    alljoyn_message_getargs(core::mem::transmute(msg), core::mem::transmute(numargs), core::mem::transmute(args))
}
#[inline]
pub unsafe fn alljoyn_message_getauthmechanism(msg: alljoyn_message) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getauthmechanism(msg : alljoyn_message) -> windows_core::PCSTR);
    alljoyn_message_getauthmechanism(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_getcallserial(msg: alljoyn_message) -> u32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getcallserial(msg : alljoyn_message) -> u32);
    alljoyn_message_getcallserial(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_getcompressiontoken(msg: alljoyn_message) -> u32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getcompressiontoken(msg : alljoyn_message) -> u32);
    alljoyn_message_getcompressiontoken(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_getdestination(msg: alljoyn_message) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getdestination(msg : alljoyn_message) -> windows_core::PCSTR);
    alljoyn_message_getdestination(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_geterrorname<P1>(msg: alljoyn_message, errormessage: P1, errormessage_size: *mut usize) -> windows_core::PCSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_geterrorname(msg : alljoyn_message, errormessage : windows_core::PCSTR, errormessage_size : *mut usize) -> windows_core::PCSTR);
    alljoyn_message_geterrorname(core::mem::transmute(msg), errormessage.param().abi(), core::mem::transmute(errormessage_size))
}
#[inline]
pub unsafe fn alljoyn_message_getflags(msg: alljoyn_message) -> u8 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getflags(msg : alljoyn_message) -> u8);
    alljoyn_message_getflags(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_getinterface(msg: alljoyn_message) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getinterface(msg : alljoyn_message) -> windows_core::PCSTR);
    alljoyn_message_getinterface(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_getmembername(msg: alljoyn_message) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getmembername(msg : alljoyn_message) -> windows_core::PCSTR);
    alljoyn_message_getmembername(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_getobjectpath(msg: alljoyn_message) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getobjectpath(msg : alljoyn_message) -> windows_core::PCSTR);
    alljoyn_message_getobjectpath(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_getreceiveendpointname(msg: alljoyn_message) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getreceiveendpointname(msg : alljoyn_message) -> windows_core::PCSTR);
    alljoyn_message_getreceiveendpointname(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_getreplyserial(msg: alljoyn_message) -> u32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getreplyserial(msg : alljoyn_message) -> u32);
    alljoyn_message_getreplyserial(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_getsender(msg: alljoyn_message) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getsender(msg : alljoyn_message) -> windows_core::PCSTR);
    alljoyn_message_getsender(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_getsessionid(msg: alljoyn_message) -> u32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getsessionid(msg : alljoyn_message) -> u32);
    alljoyn_message_getsessionid(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_getsignature(msg: alljoyn_message) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_getsignature(msg : alljoyn_message) -> windows_core::PCSTR);
    alljoyn_message_getsignature(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_gettimestamp(msg: alljoyn_message) -> u32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_gettimestamp(msg : alljoyn_message) -> u32);
    alljoyn_message_gettimestamp(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_gettype(msg: alljoyn_message) -> alljoyn_messagetype {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_gettype(msg : alljoyn_message) -> alljoyn_messagetype);
    alljoyn_message_gettype(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_isbroadcastsignal(msg: alljoyn_message) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_isbroadcastsignal(msg : alljoyn_message) -> i32);
    alljoyn_message_isbroadcastsignal(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_isencrypted(msg: alljoyn_message) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_isencrypted(msg : alljoyn_message) -> i32);
    alljoyn_message_isencrypted(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_isexpired(msg: alljoyn_message, tillexpirems: *mut u32) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_isexpired(msg : alljoyn_message, tillexpirems : *mut u32) -> i32);
    alljoyn_message_isexpired(core::mem::transmute(msg), core::mem::transmute(tillexpirems))
}
#[inline]
pub unsafe fn alljoyn_message_isglobalbroadcast(msg: alljoyn_message) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_isglobalbroadcast(msg : alljoyn_message) -> i32);
    alljoyn_message_isglobalbroadcast(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_issessionless(msg: alljoyn_message) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_issessionless(msg : alljoyn_message) -> i32);
    alljoyn_message_issessionless(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_isunreliable(msg: alljoyn_message) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_isunreliable(msg : alljoyn_message) -> i32);
    alljoyn_message_isunreliable(core::mem::transmute(msg))
}
#[inline]
pub unsafe fn alljoyn_message_parseargs<P1>(msg: alljoyn_message, signature: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "cdecl" fn alljoyn_message_parseargs(msg : alljoyn_message, signature : windows_core::PCSTR) -> QStatus);
    alljoyn_message_parseargs(core::mem::transmute(msg), signature.param().abi())
}
#[inline]
pub unsafe fn alljoyn_message_setendianess(endian: i8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_setendianess(endian : i8));
    alljoyn_message_setendianess(core::mem::transmute(endian))
}
#[inline]
pub unsafe fn alljoyn_message_tostring<P1>(msg: alljoyn_message, str: P1, buf: usize) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_message_tostring(msg : alljoyn_message, str : windows_core::PCSTR, buf : usize) -> usize);
    alljoyn_message_tostring(core::mem::transmute(msg), str.param().abi(), core::mem::transmute(buf))
}
#[inline]
pub unsafe fn alljoyn_msgarg_array_create(size: usize) -> alljoyn_msgarg {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_array_create(size : usize) -> alljoyn_msgarg);
    alljoyn_msgarg_array_create(core::mem::transmute(size))
}
#[inline]
pub unsafe fn alljoyn_msgarg_array_element(arg: alljoyn_msgarg, index: usize) -> alljoyn_msgarg {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_array_element(arg : alljoyn_msgarg, index : usize) -> alljoyn_msgarg);
    alljoyn_msgarg_array_element(core::mem::transmute(arg), core::mem::transmute(index))
}
#[inline]
pub unsafe fn alljoyn_msgarg_array_get<P2>(args: alljoyn_msgarg, numargs: usize, signature: P2) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "cdecl" fn alljoyn_msgarg_array_get(args : alljoyn_msgarg, numargs : usize, signature : windows_core::PCSTR) -> QStatus);
    alljoyn_msgarg_array_get(core::mem::transmute(args), core::mem::transmute(numargs), signature.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_array_set<P2>(args: alljoyn_msgarg, numargs: *mut usize, signature: P2) -> QStatus
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "cdecl" fn alljoyn_msgarg_array_set(args : alljoyn_msgarg, numargs : *mut usize, signature : windows_core::PCSTR) -> QStatus);
    alljoyn_msgarg_array_set(core::mem::transmute(args), core::mem::transmute(numargs), signature.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_array_set_offset<P3>(args: alljoyn_msgarg, argoffset: usize, numargs: *mut usize, signature: P3) -> QStatus
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "cdecl" fn alljoyn_msgarg_array_set_offset(args : alljoyn_msgarg, argoffset : usize, numargs : *mut usize, signature : windows_core::PCSTR) -> QStatus);
    alljoyn_msgarg_array_set_offset(core::mem::transmute(args), core::mem::transmute(argoffset), core::mem::transmute(numargs), signature.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_array_signature<P2>(values: alljoyn_msgarg, numvalues: usize, str: P2, buf: usize) -> usize
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_array_signature(values : alljoyn_msgarg, numvalues : usize, str : windows_core::PCSTR, buf : usize) -> usize);
    alljoyn_msgarg_array_signature(core::mem::transmute(values), core::mem::transmute(numvalues), str.param().abi(), core::mem::transmute(buf))
}
#[inline]
pub unsafe fn alljoyn_msgarg_array_tostring<P2>(args: alljoyn_msgarg, numargs: usize, str: P2, buf: usize, indent: usize) -> usize
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_array_tostring(args : alljoyn_msgarg, numargs : usize, str : windows_core::PCSTR, buf : usize, indent : usize) -> usize);
    alljoyn_msgarg_array_tostring(core::mem::transmute(args), core::mem::transmute(numargs), str.param().abi(), core::mem::transmute(buf), core::mem::transmute(indent))
}
#[inline]
pub unsafe fn alljoyn_msgarg_clear(arg: alljoyn_msgarg) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_clear(arg : alljoyn_msgarg));
    alljoyn_msgarg_clear(core::mem::transmute(arg))
}
#[inline]
pub unsafe fn alljoyn_msgarg_clone(destination: alljoyn_msgarg, source: alljoyn_msgarg) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_clone(destination : alljoyn_msgarg, source : alljoyn_msgarg));
    alljoyn_msgarg_clone(core::mem::transmute(destination), core::mem::transmute(source))
}
#[inline]
pub unsafe fn alljoyn_msgarg_copy(source: alljoyn_msgarg) -> alljoyn_msgarg {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_copy(source : alljoyn_msgarg) -> alljoyn_msgarg);
    alljoyn_msgarg_copy(core::mem::transmute(source))
}
#[inline]
pub unsafe fn alljoyn_msgarg_create() -> alljoyn_msgarg {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_create() -> alljoyn_msgarg);
    alljoyn_msgarg_create()
}
#[inline]
pub unsafe fn alljoyn_msgarg_create_and_set<P0>(signature: P0) -> alljoyn_msgarg
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "cdecl" fn alljoyn_msgarg_create_and_set(signature : windows_core::PCSTR) -> alljoyn_msgarg);
    alljoyn_msgarg_create_and_set(signature.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_destroy(arg: alljoyn_msgarg) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_destroy(arg : alljoyn_msgarg));
    alljoyn_msgarg_destroy(core::mem::transmute(arg))
}
#[inline]
pub unsafe fn alljoyn_msgarg_equal(lhv: alljoyn_msgarg, rhv: alljoyn_msgarg) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_equal(lhv : alljoyn_msgarg, rhv : alljoyn_msgarg) -> i32);
    alljoyn_msgarg_equal(core::mem::transmute(lhv), core::mem::transmute(rhv))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get<P1>(arg: alljoyn_msgarg, signature: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "cdecl" fn alljoyn_msgarg_get(arg : alljoyn_msgarg, signature : windows_core::PCSTR) -> QStatus);
    alljoyn_msgarg_get(core::mem::transmute(arg), signature.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_array_element(arg: alljoyn_msgarg, index: usize, element: *mut alljoyn_msgarg) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_array_element(arg : alljoyn_msgarg, index : usize, element : *mut alljoyn_msgarg));
    alljoyn_msgarg_get_array_element(core::mem::transmute(arg), core::mem::transmute(index), core::mem::transmute(element))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_array_elementsignature(arg: alljoyn_msgarg, index: usize) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_array_elementsignature(arg : alljoyn_msgarg, index : usize) -> windows_core::PCSTR);
    alljoyn_msgarg_get_array_elementsignature(core::mem::transmute(arg), core::mem::transmute(index))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_array_numberofelements(arg: alljoyn_msgarg) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_array_numberofelements(arg : alljoyn_msgarg) -> usize);
    alljoyn_msgarg_get_array_numberofelements(core::mem::transmute(arg))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_bool(arg: alljoyn_msgarg, b: *mut i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_bool(arg : alljoyn_msgarg, b : *mut i32) -> QStatus);
    alljoyn_msgarg_get_bool(core::mem::transmute(arg), core::mem::transmute(b))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_bool_array(arg: alljoyn_msgarg, length: *mut usize, ab: *mut i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_bool_array(arg : alljoyn_msgarg, length : *mut usize, ab : *mut i32) -> QStatus);
    alljoyn_msgarg_get_bool_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ab))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_double(arg: alljoyn_msgarg, d: *mut f64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_double(arg : alljoyn_msgarg, d : *mut f64) -> QStatus);
    alljoyn_msgarg_get_double(core::mem::transmute(arg), core::mem::transmute(d))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_double_array(arg: alljoyn_msgarg, length: *mut usize, ad: *mut f64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_double_array(arg : alljoyn_msgarg, length : *mut usize, ad : *mut f64) -> QStatus);
    alljoyn_msgarg_get_double_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ad))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_int16(arg: alljoyn_msgarg, n: *mut i16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_int16(arg : alljoyn_msgarg, n : *mut i16) -> QStatus);
    alljoyn_msgarg_get_int16(core::mem::transmute(arg), core::mem::transmute(n))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_int16_array(arg: alljoyn_msgarg, length: *mut usize, an: *mut i16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_int16_array(arg : alljoyn_msgarg, length : *mut usize, an : *mut i16) -> QStatus);
    alljoyn_msgarg_get_int16_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(an))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_int32(arg: alljoyn_msgarg, i: *mut i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_int32(arg : alljoyn_msgarg, i : *mut i32) -> QStatus);
    alljoyn_msgarg_get_int32(core::mem::transmute(arg), core::mem::transmute(i))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_int32_array(arg: alljoyn_msgarg, length: *mut usize, ai: *mut i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_int32_array(arg : alljoyn_msgarg, length : *mut usize, ai : *mut i32) -> QStatus);
    alljoyn_msgarg_get_int32_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ai))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_int64(arg: alljoyn_msgarg, x: *mut i64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_int64(arg : alljoyn_msgarg, x : *mut i64) -> QStatus);
    alljoyn_msgarg_get_int64(core::mem::transmute(arg), core::mem::transmute(x))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_int64_array(arg: alljoyn_msgarg, length: *mut usize, ax: *mut i64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_int64_array(arg : alljoyn_msgarg, length : *mut usize, ax : *mut i64) -> QStatus);
    alljoyn_msgarg_get_int64_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ax))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_objectpath(arg: alljoyn_msgarg, o: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_objectpath(arg : alljoyn_msgarg, o : *mut *mut i8) -> QStatus);
    alljoyn_msgarg_get_objectpath(core::mem::transmute(arg), core::mem::transmute(o))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_signature(arg: alljoyn_msgarg, g: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_signature(arg : alljoyn_msgarg, g : *mut *mut i8) -> QStatus);
    alljoyn_msgarg_get_signature(core::mem::transmute(arg), core::mem::transmute(g))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_string(arg: alljoyn_msgarg, s: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_string(arg : alljoyn_msgarg, s : *mut *mut i8) -> QStatus);
    alljoyn_msgarg_get_string(core::mem::transmute(arg), core::mem::transmute(s))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint16(arg: alljoyn_msgarg, q: *mut u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_uint16(arg : alljoyn_msgarg, q : *mut u16) -> QStatus);
    alljoyn_msgarg_get_uint16(core::mem::transmute(arg), core::mem::transmute(q))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint16_array(arg: alljoyn_msgarg, length: *mut usize, aq: *mut u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_uint16_array(arg : alljoyn_msgarg, length : *mut usize, aq : *mut u16) -> QStatus);
    alljoyn_msgarg_get_uint16_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(aq))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint32(arg: alljoyn_msgarg, u: *mut u32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_uint32(arg : alljoyn_msgarg, u : *mut u32) -> QStatus);
    alljoyn_msgarg_get_uint32(core::mem::transmute(arg), core::mem::transmute(u))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint32_array(arg: alljoyn_msgarg, length: *mut usize, au: *mut u32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_uint32_array(arg : alljoyn_msgarg, length : *mut usize, au : *mut u32) -> QStatus);
    alljoyn_msgarg_get_uint32_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(au))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint64(arg: alljoyn_msgarg, t: *mut u64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_uint64(arg : alljoyn_msgarg, t : *mut u64) -> QStatus);
    alljoyn_msgarg_get_uint64(core::mem::transmute(arg), core::mem::transmute(t))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint64_array(arg: alljoyn_msgarg, length: *mut usize, at: *mut u64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_uint64_array(arg : alljoyn_msgarg, length : *mut usize, at : *mut u64) -> QStatus);
    alljoyn_msgarg_get_uint64_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(at))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint8(arg: alljoyn_msgarg, y: *mut u8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_uint8(arg : alljoyn_msgarg, y : *mut u8) -> QStatus);
    alljoyn_msgarg_get_uint8(core::mem::transmute(arg), core::mem::transmute(y))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_uint8_array(arg: alljoyn_msgarg, length: *mut usize, ay: *mut u8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_uint8_array(arg : alljoyn_msgarg, length : *mut usize, ay : *mut u8) -> QStatus);
    alljoyn_msgarg_get_uint8_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ay))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_variant(arg: alljoyn_msgarg, v: alljoyn_msgarg) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_variant(arg : alljoyn_msgarg, v : alljoyn_msgarg) -> QStatus);
    alljoyn_msgarg_get_variant(core::mem::transmute(arg), core::mem::transmute(v))
}
#[inline]
pub unsafe fn alljoyn_msgarg_get_variant_array<P1>(arg: alljoyn_msgarg, signature: P1, length: *mut usize, av: *mut alljoyn_msgarg) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_get_variant_array(arg : alljoyn_msgarg, signature : windows_core::PCSTR, length : *mut usize, av : *mut alljoyn_msgarg) -> QStatus);
    alljoyn_msgarg_get_variant_array(core::mem::transmute(arg), signature.param().abi(), core::mem::transmute(length), core::mem::transmute(av))
}
#[inline]
pub unsafe fn alljoyn_msgarg_getdictelement<P1>(arg: alljoyn_msgarg, elemsig: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "cdecl" fn alljoyn_msgarg_getdictelement(arg : alljoyn_msgarg, elemsig : windows_core::PCSTR) -> QStatus);
    alljoyn_msgarg_getdictelement(core::mem::transmute(arg), elemsig.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_getkey(arg: alljoyn_msgarg) -> alljoyn_msgarg {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_getkey(arg : alljoyn_msgarg) -> alljoyn_msgarg);
    alljoyn_msgarg_getkey(core::mem::transmute(arg))
}
#[inline]
pub unsafe fn alljoyn_msgarg_getmember(arg: alljoyn_msgarg, index: usize) -> alljoyn_msgarg {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_getmember(arg : alljoyn_msgarg, index : usize) -> alljoyn_msgarg);
    alljoyn_msgarg_getmember(core::mem::transmute(arg), core::mem::transmute(index))
}
#[inline]
pub unsafe fn alljoyn_msgarg_getnummembers(arg: alljoyn_msgarg) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_getnummembers(arg : alljoyn_msgarg) -> usize);
    alljoyn_msgarg_getnummembers(core::mem::transmute(arg))
}
#[inline]
pub unsafe fn alljoyn_msgarg_gettype(arg: alljoyn_msgarg) -> alljoyn_typeid {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_gettype(arg : alljoyn_msgarg) -> alljoyn_typeid);
    alljoyn_msgarg_gettype(core::mem::transmute(arg))
}
#[inline]
pub unsafe fn alljoyn_msgarg_getvalue(arg: alljoyn_msgarg) -> alljoyn_msgarg {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_getvalue(arg : alljoyn_msgarg) -> alljoyn_msgarg);
    alljoyn_msgarg_getvalue(core::mem::transmute(arg))
}
#[inline]
pub unsafe fn alljoyn_msgarg_hassignature<P1>(arg: alljoyn_msgarg, signature: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_hassignature(arg : alljoyn_msgarg, signature : windows_core::PCSTR) -> i32);
    alljoyn_msgarg_hassignature(core::mem::transmute(arg), signature.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_set<P1>(arg: alljoyn_msgarg, signature: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "cdecl" fn alljoyn_msgarg_set(arg : alljoyn_msgarg, signature : windows_core::PCSTR) -> QStatus);
    alljoyn_msgarg_set(core::mem::transmute(arg), signature.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_and_stabilize<P1>(arg: alljoyn_msgarg, signature: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "cdecl" fn alljoyn_msgarg_set_and_stabilize(arg : alljoyn_msgarg, signature : windows_core::PCSTR) -> QStatus);
    alljoyn_msgarg_set_and_stabilize(core::mem::transmute(arg), signature.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_bool(arg: alljoyn_msgarg, b: i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_bool(arg : alljoyn_msgarg, b : i32) -> QStatus);
    alljoyn_msgarg_set_bool(core::mem::transmute(arg), core::mem::transmute(b))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_bool_array(arg: alljoyn_msgarg, length: usize, ab: *mut i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_bool_array(arg : alljoyn_msgarg, length : usize, ab : *mut i32) -> QStatus);
    alljoyn_msgarg_set_bool_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ab))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_double(arg: alljoyn_msgarg, d: f64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_double(arg : alljoyn_msgarg, d : f64) -> QStatus);
    alljoyn_msgarg_set_double(core::mem::transmute(arg), core::mem::transmute(d))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_double_array(arg: alljoyn_msgarg, length: usize, ad: *mut f64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_double_array(arg : alljoyn_msgarg, length : usize, ad : *mut f64) -> QStatus);
    alljoyn_msgarg_set_double_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ad))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_int16(arg: alljoyn_msgarg, n: i16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_int16(arg : alljoyn_msgarg, n : i16) -> QStatus);
    alljoyn_msgarg_set_int16(core::mem::transmute(arg), core::mem::transmute(n))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_int16_array(arg: alljoyn_msgarg, length: usize, an: *mut i16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_int16_array(arg : alljoyn_msgarg, length : usize, an : *mut i16) -> QStatus);
    alljoyn_msgarg_set_int16_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(an))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_int32(arg: alljoyn_msgarg, i: i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_int32(arg : alljoyn_msgarg, i : i32) -> QStatus);
    alljoyn_msgarg_set_int32(core::mem::transmute(arg), core::mem::transmute(i))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_int32_array(arg: alljoyn_msgarg, length: usize, ai: *mut i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_int32_array(arg : alljoyn_msgarg, length : usize, ai : *mut i32) -> QStatus);
    alljoyn_msgarg_set_int32_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ai))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_int64(arg: alljoyn_msgarg, x: i64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_int64(arg : alljoyn_msgarg, x : i64) -> QStatus);
    alljoyn_msgarg_set_int64(core::mem::transmute(arg), core::mem::transmute(x))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_int64_array(arg: alljoyn_msgarg, length: usize, ax: *mut i64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_int64_array(arg : alljoyn_msgarg, length : usize, ax : *mut i64) -> QStatus);
    alljoyn_msgarg_set_int64_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ax))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_objectpath<P1>(arg: alljoyn_msgarg, o: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_objectpath(arg : alljoyn_msgarg, o : windows_core::PCSTR) -> QStatus);
    alljoyn_msgarg_set_objectpath(core::mem::transmute(arg), o.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_objectpath_array(arg: alljoyn_msgarg, length: usize, ao: *const *const i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_objectpath_array(arg : alljoyn_msgarg, length : usize, ao : *const *const i8) -> QStatus);
    alljoyn_msgarg_set_objectpath_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ao))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_signature<P1>(arg: alljoyn_msgarg, g: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_signature(arg : alljoyn_msgarg, g : windows_core::PCSTR) -> QStatus);
    alljoyn_msgarg_set_signature(core::mem::transmute(arg), g.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_signature_array(arg: alljoyn_msgarg, length: usize, ag: *const *const i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_signature_array(arg : alljoyn_msgarg, length : usize, ag : *const *const i8) -> QStatus);
    alljoyn_msgarg_set_signature_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ag))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_string<P1>(arg: alljoyn_msgarg, s: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_string(arg : alljoyn_msgarg, s : windows_core::PCSTR) -> QStatus);
    alljoyn_msgarg_set_string(core::mem::transmute(arg), s.param().abi())
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_string_array(arg: alljoyn_msgarg, length: usize, r#as: *const *const i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_string_array(arg : alljoyn_msgarg, length : usize, r#as : *const *const i8) -> QStatus);
    alljoyn_msgarg_set_string_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(r#as))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint16(arg: alljoyn_msgarg, q: u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_uint16(arg : alljoyn_msgarg, q : u16) -> QStatus);
    alljoyn_msgarg_set_uint16(core::mem::transmute(arg), core::mem::transmute(q))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint16_array(arg: alljoyn_msgarg, length: usize, aq: *mut u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_uint16_array(arg : alljoyn_msgarg, length : usize, aq : *mut u16) -> QStatus);
    alljoyn_msgarg_set_uint16_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(aq))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint32(arg: alljoyn_msgarg, u: u32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_uint32(arg : alljoyn_msgarg, u : u32) -> QStatus);
    alljoyn_msgarg_set_uint32(core::mem::transmute(arg), core::mem::transmute(u))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint32_array(arg: alljoyn_msgarg, length: usize, au: *mut u32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_uint32_array(arg : alljoyn_msgarg, length : usize, au : *mut u32) -> QStatus);
    alljoyn_msgarg_set_uint32_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(au))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint64(arg: alljoyn_msgarg, t: u64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_uint64(arg : alljoyn_msgarg, t : u64) -> QStatus);
    alljoyn_msgarg_set_uint64(core::mem::transmute(arg), core::mem::transmute(t))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint64_array(arg: alljoyn_msgarg, length: usize, at: *mut u64) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_uint64_array(arg : alljoyn_msgarg, length : usize, at : *mut u64) -> QStatus);
    alljoyn_msgarg_set_uint64_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(at))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint8(arg: alljoyn_msgarg, y: u8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_uint8(arg : alljoyn_msgarg, y : u8) -> QStatus);
    alljoyn_msgarg_set_uint8(core::mem::transmute(arg), core::mem::transmute(y))
}
#[inline]
pub unsafe fn alljoyn_msgarg_set_uint8_array(arg: alljoyn_msgarg, length: usize, ay: *mut u8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_set_uint8_array(arg : alljoyn_msgarg, length : usize, ay : *mut u8) -> QStatus);
    alljoyn_msgarg_set_uint8_array(core::mem::transmute(arg), core::mem::transmute(length), core::mem::transmute(ay))
}
#[inline]
pub unsafe fn alljoyn_msgarg_setdictentry(arg: alljoyn_msgarg, key: alljoyn_msgarg, value: alljoyn_msgarg) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_setdictentry(arg : alljoyn_msgarg, key : alljoyn_msgarg, value : alljoyn_msgarg) -> QStatus);
    alljoyn_msgarg_setdictentry(core::mem::transmute(arg), core::mem::transmute(key), core::mem::transmute(value))
}
#[inline]
pub unsafe fn alljoyn_msgarg_setstruct(arg: alljoyn_msgarg, struct_members: alljoyn_msgarg, num_members: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_setstruct(arg : alljoyn_msgarg, struct_members : alljoyn_msgarg, num_members : usize) -> QStatus);
    alljoyn_msgarg_setstruct(core::mem::transmute(arg), core::mem::transmute(struct_members), core::mem::transmute(num_members))
}
#[inline]
pub unsafe fn alljoyn_msgarg_signature<P1>(arg: alljoyn_msgarg, str: P1, buf: usize) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_signature(arg : alljoyn_msgarg, str : windows_core::PCSTR, buf : usize) -> usize);
    alljoyn_msgarg_signature(core::mem::transmute(arg), str.param().abi(), core::mem::transmute(buf))
}
#[inline]
pub unsafe fn alljoyn_msgarg_stabilize(arg: alljoyn_msgarg) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_stabilize(arg : alljoyn_msgarg));
    alljoyn_msgarg_stabilize(core::mem::transmute(arg))
}
#[inline]
pub unsafe fn alljoyn_msgarg_tostring<P1>(arg: alljoyn_msgarg, str: P1, buf: usize, indent: usize) -> usize
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_msgarg_tostring(arg : alljoyn_msgarg, str : windows_core::PCSTR, buf : usize, indent : usize) -> usize);
    alljoyn_msgarg_tostring(core::mem::transmute(arg), str.param().abi(), core::mem::transmute(buf), core::mem::transmute(indent))
}
#[inline]
pub unsafe fn alljoyn_observer_create(bus: alljoyn_busattachment, mandatoryinterfaces: *const *const i8, nummandatoryinterfaces: usize) -> alljoyn_observer {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_observer_create(bus : alljoyn_busattachment, mandatoryinterfaces : *const *const i8, nummandatoryinterfaces : usize) -> alljoyn_observer);
    alljoyn_observer_create(core::mem::transmute(bus), core::mem::transmute(mandatoryinterfaces), core::mem::transmute(nummandatoryinterfaces))
}
#[inline]
pub unsafe fn alljoyn_observer_destroy(observer: alljoyn_observer) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_observer_destroy(observer : alljoyn_observer));
    alljoyn_observer_destroy(core::mem::transmute(observer))
}
#[inline]
pub unsafe fn alljoyn_observer_get<P1, P2>(observer: alljoyn_observer, uniquebusname: P1, objectpath: P2) -> alljoyn_proxybusobject_ref
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_observer_get(observer : alljoyn_observer, uniquebusname : windows_core::PCSTR, objectpath : windows_core::PCSTR) -> alljoyn_proxybusobject_ref);
    alljoyn_observer_get(core::mem::transmute(observer), uniquebusname.param().abi(), objectpath.param().abi())
}
#[inline]
pub unsafe fn alljoyn_observer_getfirst(observer: alljoyn_observer) -> alljoyn_proxybusobject_ref {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_observer_getfirst(observer : alljoyn_observer) -> alljoyn_proxybusobject_ref);
    alljoyn_observer_getfirst(core::mem::transmute(observer))
}
#[inline]
pub unsafe fn alljoyn_observer_getnext(observer: alljoyn_observer, proxyref: alljoyn_proxybusobject_ref) -> alljoyn_proxybusobject_ref {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_observer_getnext(observer : alljoyn_observer, proxyref : alljoyn_proxybusobject_ref) -> alljoyn_proxybusobject_ref);
    alljoyn_observer_getnext(core::mem::transmute(observer), core::mem::transmute(proxyref))
}
#[inline]
pub unsafe fn alljoyn_observer_registerlistener(observer: alljoyn_observer, listener: alljoyn_observerlistener, triggeronexisting: i32) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_observer_registerlistener(observer : alljoyn_observer, listener : alljoyn_observerlistener, triggeronexisting : i32));
    alljoyn_observer_registerlistener(core::mem::transmute(observer), core::mem::transmute(listener), core::mem::transmute(triggeronexisting))
}
#[inline]
pub unsafe fn alljoyn_observer_unregisteralllisteners(observer: alljoyn_observer) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_observer_unregisteralllisteners(observer : alljoyn_observer));
    alljoyn_observer_unregisteralllisteners(core::mem::transmute(observer))
}
#[inline]
pub unsafe fn alljoyn_observer_unregisterlistener(observer: alljoyn_observer, listener: alljoyn_observerlistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_observer_unregisterlistener(observer : alljoyn_observer, listener : alljoyn_observerlistener));
    alljoyn_observer_unregisterlistener(core::mem::transmute(observer), core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_observerlistener_create(callback: *const alljoyn_observerlistener_callback, context: *const core::ffi::c_void) -> alljoyn_observerlistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_observerlistener_create(callback : *const alljoyn_observerlistener_callback, context : *const core::ffi::c_void) -> alljoyn_observerlistener);
    alljoyn_observerlistener_create(core::mem::transmute(callback), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_observerlistener_destroy(listener: alljoyn_observerlistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_observerlistener_destroy(listener : alljoyn_observerlistener));
    alljoyn_observerlistener_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_passwordmanager_setcredentials<P0, P1>(authmechanism: P0, password: P1) -> QStatus
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_passwordmanager_setcredentials(authmechanism : windows_core::PCSTR, password : windows_core::PCSTR) -> QStatus);
    alljoyn_passwordmanager_setcredentials(authmechanism.param().abi(), password.param().abi())
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurationlistener_create(callbacks: *const alljoyn_permissionconfigurationlistener_callbacks, context: *const core::ffi::c_void) -> alljoyn_permissionconfigurationlistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurationlistener_create(callbacks : *const alljoyn_permissionconfigurationlistener_callbacks, context : *const core::ffi::c_void) -> alljoyn_permissionconfigurationlistener);
    alljoyn_permissionconfigurationlistener_create(core::mem::transmute(callbacks), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurationlistener_destroy(listener: alljoyn_permissionconfigurationlistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurationlistener_destroy(listener : alljoyn_permissionconfigurationlistener));
    alljoyn_permissionconfigurationlistener_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_certificatechain_destroy(certificatechain: *mut i8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_certificatechain_destroy(certificatechain : *mut i8));
    alljoyn_permissionconfigurator_certificatechain_destroy(core::mem::transmute(certificatechain))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_certificateid_cleanup(certificateid: *mut alljoyn_certificateid) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_certificateid_cleanup(certificateid : *mut alljoyn_certificateid));
    alljoyn_permissionconfigurator_certificateid_cleanup(core::mem::transmute(certificateid))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_certificateidarray_cleanup(certificateidarray: *mut alljoyn_certificateidarray) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_certificateidarray_cleanup(certificateidarray : *mut alljoyn_certificateidarray));
    alljoyn_permissionconfigurator_certificateidarray_cleanup(core::mem::transmute(certificateidarray))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_claim(configurator: alljoyn_permissionconfigurator, cakey: *mut i8, identitycertificatechain: *mut i8, groupid: *const u8, groupsize: usize, groupauthority: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_claim(configurator : alljoyn_permissionconfigurator, cakey : *mut i8, identitycertificatechain : *mut i8, groupid : *const u8, groupsize : usize, groupauthority : *mut i8, manifestsxmls : *mut *mut i8, manifestscount : usize) -> QStatus);
    alljoyn_permissionconfigurator_claim(core::mem::transmute(configurator), core::mem::transmute(cakey), core::mem::transmute(identitycertificatechain), core::mem::transmute(groupid), core::mem::transmute(groupsize), core::mem::transmute(groupauthority), core::mem::transmute(manifestsxmls), core::mem::transmute(manifestscount))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_endmanagement(configurator: alljoyn_permissionconfigurator) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_endmanagement(configurator : alljoyn_permissionconfigurator) -> QStatus);
    alljoyn_permissionconfigurator_endmanagement(core::mem::transmute(configurator))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getapplicationstate(configurator: alljoyn_permissionconfigurator, state: *mut alljoyn_applicationstate) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getapplicationstate(configurator : alljoyn_permissionconfigurator, state : *mut alljoyn_applicationstate) -> QStatus);
    alljoyn_permissionconfigurator_getapplicationstate(core::mem::transmute(configurator), core::mem::transmute(state))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getclaimcapabilities(configurator: alljoyn_permissionconfigurator, claimcapabilities: *mut u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getclaimcapabilities(configurator : alljoyn_permissionconfigurator, claimcapabilities : *mut u16) -> QStatus);
    alljoyn_permissionconfigurator_getclaimcapabilities(core::mem::transmute(configurator), core::mem::transmute(claimcapabilities))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getclaimcapabilitiesadditionalinfo(configurator: alljoyn_permissionconfigurator, additionalinfo: *mut u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getclaimcapabilitiesadditionalinfo(configurator : alljoyn_permissionconfigurator, additionalinfo : *mut u16) -> QStatus);
    alljoyn_permissionconfigurator_getclaimcapabilitiesadditionalinfo(core::mem::transmute(configurator), core::mem::transmute(additionalinfo))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getdefaultclaimcapabilities() -> u16 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getdefaultclaimcapabilities() -> u16);
    alljoyn_permissionconfigurator_getdefaultclaimcapabilities()
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getdefaultpolicy(configurator: alljoyn_permissionconfigurator, policyxml: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getdefaultpolicy(configurator : alljoyn_permissionconfigurator, policyxml : *mut *mut i8) -> QStatus);
    alljoyn_permissionconfigurator_getdefaultpolicy(core::mem::transmute(configurator), core::mem::transmute(policyxml))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getidentity(configurator: alljoyn_permissionconfigurator, identitycertificatechain: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getidentity(configurator : alljoyn_permissionconfigurator, identitycertificatechain : *mut *mut i8) -> QStatus);
    alljoyn_permissionconfigurator_getidentity(core::mem::transmute(configurator), core::mem::transmute(identitycertificatechain))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getidentitycertificateid(configurator: alljoyn_permissionconfigurator, certificateid: *mut alljoyn_certificateid) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getidentitycertificateid(configurator : alljoyn_permissionconfigurator, certificateid : *mut alljoyn_certificateid) -> QStatus);
    alljoyn_permissionconfigurator_getidentitycertificateid(core::mem::transmute(configurator), core::mem::transmute(certificateid))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getmanifests(configurator: alljoyn_permissionconfigurator, manifestarray: *mut alljoyn_manifestarray) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getmanifests(configurator : alljoyn_permissionconfigurator, manifestarray : *mut alljoyn_manifestarray) -> QStatus);
    alljoyn_permissionconfigurator_getmanifests(core::mem::transmute(configurator), core::mem::transmute(manifestarray))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getmanifesttemplate(configurator: alljoyn_permissionconfigurator, manifesttemplatexml: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getmanifesttemplate(configurator : alljoyn_permissionconfigurator, manifesttemplatexml : *mut *mut i8) -> QStatus);
    alljoyn_permissionconfigurator_getmanifesttemplate(core::mem::transmute(configurator), core::mem::transmute(manifesttemplatexml))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getmembershipsummaries(configurator: alljoyn_permissionconfigurator, certificateids: *mut alljoyn_certificateidarray) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getmembershipsummaries(configurator : alljoyn_permissionconfigurator, certificateids : *mut alljoyn_certificateidarray) -> QStatus);
    alljoyn_permissionconfigurator_getmembershipsummaries(core::mem::transmute(configurator), core::mem::transmute(certificateids))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getpolicy(configurator: alljoyn_permissionconfigurator, policyxml: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getpolicy(configurator : alljoyn_permissionconfigurator, policyxml : *mut *mut i8) -> QStatus);
    alljoyn_permissionconfigurator_getpolicy(core::mem::transmute(configurator), core::mem::transmute(policyxml))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_getpublickey(configurator: alljoyn_permissionconfigurator, publickey: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_getpublickey(configurator : alljoyn_permissionconfigurator, publickey : *mut *mut i8) -> QStatus);
    alljoyn_permissionconfigurator_getpublickey(core::mem::transmute(configurator), core::mem::transmute(publickey))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_installmanifests(configurator: alljoyn_permissionconfigurator, manifestsxmls: *mut *mut i8, manifestscount: usize, append: i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_installmanifests(configurator : alljoyn_permissionconfigurator, manifestsxmls : *mut *mut i8, manifestscount : usize, append : i32) -> QStatus);
    alljoyn_permissionconfigurator_installmanifests(core::mem::transmute(configurator), core::mem::transmute(manifestsxmls), core::mem::transmute(manifestscount), core::mem::transmute(append))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_installmembership(configurator: alljoyn_permissionconfigurator, membershipcertificatechain: *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_installmembership(configurator : alljoyn_permissionconfigurator, membershipcertificatechain : *mut i8) -> QStatus);
    alljoyn_permissionconfigurator_installmembership(core::mem::transmute(configurator), core::mem::transmute(membershipcertificatechain))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_manifestarray_cleanup(manifestarray: *mut alljoyn_manifestarray) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_manifestarray_cleanup(manifestarray : *mut alljoyn_manifestarray));
    alljoyn_permissionconfigurator_manifestarray_cleanup(core::mem::transmute(manifestarray))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_manifesttemplate_destroy(manifesttemplatexml: *mut i8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_manifesttemplate_destroy(manifesttemplatexml : *mut i8));
    alljoyn_permissionconfigurator_manifesttemplate_destroy(core::mem::transmute(manifesttemplatexml))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_policy_destroy(policyxml: *mut i8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_policy_destroy(policyxml : *mut i8));
    alljoyn_permissionconfigurator_policy_destroy(core::mem::transmute(policyxml))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_publickey_destroy(publickey: *mut i8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_publickey_destroy(publickey : *mut i8));
    alljoyn_permissionconfigurator_publickey_destroy(core::mem::transmute(publickey))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_removemembership(configurator: alljoyn_permissionconfigurator, serial: *const u8, seriallen: usize, issuerpublickey: *mut i8, issueraki: *const u8, issuerakilen: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_removemembership(configurator : alljoyn_permissionconfigurator, serial : *const u8, seriallen : usize, issuerpublickey : *mut i8, issueraki : *const u8, issuerakilen : usize) -> QStatus);
    alljoyn_permissionconfigurator_removemembership(core::mem::transmute(configurator), core::mem::transmute(serial), core::mem::transmute(seriallen), core::mem::transmute(issuerpublickey), core::mem::transmute(issueraki), core::mem::transmute(issuerakilen))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_reset(configurator: alljoyn_permissionconfigurator) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_reset(configurator : alljoyn_permissionconfigurator) -> QStatus);
    alljoyn_permissionconfigurator_reset(core::mem::transmute(configurator))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_resetpolicy(configurator: alljoyn_permissionconfigurator) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_resetpolicy(configurator : alljoyn_permissionconfigurator) -> QStatus);
    alljoyn_permissionconfigurator_resetpolicy(core::mem::transmute(configurator))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_setapplicationstate(configurator: alljoyn_permissionconfigurator, state: alljoyn_applicationstate) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_setapplicationstate(configurator : alljoyn_permissionconfigurator, state : alljoyn_applicationstate) -> QStatus);
    alljoyn_permissionconfigurator_setapplicationstate(core::mem::transmute(configurator), core::mem::transmute(state))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_setclaimcapabilities(configurator: alljoyn_permissionconfigurator, claimcapabilities: u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_setclaimcapabilities(configurator : alljoyn_permissionconfigurator, claimcapabilities : u16) -> QStatus);
    alljoyn_permissionconfigurator_setclaimcapabilities(core::mem::transmute(configurator), core::mem::transmute(claimcapabilities))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_setclaimcapabilitiesadditionalinfo(configurator: alljoyn_permissionconfigurator, additionalinfo: u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_setclaimcapabilitiesadditionalinfo(configurator : alljoyn_permissionconfigurator, additionalinfo : u16) -> QStatus);
    alljoyn_permissionconfigurator_setclaimcapabilitiesadditionalinfo(core::mem::transmute(configurator), core::mem::transmute(additionalinfo))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_setmanifesttemplatefromxml(configurator: alljoyn_permissionconfigurator, manifesttemplatexml: *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_setmanifesttemplatefromxml(configurator : alljoyn_permissionconfigurator, manifesttemplatexml : *mut i8) -> QStatus);
    alljoyn_permissionconfigurator_setmanifesttemplatefromxml(core::mem::transmute(configurator), core::mem::transmute(manifesttemplatexml))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_startmanagement(configurator: alljoyn_permissionconfigurator) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_startmanagement(configurator : alljoyn_permissionconfigurator) -> QStatus);
    alljoyn_permissionconfigurator_startmanagement(core::mem::transmute(configurator))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_updateidentity(configurator: alljoyn_permissionconfigurator, identitycertificatechain: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_updateidentity(configurator : alljoyn_permissionconfigurator, identitycertificatechain : *mut i8, manifestsxmls : *mut *mut i8, manifestscount : usize) -> QStatus);
    alljoyn_permissionconfigurator_updateidentity(core::mem::transmute(configurator), core::mem::transmute(identitycertificatechain), core::mem::transmute(manifestsxmls), core::mem::transmute(manifestscount))
}
#[inline]
pub unsafe fn alljoyn_permissionconfigurator_updatepolicy(configurator: alljoyn_permissionconfigurator, policyxml: *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_permissionconfigurator_updatepolicy(configurator : alljoyn_permissionconfigurator, policyxml : *mut i8) -> QStatus);
    alljoyn_permissionconfigurator_updatepolicy(core::mem::transmute(configurator), core::mem::transmute(policyxml))
}
#[inline]
pub unsafe fn alljoyn_pinglistener_create(callback: *const alljoyn_pinglistener_callback, context: *const core::ffi::c_void) -> alljoyn_pinglistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_pinglistener_create(callback : *const alljoyn_pinglistener_callback, context : *const core::ffi::c_void) -> alljoyn_pinglistener);
    alljoyn_pinglistener_create(core::mem::transmute(callback), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_pinglistener_destroy(listener: alljoyn_pinglistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_pinglistener_destroy(listener : alljoyn_pinglistener));
    alljoyn_pinglistener_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_addchild(proxyobj: alljoyn_proxybusobject, child: alljoyn_proxybusobject) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_addchild(proxyobj : alljoyn_proxybusobject, child : alljoyn_proxybusobject) -> QStatus);
    alljoyn_proxybusobject_addchild(core::mem::transmute(proxyobj), core::mem::transmute(child))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_addinterface(proxyobj: alljoyn_proxybusobject, iface: alljoyn_interfacedescription) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_addinterface(proxyobj : alljoyn_proxybusobject, iface : alljoyn_interfacedescription) -> QStatus);
    alljoyn_proxybusobject_addinterface(core::mem::transmute(proxyobj), core::mem::transmute(iface))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_addinterface_by_name<P1>(proxyobj: alljoyn_proxybusobject, name: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_addinterface_by_name(proxyobj : alljoyn_proxybusobject, name : windows_core::PCSTR) -> QStatus);
    alljoyn_proxybusobject_addinterface_by_name(core::mem::transmute(proxyobj), name.param().abi())
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_copy(source: alljoyn_proxybusobject) -> alljoyn_proxybusobject {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_copy(source : alljoyn_proxybusobject) -> alljoyn_proxybusobject);
    alljoyn_proxybusobject_copy(core::mem::transmute(source))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_create<P1, P2>(bus: alljoyn_busattachment, service: P1, path: P2, sessionid: u32) -> alljoyn_proxybusobject
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_create(bus : alljoyn_busattachment, service : windows_core::PCSTR, path : windows_core::PCSTR, sessionid : u32) -> alljoyn_proxybusobject);
    alljoyn_proxybusobject_create(core::mem::transmute(bus), service.param().abi(), path.param().abi(), core::mem::transmute(sessionid))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_create_secure<P1, P2>(bus: alljoyn_busattachment, service: P1, path: P2, sessionid: u32) -> alljoyn_proxybusobject
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_create_secure(bus : alljoyn_busattachment, service : windows_core::PCSTR, path : windows_core::PCSTR, sessionid : u32) -> alljoyn_proxybusobject);
    alljoyn_proxybusobject_create_secure(core::mem::transmute(bus), service.param().abi(), path.param().abi(), core::mem::transmute(sessionid))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_destroy(proxyobj: alljoyn_proxybusobject) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_destroy(proxyobj : alljoyn_proxybusobject));
    alljoyn_proxybusobject_destroy(core::mem::transmute(proxyobj))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_enablepropertycaching(proxyobj: alljoyn_proxybusobject) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_enablepropertycaching(proxyobj : alljoyn_proxybusobject));
    alljoyn_proxybusobject_enablepropertycaching(core::mem::transmute(proxyobj))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getallproperties<P1>(proxyobj: alljoyn_proxybusobject, iface: P1, values: alljoyn_msgarg) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getallproperties(proxyobj : alljoyn_proxybusobject, iface : windows_core::PCSTR, values : alljoyn_msgarg) -> QStatus);
    alljoyn_proxybusobject_getallproperties(core::mem::transmute(proxyobj), iface.param().abi(), core::mem::transmute(values))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getallpropertiesasync<P1>(proxyobj: alljoyn_proxybusobject, iface: P1, callback: alljoyn_proxybusobject_listener_getallpropertiescb_ptr, timeout: u32, context: *mut core::ffi::c_void) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getallpropertiesasync(proxyobj : alljoyn_proxybusobject, iface : windows_core::PCSTR, callback : alljoyn_proxybusobject_listener_getallpropertiescb_ptr, timeout : u32, context : *mut core::ffi::c_void) -> QStatus);
    alljoyn_proxybusobject_getallpropertiesasync(core::mem::transmute(proxyobj), iface.param().abi(), core::mem::transmute(callback), core::mem::transmute(timeout), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getchild<P1>(proxyobj: alljoyn_proxybusobject, path: P1) -> alljoyn_proxybusobject
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getchild(proxyobj : alljoyn_proxybusobject, path : windows_core::PCSTR) -> alljoyn_proxybusobject);
    alljoyn_proxybusobject_getchild(core::mem::transmute(proxyobj), path.param().abi())
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getchildren(proxyobj: alljoyn_proxybusobject, children: *mut alljoyn_proxybusobject, numchildren: usize) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getchildren(proxyobj : alljoyn_proxybusobject, children : *mut alljoyn_proxybusobject, numchildren : usize) -> usize);
    alljoyn_proxybusobject_getchildren(core::mem::transmute(proxyobj), core::mem::transmute(children), core::mem::transmute(numchildren))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getinterface<P1>(proxyobj: alljoyn_proxybusobject, iface: P1) -> alljoyn_interfacedescription
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getinterface(proxyobj : alljoyn_proxybusobject, iface : windows_core::PCSTR) -> alljoyn_interfacedescription);
    alljoyn_proxybusobject_getinterface(core::mem::transmute(proxyobj), iface.param().abi())
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getinterfaces(proxyobj: alljoyn_proxybusobject, ifaces: *const alljoyn_interfacedescription, numifaces: usize) -> usize {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getinterfaces(proxyobj : alljoyn_proxybusobject, ifaces : *const alljoyn_interfacedescription, numifaces : usize) -> usize);
    alljoyn_proxybusobject_getinterfaces(core::mem::transmute(proxyobj), core::mem::transmute(ifaces), core::mem::transmute(numifaces))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getpath(proxyobj: alljoyn_proxybusobject) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getpath(proxyobj : alljoyn_proxybusobject) -> windows_core::PCSTR);
    alljoyn_proxybusobject_getpath(core::mem::transmute(proxyobj))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getproperty<P1, P2>(proxyobj: alljoyn_proxybusobject, iface: P1, property: P2, value: alljoyn_msgarg) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getproperty(proxyobj : alljoyn_proxybusobject, iface : windows_core::PCSTR, property : windows_core::PCSTR, value : alljoyn_msgarg) -> QStatus);
    alljoyn_proxybusobject_getproperty(core::mem::transmute(proxyobj), iface.param().abi(), property.param().abi(), core::mem::transmute(value))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getpropertyasync<P1, P2>(proxyobj: alljoyn_proxybusobject, iface: P1, property: P2, callback: alljoyn_proxybusobject_listener_getpropertycb_ptr, timeout: u32, context: *mut core::ffi::c_void) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getpropertyasync(proxyobj : alljoyn_proxybusobject, iface : windows_core::PCSTR, property : windows_core::PCSTR, callback : alljoyn_proxybusobject_listener_getpropertycb_ptr, timeout : u32, context : *mut core::ffi::c_void) -> QStatus);
    alljoyn_proxybusobject_getpropertyasync(core::mem::transmute(proxyobj), iface.param().abi(), property.param().abi(), core::mem::transmute(callback), core::mem::transmute(timeout), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getservicename(proxyobj: alljoyn_proxybusobject) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getservicename(proxyobj : alljoyn_proxybusobject) -> windows_core::PCSTR);
    alljoyn_proxybusobject_getservicename(core::mem::transmute(proxyobj))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getsessionid(proxyobj: alljoyn_proxybusobject) -> u32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getsessionid(proxyobj : alljoyn_proxybusobject) -> u32);
    alljoyn_proxybusobject_getsessionid(core::mem::transmute(proxyobj))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_getuniquename(proxyobj: alljoyn_proxybusobject) -> windows_core::PCSTR {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_getuniquename(proxyobj : alljoyn_proxybusobject) -> windows_core::PCSTR);
    alljoyn_proxybusobject_getuniquename(core::mem::transmute(proxyobj))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_implementsinterface<P1>(proxyobj: alljoyn_proxybusobject, iface: P1) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_implementsinterface(proxyobj : alljoyn_proxybusobject, iface : windows_core::PCSTR) -> i32);
    alljoyn_proxybusobject_implementsinterface(core::mem::transmute(proxyobj), iface.param().abi())
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_introspectremoteobject(proxyobj: alljoyn_proxybusobject) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_introspectremoteobject(proxyobj : alljoyn_proxybusobject) -> QStatus);
    alljoyn_proxybusobject_introspectremoteobject(core::mem::transmute(proxyobj))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_introspectremoteobjectasync(proxyobj: alljoyn_proxybusobject, callback: alljoyn_proxybusobject_listener_introspectcb_ptr, context: *mut core::ffi::c_void) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_introspectremoteobjectasync(proxyobj : alljoyn_proxybusobject, callback : alljoyn_proxybusobject_listener_introspectcb_ptr, context : *mut core::ffi::c_void) -> QStatus);
    alljoyn_proxybusobject_introspectremoteobjectasync(core::mem::transmute(proxyobj), core::mem::transmute(callback), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_issecure(proxyobj: alljoyn_proxybusobject) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_issecure(proxyobj : alljoyn_proxybusobject) -> i32);
    alljoyn_proxybusobject_issecure(core::mem::transmute(proxyobj))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_isvalid(proxyobj: alljoyn_proxybusobject) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_isvalid(proxyobj : alljoyn_proxybusobject) -> i32);
    alljoyn_proxybusobject_isvalid(core::mem::transmute(proxyobj))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcall<P1, P2>(proxyobj: alljoyn_proxybusobject, ifacename: P1, methodname: P2, args: alljoyn_msgarg, numargs: usize, replymsg: alljoyn_message, timeout: u32, flags: u8) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_methodcall(proxyobj : alljoyn_proxybusobject, ifacename : windows_core::PCSTR, methodname : windows_core::PCSTR, args : alljoyn_msgarg, numargs : usize, replymsg : alljoyn_message, timeout : u32, flags : u8) -> QStatus);
    alljoyn_proxybusobject_methodcall(core::mem::transmute(proxyobj), ifacename.param().abi(), methodname.param().abi(), core::mem::transmute(args), core::mem::transmute(numargs), core::mem::transmute(replymsg), core::mem::transmute(timeout), core::mem::transmute(flags))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcall_member(proxyobj: alljoyn_proxybusobject, method: alljoyn_interfacedescription_member, args: alljoyn_msgarg, numargs: usize, replymsg: alljoyn_message, timeout: u32, flags: u8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_methodcall_member(proxyobj : alljoyn_proxybusobject, method : alljoyn_interfacedescription_member, args : alljoyn_msgarg, numargs : usize, replymsg : alljoyn_message, timeout : u32, flags : u8) -> QStatus);
    alljoyn_proxybusobject_methodcall_member(core::mem::transmute(proxyobj), core::mem::transmute(method), core::mem::transmute(args), core::mem::transmute(numargs), core::mem::transmute(replymsg), core::mem::transmute(timeout), core::mem::transmute(flags))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcall_member_noreply(proxyobj: alljoyn_proxybusobject, method: alljoyn_interfacedescription_member, args: alljoyn_msgarg, numargs: usize, flags: u8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_methodcall_member_noreply(proxyobj : alljoyn_proxybusobject, method : alljoyn_interfacedescription_member, args : alljoyn_msgarg, numargs : usize, flags : u8) -> QStatus);
    alljoyn_proxybusobject_methodcall_member_noreply(core::mem::transmute(proxyobj), core::mem::transmute(method), core::mem::transmute(args), core::mem::transmute(numargs), core::mem::transmute(flags))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcall_noreply<P1, P2>(proxyobj: alljoyn_proxybusobject, ifacename: P1, methodname: P2, args: alljoyn_msgarg, numargs: usize, flags: u8) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_methodcall_noreply(proxyobj : alljoyn_proxybusobject, ifacename : windows_core::PCSTR, methodname : windows_core::PCSTR, args : alljoyn_msgarg, numargs : usize, flags : u8) -> QStatus);
    alljoyn_proxybusobject_methodcall_noreply(core::mem::transmute(proxyobj), ifacename.param().abi(), methodname.param().abi(), core::mem::transmute(args), core::mem::transmute(numargs), core::mem::transmute(flags))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcallasync<P1, P2>(proxyobj: alljoyn_proxybusobject, ifacename: P1, methodname: P2, replyfunc: alljoyn_messagereceiver_replyhandler_ptr, args: alljoyn_msgarg, numargs: usize, context: *mut core::ffi::c_void, timeout: u32, flags: u8) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_methodcallasync(proxyobj : alljoyn_proxybusobject, ifacename : windows_core::PCSTR, methodname : windows_core::PCSTR, replyfunc : alljoyn_messagereceiver_replyhandler_ptr, args : alljoyn_msgarg, numargs : usize, context : *mut core::ffi::c_void, timeout : u32, flags : u8) -> QStatus);
    alljoyn_proxybusobject_methodcallasync(core::mem::transmute(proxyobj), ifacename.param().abi(), methodname.param().abi(), core::mem::transmute(replyfunc), core::mem::transmute(args), core::mem::transmute(numargs), core::mem::transmute(context), core::mem::transmute(timeout), core::mem::transmute(flags))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_methodcallasync_member(proxyobj: alljoyn_proxybusobject, method: alljoyn_interfacedescription_member, replyfunc: alljoyn_messagereceiver_replyhandler_ptr, args: alljoyn_msgarg, numargs: usize, context: *mut core::ffi::c_void, timeout: u32, flags: u8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_methodcallasync_member(proxyobj : alljoyn_proxybusobject, method : alljoyn_interfacedescription_member, replyfunc : alljoyn_messagereceiver_replyhandler_ptr, args : alljoyn_msgarg, numargs : usize, context : *mut core::ffi::c_void, timeout : u32, flags : u8) -> QStatus);
    alljoyn_proxybusobject_methodcallasync_member(core::mem::transmute(proxyobj), core::mem::transmute(method), core::mem::transmute(replyfunc), core::mem::transmute(args), core::mem::transmute(numargs), core::mem::transmute(context), core::mem::transmute(timeout), core::mem::transmute(flags))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_parsexml<P1, P2>(proxyobj: alljoyn_proxybusobject, xml: P1, identifier: P2) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_parsexml(proxyobj : alljoyn_proxybusobject, xml : windows_core::PCSTR, identifier : windows_core::PCSTR) -> QStatus);
    alljoyn_proxybusobject_parsexml(core::mem::transmute(proxyobj), xml.param().abi(), identifier.param().abi())
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_ref_create(proxy: alljoyn_proxybusobject) -> alljoyn_proxybusobject_ref {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_ref_create(proxy : alljoyn_proxybusobject) -> alljoyn_proxybusobject_ref);
    alljoyn_proxybusobject_ref_create(core::mem::transmute(proxy))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_ref_decref(r#ref: alljoyn_proxybusobject_ref) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_ref_decref(r#ref : alljoyn_proxybusobject_ref));
    alljoyn_proxybusobject_ref_decref(core::mem::transmute(r#ref))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_ref_get(r#ref: alljoyn_proxybusobject_ref) -> alljoyn_proxybusobject {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_ref_get(r#ref : alljoyn_proxybusobject_ref) -> alljoyn_proxybusobject);
    alljoyn_proxybusobject_ref_get(core::mem::transmute(r#ref))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_ref_incref(r#ref: alljoyn_proxybusobject_ref) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_ref_incref(r#ref : alljoyn_proxybusobject_ref));
    alljoyn_proxybusobject_ref_incref(core::mem::transmute(r#ref))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_registerpropertieschangedlistener<P1>(proxyobj: alljoyn_proxybusobject, iface: P1, properties: *const *const i8, numproperties: usize, callback: alljoyn_proxybusobject_listener_propertieschanged_ptr, context: *mut core::ffi::c_void) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_registerpropertieschangedlistener(proxyobj : alljoyn_proxybusobject, iface : windows_core::PCSTR, properties : *const *const i8, numproperties : usize, callback : alljoyn_proxybusobject_listener_propertieschanged_ptr, context : *mut core::ffi::c_void) -> QStatus);
    alljoyn_proxybusobject_registerpropertieschangedlistener(core::mem::transmute(proxyobj), iface.param().abi(), core::mem::transmute(properties), core::mem::transmute(numproperties), core::mem::transmute(callback), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_removechild<P1>(proxyobj: alljoyn_proxybusobject, path: P1) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_removechild(proxyobj : alljoyn_proxybusobject, path : windows_core::PCSTR) -> QStatus);
    alljoyn_proxybusobject_removechild(core::mem::transmute(proxyobj), path.param().abi())
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_secureconnection(proxyobj: alljoyn_proxybusobject, forceauth: i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_secureconnection(proxyobj : alljoyn_proxybusobject, forceauth : i32) -> QStatus);
    alljoyn_proxybusobject_secureconnection(core::mem::transmute(proxyobj), core::mem::transmute(forceauth))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_secureconnectionasync(proxyobj: alljoyn_proxybusobject, forceauth: i32) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_secureconnectionasync(proxyobj : alljoyn_proxybusobject, forceauth : i32) -> QStatus);
    alljoyn_proxybusobject_secureconnectionasync(core::mem::transmute(proxyobj), core::mem::transmute(forceauth))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_setproperty<P1, P2>(proxyobj: alljoyn_proxybusobject, iface: P1, property: P2, value: alljoyn_msgarg) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_setproperty(proxyobj : alljoyn_proxybusobject, iface : windows_core::PCSTR, property : windows_core::PCSTR, value : alljoyn_msgarg) -> QStatus);
    alljoyn_proxybusobject_setproperty(core::mem::transmute(proxyobj), iface.param().abi(), property.param().abi(), core::mem::transmute(value))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_setpropertyasync<P1, P2>(proxyobj: alljoyn_proxybusobject, iface: P1, property: P2, value: alljoyn_msgarg, callback: alljoyn_proxybusobject_listener_setpropertycb_ptr, timeout: u32, context: *mut core::ffi::c_void) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_setpropertyasync(proxyobj : alljoyn_proxybusobject, iface : windows_core::PCSTR, property : windows_core::PCSTR, value : alljoyn_msgarg, callback : alljoyn_proxybusobject_listener_setpropertycb_ptr, timeout : u32, context : *mut core::ffi::c_void) -> QStatus);
    alljoyn_proxybusobject_setpropertyasync(core::mem::transmute(proxyobj), iface.param().abi(), property.param().abi(), core::mem::transmute(value), core::mem::transmute(callback), core::mem::transmute(timeout), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_proxybusobject_unregisterpropertieschangedlistener<P1>(proxyobj: alljoyn_proxybusobject, iface: P1, callback: alljoyn_proxybusobject_listener_propertieschanged_ptr) -> QStatus
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_proxybusobject_unregisterpropertieschangedlistener(proxyobj : alljoyn_proxybusobject, iface : windows_core::PCSTR, callback : alljoyn_proxybusobject_listener_propertieschanged_ptr) -> QStatus);
    alljoyn_proxybusobject_unregisterpropertieschangedlistener(core::mem::transmute(proxyobj), iface.param().abi(), core::mem::transmute(callback))
}
#[inline]
pub unsafe fn alljoyn_routerinit() -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_routerinit() -> QStatus);
    alljoyn_routerinit()
}
#[inline]
pub unsafe fn alljoyn_routerinitwithconfig(configxml: *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_routerinitwithconfig(configxml : *mut i8) -> QStatus);
    alljoyn_routerinitwithconfig(core::mem::transmute(configxml))
}
#[inline]
pub unsafe fn alljoyn_routershutdown() -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_routershutdown() -> QStatus);
    alljoyn_routershutdown()
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_claim(proxy: alljoyn_securityapplicationproxy, cakey: *mut i8, identitycertificatechain: *mut i8, groupid: *const u8, groupsize: usize, groupauthority: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_claim(proxy : alljoyn_securityapplicationproxy, cakey : *mut i8, identitycertificatechain : *mut i8, groupid : *const u8, groupsize : usize, groupauthority : *mut i8, manifestsxmls : *mut *mut i8, manifestscount : usize) -> QStatus);
    alljoyn_securityapplicationproxy_claim(core::mem::transmute(proxy), core::mem::transmute(cakey), core::mem::transmute(identitycertificatechain), core::mem::transmute(groupid), core::mem::transmute(groupsize), core::mem::transmute(groupauthority), core::mem::transmute(manifestsxmls), core::mem::transmute(manifestscount))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_computemanifestdigest(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, digest: *mut *mut u8, digestsize: *mut usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_computemanifestdigest(unsignedmanifestxml : *mut i8, identitycertificatepem : *mut i8, digest : *mut *mut u8, digestsize : *mut usize) -> QStatus);
    alljoyn_securityapplicationproxy_computemanifestdigest(core::mem::transmute(unsignedmanifestxml), core::mem::transmute(identitycertificatepem), core::mem::transmute(digest), core::mem::transmute(digestsize))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_create(bus: alljoyn_busattachment, appbusname: *mut i8, sessionid: u32) -> alljoyn_securityapplicationproxy {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_create(bus : alljoyn_busattachment, appbusname : *mut i8, sessionid : u32) -> alljoyn_securityapplicationproxy);
    alljoyn_securityapplicationproxy_create(core::mem::transmute(bus), core::mem::transmute(appbusname), core::mem::transmute(sessionid))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_destroy(proxy: alljoyn_securityapplicationproxy) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_destroy(proxy : alljoyn_securityapplicationproxy));
    alljoyn_securityapplicationproxy_destroy(core::mem::transmute(proxy))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_digest_destroy(digest: *mut u8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_digest_destroy(digest : *mut u8));
    alljoyn_securityapplicationproxy_digest_destroy(core::mem::transmute(digest))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_eccpublickey_destroy(eccpublickey: *mut i8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_eccpublickey_destroy(eccpublickey : *mut i8));
    alljoyn_securityapplicationproxy_eccpublickey_destroy(core::mem::transmute(eccpublickey))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_endmanagement(proxy: alljoyn_securityapplicationproxy) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_endmanagement(proxy : alljoyn_securityapplicationproxy) -> QStatus);
    alljoyn_securityapplicationproxy_endmanagement(core::mem::transmute(proxy))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getapplicationstate(proxy: alljoyn_securityapplicationproxy, applicationstate: *mut alljoyn_applicationstate) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_getapplicationstate(proxy : alljoyn_securityapplicationproxy, applicationstate : *mut alljoyn_applicationstate) -> QStatus);
    alljoyn_securityapplicationproxy_getapplicationstate(core::mem::transmute(proxy), core::mem::transmute(applicationstate))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getclaimcapabilities(proxy: alljoyn_securityapplicationproxy, capabilities: *mut u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_getclaimcapabilities(proxy : alljoyn_securityapplicationproxy, capabilities : *mut u16) -> QStatus);
    alljoyn_securityapplicationproxy_getclaimcapabilities(core::mem::transmute(proxy), core::mem::transmute(capabilities))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getclaimcapabilitiesadditionalinfo(proxy: alljoyn_securityapplicationproxy, additionalinfo: *mut u16) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_getclaimcapabilitiesadditionalinfo(proxy : alljoyn_securityapplicationproxy, additionalinfo : *mut u16) -> QStatus);
    alljoyn_securityapplicationproxy_getclaimcapabilitiesadditionalinfo(core::mem::transmute(proxy), core::mem::transmute(additionalinfo))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getdefaultpolicy(proxy: alljoyn_securityapplicationproxy, policyxml: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_getdefaultpolicy(proxy : alljoyn_securityapplicationproxy, policyxml : *mut *mut i8) -> QStatus);
    alljoyn_securityapplicationproxy_getdefaultpolicy(core::mem::transmute(proxy), core::mem::transmute(policyxml))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_geteccpublickey(proxy: alljoyn_securityapplicationproxy, eccpublickey: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_geteccpublickey(proxy : alljoyn_securityapplicationproxy, eccpublickey : *mut *mut i8) -> QStatus);
    alljoyn_securityapplicationproxy_geteccpublickey(core::mem::transmute(proxy), core::mem::transmute(eccpublickey))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getmanifesttemplate(proxy: alljoyn_securityapplicationproxy, manifesttemplatexml: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_getmanifesttemplate(proxy : alljoyn_securityapplicationproxy, manifesttemplatexml : *mut *mut i8) -> QStatus);
    alljoyn_securityapplicationproxy_getmanifesttemplate(core::mem::transmute(proxy), core::mem::transmute(manifesttemplatexml))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getpermissionmanagementsessionport() -> u16 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_getpermissionmanagementsessionport() -> u16);
    alljoyn_securityapplicationproxy_getpermissionmanagementsessionport()
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_getpolicy(proxy: alljoyn_securityapplicationproxy, policyxml: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_getpolicy(proxy : alljoyn_securityapplicationproxy, policyxml : *mut *mut i8) -> QStatus);
    alljoyn_securityapplicationproxy_getpolicy(core::mem::transmute(proxy), core::mem::transmute(policyxml))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_installmembership(proxy: alljoyn_securityapplicationproxy, membershipcertificatechain: *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_installmembership(proxy : alljoyn_securityapplicationproxy, membershipcertificatechain : *mut i8) -> QStatus);
    alljoyn_securityapplicationproxy_installmembership(core::mem::transmute(proxy), core::mem::transmute(membershipcertificatechain))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_manifest_destroy(signedmanifestxml: *mut i8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_manifest_destroy(signedmanifestxml : *mut i8));
    alljoyn_securityapplicationproxy_manifest_destroy(core::mem::transmute(signedmanifestxml))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_manifesttemplate_destroy(manifesttemplatexml: *mut i8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_manifesttemplate_destroy(manifesttemplatexml : *mut i8));
    alljoyn_securityapplicationproxy_manifesttemplate_destroy(core::mem::transmute(manifesttemplatexml))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_policy_destroy(policyxml: *mut i8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_policy_destroy(policyxml : *mut i8));
    alljoyn_securityapplicationproxy_policy_destroy(core::mem::transmute(policyxml))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_reset(proxy: alljoyn_securityapplicationproxy) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_reset(proxy : alljoyn_securityapplicationproxy) -> QStatus);
    alljoyn_securityapplicationproxy_reset(core::mem::transmute(proxy))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_resetpolicy(proxy: alljoyn_securityapplicationproxy) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_resetpolicy(proxy : alljoyn_securityapplicationproxy) -> QStatus);
    alljoyn_securityapplicationproxy_resetpolicy(core::mem::transmute(proxy))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_setmanifestsignature(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, signature: *const u8, signaturesize: usize, signedmanifestxml: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_setmanifestsignature(unsignedmanifestxml : *mut i8, identitycertificatepem : *mut i8, signature : *const u8, signaturesize : usize, signedmanifestxml : *mut *mut i8) -> QStatus);
    alljoyn_securityapplicationproxy_setmanifestsignature(core::mem::transmute(unsignedmanifestxml), core::mem::transmute(identitycertificatepem), core::mem::transmute(signature), core::mem::transmute(signaturesize), core::mem::transmute(signedmanifestxml))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_signmanifest(unsignedmanifestxml: *mut i8, identitycertificatepem: *mut i8, signingprivatekeypem: *mut i8, signedmanifestxml: *mut *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_signmanifest(unsignedmanifestxml : *mut i8, identitycertificatepem : *mut i8, signingprivatekeypem : *mut i8, signedmanifestxml : *mut *mut i8) -> QStatus);
    alljoyn_securityapplicationproxy_signmanifest(core::mem::transmute(unsignedmanifestxml), core::mem::transmute(identitycertificatepem), core::mem::transmute(signingprivatekeypem), core::mem::transmute(signedmanifestxml))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_startmanagement(proxy: alljoyn_securityapplicationproxy) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_startmanagement(proxy : alljoyn_securityapplicationproxy) -> QStatus);
    alljoyn_securityapplicationproxy_startmanagement(core::mem::transmute(proxy))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_updateidentity(proxy: alljoyn_securityapplicationproxy, identitycertificatechain: *mut i8, manifestsxmls: *mut *mut i8, manifestscount: usize) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_updateidentity(proxy : alljoyn_securityapplicationproxy, identitycertificatechain : *mut i8, manifestsxmls : *mut *mut i8, manifestscount : usize) -> QStatus);
    alljoyn_securityapplicationproxy_updateidentity(core::mem::transmute(proxy), core::mem::transmute(identitycertificatechain), core::mem::transmute(manifestsxmls), core::mem::transmute(manifestscount))
}
#[inline]
pub unsafe fn alljoyn_securityapplicationproxy_updatepolicy(proxy: alljoyn_securityapplicationproxy, policyxml: *mut i8) -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_securityapplicationproxy_updatepolicy(proxy : alljoyn_securityapplicationproxy, policyxml : *mut i8) -> QStatus);
    alljoyn_securityapplicationproxy_updatepolicy(core::mem::transmute(proxy), core::mem::transmute(policyxml))
}
#[inline]
pub unsafe fn alljoyn_sessionlistener_create(callbacks: *const alljoyn_sessionlistener_callbacks, context: *const core::ffi::c_void) -> alljoyn_sessionlistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionlistener_create(callbacks : *const alljoyn_sessionlistener_callbacks, context : *const core::ffi::c_void) -> alljoyn_sessionlistener);
    alljoyn_sessionlistener_create(core::mem::transmute(callbacks), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_sessionlistener_destroy(listener: alljoyn_sessionlistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionlistener_destroy(listener : alljoyn_sessionlistener));
    alljoyn_sessionlistener_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_cmp(one: alljoyn_sessionopts, other: alljoyn_sessionopts) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_cmp(one : alljoyn_sessionopts, other : alljoyn_sessionopts) -> i32);
    alljoyn_sessionopts_cmp(core::mem::transmute(one), core::mem::transmute(other))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_create(traffic: u8, ismultipoint: i32, proximity: u8, transports: u16) -> alljoyn_sessionopts {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_create(traffic : u8, ismultipoint : i32, proximity : u8, transports : u16) -> alljoyn_sessionopts);
    alljoyn_sessionopts_create(core::mem::transmute(traffic), core::mem::transmute(ismultipoint), core::mem::transmute(proximity), core::mem::transmute(transports))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_destroy(opts: alljoyn_sessionopts) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_destroy(opts : alljoyn_sessionopts));
    alljoyn_sessionopts_destroy(core::mem::transmute(opts))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_get_multipoint(opts: alljoyn_sessionopts) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_get_multipoint(opts : alljoyn_sessionopts) -> i32);
    alljoyn_sessionopts_get_multipoint(core::mem::transmute(opts))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_get_proximity(opts: alljoyn_sessionopts) -> u8 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_get_proximity(opts : alljoyn_sessionopts) -> u8);
    alljoyn_sessionopts_get_proximity(core::mem::transmute(opts))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_get_traffic(opts: alljoyn_sessionopts) -> u8 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_get_traffic(opts : alljoyn_sessionopts) -> u8);
    alljoyn_sessionopts_get_traffic(core::mem::transmute(opts))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_get_transports(opts: alljoyn_sessionopts) -> u16 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_get_transports(opts : alljoyn_sessionopts) -> u16);
    alljoyn_sessionopts_get_transports(core::mem::transmute(opts))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_iscompatible(one: alljoyn_sessionopts, other: alljoyn_sessionopts) -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_iscompatible(one : alljoyn_sessionopts, other : alljoyn_sessionopts) -> i32);
    alljoyn_sessionopts_iscompatible(core::mem::transmute(one), core::mem::transmute(other))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_set_multipoint(opts: alljoyn_sessionopts, ismultipoint: i32) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_set_multipoint(opts : alljoyn_sessionopts, ismultipoint : i32));
    alljoyn_sessionopts_set_multipoint(core::mem::transmute(opts), core::mem::transmute(ismultipoint))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_set_proximity(opts: alljoyn_sessionopts, proximity: u8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_set_proximity(opts : alljoyn_sessionopts, proximity : u8));
    alljoyn_sessionopts_set_proximity(core::mem::transmute(opts), core::mem::transmute(proximity))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_set_traffic(opts: alljoyn_sessionopts, traffic: u8) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_set_traffic(opts : alljoyn_sessionopts, traffic : u8));
    alljoyn_sessionopts_set_traffic(core::mem::transmute(opts), core::mem::transmute(traffic))
}
#[inline]
pub unsafe fn alljoyn_sessionopts_set_transports(opts: alljoyn_sessionopts, transports: u16) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionopts_set_transports(opts : alljoyn_sessionopts, transports : u16));
    alljoyn_sessionopts_set_transports(core::mem::transmute(opts), core::mem::transmute(transports))
}
#[inline]
pub unsafe fn alljoyn_sessionportlistener_create(callbacks: *const alljoyn_sessionportlistener_callbacks, context: *const core::ffi::c_void) -> alljoyn_sessionportlistener {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionportlistener_create(callbacks : *const alljoyn_sessionportlistener_callbacks, context : *const core::ffi::c_void) -> alljoyn_sessionportlistener);
    alljoyn_sessionportlistener_create(core::mem::transmute(callbacks), core::mem::transmute(context))
}
#[inline]
pub unsafe fn alljoyn_sessionportlistener_destroy(listener: alljoyn_sessionportlistener) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_sessionportlistener_destroy(listener : alljoyn_sessionportlistener));
    alljoyn_sessionportlistener_destroy(core::mem::transmute(listener))
}
#[inline]
pub unsafe fn alljoyn_shutdown() -> QStatus {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_shutdown() -> QStatus);
    alljoyn_shutdown()
}
#[inline]
pub unsafe fn alljoyn_unity_deferred_callbacks_process() -> i32 {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_unity_deferred_callbacks_process() -> i32);
    alljoyn_unity_deferred_callbacks_process()
}
#[inline]
pub unsafe fn alljoyn_unity_set_deferred_callback_mainthread_only(mainthread_only: i32) {
    windows_targets::link!("msajapi.dll" "system" fn alljoyn_unity_set_deferred_callback_mainthread_only(mainthread_only : i32));
    alljoyn_unity_set_deferred_callback_mainthread_only(core::mem::transmute(mainthread_only))
}
pub const AJ_IFC_SECURITY_INHERIT: alljoyn_interfacedescription_securitypolicy = alljoyn_interfacedescription_securitypolicy(0i32);
pub const AJ_IFC_SECURITY_OFF: alljoyn_interfacedescription_securitypolicy = alljoyn_interfacedescription_securitypolicy(2i32);
pub const AJ_IFC_SECURITY_REQUIRED: alljoyn_interfacedescription_securitypolicy = alljoyn_interfacedescription_securitypolicy(1i32);
pub const ALLJOYN_ARRAY: alljoyn_typeid = alljoyn_typeid(97i32);
pub const ALLJOYN_BIG_ENDIAN: u8 = 66u8;
pub const ALLJOYN_BOOLEAN: alljoyn_typeid = alljoyn_typeid(98i32);
pub const ALLJOYN_BOOLEAN_ARRAY: alljoyn_typeid = alljoyn_typeid(25185i32);
pub const ALLJOYN_BYTE: alljoyn_typeid = alljoyn_typeid(121i32);
pub const ALLJOYN_BYTE_ARRAY: alljoyn_typeid = alljoyn_typeid(31073i32);
pub const ALLJOYN_CRED_CERT_CHAIN: u16 = 4u16;
pub const ALLJOYN_CRED_EXPIRATION: u16 = 32u16;
pub const ALLJOYN_CRED_LOGON_ENTRY: u16 = 16u16;
pub const ALLJOYN_CRED_NEW_PASSWORD: u16 = 4097u16;
pub const ALLJOYN_CRED_ONE_TIME_PWD: u16 = 8193u16;
pub const ALLJOYN_CRED_PASSWORD: u16 = 1u16;
pub const ALLJOYN_CRED_PRIVATE_KEY: u16 = 8u16;
pub const ALLJOYN_CRED_USER_NAME: u16 = 2u16;
pub const ALLJOYN_DICT_ENTRY: alljoyn_typeid = alljoyn_typeid(101i32);
pub const ALLJOYN_DICT_ENTRY_CLOSE: alljoyn_typeid = alljoyn_typeid(125i32);
pub const ALLJOYN_DICT_ENTRY_OPEN: alljoyn_typeid = alljoyn_typeid(123i32);
pub const ALLJOYN_DISCONNECTED: u32 = 4u32;
pub const ALLJOYN_DOUBLE: alljoyn_typeid = alljoyn_typeid(100i32);
pub const ALLJOYN_DOUBLE_ARRAY: alljoyn_typeid = alljoyn_typeid(25697i32);
pub const ALLJOYN_HANDLE: alljoyn_typeid = alljoyn_typeid(104i32);
pub const ALLJOYN_INT16: alljoyn_typeid = alljoyn_typeid(110i32);
pub const ALLJOYN_INT16_ARRAY: alljoyn_typeid = alljoyn_typeid(28257i32);
pub const ALLJOYN_INT32: alljoyn_typeid = alljoyn_typeid(105i32);
pub const ALLJOYN_INT32_ARRAY: alljoyn_typeid = alljoyn_typeid(26977i32);
pub const ALLJOYN_INT64: alljoyn_typeid = alljoyn_typeid(120i32);
pub const ALLJOYN_INT64_ARRAY: alljoyn_typeid = alljoyn_typeid(30817i32);
pub const ALLJOYN_INVALID: alljoyn_typeid = alljoyn_typeid(0i32);
pub const ALLJOYN_LITTLE_ENDIAN: u8 = 108u8;
pub const ALLJOYN_MEMBER_ANNOTATE_DEPRECATED: u8 = 2u8;
pub const ALLJOYN_MEMBER_ANNOTATE_GLOBAL_BROADCAST: u8 = 32u8;
pub const ALLJOYN_MEMBER_ANNOTATE_NO_REPLY: u8 = 1u8;
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONCAST: u8 = 4u8;
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONLESS: u8 = 8u8;
pub const ALLJOYN_MEMBER_ANNOTATE_UNICAST: u8 = 16u8;
pub const ALLJOYN_MESSAGE_DEFAULT_TIMEOUT: u32 = 25000u32;
pub const ALLJOYN_MESSAGE_ERROR: alljoyn_messagetype = alljoyn_messagetype(3i32);
pub const ALLJOYN_MESSAGE_FLAG_ALLOW_REMOTE_MSG: u32 = 4u32;
pub const ALLJOYN_MESSAGE_FLAG_AUTO_START: u32 = 2u32;
pub const ALLJOYN_MESSAGE_FLAG_ENCRYPTED: u32 = 128u32;
pub const ALLJOYN_MESSAGE_FLAG_GLOBAL_BROADCAST: u32 = 32u32;
pub const ALLJOYN_MESSAGE_FLAG_NO_REPLY_EXPECTED: u32 = 1u32;
pub const ALLJOYN_MESSAGE_FLAG_SESSIONLESS: u32 = 16u32;
pub const ALLJOYN_MESSAGE_INVALID: alljoyn_messagetype = alljoyn_messagetype(0i32);
pub const ALLJOYN_MESSAGE_METHOD_CALL: alljoyn_messagetype = alljoyn_messagetype(1i32);
pub const ALLJOYN_MESSAGE_METHOD_RET: alljoyn_messagetype = alljoyn_messagetype(2i32);
pub const ALLJOYN_MESSAGE_SIGNAL: alljoyn_messagetype = alljoyn_messagetype(4i32);
pub const ALLJOYN_NAMED_PIPE_CONNECT_SPEC: windows_core::PCWSTR = windows_core::w!("npipe:");
pub const ALLJOYN_OBJECT_PATH: alljoyn_typeid = alljoyn_typeid(111i32);
pub const ALLJOYN_PROP_ACCESS_READ: u8 = 1u8;
pub const ALLJOYN_PROP_ACCESS_RW: u8 = 3u8;
pub const ALLJOYN_PROP_ACCESS_WRITE: u8 = 2u8;
pub const ALLJOYN_PROXIMITY_ANY: u32 = 255u32;
pub const ALLJOYN_PROXIMITY_NETWORK: u32 = 2u32;
pub const ALLJOYN_PROXIMITY_PHYSICAL: u32 = 1u32;
pub const ALLJOYN_READ_READY: u32 = 1u32;
pub const ALLJOYN_SESSIONLOST_INVALID: alljoyn_sessionlostreason = alljoyn_sessionlostreason(0i32);
pub const ALLJOYN_SESSIONLOST_LINK_TIMEOUT: alljoyn_sessionlostreason = alljoyn_sessionlostreason(4i32);
pub const ALLJOYN_SESSIONLOST_REASON_OTHER: alljoyn_sessionlostreason = alljoyn_sessionlostreason(5i32);
pub const ALLJOYN_SESSIONLOST_REMOTE_END_CLOSED_ABRUPTLY: alljoyn_sessionlostreason = alljoyn_sessionlostreason(2i32);
pub const ALLJOYN_SESSIONLOST_REMOTE_END_LEFT_SESSION: alljoyn_sessionlostreason = alljoyn_sessionlostreason(1i32);
pub const ALLJOYN_SESSIONLOST_REMOVED_BY_BINDER: alljoyn_sessionlostreason = alljoyn_sessionlostreason(3i32);
pub const ALLJOYN_SIGNATURE: alljoyn_typeid = alljoyn_typeid(103i32);
pub const ALLJOYN_STRING: alljoyn_typeid = alljoyn_typeid(115i32);
pub const ALLJOYN_STRUCT: alljoyn_typeid = alljoyn_typeid(114i32);
pub const ALLJOYN_STRUCT_CLOSE: alljoyn_typeid = alljoyn_typeid(41i32);
pub const ALLJOYN_STRUCT_OPEN: alljoyn_typeid = alljoyn_typeid(40i32);
pub const ALLJOYN_TRAFFIC_TYPE_MESSAGES: u32 = 1u32;
pub const ALLJOYN_TRAFFIC_TYPE_RAW_RELIABLE: u32 = 4u32;
pub const ALLJOYN_TRAFFIC_TYPE_RAW_UNRELIABLE: u32 = 2u32;
pub const ALLJOYN_UINT16: alljoyn_typeid = alljoyn_typeid(113i32);
pub const ALLJOYN_UINT16_ARRAY: alljoyn_typeid = alljoyn_typeid(29025i32);
pub const ALLJOYN_UINT32: alljoyn_typeid = alljoyn_typeid(117i32);
pub const ALLJOYN_UINT32_ARRAY: alljoyn_typeid = alljoyn_typeid(30049i32);
pub const ALLJOYN_UINT64: alljoyn_typeid = alljoyn_typeid(116i32);
pub const ALLJOYN_UINT64_ARRAY: alljoyn_typeid = alljoyn_typeid(29793i32);
pub const ALLJOYN_VARIANT: alljoyn_typeid = alljoyn_typeid(118i32);
pub const ALLJOYN_WILDCARD: alljoyn_typeid = alljoyn_typeid(42i32);
pub const ALLJOYN_WRITE_READY: u32 = 2u32;
pub const ANNOUNCED: alljoyn_about_announceflag = alljoyn_about_announceflag(1i32);
pub const CAPABLE_ECDHE_ECDSA: alljoyn_claimcapability_masks = alljoyn_claimcapability_masks(4i32);
pub const CAPABLE_ECDHE_NULL: alljoyn_claimcapability_masks = alljoyn_claimcapability_masks(1i32);
pub const CAPABLE_ECDHE_SPEKE: alljoyn_claimcapability_masks = alljoyn_claimcapability_masks(8i32);
pub const CLAIMABLE: alljoyn_applicationstate = alljoyn_applicationstate(1i32);
pub const CLAIMED: alljoyn_applicationstate = alljoyn_applicationstate(2i32);
pub const ER_ABOUT_ABOUTDATA_MISSING_REQUIRED_FIELD: QStatus = QStatus(37157i32);
pub const ER_ABOUT_DEFAULT_LANGUAGE_NOT_SPECIFIED: QStatus = QStatus(37155i32);
pub const ER_ABOUT_FIELD_ALREADY_SPECIFIED: QStatus = QStatus(37147i32);
pub const ER_ABOUT_INVALID_ABOUTDATA_FIELD_APPID_SIZE: QStatus = QStatus(37163i32);
pub const ER_ABOUT_INVALID_ABOUTDATA_FIELD_VALUE: QStatus = QStatus(37162i32);
pub const ER_ABOUT_INVALID_ABOUTDATA_LISTENER: QStatus = QStatus(37158i32);
pub const ER_ABOUT_SESSIONPORT_NOT_BOUND: QStatus = QStatus(37156i32);
pub const ER_ALERTED_THREAD: QStatus = QStatus(4098i32);
pub const ER_ALLJOYN_ACCESS_PERMISSION_ERROR: QStatus = QStatus(37028i32);
pub const ER_ALLJOYN_ACCESS_PERMISSION_WARNING: QStatus = QStatus(37027i32);
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_ALREADY_ADVERTISING: QStatus = QStatus(37005i32);
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_FAILED: QStatus = QStatus(37006i32);
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_TRANSPORT_NOT_AVAILABLE: QStatus = QStatus(37004i32);
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_ALREADY_EXISTS: QStatus = QStatus(36992i32);
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_FAILED: QStatus = QStatus(36993i32);
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_INVALID_OPTS: QStatus = QStatus(37018i32);
pub const ER_ALLJOYN_CANCELADVERTISENAME_REPLY_FAILED: QStatus = QStatus(37008i32);
pub const ER_ALLJOYN_CANCELFINDADVERTISEDNAME_REPLY_FAILED: QStatus = QStatus(37013i32);
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_ALREADY_DISCOVERING: QStatus = QStatus(37010i32);
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_FAILED: QStatus = QStatus(37011i32);
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_TRANSPORT_NOT_AVAILABLE: QStatus = QStatus(37009i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_ALREADY_JOINED: QStatus = QStatus(37019i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_BAD_SESSION_OPTS: QStatus = QStatus(36999i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_CONNECT_FAILED: QStatus = QStatus(36997i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_FAILED: QStatus = QStatus(37000i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_NO_SESSION: QStatus = QStatus(36995i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_REJECTED: QStatus = QStatus(36998i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_UNREACHABLE: QStatus = QStatus(36996i32);
pub const ER_ALLJOYN_LEAVESESSION_REPLY_FAILED: QStatus = QStatus(37003i32);
pub const ER_ALLJOYN_LEAVESESSION_REPLY_NO_SESSION: QStatus = QStatus(37002i32);
pub const ER_ALLJOYN_ONAPPRESUME_REPLY_FAILED: QStatus = QStatus(37100i32);
pub const ER_ALLJOYN_ONAPPRESUME_REPLY_UNSUPPORTED: QStatus = QStatus(37101i32);
pub const ER_ALLJOYN_ONAPPSUSPEND_REPLY_FAILED: QStatus = QStatus(37098i32);
pub const ER_ALLJOYN_ONAPPSUSPEND_REPLY_UNSUPPORTED: QStatus = QStatus(37099i32);
pub const ER_ALLJOYN_PING_FAILED: QStatus = QStatus(37111i32);
pub const ER_ALLJOYN_PING_REPLY_FAILED: QStatus = QStatus(37143i32);
pub const ER_ALLJOYN_PING_REPLY_INCOMPATIBLE_REMOTE_ROUTING_NODE: QStatus = QStatus(37140i32);
pub const ER_ALLJOYN_PING_REPLY_IN_PROGRESS: QStatus = QStatus(37145i32);
pub const ER_ALLJOYN_PING_REPLY_TIMEOUT: QStatus = QStatus(37141i32);
pub const ER_ALLJOYN_PING_REPLY_UNKNOWN_NAME: QStatus = QStatus(37142i32);
pub const ER_ALLJOYN_PING_REPLY_UNREACHABLE: QStatus = QStatus(37112i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_INCOMPATIBLE_REMOTE_DAEMON: QStatus = QStatus(37107i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_BINDER: QStatus = QStatus(37104i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_FOUND: QStatus = QStatus(37106i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_MULTIPOINT: QStatus = QStatus(37105i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_REPLY_FAILED: QStatus = QStatus(37108i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_REPLY_NO_SESSION: QStatus = QStatus(37103i32);
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_FAILED: QStatus = QStatus(37026i32);
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_NOT_SUPPORTED: QStatus = QStatus(37024i32);
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_NO_DEST_SUPPORT: QStatus = QStatus(37025i32);
pub const ER_ALLJOYN_UNBINDSESSIONPORT_REPLY_BAD_PORT: QStatus = QStatus(37016i32);
pub const ER_ALLJOYN_UNBINDSESSIONPORT_REPLY_FAILED: QStatus = QStatus(37017i32);
pub const ER_APPLICATION_STATE_LISTENER_ALREADY_EXISTS: QStatus = QStatus(37184i32);
pub const ER_APPLICATION_STATE_LISTENER_NO_SUCH_LISTENER: QStatus = QStatus(37185i32);
pub const ER_ARDP_BACKPRESSURE: QStatus = QStatus(37122i32);
pub const ER_ARDP_DISCONNECTING: QStatus = QStatus(37139i32);
pub const ER_ARDP_INVALID_CONNECTION: QStatus = QStatus(37135i32);
pub const ER_ARDP_INVALID_RESPONSE: QStatus = QStatus(37134i32);
pub const ER_ARDP_INVALID_STATE: QStatus = QStatus(37124i32);
pub const ER_ARDP_PERSIST_TIMEOUT: QStatus = QStatus(37126i32);
pub const ER_ARDP_PROBE_TIMEOUT: QStatus = QStatus(37127i32);
pub const ER_ARDP_REMOTE_CONNECTION_RESET: QStatus = QStatus(37128i32);
pub const ER_ARDP_TTL_EXPIRED: QStatus = QStatus(37125i32);
pub const ER_ARDP_VERSION_NOT_SUPPORTED: QStatus = QStatus(37151i32);
pub const ER_ARDP_WRITE_BLOCKED: QStatus = QStatus(37153i32);
pub const ER_AUTH_FAIL: QStatus = QStatus(4100i32);
pub const ER_AUTH_USER_REJECT: QStatus = QStatus(4101i32);
pub const ER_BAD_ARG_1: QStatus = QStatus(12i32);
pub const ER_BAD_ARG_2: QStatus = QStatus(13i32);
pub const ER_BAD_ARG_3: QStatus = QStatus(14i32);
pub const ER_BAD_ARG_4: QStatus = QStatus(15i32);
pub const ER_BAD_ARG_5: QStatus = QStatus(16i32);
pub const ER_BAD_ARG_6: QStatus = QStatus(17i32);
pub const ER_BAD_ARG_7: QStatus = QStatus(18i32);
pub const ER_BAD_ARG_8: QStatus = QStatus(19i32);
pub const ER_BAD_ARG_COUNT: QStatus = QStatus(28i32);
pub const ER_BAD_HOSTNAME: QStatus = QStatus(4112i32);
pub const ER_BAD_STRING_ENCODING: QStatus = QStatus(4120i32);
pub const ER_BAD_TRANSPORT_MASK: QStatus = QStatus(37088i32);
pub const ER_BUFFER_TOO_SMALL: QStatus = QStatus(3i32);
pub const ER_BUS_ALREADY_CONNECTED: QStatus = QStatus(36932i32);
pub const ER_BUS_ALREADY_LISTENING: QStatus = QStatus(36934i32);
pub const ER_BUS_ANNOTATION_ALREADY_EXISTS: QStatus = QStatus(37082i32);
pub const ER_BUS_AUTHENTICATION_PENDING: QStatus = QStatus(37031i32);
pub const ER_BUS_BAD_BODY_LEN: QStatus = QStatus(36879i32);
pub const ER_BUS_BAD_BUS_NAME: QStatus = QStatus(36874i32);
pub const ER_BUS_BAD_CHILD_PATH: QStatus = QStatus(36927i32);
pub const ER_BUS_BAD_ERROR_NAME: QStatus = QStatus(36873i32);
pub const ER_BUS_BAD_HDR_FLAGS: QStatus = QStatus(36878i32);
pub const ER_BUS_BAD_HEADER_FIELD: QStatus = QStatus(36868i32);
pub const ER_BUS_BAD_HEADER_LEN: QStatus = QStatus(36880i32);
pub const ER_BUS_BAD_INTERFACE_NAME: QStatus = QStatus(36872i32);
pub const ER_BUS_BAD_LENGTH: QStatus = QStatus(36876i32);
pub const ER_BUS_BAD_MEMBER_NAME: QStatus = QStatus(36871i32);
pub const ER_BUS_BAD_OBJ_PATH: QStatus = QStatus(36870i32);
pub const ER_BUS_BAD_SENDER_ID: QStatus = QStatus(36908i32);
pub const ER_BUS_BAD_SEND_PARAMETER: QStatus = QStatus(36906i32);
pub const ER_BUS_BAD_SESSION_OPTS: QStatus = QStatus(36980i32);
pub const ER_BUS_BAD_SIGNATURE: QStatus = QStatus(36869i32);
pub const ER_BUS_BAD_TRANSPORT_ARGS: QStatus = QStatus(36903i32);
pub const ER_BUS_BAD_VALUE: QStatus = QStatus(36877i32);
pub const ER_BUS_BAD_VALUE_TYPE: QStatus = QStatus(36867i32);
pub const ER_BUS_BAD_XML: QStatus = QStatus(36926i32);
pub const ER_BUS_BLOCKING_CALL_NOT_ALLOWED: QStatus = QStatus(36960i32);
pub const ER_BUS_BUS_ALREADY_STARTED: QStatus = QStatus(36939i32);
pub const ER_BUS_BUS_NOT_STARTED: QStatus = QStatus(36940i32);
pub const ER_BUS_CANNOT_ADD_HANDLER: QStatus = QStatus(36965i32);
pub const ER_BUS_CANNOT_ADD_INTERFACE: QStatus = QStatus(36964i32);
pub const ER_BUS_CANNOT_EXPAND_MESSAGE: QStatus = QStatus(36930i32);
pub const ER_BUS_CONNECTION_REJECTED: QStatus = QStatus(36981i32);
pub const ER_BUS_CONNECT_FAILED: QStatus = QStatus(36913i32);
pub const ER_BUS_CORRUPT_KEYSTORE: QStatus = QStatus(36952i32);
pub const ER_BUS_DESCRIPTION_ALREADY_EXISTS: QStatus = QStatus(37188i32);
pub const ER_BUS_DESTINATION_NOT_AUTHENTICATED: QStatus = QStatus(37029i32);
pub const ER_BUS_ELEMENT_NOT_FOUND: QStatus = QStatus(36976i32);
pub const ER_BUS_EMPTY_MESSAGE: QStatus = QStatus(36910i32);
pub const ER_BUS_ENDPOINT_CLOSING: QStatus = QStatus(36920i32);
pub const ER_BUS_ENDPOINT_REDIRECTED: QStatus = QStatus(37030i32);
pub const ER_BUS_ERRORS: QStatus = QStatus(36864i32);
pub const ER_BUS_ERROR_NAME_MISSING: QStatus = QStatus(36890i32);
pub const ER_BUS_ERROR_RESPONSE: QStatus = QStatus(36925i32);
pub const ER_BUS_ESTABLISH_FAILED: QStatus = QStatus(36884i32);
pub const ER_BUS_HANDLES_MISMATCH: QStatus = QStatus(36973i32);
pub const ER_BUS_HANDLES_NOT_ENABLED: QStatus = QStatus(36972i32);
pub const ER_BUS_HDR_EXPANSION_INVALID: QStatus = QStatus(36946i32);
pub const ER_BUS_IFACE_ALREADY_EXISTS: QStatus = QStatus(36924i32);
pub const ER_BUS_INCOMPATIBLE_DAEMON: QStatus = QStatus(37094i32);
pub const ER_BUS_INTERFACE_ACTIVATED: QStatus = QStatus(37015i32);
pub const ER_BUS_INTERFACE_MISMATCH: QStatus = QStatus(36921i32);
pub const ER_BUS_INTERFACE_MISSING: QStatus = QStatus(36886i32);
pub const ER_BUS_INTERFACE_NO_SUCH_MEMBER: QStatus = QStatus(36891i32);
pub const ER_BUS_INVALID_AUTH_MECHANISM: QStatus = QStatus(36958i32);
pub const ER_BUS_INVALID_HEADER_CHECKSUM: QStatus = QStatus(36942i32);
pub const ER_BUS_INVALID_HEADER_SERIAL: QStatus = QStatus(36944i32);
pub const ER_BUS_KEYBLOB_OP_INVALID: QStatus = QStatus(36941i32);
pub const ER_BUS_KEYSTORE_NOT_LOADED: QStatus = QStatus(36966i32);
pub const ER_BUS_KEYSTORE_VERSION_MISMATCH: QStatus = QStatus(36959i32);
pub const ER_BUS_KEY_EXPIRED: QStatus = QStatus(36951i32);
pub const ER_BUS_KEY_STORE_NOT_LOADED: QStatus = QStatus(36937i32);
pub const ER_BUS_KEY_UNAVAILABLE: QStatus = QStatus(36935i32);
pub const ER_BUS_LISTENER_ALREADY_SET: QStatus = QStatus(37022i32);
pub const ER_BUS_MATCH_RULE_NOT_FOUND: QStatus = QStatus(37110i32);
pub const ER_BUS_MEMBER_ALREADY_EXISTS: QStatus = QStatus(36922i32);
pub const ER_BUS_MEMBER_MISSING: QStatus = QStatus(36888i32);
pub const ER_BUS_MEMBER_NO_SUCH_SIGNATURE: QStatus = QStatus(36896i32);
pub const ER_BUS_MESSAGE_DECRYPTION_FAILED: QStatus = QStatus(36949i32);
pub const ER_BUS_MESSAGE_NOT_ENCRYPTED: QStatus = QStatus(36943i32);
pub const ER_BUS_METHOD_CALL_ABORTED: QStatus = QStatus(36963i32);
pub const ER_BUS_MISSING_COMPRESSION_TOKEN: QStatus = QStatus(36947i32);
pub const ER_BUS_NAME_TOO_LONG: QStatus = QStatus(36875i32);
pub const ER_BUS_NOT_ALLOWED: QStatus = QStatus(36918i32);
pub const ER_BUS_NOT_AUTHENTICATING: QStatus = QStatus(36915i32);
pub const ER_BUS_NOT_AUTHORIZED: QStatus = QStatus(37032i32);
pub const ER_BUS_NOT_A_COMPLETE_TYPE: QStatus = QStatus(36954i32);
pub const ER_BUS_NOT_A_DICTIONARY: QStatus = QStatus(36977i32);
pub const ER_BUS_NOT_COMPRESSED: QStatus = QStatus(36931i32);
pub const ER_BUS_NOT_CONNECTED: QStatus = QStatus(36933i32);
pub const ER_BUS_NOT_NUL_TERMINATED: QStatus = QStatus(36897i32);
pub const ER_BUS_NOT_OWNER: QStatus = QStatus(36911i32);
pub const ER_BUS_NO_AUTHENTICATION_MECHANISM: QStatus = QStatus(36938i32);
pub const ER_BUS_NO_CALL_FOR_REPLY: QStatus = QStatus(36953i32);
pub const ER_BUS_NO_ENDPOINT: QStatus = QStatus(36905i32);
pub const ER_BUS_NO_LISTENER: QStatus = QStatus(36916i32);
pub const ER_BUS_NO_PEER_GUID: QStatus = QStatus(36948i32);
pub const ER_BUS_NO_ROUTE: QStatus = QStatus(36904i32);
pub const ER_BUS_NO_SESSION: QStatus = QStatus(36975i32);
pub const ER_BUS_NO_SUCH_ANNOTATION: QStatus = QStatus(37081i32);
pub const ER_BUS_NO_SUCH_HANDLE: QStatus = QStatus(36971i32);
pub const ER_BUS_NO_SUCH_INTERFACE: QStatus = QStatus(36895i32);
pub const ER_BUS_NO_SUCH_MESSAGE: QStatus = QStatus(37102i32);
pub const ER_BUS_NO_SUCH_OBJECT: QStatus = QStatus(36892i32);
pub const ER_BUS_NO_SUCH_PROPERTY: QStatus = QStatus(36898i32);
pub const ER_BUS_NO_SUCH_SERVICE: QStatus = QStatus(36956i32);
pub const ER_BUS_NO_TRANSPORTS: QStatus = QStatus(36902i32);
pub const ER_BUS_OBJECT_NOT_REGISTERED: QStatus = QStatus(37091i32);
pub const ER_BUS_OBJECT_NO_SUCH_INTERFACE: QStatus = QStatus(36894i32);
pub const ER_BUS_OBJECT_NO_SUCH_MEMBER: QStatus = QStatus(36893i32);
pub const ER_BUS_OBJ_ALREADY_EXISTS: QStatus = QStatus(36928i32);
pub const ER_BUS_OBJ_NOT_FOUND: QStatus = QStatus(36929i32);
pub const ER_BUS_PATH_MISSING: QStatus = QStatus(36887i32);
pub const ER_BUS_PEER_AUTH_VERSION_MISMATCH: QStatus = QStatus(37023i32);
pub const ER_BUS_PING_GROUP_NOT_FOUND: QStatus = QStatus(37159i32);
pub const ER_BUS_POLICY_VIOLATION: QStatus = QStatus(36955i32);
pub const ER_BUS_PROPERTY_ACCESS_DENIED: QStatus = QStatus(36901i32);
pub const ER_BUS_PROPERTY_ALREADY_EXISTS: QStatus = QStatus(36923i32);
pub const ER_BUS_PROPERTY_VALUE_NOT_SET: QStatus = QStatus(36900i32);
pub const ER_BUS_READ_ERROR: QStatus = QStatus(36865i32);
pub const ER_BUS_REMOVED_BY_BINDER: QStatus = QStatus(37109i32);
pub const ER_BUS_REMOVED_BY_BINDER_SELF: QStatus = QStatus(37160i32);
pub const ER_BUS_REPLY_IS_ERROR_MESSAGE: QStatus = QStatus(36914i32);
pub const ER_BUS_REPLY_SERIAL_MISSING: QStatus = QStatus(36889i32);
pub const ER_BUS_SECURITY_FATAL: QStatus = QStatus(36950i32);
pub const ER_BUS_SECURITY_NOT_ENABLED: QStatus = QStatus(37021i32);
pub const ER_BUS_SELF_CONNECT: QStatus = QStatus(37020i32);
pub const ER_BUS_SET_PROPERTY_REJECTED: QStatus = QStatus(36912i32);
pub const ER_BUS_SET_WRONG_SIGNATURE: QStatus = QStatus(36899i32);
pub const ER_BUS_SIGNATURE_MISMATCH: QStatus = QStatus(36961i32);
pub const ER_BUS_STOPPING: QStatus = QStatus(36962i32);
pub const ER_BUS_TIME_TO_LIVE_EXPIRED: QStatus = QStatus(36945i32);
pub const ER_BUS_TRANSPORT_ACCESS_DENIED: QStatus = QStatus(37164i32);
pub const ER_BUS_TRANSPORT_NOT_AVAILABLE: QStatus = QStatus(36957i32);
pub const ER_BUS_TRANSPORT_NOT_STARTED: QStatus = QStatus(36909i32);
pub const ER_BUS_TRUNCATED: QStatus = QStatus(36936i32);
pub const ER_BUS_UNEXPECTED_DISPOSITION: QStatus = QStatus(37014i32);
pub const ER_BUS_UNEXPECTED_SIGNATURE: QStatus = QStatus(36885i32);
pub const ER_BUS_UNKNOWN_INTERFACE: QStatus = QStatus(36883i32);
pub const ER_BUS_UNKNOWN_PATH: QStatus = QStatus(36882i32);
pub const ER_BUS_UNKNOWN_SERIAL: QStatus = QStatus(36881i32);
pub const ER_BUS_UNMATCHED_REPLY_SERIAL: QStatus = QStatus(36907i32);
pub const ER_BUS_WAIT_FAILED: QStatus = QStatus(36978i32);
pub const ER_BUS_WRITE_ERROR: QStatus = QStatus(36866i32);
pub const ER_BUS_WRITE_QUEUE_FULL: QStatus = QStatus(36919i32);
pub const ER_CERTIFICATE_NOT_FOUND: QStatus = QStatus(37166i32);
pub const ER_COMMON_ERRORS: QStatus = QStatus(4096i32);
pub const ER_CONNECTION_LIMIT_EXCEEDED: QStatus = QStatus(37152i32);
pub const ER_CONN_REFUSED: QStatus = QStatus(27i32);
pub const ER_CORRUPT_KEYBLOB: QStatus = QStatus(4115i32);
pub const ER_CRYPTO_ERROR: QStatus = QStatus(4109i32);
pub const ER_CRYPTO_HASH_UNINITIALIZED: QStatus = QStatus(4123i32);
pub const ER_CRYPTO_ILLEGAL_PARAMETERS: QStatus = QStatus(4122i32);
pub const ER_CRYPTO_INSUFFICIENT_SECURITY: QStatus = QStatus(4121i32);
pub const ER_CRYPTO_KEY_UNAVAILABLE: QStatus = QStatus(4111i32);
pub const ER_CRYPTO_KEY_UNUSABLE: QStatus = QStatus(4113i32);
pub const ER_CRYPTO_TRUNCATED: QStatus = QStatus(4110i32);
pub const ER_DBUS_RELEASE_NAME_REPLY_NON_EXISTENT: QStatus = QStatus(36987i32);
pub const ER_DBUS_RELEASE_NAME_REPLY_NOT_OWNER: QStatus = QStatus(36988i32);
pub const ER_DBUS_RELEASE_NAME_REPLY_RELEASED: QStatus = QStatus(36986i32);
pub const ER_DBUS_REQUEST_NAME_REPLY_ALREADY_OWNER: QStatus = QStatus(36985i32);
pub const ER_DBUS_REQUEST_NAME_REPLY_EXISTS: QStatus = QStatus(36984i32);
pub const ER_DBUS_REQUEST_NAME_REPLY_IN_QUEUE: QStatus = QStatus(36983i32);
pub const ER_DBUS_REQUEST_NAME_REPLY_PRIMARY_OWNER: QStatus = QStatus(36982i32);
pub const ER_DBUS_START_REPLY_ALREADY_RUNNING: QStatus = QStatus(36990i32);
pub const ER_DEADLOCK: QStatus = QStatus(31i32);
pub const ER_DEAD_THREAD: QStatus = QStatus(4117i32);
pub const ER_DIGEST_MISMATCH: QStatus = QStatus(37170i32);
pub const ER_DUPLICATE_CERTIFICATE: QStatus = QStatus(37167i32);
pub const ER_DUPLICATE_KEY: QStatus = QStatus(37171i32);
pub const ER_EMPTY_KEY_BLOB: QStatus = QStatus(4114i32);
pub const ER_END_OF_DATA: QStatus = QStatus(26i32);
pub const ER_EOF: QStatus = QStatus(30i32);
pub const ER_EXTERNAL_THREAD: QStatus = QStatus(4108i32);
pub const ER_FAIL: QStatus = QStatus(1i32);
pub const ER_FEATURE_NOT_AVAILABLE: QStatus = QStatus(37177i32);
pub const ER_INIT_FAILED: QStatus = QStatus(7i32);
pub const ER_INVALID_ADDRESS: QStatus = QStatus(20i32);
pub const ER_INVALID_APPLICATION_STATE: QStatus = QStatus(37176i32);
pub const ER_INVALID_CERTIFICATE: QStatus = QStatus(37165i32);
pub const ER_INVALID_CERTIFICATE_USAGE: QStatus = QStatus(37182i32);
pub const ER_INVALID_CERT_CHAIN: QStatus = QStatus(37174i32);
pub const ER_INVALID_CONFIG: QStatus = QStatus(37161i32);
pub const ER_INVALID_DATA: QStatus = QStatus(21i32);
pub const ER_INVALID_GUID: QStatus = QStatus(4126i32);
pub const ER_INVALID_HTTP_METHOD_USED_FOR_RENDEZVOUS_SERVER_INTERFACE_MESSAGE: QStatus = QStatus(37075i32);
pub const ER_INVALID_KEY_ENCODING: QStatus = QStatus(4116i32);
pub const ER_INVALID_ON_DEMAND_CONNECTION_MESSAGE_RESPONSE: QStatus = QStatus(37074i32);
pub const ER_INVALID_PERSISTENT_CONNECTION_MESSAGE_RESPONSE: QStatus = QStatus(37073i32);
pub const ER_INVALID_RENDEZVOUS_SERVER_INTERFACE_MESSAGE: QStatus = QStatus(37072i32);
pub const ER_INVALID_SIGNAL_EMISSION_TYPE: QStatus = QStatus(37183i32);
pub const ER_INVALID_STREAM: QStatus = QStatus(4129i32);
pub const ER_IODISPATCH_STOPPING: QStatus = QStatus(4131i32);
pub const ER_KEY_STORE_ALREADY_INITIALIZED: QStatus = QStatus(37178i32);
pub const ER_KEY_STORE_ID_NOT_YET_SET: QStatus = QStatus(37179i32);
pub const ER_LANGUAGE_NOT_SUPPORTED: QStatus = QStatus(37146i32);
pub const ER_MANAGEMENT_ALREADY_STARTED: QStatus = QStatus(37186i32);
pub const ER_MANAGEMENT_NOT_STARTED: QStatus = QStatus(37187i32);
pub const ER_MANIFEST_NOT_FOUND: QStatus = QStatus(37173i32);
pub const ER_MANIFEST_REJECTED: QStatus = QStatus(37181i32);
pub const ER_MISSING_DIGEST_IN_CERTIFICATE: QStatus = QStatus(37169i32);
pub const ER_NONE: QStatus = QStatus(65535i32);
pub const ER_NOT_CONN: QStatus = QStatus(4141i32);
pub const ER_NOT_CONNECTED_TO_RENDEZVOUS_SERVER: QStatus = QStatus(37070i32);
pub const ER_NOT_IMPLEMENTED: QStatus = QStatus(9i32);
pub const ER_NO_COMMON_TRUST: QStatus = QStatus(37172i32);
pub const ER_NO_SUCH_ALARM: QStatus = QStatus(4102i32);
pub const ER_NO_SUCH_DEVICE: QStatus = QStatus(37084i32);
pub const ER_NO_TRUST_ANCHOR: QStatus = QStatus(37175i32);
pub const ER_OK: QStatus = QStatus(0i32);
pub const ER_OPEN_FAILED: QStatus = QStatus(24i32);
pub const ER_OS_ERROR: QStatus = QStatus(4i32);
pub const ER_OUT_OF_MEMORY: QStatus = QStatus(5i32);
pub const ER_P2P: QStatus = QStatus(37085i32);
pub const ER_P2P_BUSY: QStatus = QStatus(37093i32);
pub const ER_P2P_DISABLED: QStatus = QStatus(37092i32);
pub const ER_P2P_FORBIDDEN: QStatus = QStatus(37097i32);
pub const ER_P2P_NOT_CONNECTED: QStatus = QStatus(37087i32);
pub const ER_P2P_NO_GO: QStatus = QStatus(37095i32);
pub const ER_P2P_NO_STA: QStatus = QStatus(37096i32);
pub const ER_P2P_TIMEOUT: QStatus = QStatus(37086i32);
pub const ER_PACKET_BAD_CRC: QStatus = QStatus(37039i32);
pub const ER_PACKET_BAD_FORMAT: QStatus = QStatus(37034i32);
pub const ER_PACKET_BAD_PARAMETER: QStatus = QStatus(37038i32);
pub const ER_PACKET_BUS_NO_SUCH_CHANNEL: QStatus = QStatus(37033i32);
pub const ER_PACKET_CHANNEL_FAIL: QStatus = QStatus(37036i32);
pub const ER_PACKET_CONNECT_TIMEOUT: QStatus = QStatus(37035i32);
pub const ER_PACKET_TOO_LARGE: QStatus = QStatus(37037i32);
pub const ER_PARSE_ERROR: QStatus = QStatus(25i32);
pub const ER_PERMISSION_DENIED: QStatus = QStatus(37154i32);
pub const ER_POLICY_NOT_NEWER: QStatus = QStatus(37180i32);
pub const ER_PROXIMITY_CONNECTION_ESTABLISH_FAIL: QStatus = QStatus(37089i32);
pub const ER_PROXIMITY_NO_PEERS_FOUND: QStatus = QStatus(37090i32);
pub const ER_READ_ERROR: QStatus = QStatus(22i32);
pub const ER_RENDEZVOUS_SERVER_DEACTIVATED_USER: QStatus = QStatus(37067i32);
pub const ER_RENDEZVOUS_SERVER_ERR401_UNAUTHORIZED_REQUEST: QStatus = QStatus(37078i32);
pub const ER_RENDEZVOUS_SERVER_ERR500_INTERNAL_ERROR: QStatus = QStatus(37076i32);
pub const ER_RENDEZVOUS_SERVER_ERR503_STATUS_UNAVAILABLE: QStatus = QStatus(37077i32);
pub const ER_RENDEZVOUS_SERVER_ROOT_CERTIFICATE_UNINITIALIZED: QStatus = QStatus(37080i32);
pub const ER_RENDEZVOUS_SERVER_UNKNOWN_USER: QStatus = QStatus(37068i32);
pub const ER_RENDEZVOUS_SERVER_UNRECOVERABLE_ERROR: QStatus = QStatus(37079i32);
pub const ER_SLAP_CRC_ERROR: QStatus = QStatus(4137i32);
pub const ER_SLAP_ERROR: QStatus = QStatus(4138i32);
pub const ER_SLAP_HDR_CHECKSUM_ERROR: QStatus = QStatus(4133i32);
pub const ER_SLAP_INVALID_PACKET_LEN: QStatus = QStatus(4132i32);
pub const ER_SLAP_INVALID_PACKET_TYPE: QStatus = QStatus(4134i32);
pub const ER_SLAP_LEN_MISMATCH: QStatus = QStatus(4135i32);
pub const ER_SLAP_OTHER_END_CLOSED: QStatus = QStatus(4139i32);
pub const ER_SLAP_PACKET_TYPE_MISMATCH: QStatus = QStatus(4136i32);
pub const ER_SOCKET_BIND_ERROR: QStatus = QStatus(6i32);
pub const ER_SOCK_CLOSING: QStatus = QStatus(37083i32);
pub const ER_SOCK_OTHER_END_CLOSED: QStatus = QStatus(11i32);
pub const ER_SSL_CONNECT: QStatus = QStatus(4106i32);
pub const ER_SSL_ERRORS: QStatus = QStatus(4104i32);
pub const ER_SSL_INIT: QStatus = QStatus(4105i32);
pub const ER_SSL_VERIFY: QStatus = QStatus(4107i32);
pub const ER_STOPPING_THREAD: QStatus = QStatus(4097i32);
pub const ER_TCP_MAX_UNTRUSTED: QStatus = QStatus(37144i32);
pub const ER_THREADPOOL_EXHAUSTED: QStatus = QStatus(4127i32);
pub const ER_THREADPOOL_STOPPING: QStatus = QStatus(4128i32);
pub const ER_THREAD_NO_WAIT: QStatus = QStatus(4124i32);
pub const ER_THREAD_RUNNING: QStatus = QStatus(4118i32);
pub const ER_THREAD_STOPPING: QStatus = QStatus(4119i32);
pub const ER_TIMEOUT: QStatus = QStatus(10i32);
pub const ER_TIMER_EXITING: QStatus = QStatus(4125i32);
pub const ER_TIMER_FALLBEHIND: QStatus = QStatus(4103i32);
pub const ER_TIMER_FULL: QStatus = QStatus(4130i32);
pub const ER_TIMER_NOT_ALLOWED: QStatus = QStatus(4140i32);
pub const ER_UDP_BACKPRESSURE: QStatus = QStatus(37123i32);
pub const ER_UDP_BUSHELLO: QStatus = QStatus(37129i32);
pub const ER_UDP_DEMUX_NO_ENDPOINT: QStatus = QStatus(37114i32);
pub const ER_UDP_DISCONNECT: QStatus = QStatus(37118i32);
pub const ER_UDP_EARLY_EXIT: QStatus = QStatus(37137i32);
pub const ER_UDP_ENDPOINT_NOT_STARTED: QStatus = QStatus(37149i32);
pub const ER_UDP_ENDPOINT_REMOVED: QStatus = QStatus(37150i32);
pub const ER_UDP_ENDPOINT_STALLED: QStatus = QStatus(37133i32);
pub const ER_UDP_INVALID: QStatus = QStatus(37131i32);
pub const ER_UDP_LOCAL_DISCONNECT: QStatus = QStatus(37136i32);
pub const ER_UDP_LOCAL_DISCONNECT_FAIL: QStatus = QStatus(37138i32);
pub const ER_UDP_MESSAGE: QStatus = QStatus(37130i32);
pub const ER_UDP_MSG_TOO_LONG: QStatus = QStatus(37113i32);
pub const ER_UDP_NOT_DISCONNECTED: QStatus = QStatus(37148i32);
pub const ER_UDP_NOT_IMPLEMENTED: QStatus = QStatus(37119i32);
pub const ER_UDP_NO_LISTENER: QStatus = QStatus(37120i32);
pub const ER_UDP_NO_NETWORK: QStatus = QStatus(37115i32);
pub const ER_UDP_STOPPING: QStatus = QStatus(37121i32);
pub const ER_UDP_UNEXPECTED_FLOW: QStatus = QStatus(37117i32);
pub const ER_UDP_UNEXPECTED_LENGTH: QStatus = QStatus(37116i32);
pub const ER_UDP_UNSUPPORTED: QStatus = QStatus(37132i32);
pub const ER_UNABLE_TO_CONNECT_TO_RENDEZVOUS_SERVER: QStatus = QStatus(37069i32);
pub const ER_UNABLE_TO_SEND_MESSAGE_TO_RENDEZVOUS_SERVER: QStatus = QStatus(37071i32);
pub const ER_UNKNOWN_CERTIFICATE: QStatus = QStatus(37168i32);
pub const ER_UTF_CONVERSION_FAILED: QStatus = QStatus(2i32);
pub const ER_WARNING: QStatus = QStatus(29i32);
pub const ER_WOULDBLOCK: QStatus = QStatus(8i32);
pub const ER_WRITE_ERROR: QStatus = QStatus(23i32);
pub const ER_XML_ACLS_MISSING: QStatus = QStatus(8211i32);
pub const ER_XML_ACL_ALL_TYPE_PEER_WITH_OTHERS: QStatus = QStatus(8207i32);
pub const ER_XML_ACL_PEERS_MISSING: QStatus = QStatus(8212i32);
pub const ER_XML_ACL_PEER_NOT_UNIQUE: QStatus = QStatus(8209i32);
pub const ER_XML_ACL_PEER_PUBLIC_KEY_SET: QStatus = QStatus(8210i32);
pub const ER_XML_ANNOTATION_NOT_UNIQUE: QStatus = QStatus(8222i32);
pub const ER_XML_CONVERTER_ERROR: QStatus = QStatus(8192i32);
pub const ER_XML_INTERFACE_MEMBERS_MISSING: QStatus = QStatus(8194i32);
pub const ER_XML_INTERFACE_NAME_NOT_UNIQUE: QStatus = QStatus(8219i32);
pub const ER_XML_INVALID_ACL_PEER_CHILDREN_COUNT: QStatus = QStatus(8206i32);
pub const ER_XML_INVALID_ACL_PEER_PUBLIC_KEY: QStatus = QStatus(8208i32);
pub const ER_XML_INVALID_ACL_PEER_TYPE: QStatus = QStatus(8205i32);
pub const ER_XML_INVALID_ANNOTATIONS_COUNT: QStatus = QStatus(8198i32);
pub const ER_XML_INVALID_ATTRIBUTE_VALUE: QStatus = QStatus(8200i32);
pub const ER_XML_INVALID_BASE64: QStatus = QStatus(8218i32);
pub const ER_XML_INVALID_ELEMENT_CHILDREN_COUNT: QStatus = QStatus(8202i32);
pub const ER_XML_INVALID_ELEMENT_NAME: QStatus = QStatus(8199i32);
pub const ER_XML_INVALID_INTERFACE_NAME: QStatus = QStatus(8214i32);
pub const ER_XML_INVALID_MANIFEST_VERSION: QStatus = QStatus(8216i32);
pub const ER_XML_INVALID_MEMBER_ACTION: QStatus = QStatus(8196i32);
pub const ER_XML_INVALID_MEMBER_NAME: QStatus = QStatus(8215i32);
pub const ER_XML_INVALID_MEMBER_TYPE: QStatus = QStatus(8195i32);
pub const ER_XML_INVALID_OBJECT_PATH: QStatus = QStatus(8213i32);
pub const ER_XML_INVALID_OID: QStatus = QStatus(8217i32);
pub const ER_XML_INVALID_POLICY_SERIAL_NUMBER: QStatus = QStatus(8204i32);
pub const ER_XML_INVALID_POLICY_VERSION: QStatus = QStatus(8203i32);
pub const ER_XML_INVALID_RULES_COUNT: QStatus = QStatus(8193i32);
pub const ER_XML_INVALID_SECURITY_LEVEL_ANNOTATION_VALUE: QStatus = QStatus(8201i32);
pub const ER_XML_MALFORMED: QStatus = QStatus(4099i32);
pub const ER_XML_MEMBER_DENY_ACTION_WITH_OTHER: QStatus = QStatus(8197i32);
pub const ER_XML_MEMBER_NAME_NOT_UNIQUE: QStatus = QStatus(8220i32);
pub const ER_XML_OBJECT_PATH_NOT_UNIQUE: QStatus = QStatus(8221i32);
pub const NEED_UPDATE: alljoyn_applicationstate = alljoyn_applicationstate(3i32);
pub const NOT_CLAIMABLE: alljoyn_applicationstate = alljoyn_applicationstate(0i32);
pub const PASSWORD_GENERATED_BY_APPLICATION: alljoyn_claimcapabilityadditionalinfo_masks = alljoyn_claimcapabilityadditionalinfo_masks(2i32);
pub const PASSWORD_GENERATED_BY_SECURITY_MANAGER: alljoyn_claimcapabilityadditionalinfo_masks = alljoyn_claimcapabilityadditionalinfo_masks(1i32);
pub const QCC_FALSE: u32 = 0u32;
pub const QCC_TRUE: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QStatus(pub i32);
pub const UNANNOUNCED: alljoyn_about_announceflag = alljoyn_about_announceflag(0i32);
pub type alljoyn_about_announced_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, busname: windows_core::PCSTR, version: u16, port: u16, objectdescriptionarg: alljoyn_msgarg, aboutdataarg: alljoyn_msgarg)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_about_announceflag(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutdata(pub isize);
impl windows_core::TypeKind for alljoyn_aboutdata {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutdatalistener(pub isize);
impl windows_core::TypeKind for alljoyn_aboutdatalistener {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_aboutdatalistener_callbacks {
    pub about_datalistener_getaboutdata: alljoyn_aboutdatalistener_getaboutdata_ptr,
    pub about_datalistener_getannouncedaboutdata: alljoyn_aboutdatalistener_getannouncedaboutdata_ptr,
}
impl Default for alljoyn_aboutdatalistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_aboutdatalistener_callbacks {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_aboutdatalistener_getaboutdata_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, msgarg: alljoyn_msgarg, language: windows_core::PCSTR) -> QStatus>;
pub type alljoyn_aboutdatalistener_getannouncedaboutdata_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, msgarg: alljoyn_msgarg) -> QStatus>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_abouticon(pub isize);
impl windows_core::TypeKind for alljoyn_abouticon {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_abouticonobj(pub isize);
impl windows_core::TypeKind for alljoyn_abouticonobj {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_abouticonproxy(pub isize);
impl windows_core::TypeKind for alljoyn_abouticonproxy {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutlistener(pub isize);
impl windows_core::TypeKind for alljoyn_aboutlistener {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_aboutlistener_callback {
    pub about_listener_announced: alljoyn_about_announced_ptr,
}
impl Default for alljoyn_aboutlistener_callback {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_aboutlistener_callback {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutobj(pub isize);
impl windows_core::TypeKind for alljoyn_aboutobj {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutobjectdescription(pub isize);
impl windows_core::TypeKind for alljoyn_aboutobjectdescription {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutproxy(pub isize);
impl windows_core::TypeKind for alljoyn_aboutproxy {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_applicationstate(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_applicationstatelistener(pub isize);
impl windows_core::TypeKind for alljoyn_applicationstatelistener {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_applicationstatelistener_callbacks {
    pub state: alljoyn_applicationstatelistener_state_ptr,
}
impl Default for alljoyn_applicationstatelistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_applicationstatelistener_callbacks {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_applicationstatelistener_state_ptr = Option<unsafe extern "system" fn(busname: *mut i8, publickey: *mut i8, applicationstate: alljoyn_applicationstate, context: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_authlistener(pub isize);
impl windows_core::TypeKind for alljoyn_authlistener {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_authlistener_authenticationcomplete_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, success: i32)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_authlistener_callbacks {
    pub request_credentials: alljoyn_authlistener_requestcredentials_ptr,
    pub verify_credentials: alljoyn_authlistener_verifycredentials_ptr,
    pub security_violation: alljoyn_authlistener_securityviolation_ptr,
    pub authentication_complete: alljoyn_authlistener_authenticationcomplete_ptr,
}
impl Default for alljoyn_authlistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_authlistener_callbacks {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_authlistener_requestcredentials_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, authcount: u16, username: windows_core::PCSTR, credmask: u16, credentials: alljoyn_credentials) -> i32>;
pub type alljoyn_authlistener_requestcredentialsasync_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_authlistener, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, authcount: u16, username: windows_core::PCSTR, credmask: u16, authcontext: *mut core::ffi::c_void) -> QStatus>;
pub type alljoyn_authlistener_securityviolation_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, status: QStatus, msg: alljoyn_message)>;
pub type alljoyn_authlistener_verifycredentials_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, credentials: alljoyn_credentials) -> i32>;
pub type alljoyn_authlistener_verifycredentialsasync_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_authlistener, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, credentials: alljoyn_credentials, authcontext: *mut core::ffi::c_void) -> QStatus>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_authlistenerasync_callbacks {
    pub request_credentials: alljoyn_authlistener_requestcredentialsasync_ptr,
    pub verify_credentials: alljoyn_authlistener_verifycredentialsasync_ptr,
    pub security_violation: alljoyn_authlistener_securityviolation_ptr,
    pub authentication_complete: alljoyn_authlistener_authenticationcomplete_ptr,
}
impl Default for alljoyn_authlistenerasync_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_authlistenerasync_callbacks {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_autopinger(pub isize);
impl windows_core::TypeKind for alljoyn_autopinger {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_autopinger_destination_found_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, group: windows_core::PCSTR, destination: windows_core::PCSTR)>;
pub type alljoyn_autopinger_destination_lost_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, group: windows_core::PCSTR, destination: windows_core::PCSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_busattachment(pub isize);
impl windows_core::TypeKind for alljoyn_busattachment {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_busattachment_joinsessioncb_ptr = Option<unsafe extern "system" fn(status: QStatus, sessionid: u32, opts: alljoyn_sessionopts, context: *mut core::ffi::c_void)>;
pub type alljoyn_busattachment_setlinktimeoutcb_ptr = Option<unsafe extern "system" fn(status: QStatus, timeout: u32, context: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_buslistener(pub isize);
impl windows_core::TypeKind for alljoyn_buslistener {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_buslistener_bus_disconnected_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_buslistener_bus_prop_changed_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, prop_name: windows_core::PCSTR, prop_value: alljoyn_msgarg)>;
pub type alljoyn_buslistener_bus_stopping_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_buslistener_callbacks {
    pub listener_registered: alljoyn_buslistener_listener_registered_ptr,
    pub listener_unregistered: alljoyn_buslistener_listener_unregistered_ptr,
    pub found_advertised_name: alljoyn_buslistener_found_advertised_name_ptr,
    pub lost_advertised_name: alljoyn_buslistener_lost_advertised_name_ptr,
    pub name_owner_changed: alljoyn_buslistener_name_owner_changed_ptr,
    pub bus_stopping: alljoyn_buslistener_bus_stopping_ptr,
    pub bus_disconnected: alljoyn_buslistener_bus_disconnected_ptr,
    pub property_changed: alljoyn_buslistener_bus_prop_changed_ptr,
}
impl Default for alljoyn_buslistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_buslistener_callbacks {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_buslistener_found_advertised_name_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, name: windows_core::PCSTR, transport: u16, nameprefix: windows_core::PCSTR)>;
pub type alljoyn_buslistener_listener_registered_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, bus: alljoyn_busattachment)>;
pub type alljoyn_buslistener_listener_unregistered_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_buslistener_lost_advertised_name_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, name: windows_core::PCSTR, transport: u16, nameprefix: windows_core::PCSTR)>;
pub type alljoyn_buslistener_name_owner_changed_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, busname: windows_core::PCSTR, previousowner: windows_core::PCSTR, newowner: windows_core::PCSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_busobject(pub isize);
impl windows_core::TypeKind for alljoyn_busobject {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_busobject_callbacks {
    pub property_get: alljoyn_busobject_prop_get_ptr,
    pub property_set: alljoyn_busobject_prop_set_ptr,
    pub object_registered: alljoyn_busobject_object_registration_ptr,
    pub object_unregistered: alljoyn_busobject_object_registration_ptr,
}
impl Default for alljoyn_busobject_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_busobject_callbacks {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_busobject_methodentry {
    pub member: *const alljoyn_interfacedescription_member,
    pub method_handler: alljoyn_messagereceiver_methodhandler_ptr,
}
impl Default for alljoyn_busobject_methodentry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_busobject_methodentry {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_busobject_object_registration_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_busobject_prop_get_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, ifcname: windows_core::PCSTR, propname: windows_core::PCSTR, val: alljoyn_msgarg) -> QStatus>;
pub type alljoyn_busobject_prop_set_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, ifcname: windows_core::PCSTR, propname: windows_core::PCSTR, val: alljoyn_msgarg) -> QStatus>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_certificateid {
    pub serial: *mut u8,
    pub serialLen: usize,
    pub issuerPublicKey: *mut i8,
    pub issuerAki: *mut u8,
    pub issuerAkiLen: usize,
}
impl Default for alljoyn_certificateid {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_certificateid {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_certificateidarray {
    pub count: usize,
    pub ids: *mut alljoyn_certificateid,
}
impl Default for alljoyn_certificateidarray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_certificateidarray {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_claimcapability_masks(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_claimcapabilityadditionalinfo_masks(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_credentials(pub isize);
impl windows_core::TypeKind for alljoyn_credentials {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_interfacedescription(pub isize);
impl windows_core::TypeKind for alljoyn_interfacedescription {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_interfacedescription_member {
    pub iface: alljoyn_interfacedescription,
    pub memberType: alljoyn_messagetype,
    pub name: windows_core::PCSTR,
    pub signature: windows_core::PCSTR,
    pub returnSignature: windows_core::PCSTR,
    pub argNames: windows_core::PCSTR,
    pub internal_member: *const core::ffi::c_void,
}
impl Default for alljoyn_interfacedescription_member {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_interfacedescription_member {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_interfacedescription_property {
    pub name: windows_core::PCSTR,
    pub signature: windows_core::PCSTR,
    pub access: u8,
    pub internal_property: *const core::ffi::c_void,
}
impl Default for alljoyn_interfacedescription_property {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_interfacedescription_property {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_interfacedescription_securitypolicy(pub i32);
pub type alljoyn_interfacedescription_translation_callback_ptr = Option<unsafe extern "system" fn(sourcelanguage: windows_core::PCSTR, targetlanguage: windows_core::PCSTR, sourcetext: windows_core::PCSTR) -> windows_core::PCSTR>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_keystore(pub isize);
impl windows_core::TypeKind for alljoyn_keystore {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_keystorelistener(pub isize);
impl windows_core::TypeKind for alljoyn_keystorelistener {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_keystorelistener_acquireexclusivelock_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener) -> QStatus>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_keystorelistener_callbacks {
    pub load_request: alljoyn_keystorelistener_loadrequest_ptr,
    pub store_request: alljoyn_keystorelistener_storerequest_ptr,
}
impl Default for alljoyn_keystorelistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_keystorelistener_callbacks {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_keystorelistener_loadrequest_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener, keystore: alljoyn_keystore) -> QStatus>;
pub type alljoyn_keystorelistener_releaseexclusivelock_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener)>;
pub type alljoyn_keystorelistener_storerequest_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener, keystore: alljoyn_keystore) -> QStatus>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_keystorelistener_with_synchronization_callbacks {
    pub load_request: alljoyn_keystorelistener_loadrequest_ptr,
    pub store_request: alljoyn_keystorelistener_storerequest_ptr,
    pub acquire_exclusive_lock: alljoyn_keystorelistener_acquireexclusivelock_ptr,
    pub release_exclusive_lock: alljoyn_keystorelistener_releaseexclusivelock_ptr,
}
impl Default for alljoyn_keystorelistener_with_synchronization_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_keystorelistener_with_synchronization_callbacks {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_manifestarray {
    pub count: usize,
    pub xmls: *mut *mut i8,
}
impl Default for alljoyn_manifestarray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_manifestarray {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_message(pub isize);
impl windows_core::TypeKind for alljoyn_message {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_messagereceiver_methodhandler_ptr = Option<unsafe extern "system" fn(bus: alljoyn_busobject, member: *const alljoyn_interfacedescription_member, message: alljoyn_message)>;
pub type alljoyn_messagereceiver_replyhandler_ptr = Option<unsafe extern "system" fn(message: alljoyn_message, context: *mut core::ffi::c_void)>;
pub type alljoyn_messagereceiver_signalhandler_ptr = Option<unsafe extern "system" fn(member: *const alljoyn_interfacedescription_member, srcpath: windows_core::PCSTR, message: alljoyn_message)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_messagetype(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_msgarg(pub isize);
impl windows_core::TypeKind for alljoyn_msgarg {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_observer(pub isize);
impl windows_core::TypeKind for alljoyn_observer {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_observer_object_discovered_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, proxyref: alljoyn_proxybusobject_ref)>;
pub type alljoyn_observer_object_lost_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, proxyref: alljoyn_proxybusobject_ref)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_observerlistener(pub isize);
impl windows_core::TypeKind for alljoyn_observerlistener {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_observerlistener_callback {
    pub object_discovered: alljoyn_observer_object_discovered_ptr,
    pub object_lost: alljoyn_observer_object_lost_ptr,
}
impl Default for alljoyn_observerlistener_callback {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_observerlistener_callback {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_permissionconfigurationlistener(pub isize);
impl windows_core::TypeKind for alljoyn_permissionconfigurationlistener {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_permissionconfigurationlistener_callbacks {
    pub factory_reset: alljoyn_permissionconfigurationlistener_factoryreset_ptr,
    pub policy_changed: alljoyn_permissionconfigurationlistener_policychanged_ptr,
    pub start_management: alljoyn_permissionconfigurationlistener_startmanagement_ptr,
    pub end_management: alljoyn_permissionconfigurationlistener_endmanagement_ptr,
}
impl Default for alljoyn_permissionconfigurationlistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_permissionconfigurationlistener_callbacks {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_permissionconfigurationlistener_endmanagement_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_permissionconfigurationlistener_factoryreset_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> QStatus>;
pub type alljoyn_permissionconfigurationlistener_policychanged_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_permissionconfigurationlistener_startmanagement_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_permissionconfigurator(pub isize);
impl windows_core::TypeKind for alljoyn_permissionconfigurator {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_pinglistener(pub isize);
impl windows_core::TypeKind for alljoyn_pinglistener {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_pinglistener_callback {
    pub destination_found: alljoyn_autopinger_destination_found_ptr,
    pub destination_lost: alljoyn_autopinger_destination_lost_ptr,
}
impl Default for alljoyn_pinglistener_callback {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_pinglistener_callback {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_proxybusobject(pub isize);
impl windows_core::TypeKind for alljoyn_proxybusobject {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_proxybusobject_listener_getallpropertiescb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, values: alljoyn_msgarg, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_getpropertycb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, value: alljoyn_msgarg, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_introspectcb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_propertieschanged_ptr = Option<unsafe extern "system" fn(obj: alljoyn_proxybusobject, ifacename: windows_core::PCSTR, changed: alljoyn_msgarg, invalidated: alljoyn_msgarg, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_setpropertycb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, context: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_proxybusobject_ref(pub isize);
impl windows_core::TypeKind for alljoyn_proxybusobject_ref {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_securityapplicationproxy(pub isize);
impl windows_core::TypeKind for alljoyn_securityapplicationproxy {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_sessionlistener(pub isize);
impl windows_core::TypeKind for alljoyn_sessionlistener {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_sessionlistener_callbacks {
    pub session_lost: alljoyn_sessionlistener_sessionlost_ptr,
    pub session_member_added: alljoyn_sessionlistener_sessionmemberadded_ptr,
    pub session_member_removed: alljoyn_sessionlistener_sessionmemberremoved_ptr,
}
impl Default for alljoyn_sessionlistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_sessionlistener_callbacks {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_sessionlistener_sessionlost_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionid: u32, reason: alljoyn_sessionlostreason)>;
pub type alljoyn_sessionlistener_sessionmemberadded_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionid: u32, uniquename: windows_core::PCSTR)>;
pub type alljoyn_sessionlistener_sessionmemberremoved_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionid: u32, uniquename: windows_core::PCSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_sessionlostreason(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_sessionopts(pub isize);
impl windows_core::TypeKind for alljoyn_sessionopts {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_sessionportlistener(pub isize);
impl windows_core::TypeKind for alljoyn_sessionportlistener {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_sessionportlistener_acceptsessionjoiner_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionport: u16, joiner: windows_core::PCSTR, opts: alljoyn_sessionopts) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_sessionportlistener_callbacks {
    pub accept_session_joiner: alljoyn_sessionportlistener_acceptsessionjoiner_ptr,
    pub session_joined: alljoyn_sessionportlistener_sessionjoined_ptr,
}
impl Default for alljoyn_sessionportlistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_sessionportlistener_callbacks {
    type TypeKind = windows_core::CopyType;
}
pub type alljoyn_sessionportlistener_sessionjoined_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionport: u16, id: u32, joiner: windows_core::PCSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_typeid(pub i32);
