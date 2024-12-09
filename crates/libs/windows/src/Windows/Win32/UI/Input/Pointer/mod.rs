#[inline]
pub unsafe fn EnableMouseInPointer<P0>(fenable: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn EnableMouseInPointer(fenable : super::super::super::Foundation:: BOOL) -> super::super::super::Foundation:: BOOL);
    EnableMouseInPointer(fenable.param().abi()).ok()
}
#[inline]
pub unsafe fn GetPointerCursorId(pointerid: u32, cursorid: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerCursorId(pointerid : u32, cursorid : *mut u32) -> super::super::super::Foundation:: BOOL);
    GetPointerCursorId(core::mem::transmute(pointerid), core::mem::transmute(cursorid)).ok()
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn GetPointerDevice(device: super::super::super::Foundation::HANDLE, pointerdevice: *mut super::super::Controls::POINTER_DEVICE_INFO) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerDevice(device : super::super::super::Foundation:: HANDLE, pointerdevice : *mut super::super::Controls:: POINTER_DEVICE_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerDevice(core::mem::transmute(device), core::mem::transmute(pointerdevice)).ok()
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn GetPointerDeviceCursors(device: super::super::super::Foundation::HANDLE, cursorcount: *mut u32, devicecursors: Option<*mut super::super::Controls::POINTER_DEVICE_CURSOR_INFO>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerDeviceCursors(device : super::super::super::Foundation:: HANDLE, cursorcount : *mut u32, devicecursors : *mut super::super::Controls:: POINTER_DEVICE_CURSOR_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerDeviceCursors(core::mem::transmute(device), core::mem::transmute(cursorcount), core::mem::transmute(devicecursors.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn GetPointerDeviceProperties(device: super::super::super::Foundation::HANDLE, propertycount: *mut u32, pointerproperties: Option<*mut super::super::Controls::POINTER_DEVICE_PROPERTY>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerDeviceProperties(device : super::super::super::Foundation:: HANDLE, propertycount : *mut u32, pointerproperties : *mut super::super::Controls:: POINTER_DEVICE_PROPERTY) -> super::super::super::Foundation:: BOOL);
    GetPointerDeviceProperties(core::mem::transmute(device), core::mem::transmute(propertycount), core::mem::transmute(pointerproperties.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn GetPointerDeviceRects(device: super::super::super::Foundation::HANDLE, pointerdevicerect: *mut super::super::super::Foundation::RECT, displayrect: *mut super::super::super::Foundation::RECT) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerDeviceRects(device : super::super::super::Foundation:: HANDLE, pointerdevicerect : *mut super::super::super::Foundation:: RECT, displayrect : *mut super::super::super::Foundation:: RECT) -> super::super::super::Foundation:: BOOL);
    GetPointerDeviceRects(core::mem::transmute(device), core::mem::transmute(pointerdevicerect), core::mem::transmute(displayrect)).ok()
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls"))]
#[inline]
pub unsafe fn GetPointerDevices(devicecount: *mut u32, pointerdevices: Option<*mut super::super::Controls::POINTER_DEVICE_INFO>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerDevices(devicecount : *mut u32, pointerdevices : *mut super::super::Controls:: POINTER_DEVICE_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerDevices(core::mem::transmute(devicecount), core::mem::transmute(pointerdevices.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerFrameInfo(pointerid: u32, pointercount: *mut u32, pointerinfo: Option<*mut POINTER_INFO>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerFrameInfo(pointerid : u32, pointercount : *mut u32, pointerinfo : *mut POINTER_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerFrameInfo(core::mem::transmute(pointerid), core::mem::transmute(pointercount), core::mem::transmute(pointerinfo.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerFrameInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, pointerinfo: Option<*mut POINTER_INFO>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerFrameInfoHistory(pointerid : u32, entriescount : *mut u32, pointercount : *mut u32, pointerinfo : *mut POINTER_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerFrameInfoHistory(core::mem::transmute(pointerid), core::mem::transmute(entriescount), core::mem::transmute(pointercount), core::mem::transmute(pointerinfo.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerFramePenInfo(pointerid: u32, pointercount: *mut u32, peninfo: Option<*mut POINTER_PEN_INFO>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerFramePenInfo(pointerid : u32, pointercount : *mut u32, peninfo : *mut POINTER_PEN_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerFramePenInfo(core::mem::transmute(pointerid), core::mem::transmute(pointercount), core::mem::transmute(peninfo.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerFramePenInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, peninfo: Option<*mut POINTER_PEN_INFO>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerFramePenInfoHistory(pointerid : u32, entriescount : *mut u32, pointercount : *mut u32, peninfo : *mut POINTER_PEN_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerFramePenInfoHistory(core::mem::transmute(pointerid), core::mem::transmute(entriescount), core::mem::transmute(pointercount), core::mem::transmute(peninfo.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerFrameTouchInfo(pointerid: u32, pointercount: *mut u32, touchinfo: Option<*mut POINTER_TOUCH_INFO>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerFrameTouchInfo(pointerid : u32, pointercount : *mut u32, touchinfo : *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerFrameTouchInfo(core::mem::transmute(pointerid), core::mem::transmute(pointercount), core::mem::transmute(touchinfo.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerFrameTouchInfoHistory(pointerid: u32, entriescount: *mut u32, pointercount: *mut u32, touchinfo: Option<*mut POINTER_TOUCH_INFO>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerFrameTouchInfoHistory(pointerid : u32, entriescount : *mut u32, pointercount : *mut u32, touchinfo : *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerFrameTouchInfoHistory(core::mem::transmute(pointerid), core::mem::transmute(entriescount), core::mem::transmute(pointercount), core::mem::transmute(touchinfo.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerInfo(pointerid: u32, pointerinfo: *mut POINTER_INFO) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerInfo(pointerid : u32, pointerinfo : *mut POINTER_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerInfo(core::mem::transmute(pointerid), core::mem::transmute(pointerinfo)).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerInfoHistory(pointerid: u32, entriescount: *mut u32, pointerinfo: Option<*mut POINTER_INFO>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerInfoHistory(pointerid : u32, entriescount : *mut u32, pointerinfo : *mut POINTER_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerInfoHistory(core::mem::transmute(pointerid), core::mem::transmute(entriescount), core::mem::transmute(pointerinfo.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn GetPointerInputTransform(pointerid: u32, inputtransform: &mut [INPUT_TRANSFORM]) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerInputTransform(pointerid : u32, historycount : u32, inputtransform : *mut INPUT_TRANSFORM) -> super::super::super::Foundation:: BOOL);
    GetPointerInputTransform(core::mem::transmute(pointerid), inputtransform.len().try_into().unwrap(), core::mem::transmute(inputtransform.as_ptr())).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerPenInfo(pointerid: u32, peninfo: *mut POINTER_PEN_INFO) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerPenInfo(pointerid : u32, peninfo : *mut POINTER_PEN_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerPenInfo(core::mem::transmute(pointerid), core::mem::transmute(peninfo)).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerPenInfoHistory(pointerid: u32, entriescount: *mut u32, peninfo: Option<*mut POINTER_PEN_INFO>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerPenInfoHistory(pointerid : u32, entriescount : *mut u32, peninfo : *mut POINTER_PEN_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerPenInfoHistory(core::mem::transmute(pointerid), core::mem::transmute(entriescount), core::mem::transmute(peninfo.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerTouchInfo(pointerid: u32, touchinfo: *mut POINTER_TOUCH_INFO) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerTouchInfo(pointerid : u32, touchinfo : *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerTouchInfo(core::mem::transmute(pointerid), core::mem::transmute(touchinfo)).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerTouchInfoHistory(pointerid: u32, entriescount: *mut u32, touchinfo: Option<*mut POINTER_TOUCH_INFO>) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerTouchInfoHistory(pointerid : u32, entriescount : *mut u32, touchinfo : *mut POINTER_TOUCH_INFO) -> super::super::super::Foundation:: BOOL);
    GetPointerTouchInfoHistory(core::mem::transmute(pointerid), core::mem::transmute(entriescount), core::mem::transmute(touchinfo.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn GetPointerType(pointerid: u32, pointertype: *mut super::super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetPointerType(pointerid : u32, pointertype : *mut super::super::WindowsAndMessaging:: POINTER_INPUT_TYPE) -> super::super::super::Foundation:: BOOL);
    GetPointerType(core::mem::transmute(pointerid), core::mem::transmute(pointertype)).ok()
}
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn GetRawPointerDeviceData(pointerid: u32, historycount: u32, pproperties: &[super::super::Controls::POINTER_DEVICE_PROPERTY], pvalues: *mut i32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn GetRawPointerDeviceData(pointerid : u32, historycount : u32, propertiescount : u32, pproperties : *const super::super::Controls:: POINTER_DEVICE_PROPERTY, pvalues : *mut i32) -> super::super::super::Foundation:: BOOL);
    GetRawPointerDeviceData(core::mem::transmute(pointerid), core::mem::transmute(historycount), pproperties.len().try_into().unwrap(), core::mem::transmute(pproperties.as_ptr()), core::mem::transmute(pvalues)).ok()
}
#[inline]
pub unsafe fn GetUnpredictedMessagePos() -> u32 {
    windows_targets::link!("user32.dll" "system" fn GetUnpredictedMessagePos() -> u32);
    GetUnpredictedMessagePos()
}
#[inline]
pub unsafe fn InitializeTouchInjection(maxcount: u32, dwmode: TOUCH_FEEDBACK_MODE) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn InitializeTouchInjection(maxcount : u32, dwmode : TOUCH_FEEDBACK_MODE) -> super::super::super::Foundation:: BOOL);
    InitializeTouchInjection(core::mem::transmute(maxcount), core::mem::transmute(dwmode)).ok()
}
#[cfg(all(feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
#[inline]
pub unsafe fn InjectSyntheticPointerInput(device: super::super::Controls::HSYNTHETICPOINTERDEVICE, pointerinfo: &[super::super::Controls::POINTER_TYPE_INFO]) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn InjectSyntheticPointerInput(device : super::super::Controls:: HSYNTHETICPOINTERDEVICE, pointerinfo : *const super::super::Controls:: POINTER_TYPE_INFO, count : u32) -> super::super::super::Foundation:: BOOL);
    InjectSyntheticPointerInput(core::mem::transmute(device), core::mem::transmute(pointerinfo.as_ptr()), pointerinfo.len().try_into().unwrap()).ok()
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn InjectTouchInput(contacts: &[POINTER_TOUCH_INFO]) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn InjectTouchInput(count : u32, contacts : *const POINTER_TOUCH_INFO) -> super::super::super::Foundation:: BOOL);
    InjectTouchInput(contacts.len().try_into().unwrap(), core::mem::transmute(contacts.as_ptr())).ok()
}
#[inline]
pub unsafe fn IsMouseInPointerEnabled() -> super::super::super::Foundation::BOOL {
    windows_targets::link!("user32.dll" "system" fn IsMouseInPointerEnabled() -> super::super::super::Foundation:: BOOL);
    IsMouseInPointerEnabled()
}
#[inline]
pub unsafe fn SkipPointerFrameMessages(pointerid: u32) -> windows_core::Result<()> {
    windows_targets::link!("user32.dll" "system" fn SkipPointerFrameMessages(pointerid : u32) -> super::super::super::Foundation:: BOOL);
    SkipPointerFrameMessages(core::mem::transmute(pointerid)).ok()
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INPUT_INJECTION_VALUE {
    pub page: u16,
    pub usage: u16,
    pub value: i32,
    pub index: u16,
}
impl Default for INPUT_INJECTION_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INPUT_INJECTION_VALUE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT_TRANSFORM {
    pub Anonymous: INPUT_TRANSFORM_0,
}
impl Default for INPUT_TRANSFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INPUT_TRANSFORM {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INPUT_TRANSFORM_0 {
    pub Anonymous: INPUT_TRANSFORM_0_0,
    pub m: [f32; 16],
}
impl Default for INPUT_TRANSFORM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INPUT_TRANSFORM_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INPUT_TRANSFORM_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
impl Default for INPUT_TRANSFORM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for INPUT_TRANSFORM_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINTER_BUTTON_CHANGE_TYPE(pub i32);
pub const POINTER_CHANGE_FIFTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(9i32);
pub const POINTER_CHANGE_FIFTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(10i32);
pub const POINTER_CHANGE_FIRSTBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(1i32);
pub const POINTER_CHANGE_FIRSTBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(2i32);
pub const POINTER_CHANGE_FOURTHBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(7i32);
pub const POINTER_CHANGE_FOURTHBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(8i32);
pub const POINTER_CHANGE_NONE: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(0i32);
pub const POINTER_CHANGE_SECONDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(3i32);
pub const POINTER_CHANGE_SECONDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(4i32);
pub const POINTER_CHANGE_THIRDBUTTON_DOWN: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(5i32);
pub const POINTER_CHANGE_THIRDBUTTON_UP: POINTER_BUTTON_CHANGE_TYPE = POINTER_BUTTON_CHANGE_TYPE(6i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINTER_FLAGS(pub u32);
impl POINTER_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for POINTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for POINTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for POINTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for POINTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for POINTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const POINTER_FLAG_CANCELED: POINTER_FLAGS = POINTER_FLAGS(32768u32);
pub const POINTER_FLAG_CAPTURECHANGED: POINTER_FLAGS = POINTER_FLAGS(2097152u32);
pub const POINTER_FLAG_CONFIDENCE: POINTER_FLAGS = POINTER_FLAGS(16384u32);
pub const POINTER_FLAG_DOWN: POINTER_FLAGS = POINTER_FLAGS(65536u32);
pub const POINTER_FLAG_FIFTHBUTTON: POINTER_FLAGS = POINTER_FLAGS(256u32);
pub const POINTER_FLAG_FIRSTBUTTON: POINTER_FLAGS = POINTER_FLAGS(16u32);
pub const POINTER_FLAG_FOURTHBUTTON: POINTER_FLAGS = POINTER_FLAGS(128u32);
pub const POINTER_FLAG_HASTRANSFORM: POINTER_FLAGS = POINTER_FLAGS(4194304u32);
pub const POINTER_FLAG_HWHEEL: POINTER_FLAGS = POINTER_FLAGS(1048576u32);
pub const POINTER_FLAG_INCONTACT: POINTER_FLAGS = POINTER_FLAGS(4u32);
pub const POINTER_FLAG_INRANGE: POINTER_FLAGS = POINTER_FLAGS(2u32);
pub const POINTER_FLAG_NEW: POINTER_FLAGS = POINTER_FLAGS(1u32);
pub const POINTER_FLAG_NONE: POINTER_FLAGS = POINTER_FLAGS(0u32);
pub const POINTER_FLAG_PRIMARY: POINTER_FLAGS = POINTER_FLAGS(8192u32);
pub const POINTER_FLAG_SECONDBUTTON: POINTER_FLAGS = POINTER_FLAGS(32u32);
pub const POINTER_FLAG_THIRDBUTTON: POINTER_FLAGS = POINTER_FLAGS(64u32);
pub const POINTER_FLAG_UP: POINTER_FLAGS = POINTER_FLAGS(262144u32);
pub const POINTER_FLAG_UPDATE: POINTER_FLAGS = POINTER_FLAGS(131072u32);
pub const POINTER_FLAG_WHEEL: POINTER_FLAGS = POINTER_FLAGS(524288u32);
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POINTER_INFO {
    pub pointerType: super::super::WindowsAndMessaging::POINTER_INPUT_TYPE,
    pub pointerId: u32,
    pub frameId: u32,
    pub pointerFlags: POINTER_FLAGS,
    pub sourceDevice: super::super::super::Foundation::HANDLE,
    pub hwndTarget: super::super::super::Foundation::HWND,
    pub ptPixelLocation: super::super::super::Foundation::POINT,
    pub ptHimetricLocation: super::super::super::Foundation::POINT,
    pub ptPixelLocationRaw: super::super::super::Foundation::POINT,
    pub ptHimetricLocationRaw: super::super::super::Foundation::POINT,
    pub dwTime: u32,
    pub historyCount: u32,
    pub InputData: i32,
    pub dwKeyStates: u32,
    pub PerformanceCount: u64,
    pub ButtonChangeType: POINTER_BUTTON_CHANGE_TYPE,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for POINTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for POINTER_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POINTER_PEN_INFO {
    pub pointerInfo: POINTER_INFO,
    pub penFlags: u32,
    pub penMask: u32,
    pub pressure: u32,
    pub rotation: u32,
    pub tiltX: i32,
    pub tiltY: i32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for POINTER_PEN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for POINTER_PEN_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POINTER_TOUCH_INFO {
    pub pointerInfo: POINTER_INFO,
    pub touchFlags: u32,
    pub touchMask: u32,
    pub rcContact: super::super::super::Foundation::RECT,
    pub rcContactRaw: super::super::super::Foundation::RECT,
    pub orientation: u32,
    pub pressure: u32,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for POINTER_TOUCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for POINTER_TOUCH_INFO {
    type TypeKind = windows_core::CopyType;
}
pub const TOUCH_FEEDBACK_DEFAULT: TOUCH_FEEDBACK_MODE = TOUCH_FEEDBACK_MODE(1u32);
pub const TOUCH_FEEDBACK_INDIRECT: TOUCH_FEEDBACK_MODE = TOUCH_FEEDBACK_MODE(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TOUCH_FEEDBACK_MODE(pub u32);
pub const TOUCH_FEEDBACK_NONE: TOUCH_FEEDBACK_MODE = TOUCH_FEEDBACK_MODE(3u32);
