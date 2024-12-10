#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ChoosePixelFormat(hdc: super::Gdi::HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> i32 {
    windows_targets::link!("gdi32.dll" "system" fn ChoosePixelFormat(hdc : super::Gdi:: HDC, ppfd : *const PIXELFORMATDESCRIPTOR) -> i32);
    ChoosePixelFormat(core::mem::transmute(hdc), core::mem::transmute(ppfd))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DescribePixelFormat(hdc: super::Gdi::HDC, ipixelformat: i32, nbytes: u32, ppfd: Option<*mut PIXELFORMATDESCRIPTOR>) -> i32 {
    windows_targets::link!("gdi32.dll" "system" fn DescribePixelFormat(hdc : super::Gdi:: HDC, ipixelformat : i32, nbytes : u32, ppfd : *mut PIXELFORMATDESCRIPTOR) -> i32);
    DescribePixelFormat(core::mem::transmute(hdc), core::mem::transmute(ipixelformat), core::mem::transmute(nbytes), core::mem::transmute(ppfd.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetEnhMetaFilePixelFormat(hemf: super::Gdi::HENHMETAFILE, cbbuffer: u32, ppfd: Option<*mut PIXELFORMATDESCRIPTOR>) -> u32 {
    windows_targets::link!("gdi32.dll" "system" fn GetEnhMetaFilePixelFormat(hemf : super::Gdi:: HENHMETAFILE, cbbuffer : u32, ppfd : *mut PIXELFORMATDESCRIPTOR) -> u32);
    GetEnhMetaFilePixelFormat(core::mem::transmute(hemf), core::mem::transmute(cbbuffer), core::mem::transmute(ppfd.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn GetPixelFormat(hdc: super::Gdi::HDC) -> i32 {
    windows_targets::link!("gdi32.dll" "system" fn GetPixelFormat(hdc : super::Gdi:: HDC) -> i32);
    GetPixelFormat(core::mem::transmute(hdc))
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SetPixelFormat(hdc: super::Gdi::HDC, format: i32, ppfd: *const PIXELFORMATDESCRIPTOR) -> windows_core::Result<()> {
    windows_targets::link!("gdi32.dll" "system" fn SetPixelFormat(hdc : super::Gdi:: HDC, format : i32, ppfd : *const PIXELFORMATDESCRIPTOR) -> super::super::Foundation:: BOOL);
    SetPixelFormat(core::mem::transmute(hdc), core::mem::transmute(format), core::mem::transmute(ppfd)).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn SwapBuffers(param0: super::Gdi::HDC) -> windows_core::Result<()> {
    windows_targets::link!("gdi32.dll" "system" fn SwapBuffers(param0 : super::Gdi:: HDC) -> super::super::Foundation:: BOOL);
    SwapBuffers(core::mem::transmute(param0)).ok()
}
#[inline]
pub unsafe fn glAccum(op: u32, value: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glAccum(op : u32, value : f32));
    glAccum(core::mem::transmute(op), core::mem::transmute(value))
}
#[inline]
pub unsafe fn glAlphaFunc(func: u32, r#ref: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glAlphaFunc(func : u32, r#ref : f32));
    glAlphaFunc(core::mem::transmute(func), core::mem::transmute(r#ref))
}
#[inline]
pub unsafe fn glAreTexturesResident(n: i32, textures: *const u32, residences: *mut u8) -> u8 {
    windows_targets::link!("opengl32.dll" "system" fn glAreTexturesResident(n : i32, textures : *const u32, residences : *mut u8) -> u8);
    glAreTexturesResident(core::mem::transmute(n), core::mem::transmute(textures), core::mem::transmute(residences))
}
#[inline]
pub unsafe fn glArrayElement(i: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glArrayElement(i : i32));
    glArrayElement(core::mem::transmute(i))
}
#[inline]
pub unsafe fn glBegin(mode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glBegin(mode : u32));
    glBegin(core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glBindTexture(target: u32, texture: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glBindTexture(target : u32, texture : u32));
    glBindTexture(core::mem::transmute(target), core::mem::transmute(texture))
}
#[inline]
pub unsafe fn glBitmap(width: i32, height: i32, xorig: f32, yorig: f32, xmove: f32, ymove: f32, bitmap: *const u8) {
    windows_targets::link!("opengl32.dll" "system" fn glBitmap(width : i32, height : i32, xorig : f32, yorig : f32, xmove : f32, ymove : f32, bitmap : *const u8));
    glBitmap(core::mem::transmute(width), core::mem::transmute(height), core::mem::transmute(xorig), core::mem::transmute(yorig), core::mem::transmute(xmove), core::mem::transmute(ymove), core::mem::transmute(bitmap))
}
#[inline]
pub unsafe fn glBlendFunc(sfactor: u32, dfactor: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glBlendFunc(sfactor : u32, dfactor : u32));
    glBlendFunc(core::mem::transmute(sfactor), core::mem::transmute(dfactor))
}
#[inline]
pub unsafe fn glCallList(list: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glCallList(list : u32));
    glCallList(core::mem::transmute(list))
}
#[inline]
pub unsafe fn glCallLists(n: i32, r#type: u32, lists: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glCallLists(n : i32, r#type : u32, lists : *const core::ffi::c_void));
    glCallLists(core::mem::transmute(n), core::mem::transmute(r#type), core::mem::transmute(lists))
}
#[inline]
pub unsafe fn glClear(mask: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glClear(mask : u32));
    glClear(core::mem::transmute(mask))
}
#[inline]
pub unsafe fn glClearAccum(red: f32, green: f32, blue: f32, alpha: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glClearAccum(red : f32, green : f32, blue : f32, alpha : f32));
    glClearAccum(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue), core::mem::transmute(alpha))
}
#[inline]
pub unsafe fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glClearColor(red : f32, green : f32, blue : f32, alpha : f32));
    glClearColor(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue), core::mem::transmute(alpha))
}
#[inline]
pub unsafe fn glClearDepth(depth: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glClearDepth(depth : f64));
    glClearDepth(core::mem::transmute(depth))
}
#[inline]
pub unsafe fn glClearIndex(c: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glClearIndex(c : f32));
    glClearIndex(core::mem::transmute(c))
}
#[inline]
pub unsafe fn glClearStencil(s: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glClearStencil(s : i32));
    glClearStencil(core::mem::transmute(s))
}
#[inline]
pub unsafe fn glClipPlane(plane: u32, equation: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glClipPlane(plane : u32, equation : *const f64));
    glClipPlane(core::mem::transmute(plane), core::mem::transmute(equation))
}
#[inline]
pub unsafe fn glColor3b(red: i8, green: i8, blue: i8) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3b(red : i8, green : i8, blue : i8));
    glColor3b(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue))
}
#[inline]
pub unsafe fn glColor3bv(v: *const i8) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3bv(v : *const i8));
    glColor3bv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor3d(red: f64, green: f64, blue: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3d(red : f64, green : f64, blue : f64));
    glColor3d(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue))
}
#[inline]
pub unsafe fn glColor3dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3dv(v : *const f64));
    glColor3dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor3f(red: f32, green: f32, blue: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3f(red : f32, green : f32, blue : f32));
    glColor3f(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue))
}
#[inline]
pub unsafe fn glColor3fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3fv(v : *const f32));
    glColor3fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor3i(red: i32, green: i32, blue: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3i(red : i32, green : i32, blue : i32));
    glColor3i(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue))
}
#[inline]
pub unsafe fn glColor3iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3iv(v : *const i32));
    glColor3iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor3s(red: i16, green: i16, blue: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3s(red : i16, green : i16, blue : i16));
    glColor3s(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue))
}
#[inline]
pub unsafe fn glColor3sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3sv(v : *const i16));
    glColor3sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor3ub(red: u8, green: u8, blue: u8) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3ub(red : u8, green : u8, blue : u8));
    glColor3ub(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue))
}
#[inline]
pub unsafe fn glColor3ubv(v: *const u8) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3ubv(v : *const u8));
    glColor3ubv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor3ui(red: u32, green: u32, blue: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3ui(red : u32, green : u32, blue : u32));
    glColor3ui(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue))
}
#[inline]
pub unsafe fn glColor3uiv(v: *const u32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3uiv(v : *const u32));
    glColor3uiv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor3us(red: u16, green: u16, blue: u16) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3us(red : u16, green : u16, blue : u16));
    glColor3us(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue))
}
#[inline]
pub unsafe fn glColor3usv(v: *const u16) {
    windows_targets::link!("opengl32.dll" "system" fn glColor3usv(v : *const u16));
    glColor3usv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor4b(red: i8, green: i8, blue: i8, alpha: i8) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4b(red : i8, green : i8, blue : i8, alpha : i8));
    glColor4b(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue), core::mem::transmute(alpha))
}
#[inline]
pub unsafe fn glColor4bv(v: *const i8) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4bv(v : *const i8));
    glColor4bv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor4d(red: f64, green: f64, blue: f64, alpha: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4d(red : f64, green : f64, blue : f64, alpha : f64));
    glColor4d(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue), core::mem::transmute(alpha))
}
#[inline]
pub unsafe fn glColor4dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4dv(v : *const f64));
    glColor4dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor4f(red: f32, green: f32, blue: f32, alpha: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4f(red : f32, green : f32, blue : f32, alpha : f32));
    glColor4f(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue), core::mem::transmute(alpha))
}
#[inline]
pub unsafe fn glColor4fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4fv(v : *const f32));
    glColor4fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor4i(red: i32, green: i32, blue: i32, alpha: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4i(red : i32, green : i32, blue : i32, alpha : i32));
    glColor4i(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue), core::mem::transmute(alpha))
}
#[inline]
pub unsafe fn glColor4iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4iv(v : *const i32));
    glColor4iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor4s(red: i16, green: i16, blue: i16, alpha: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4s(red : i16, green : i16, blue : i16, alpha : i16));
    glColor4s(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue), core::mem::transmute(alpha))
}
#[inline]
pub unsafe fn glColor4sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4sv(v : *const i16));
    glColor4sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor4ub(red: u8, green: u8, blue: u8, alpha: u8) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4ub(red : u8, green : u8, blue : u8, alpha : u8));
    glColor4ub(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue), core::mem::transmute(alpha))
}
#[inline]
pub unsafe fn glColor4ubv(v: *const u8) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4ubv(v : *const u8));
    glColor4ubv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor4ui(red: u32, green: u32, blue: u32, alpha: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4ui(red : u32, green : u32, blue : u32, alpha : u32));
    glColor4ui(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue), core::mem::transmute(alpha))
}
#[inline]
pub unsafe fn glColor4uiv(v: *const u32) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4uiv(v : *const u32));
    glColor4uiv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColor4us(red: u16, green: u16, blue: u16, alpha: u16) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4us(red : u16, green : u16, blue : u16, alpha : u16));
    glColor4us(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue), core::mem::transmute(alpha))
}
#[inline]
pub unsafe fn glColor4usv(v: *const u16) {
    windows_targets::link!("opengl32.dll" "system" fn glColor4usv(v : *const u16));
    glColor4usv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glColorMask(red: u8, green: u8, blue: u8, alpha: u8) {
    windows_targets::link!("opengl32.dll" "system" fn glColorMask(red : u8, green : u8, blue : u8, alpha : u8));
    glColorMask(core::mem::transmute(red), core::mem::transmute(green), core::mem::transmute(blue), core::mem::transmute(alpha))
}
#[inline]
pub unsafe fn glColorMaterial(face: u32, mode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glColorMaterial(face : u32, mode : u32));
    glColorMaterial(core::mem::transmute(face), core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glColorPointer(size: i32, r#type: u32, stride: i32, pointer: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glColorPointer(size : i32, r#type : u32, stride : i32, pointer : *const core::ffi::c_void));
    glColorPointer(core::mem::transmute(size), core::mem::transmute(r#type), core::mem::transmute(stride), core::mem::transmute(pointer))
}
#[inline]
pub unsafe fn glCopyPixels(x: i32, y: i32, width: i32, height: i32, r#type: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glCopyPixels(x : i32, y : i32, width : i32, height : i32, r#type : u32));
    glCopyPixels(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(width), core::mem::transmute(height), core::mem::transmute(r#type))
}
#[inline]
pub unsafe fn glCopyTexImage1D(target: u32, level: i32, internalformat: u32, x: i32, y: i32, width: i32, border: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glCopyTexImage1D(target : u32, level : i32, internalformat : u32, x : i32, y : i32, width : i32, border : i32));
    glCopyTexImage1D(core::mem::transmute(target), core::mem::transmute(level), core::mem::transmute(internalformat), core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(width), core::mem::transmute(border))
}
#[inline]
pub unsafe fn glCopyTexImage2D(target: u32, level: i32, internalformat: u32, x: i32, y: i32, width: i32, height: i32, border: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glCopyTexImage2D(target : u32, level : i32, internalformat : u32, x : i32, y : i32, width : i32, height : i32, border : i32));
    glCopyTexImage2D(core::mem::transmute(target), core::mem::transmute(level), core::mem::transmute(internalformat), core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(width), core::mem::transmute(height), core::mem::transmute(border))
}
#[inline]
pub unsafe fn glCopyTexSubImage1D(target: u32, level: i32, xoffset: i32, x: i32, y: i32, width: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glCopyTexSubImage1D(target : u32, level : i32, xoffset : i32, x : i32, y : i32, width : i32));
    glCopyTexSubImage1D(core::mem::transmute(target), core::mem::transmute(level), core::mem::transmute(xoffset), core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(width))
}
#[inline]
pub unsafe fn glCopyTexSubImage2D(target: u32, level: i32, xoffset: i32, yoffset: i32, x: i32, y: i32, width: i32, height: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glCopyTexSubImage2D(target : u32, level : i32, xoffset : i32, yoffset : i32, x : i32, y : i32, width : i32, height : i32));
    glCopyTexSubImage2D(core::mem::transmute(target), core::mem::transmute(level), core::mem::transmute(xoffset), core::mem::transmute(yoffset), core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(width), core::mem::transmute(height))
}
#[inline]
pub unsafe fn glCullFace(mode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glCullFace(mode : u32));
    glCullFace(core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glDeleteLists(list: u32, range: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glDeleteLists(list : u32, range : i32));
    glDeleteLists(core::mem::transmute(list), core::mem::transmute(range))
}
#[inline]
pub unsafe fn glDeleteTextures(n: i32, textures: *const u32) {
    windows_targets::link!("opengl32.dll" "system" fn glDeleteTextures(n : i32, textures : *const u32));
    glDeleteTextures(core::mem::transmute(n), core::mem::transmute(textures))
}
#[inline]
pub unsafe fn glDepthFunc(func: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glDepthFunc(func : u32));
    glDepthFunc(core::mem::transmute(func))
}
#[inline]
pub unsafe fn glDepthMask(flag: u8) {
    windows_targets::link!("opengl32.dll" "system" fn glDepthMask(flag : u8));
    glDepthMask(core::mem::transmute(flag))
}
#[inline]
pub unsafe fn glDepthRange(znear: f64, zfar: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glDepthRange(znear : f64, zfar : f64));
    glDepthRange(core::mem::transmute(znear), core::mem::transmute(zfar))
}
#[inline]
pub unsafe fn glDisable(cap: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glDisable(cap : u32));
    glDisable(core::mem::transmute(cap))
}
#[inline]
pub unsafe fn glDisableClientState(array: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glDisableClientState(array : u32));
    glDisableClientState(core::mem::transmute(array))
}
#[inline]
pub unsafe fn glDrawArrays(mode: u32, first: i32, count: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glDrawArrays(mode : u32, first : i32, count : i32));
    glDrawArrays(core::mem::transmute(mode), core::mem::transmute(first), core::mem::transmute(count))
}
#[inline]
pub unsafe fn glDrawBuffer(mode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glDrawBuffer(mode : u32));
    glDrawBuffer(core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glDrawElements(mode: u32, count: i32, r#type: u32, indices: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glDrawElements(mode : u32, count : i32, r#type : u32, indices : *const core::ffi::c_void));
    glDrawElements(core::mem::transmute(mode), core::mem::transmute(count), core::mem::transmute(r#type), core::mem::transmute(indices))
}
#[inline]
pub unsafe fn glDrawPixels(width: i32, height: i32, format: u32, r#type: u32, pixels: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glDrawPixels(width : i32, height : i32, format : u32, r#type : u32, pixels : *const core::ffi::c_void));
    glDrawPixels(core::mem::transmute(width), core::mem::transmute(height), core::mem::transmute(format), core::mem::transmute(r#type), core::mem::transmute(pixels))
}
#[inline]
pub unsafe fn glEdgeFlag(flag: u8) {
    windows_targets::link!("opengl32.dll" "system" fn glEdgeFlag(flag : u8));
    glEdgeFlag(core::mem::transmute(flag))
}
#[inline]
pub unsafe fn glEdgeFlagPointer(stride: i32, pointer: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glEdgeFlagPointer(stride : i32, pointer : *const core::ffi::c_void));
    glEdgeFlagPointer(core::mem::transmute(stride), core::mem::transmute(pointer))
}
#[inline]
pub unsafe fn glEdgeFlagv(flag: *const u8) {
    windows_targets::link!("opengl32.dll" "system" fn glEdgeFlagv(flag : *const u8));
    glEdgeFlagv(core::mem::transmute(flag))
}
#[inline]
pub unsafe fn glEnable(cap: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glEnable(cap : u32));
    glEnable(core::mem::transmute(cap))
}
#[inline]
pub unsafe fn glEnableClientState(array: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glEnableClientState(array : u32));
    glEnableClientState(core::mem::transmute(array))
}
#[inline]
pub unsafe fn glEnd() {
    windows_targets::link!("opengl32.dll" "system" fn glEnd());
    glEnd()
}
#[inline]
pub unsafe fn glEndList() {
    windows_targets::link!("opengl32.dll" "system" fn glEndList());
    glEndList()
}
#[inline]
pub unsafe fn glEvalCoord1d(u: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalCoord1d(u : f64));
    glEvalCoord1d(core::mem::transmute(u))
}
#[inline]
pub unsafe fn glEvalCoord1dv(u: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalCoord1dv(u : *const f64));
    glEvalCoord1dv(core::mem::transmute(u))
}
#[inline]
pub unsafe fn glEvalCoord1f(u: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalCoord1f(u : f32));
    glEvalCoord1f(core::mem::transmute(u))
}
#[inline]
pub unsafe fn glEvalCoord1fv(u: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalCoord1fv(u : *const f32));
    glEvalCoord1fv(core::mem::transmute(u))
}
#[inline]
pub unsafe fn glEvalCoord2d(u: f64, v: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalCoord2d(u : f64, v : f64));
    glEvalCoord2d(core::mem::transmute(u), core::mem::transmute(v))
}
#[inline]
pub unsafe fn glEvalCoord2dv(u: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalCoord2dv(u : *const f64));
    glEvalCoord2dv(core::mem::transmute(u))
}
#[inline]
pub unsafe fn glEvalCoord2f(u: f32, v: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalCoord2f(u : f32, v : f32));
    glEvalCoord2f(core::mem::transmute(u), core::mem::transmute(v))
}
#[inline]
pub unsafe fn glEvalCoord2fv(u: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalCoord2fv(u : *const f32));
    glEvalCoord2fv(core::mem::transmute(u))
}
#[inline]
pub unsafe fn glEvalMesh1(mode: u32, i1: i32, i2: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalMesh1(mode : u32, i1 : i32, i2 : i32));
    glEvalMesh1(core::mem::transmute(mode), core::mem::transmute(i1), core::mem::transmute(i2))
}
#[inline]
pub unsafe fn glEvalMesh2(mode: u32, i1: i32, i2: i32, j1: i32, j2: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalMesh2(mode : u32, i1 : i32, i2 : i32, j1 : i32, j2 : i32));
    glEvalMesh2(core::mem::transmute(mode), core::mem::transmute(i1), core::mem::transmute(i2), core::mem::transmute(j1), core::mem::transmute(j2))
}
#[inline]
pub unsafe fn glEvalPoint1(i: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalPoint1(i : i32));
    glEvalPoint1(core::mem::transmute(i))
}
#[inline]
pub unsafe fn glEvalPoint2(i: i32, j: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glEvalPoint2(i : i32, j : i32));
    glEvalPoint2(core::mem::transmute(i), core::mem::transmute(j))
}
#[inline]
pub unsafe fn glFeedbackBuffer(size: i32, r#type: u32, buffer: *mut f32) {
    windows_targets::link!("opengl32.dll" "system" fn glFeedbackBuffer(size : i32, r#type : u32, buffer : *mut f32));
    glFeedbackBuffer(core::mem::transmute(size), core::mem::transmute(r#type), core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn glFinish() {
    windows_targets::link!("opengl32.dll" "system" fn glFinish());
    glFinish()
}
#[inline]
pub unsafe fn glFlush() {
    windows_targets::link!("opengl32.dll" "system" fn glFlush());
    glFlush()
}
#[inline]
pub unsafe fn glFogf(pname: u32, param1: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glFogf(pname : u32, param1 : f32));
    glFogf(core::mem::transmute(pname), core::mem::transmute(param1))
}
#[inline]
pub unsafe fn glFogfv(pname: u32, params: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glFogfv(pname : u32, params : *const f32));
    glFogfv(core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glFogi(pname: u32, param1: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glFogi(pname : u32, param1 : i32));
    glFogi(core::mem::transmute(pname), core::mem::transmute(param1))
}
#[inline]
pub unsafe fn glFogiv(pname: u32, params: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glFogiv(pname : u32, params : *const i32));
    glFogiv(core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glFrontFace(mode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glFrontFace(mode : u32));
    glFrontFace(core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glFrustum(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glFrustum(left : f64, right : f64, bottom : f64, top : f64, znear : f64, zfar : f64));
    glFrustum(core::mem::transmute(left), core::mem::transmute(right), core::mem::transmute(bottom), core::mem::transmute(top), core::mem::transmute(znear), core::mem::transmute(zfar))
}
#[inline]
pub unsafe fn glGenLists(range: i32) -> u32 {
    windows_targets::link!("opengl32.dll" "system" fn glGenLists(range : i32) -> u32);
    glGenLists(core::mem::transmute(range))
}
#[inline]
pub unsafe fn glGenTextures(n: i32, textures: *mut u32) {
    windows_targets::link!("opengl32.dll" "system" fn glGenTextures(n : i32, textures : *mut u32));
    glGenTextures(core::mem::transmute(n), core::mem::transmute(textures))
}
#[inline]
pub unsafe fn glGetBooleanv(pname: u32, params: *mut u8) {
    windows_targets::link!("opengl32.dll" "system" fn glGetBooleanv(pname : u32, params : *mut u8));
    glGetBooleanv(core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetClipPlane(plane: u32, equation: *mut f64) {
    windows_targets::link!("opengl32.dll" "system" fn glGetClipPlane(plane : u32, equation : *mut f64));
    glGetClipPlane(core::mem::transmute(plane), core::mem::transmute(equation))
}
#[inline]
pub unsafe fn glGetDoublev(pname: u32, params: *mut f64) {
    windows_targets::link!("opengl32.dll" "system" fn glGetDoublev(pname : u32, params : *mut f64));
    glGetDoublev(core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetError() -> u32 {
    windows_targets::link!("opengl32.dll" "system" fn glGetError() -> u32);
    glGetError()
}
#[inline]
pub unsafe fn glGetFloatv(pname: u32, params: *mut f32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetFloatv(pname : u32, params : *mut f32));
    glGetFloatv(core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetIntegerv(pname: u32, params: *mut i32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetIntegerv(pname : u32, params : *mut i32));
    glGetIntegerv(core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetLightfv(light: u32, pname: u32, params: *mut f32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetLightfv(light : u32, pname : u32, params : *mut f32));
    glGetLightfv(core::mem::transmute(light), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetLightiv(light: u32, pname: u32, params: *mut i32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetLightiv(light : u32, pname : u32, params : *mut i32));
    glGetLightiv(core::mem::transmute(light), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetMapdv(target: u32, query: u32, v: *mut f64) {
    windows_targets::link!("opengl32.dll" "system" fn glGetMapdv(target : u32, query : u32, v : *mut f64));
    glGetMapdv(core::mem::transmute(target), core::mem::transmute(query), core::mem::transmute(v))
}
#[inline]
pub unsafe fn glGetMapfv(target: u32, query: u32, v: *mut f32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetMapfv(target : u32, query : u32, v : *mut f32));
    glGetMapfv(core::mem::transmute(target), core::mem::transmute(query), core::mem::transmute(v))
}
#[inline]
pub unsafe fn glGetMapiv(target: u32, query: u32, v: *mut i32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetMapiv(target : u32, query : u32, v : *mut i32));
    glGetMapiv(core::mem::transmute(target), core::mem::transmute(query), core::mem::transmute(v))
}
#[inline]
pub unsafe fn glGetMaterialfv(face: u32, pname: u32, params: *mut f32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetMaterialfv(face : u32, pname : u32, params : *mut f32));
    glGetMaterialfv(core::mem::transmute(face), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetMaterialiv(face: u32, pname: u32, params: *mut i32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetMaterialiv(face : u32, pname : u32, params : *mut i32));
    glGetMaterialiv(core::mem::transmute(face), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetPixelMapfv(map: u32, values: *mut f32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetPixelMapfv(map : u32, values : *mut f32));
    glGetPixelMapfv(core::mem::transmute(map), core::mem::transmute(values))
}
#[inline]
pub unsafe fn glGetPixelMapuiv(map: u32, values: *mut u32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetPixelMapuiv(map : u32, values : *mut u32));
    glGetPixelMapuiv(core::mem::transmute(map), core::mem::transmute(values))
}
#[inline]
pub unsafe fn glGetPixelMapusv(map: u32, values: *mut u16) {
    windows_targets::link!("opengl32.dll" "system" fn glGetPixelMapusv(map : u32, values : *mut u16));
    glGetPixelMapusv(core::mem::transmute(map), core::mem::transmute(values))
}
#[inline]
pub unsafe fn glGetPointerv(pname: u32, params: *mut *mut core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glGetPointerv(pname : u32, params : *mut *mut core::ffi::c_void));
    glGetPointerv(core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetPolygonStipple(mask: *mut u8) {
    windows_targets::link!("opengl32.dll" "system" fn glGetPolygonStipple(mask : *mut u8));
    glGetPolygonStipple(core::mem::transmute(mask))
}
#[inline]
pub unsafe fn glGetString(name: u32) -> *mut u8 {
    windows_targets::link!("opengl32.dll" "system" fn glGetString(name : u32) -> *mut u8);
    glGetString(core::mem::transmute(name))
}
#[inline]
pub unsafe fn glGetTexEnvfv(target: u32, pname: u32, params: *mut f32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetTexEnvfv(target : u32, pname : u32, params : *mut f32));
    glGetTexEnvfv(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetTexEnviv(target: u32, pname: u32, params: *mut i32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetTexEnviv(target : u32, pname : u32, params : *mut i32));
    glGetTexEnviv(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetTexGendv(coord: u32, pname: u32, params: *mut f64) {
    windows_targets::link!("opengl32.dll" "system" fn glGetTexGendv(coord : u32, pname : u32, params : *mut f64));
    glGetTexGendv(core::mem::transmute(coord), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetTexGenfv(coord: u32, pname: u32, params: *mut f32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetTexGenfv(coord : u32, pname : u32, params : *mut f32));
    glGetTexGenfv(core::mem::transmute(coord), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetTexGeniv(coord: u32, pname: u32, params: *mut i32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetTexGeniv(coord : u32, pname : u32, params : *mut i32));
    glGetTexGeniv(core::mem::transmute(coord), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetTexImage(target: u32, level: i32, format: u32, r#type: u32, pixels: *mut core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glGetTexImage(target : u32, level : i32, format : u32, r#type : u32, pixels : *mut core::ffi::c_void));
    glGetTexImage(core::mem::transmute(target), core::mem::transmute(level), core::mem::transmute(format), core::mem::transmute(r#type), core::mem::transmute(pixels))
}
#[inline]
pub unsafe fn glGetTexLevelParameterfv(target: u32, level: i32, pname: u32, params: *mut f32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetTexLevelParameterfv(target : u32, level : i32, pname : u32, params : *mut f32));
    glGetTexLevelParameterfv(core::mem::transmute(target), core::mem::transmute(level), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetTexLevelParameteriv(target: u32, level: i32, pname: u32, params: *mut i32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetTexLevelParameteriv(target : u32, level : i32, pname : u32, params : *mut i32));
    glGetTexLevelParameteriv(core::mem::transmute(target), core::mem::transmute(level), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetTexParameterfv(target: u32, pname: u32, params: *mut f32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetTexParameterfv(target : u32, pname : u32, params : *mut f32));
    glGetTexParameterfv(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glGetTexParameteriv(target: u32, pname: u32, params: *mut i32) {
    windows_targets::link!("opengl32.dll" "system" fn glGetTexParameteriv(target : u32, pname : u32, params : *mut i32));
    glGetTexParameteriv(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glHint(target: u32, mode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glHint(target : u32, mode : u32));
    glHint(core::mem::transmute(target), core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glIndexMask(mask: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexMask(mask : u32));
    glIndexMask(core::mem::transmute(mask))
}
#[inline]
pub unsafe fn glIndexPointer(r#type: u32, stride: i32, pointer: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexPointer(r#type : u32, stride : i32, pointer : *const core::ffi::c_void));
    glIndexPointer(core::mem::transmute(r#type), core::mem::transmute(stride), core::mem::transmute(pointer))
}
#[inline]
pub unsafe fn glIndexd(c: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexd(c : f64));
    glIndexd(core::mem::transmute(c))
}
#[inline]
pub unsafe fn glIndexdv(c: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexdv(c : *const f64));
    glIndexdv(core::mem::transmute(c))
}
#[inline]
pub unsafe fn glIndexf(c: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexf(c : f32));
    glIndexf(core::mem::transmute(c))
}
#[inline]
pub unsafe fn glIndexfv(c: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexfv(c : *const f32));
    glIndexfv(core::mem::transmute(c))
}
#[inline]
pub unsafe fn glIndexi(c: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexi(c : i32));
    glIndexi(core::mem::transmute(c))
}
#[inline]
pub unsafe fn glIndexiv(c: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexiv(c : *const i32));
    glIndexiv(core::mem::transmute(c))
}
#[inline]
pub unsafe fn glIndexs(c: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexs(c : i16));
    glIndexs(core::mem::transmute(c))
}
#[inline]
pub unsafe fn glIndexsv(c: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexsv(c : *const i16));
    glIndexsv(core::mem::transmute(c))
}
#[inline]
pub unsafe fn glIndexub(c: u8) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexub(c : u8));
    glIndexub(core::mem::transmute(c))
}
#[inline]
pub unsafe fn glIndexubv(c: *const u8) {
    windows_targets::link!("opengl32.dll" "system" fn glIndexubv(c : *const u8));
    glIndexubv(core::mem::transmute(c))
}
#[inline]
pub unsafe fn glInitNames() {
    windows_targets::link!("opengl32.dll" "system" fn glInitNames());
    glInitNames()
}
#[inline]
pub unsafe fn glInterleavedArrays(format: u32, stride: i32, pointer: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glInterleavedArrays(format : u32, stride : i32, pointer : *const core::ffi::c_void));
    glInterleavedArrays(core::mem::transmute(format), core::mem::transmute(stride), core::mem::transmute(pointer))
}
#[inline]
pub unsafe fn glIsEnabled(cap: u32) -> u8 {
    windows_targets::link!("opengl32.dll" "system" fn glIsEnabled(cap : u32) -> u8);
    glIsEnabled(core::mem::transmute(cap))
}
#[inline]
pub unsafe fn glIsList(list: u32) -> u8 {
    windows_targets::link!("opengl32.dll" "system" fn glIsList(list : u32) -> u8);
    glIsList(core::mem::transmute(list))
}
#[inline]
pub unsafe fn glIsTexture(texture: u32) -> u8 {
    windows_targets::link!("opengl32.dll" "system" fn glIsTexture(texture : u32) -> u8);
    glIsTexture(core::mem::transmute(texture))
}
#[inline]
pub unsafe fn glLightModelf(pname: u32, param1: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glLightModelf(pname : u32, param1 : f32));
    glLightModelf(core::mem::transmute(pname), core::mem::transmute(param1))
}
#[inline]
pub unsafe fn glLightModelfv(pname: u32, params: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glLightModelfv(pname : u32, params : *const f32));
    glLightModelfv(core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glLightModeli(pname: u32, param1: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glLightModeli(pname : u32, param1 : i32));
    glLightModeli(core::mem::transmute(pname), core::mem::transmute(param1))
}
#[inline]
pub unsafe fn glLightModeliv(pname: u32, params: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glLightModeliv(pname : u32, params : *const i32));
    glLightModeliv(core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glLightf(light: u32, pname: u32, param2: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glLightf(light : u32, pname : u32, param2 : f32));
    glLightf(core::mem::transmute(light), core::mem::transmute(pname), core::mem::transmute(param2))
}
#[inline]
pub unsafe fn glLightfv(light: u32, pname: u32, params: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glLightfv(light : u32, pname : u32, params : *const f32));
    glLightfv(core::mem::transmute(light), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glLighti(light: u32, pname: u32, param2: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glLighti(light : u32, pname : u32, param2 : i32));
    glLighti(core::mem::transmute(light), core::mem::transmute(pname), core::mem::transmute(param2))
}
#[inline]
pub unsafe fn glLightiv(light: u32, pname: u32, params: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glLightiv(light : u32, pname : u32, params : *const i32));
    glLightiv(core::mem::transmute(light), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glLineStipple(factor: i32, pattern: u16) {
    windows_targets::link!("opengl32.dll" "system" fn glLineStipple(factor : i32, pattern : u16));
    glLineStipple(core::mem::transmute(factor), core::mem::transmute(pattern))
}
#[inline]
pub unsafe fn glLineWidth(width: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glLineWidth(width : f32));
    glLineWidth(core::mem::transmute(width))
}
#[inline]
pub unsafe fn glListBase(base: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glListBase(base : u32));
    glListBase(core::mem::transmute(base))
}
#[inline]
pub unsafe fn glLoadIdentity() {
    windows_targets::link!("opengl32.dll" "system" fn glLoadIdentity());
    glLoadIdentity()
}
#[inline]
pub unsafe fn glLoadMatrixd(m: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glLoadMatrixd(m : *const f64));
    glLoadMatrixd(core::mem::transmute(m))
}
#[inline]
pub unsafe fn glLoadMatrixf(m: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glLoadMatrixf(m : *const f32));
    glLoadMatrixf(core::mem::transmute(m))
}
#[inline]
pub unsafe fn glLoadName(name: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glLoadName(name : u32));
    glLoadName(core::mem::transmute(name))
}
#[inline]
pub unsafe fn glLogicOp(opcode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glLogicOp(opcode : u32));
    glLogicOp(core::mem::transmute(opcode))
}
#[inline]
pub unsafe fn glMap1d(target: u32, u1: f64, u2: f64, stride: i32, order: i32, points: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glMap1d(target : u32, u1 : f64, u2 : f64, stride : i32, order : i32, points : *const f64));
    glMap1d(core::mem::transmute(target), core::mem::transmute(u1), core::mem::transmute(u2), core::mem::transmute(stride), core::mem::transmute(order), core::mem::transmute(points))
}
#[inline]
pub unsafe fn glMap1f(target: u32, u1: f32, u2: f32, stride: i32, order: i32, points: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glMap1f(target : u32, u1 : f32, u2 : f32, stride : i32, order : i32, points : *const f32));
    glMap1f(core::mem::transmute(target), core::mem::transmute(u1), core::mem::transmute(u2), core::mem::transmute(stride), core::mem::transmute(order), core::mem::transmute(points))
}
#[inline]
pub unsafe fn glMap2d(target: u32, u1: f64, u2: f64, ustride: i32, uorder: i32, v1: f64, v2: f64, vstride: i32, vorder: i32, points: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glMap2d(target : u32, u1 : f64, u2 : f64, ustride : i32, uorder : i32, v1 : f64, v2 : f64, vstride : i32, vorder : i32, points : *const f64));
    glMap2d(core::mem::transmute(target), core::mem::transmute(u1), core::mem::transmute(u2), core::mem::transmute(ustride), core::mem::transmute(uorder), core::mem::transmute(v1), core::mem::transmute(v2), core::mem::transmute(vstride), core::mem::transmute(vorder), core::mem::transmute(points))
}
#[inline]
pub unsafe fn glMap2f(target: u32, u1: f32, u2: f32, ustride: i32, uorder: i32, v1: f32, v2: f32, vstride: i32, vorder: i32, points: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glMap2f(target : u32, u1 : f32, u2 : f32, ustride : i32, uorder : i32, v1 : f32, v2 : f32, vstride : i32, vorder : i32, points : *const f32));
    glMap2f(core::mem::transmute(target), core::mem::transmute(u1), core::mem::transmute(u2), core::mem::transmute(ustride), core::mem::transmute(uorder), core::mem::transmute(v1), core::mem::transmute(v2), core::mem::transmute(vstride), core::mem::transmute(vorder), core::mem::transmute(points))
}
#[inline]
pub unsafe fn glMapGrid1d(un: i32, u1: f64, u2: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glMapGrid1d(un : i32, u1 : f64, u2 : f64));
    glMapGrid1d(core::mem::transmute(un), core::mem::transmute(u1), core::mem::transmute(u2))
}
#[inline]
pub unsafe fn glMapGrid1f(un: i32, u1: f32, u2: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glMapGrid1f(un : i32, u1 : f32, u2 : f32));
    glMapGrid1f(core::mem::transmute(un), core::mem::transmute(u1), core::mem::transmute(u2))
}
#[inline]
pub unsafe fn glMapGrid2d(un: i32, u1: f64, u2: f64, vn: i32, v1: f64, v2: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glMapGrid2d(un : i32, u1 : f64, u2 : f64, vn : i32, v1 : f64, v2 : f64));
    glMapGrid2d(core::mem::transmute(un), core::mem::transmute(u1), core::mem::transmute(u2), core::mem::transmute(vn), core::mem::transmute(v1), core::mem::transmute(v2))
}
#[inline]
pub unsafe fn glMapGrid2f(un: i32, u1: f32, u2: f32, vn: i32, v1: f32, v2: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glMapGrid2f(un : i32, u1 : f32, u2 : f32, vn : i32, v1 : f32, v2 : f32));
    glMapGrid2f(core::mem::transmute(un), core::mem::transmute(u1), core::mem::transmute(u2), core::mem::transmute(vn), core::mem::transmute(v1), core::mem::transmute(v2))
}
#[inline]
pub unsafe fn glMaterialf(face: u32, pname: u32, param2: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glMaterialf(face : u32, pname : u32, param2 : f32));
    glMaterialf(core::mem::transmute(face), core::mem::transmute(pname), core::mem::transmute(param2))
}
#[inline]
pub unsafe fn glMaterialfv(face: u32, pname: u32, params: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glMaterialfv(face : u32, pname : u32, params : *const f32));
    glMaterialfv(core::mem::transmute(face), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glMateriali(face: u32, pname: u32, param2: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glMateriali(face : u32, pname : u32, param2 : i32));
    glMateriali(core::mem::transmute(face), core::mem::transmute(pname), core::mem::transmute(param2))
}
#[inline]
pub unsafe fn glMaterialiv(face: u32, pname: u32, params: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glMaterialiv(face : u32, pname : u32, params : *const i32));
    glMaterialiv(core::mem::transmute(face), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glMatrixMode(mode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glMatrixMode(mode : u32));
    glMatrixMode(core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glMultMatrixd(m: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glMultMatrixd(m : *const f64));
    glMultMatrixd(core::mem::transmute(m))
}
#[inline]
pub unsafe fn glMultMatrixf(m: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glMultMatrixf(m : *const f32));
    glMultMatrixf(core::mem::transmute(m))
}
#[inline]
pub unsafe fn glNewList(list: u32, mode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glNewList(list : u32, mode : u32));
    glNewList(core::mem::transmute(list), core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glNormal3b(nx: i8, ny: i8, nz: i8) {
    windows_targets::link!("opengl32.dll" "system" fn glNormal3b(nx : i8, ny : i8, nz : i8));
    glNormal3b(core::mem::transmute(nx), core::mem::transmute(ny), core::mem::transmute(nz))
}
#[inline]
pub unsafe fn glNormal3bv(v: *const i8) {
    windows_targets::link!("opengl32.dll" "system" fn glNormal3bv(v : *const i8));
    glNormal3bv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glNormal3d(nx: f64, ny: f64, nz: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glNormal3d(nx : f64, ny : f64, nz : f64));
    glNormal3d(core::mem::transmute(nx), core::mem::transmute(ny), core::mem::transmute(nz))
}
#[inline]
pub unsafe fn glNormal3dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glNormal3dv(v : *const f64));
    glNormal3dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glNormal3f(nx: f32, ny: f32, nz: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glNormal3f(nx : f32, ny : f32, nz : f32));
    glNormal3f(core::mem::transmute(nx), core::mem::transmute(ny), core::mem::transmute(nz))
}
#[inline]
pub unsafe fn glNormal3fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glNormal3fv(v : *const f32));
    glNormal3fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glNormal3i(nx: i32, ny: i32, nz: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glNormal3i(nx : i32, ny : i32, nz : i32));
    glNormal3i(core::mem::transmute(nx), core::mem::transmute(ny), core::mem::transmute(nz))
}
#[inline]
pub unsafe fn glNormal3iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glNormal3iv(v : *const i32));
    glNormal3iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glNormal3s(nx: i16, ny: i16, nz: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glNormal3s(nx : i16, ny : i16, nz : i16));
    glNormal3s(core::mem::transmute(nx), core::mem::transmute(ny), core::mem::transmute(nz))
}
#[inline]
pub unsafe fn glNormal3sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glNormal3sv(v : *const i16));
    glNormal3sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glNormalPointer(r#type: u32, stride: i32, pointer: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glNormalPointer(r#type : u32, stride : i32, pointer : *const core::ffi::c_void));
    glNormalPointer(core::mem::transmute(r#type), core::mem::transmute(stride), core::mem::transmute(pointer))
}
#[inline]
pub unsafe fn glOrtho(left: f64, right: f64, bottom: f64, top: f64, znear: f64, zfar: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glOrtho(left : f64, right : f64, bottom : f64, top : f64, znear : f64, zfar : f64));
    glOrtho(core::mem::transmute(left), core::mem::transmute(right), core::mem::transmute(bottom), core::mem::transmute(top), core::mem::transmute(znear), core::mem::transmute(zfar))
}
#[inline]
pub unsafe fn glPassThrough(token: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glPassThrough(token : f32));
    glPassThrough(core::mem::transmute(token))
}
#[inline]
pub unsafe fn glPixelMapfv(map: u32, mapsize: i32, values: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glPixelMapfv(map : u32, mapsize : i32, values : *const f32));
    glPixelMapfv(core::mem::transmute(map), core::mem::transmute(mapsize), core::mem::transmute(values))
}
#[inline]
pub unsafe fn glPixelMapuiv(map: u32, mapsize: i32, values: *const u32) {
    windows_targets::link!("opengl32.dll" "system" fn glPixelMapuiv(map : u32, mapsize : i32, values : *const u32));
    glPixelMapuiv(core::mem::transmute(map), core::mem::transmute(mapsize), core::mem::transmute(values))
}
#[inline]
pub unsafe fn glPixelMapusv(map: u32, mapsize: i32, values: *const u16) {
    windows_targets::link!("opengl32.dll" "system" fn glPixelMapusv(map : u32, mapsize : i32, values : *const u16));
    glPixelMapusv(core::mem::transmute(map), core::mem::transmute(mapsize), core::mem::transmute(values))
}
#[inline]
pub unsafe fn glPixelStoref(pname: u32, param1: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glPixelStoref(pname : u32, param1 : f32));
    glPixelStoref(core::mem::transmute(pname), core::mem::transmute(param1))
}
#[inline]
pub unsafe fn glPixelStorei(pname: u32, param1: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glPixelStorei(pname : u32, param1 : i32));
    glPixelStorei(core::mem::transmute(pname), core::mem::transmute(param1))
}
#[inline]
pub unsafe fn glPixelTransferf(pname: u32, param1: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glPixelTransferf(pname : u32, param1 : f32));
    glPixelTransferf(core::mem::transmute(pname), core::mem::transmute(param1))
}
#[inline]
pub unsafe fn glPixelTransferi(pname: u32, param1: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glPixelTransferi(pname : u32, param1 : i32));
    glPixelTransferi(core::mem::transmute(pname), core::mem::transmute(param1))
}
#[inline]
pub unsafe fn glPixelZoom(xfactor: f32, yfactor: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glPixelZoom(xfactor : f32, yfactor : f32));
    glPixelZoom(core::mem::transmute(xfactor), core::mem::transmute(yfactor))
}
#[inline]
pub unsafe fn glPointSize(size: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glPointSize(size : f32));
    glPointSize(core::mem::transmute(size))
}
#[inline]
pub unsafe fn glPolygonMode(face: u32, mode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glPolygonMode(face : u32, mode : u32));
    glPolygonMode(core::mem::transmute(face), core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glPolygonOffset(factor: f32, units: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glPolygonOffset(factor : f32, units : f32));
    glPolygonOffset(core::mem::transmute(factor), core::mem::transmute(units))
}
#[inline]
pub unsafe fn glPolygonStipple(mask: *const u8) {
    windows_targets::link!("opengl32.dll" "system" fn glPolygonStipple(mask : *const u8));
    glPolygonStipple(core::mem::transmute(mask))
}
#[inline]
pub unsafe fn glPopAttrib() {
    windows_targets::link!("opengl32.dll" "system" fn glPopAttrib());
    glPopAttrib()
}
#[inline]
pub unsafe fn glPopClientAttrib() {
    windows_targets::link!("opengl32.dll" "system" fn glPopClientAttrib());
    glPopClientAttrib()
}
#[inline]
pub unsafe fn glPopMatrix() {
    windows_targets::link!("opengl32.dll" "system" fn glPopMatrix());
    glPopMatrix()
}
#[inline]
pub unsafe fn glPopName() {
    windows_targets::link!("opengl32.dll" "system" fn glPopName());
    glPopName()
}
#[inline]
pub unsafe fn glPrioritizeTextures(n: i32, textures: *const u32, priorities: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glPrioritizeTextures(n : i32, textures : *const u32, priorities : *const f32));
    glPrioritizeTextures(core::mem::transmute(n), core::mem::transmute(textures), core::mem::transmute(priorities))
}
#[inline]
pub unsafe fn glPushAttrib(mask: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glPushAttrib(mask : u32));
    glPushAttrib(core::mem::transmute(mask))
}
#[inline]
pub unsafe fn glPushClientAttrib(mask: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glPushClientAttrib(mask : u32));
    glPushClientAttrib(core::mem::transmute(mask))
}
#[inline]
pub unsafe fn glPushMatrix() {
    windows_targets::link!("opengl32.dll" "system" fn glPushMatrix());
    glPushMatrix()
}
#[inline]
pub unsafe fn glPushName(name: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glPushName(name : u32));
    glPushName(core::mem::transmute(name))
}
#[inline]
pub unsafe fn glRasterPos2d(x: f64, y: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos2d(x : f64, y : f64));
    glRasterPos2d(core::mem::transmute(x), core::mem::transmute(y))
}
#[inline]
pub unsafe fn glRasterPos2dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos2dv(v : *const f64));
    glRasterPos2dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glRasterPos2f(x: f32, y: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos2f(x : f32, y : f32));
    glRasterPos2f(core::mem::transmute(x), core::mem::transmute(y))
}
#[inline]
pub unsafe fn glRasterPos2fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos2fv(v : *const f32));
    glRasterPos2fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glRasterPos2i(x: i32, y: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos2i(x : i32, y : i32));
    glRasterPos2i(core::mem::transmute(x), core::mem::transmute(y))
}
#[inline]
pub unsafe fn glRasterPos2iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos2iv(v : *const i32));
    glRasterPos2iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glRasterPos2s(x: i16, y: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos2s(x : i16, y : i16));
    glRasterPos2s(core::mem::transmute(x), core::mem::transmute(y))
}
#[inline]
pub unsafe fn glRasterPos2sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos2sv(v : *const i16));
    glRasterPos2sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glRasterPos3d(x: f64, y: f64, z: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos3d(x : f64, y : f64, z : f64));
    glRasterPos3d(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glRasterPos3dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos3dv(v : *const f64));
    glRasterPos3dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glRasterPos3f(x: f32, y: f32, z: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos3f(x : f32, y : f32, z : f32));
    glRasterPos3f(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glRasterPos3fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos3fv(v : *const f32));
    glRasterPos3fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glRasterPos3i(x: i32, y: i32, z: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos3i(x : i32, y : i32, z : i32));
    glRasterPos3i(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glRasterPos3iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos3iv(v : *const i32));
    glRasterPos3iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glRasterPos3s(x: i16, y: i16, z: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos3s(x : i16, y : i16, z : i16));
    glRasterPos3s(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glRasterPos3sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos3sv(v : *const i16));
    glRasterPos3sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glRasterPos4d(x: f64, y: f64, z: f64, w: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos4d(x : f64, y : f64, z : f64, w : f64));
    glRasterPos4d(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z), core::mem::transmute(w))
}
#[inline]
pub unsafe fn glRasterPos4dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos4dv(v : *const f64));
    glRasterPos4dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glRasterPos4f(x: f32, y: f32, z: f32, w: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos4f(x : f32, y : f32, z : f32, w : f32));
    glRasterPos4f(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z), core::mem::transmute(w))
}
#[inline]
pub unsafe fn glRasterPos4fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos4fv(v : *const f32));
    glRasterPos4fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glRasterPos4i(x: i32, y: i32, z: i32, w: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos4i(x : i32, y : i32, z : i32, w : i32));
    glRasterPos4i(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z), core::mem::transmute(w))
}
#[inline]
pub unsafe fn glRasterPos4iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos4iv(v : *const i32));
    glRasterPos4iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glRasterPos4s(x: i16, y: i16, z: i16, w: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos4s(x : i16, y : i16, z : i16, w : i16));
    glRasterPos4s(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z), core::mem::transmute(w))
}
#[inline]
pub unsafe fn glRasterPos4sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glRasterPos4sv(v : *const i16));
    glRasterPos4sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glReadBuffer(mode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glReadBuffer(mode : u32));
    glReadBuffer(core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glReadPixels(x: i32, y: i32, width: i32, height: i32, format: u32, r#type: u32, pixels: *mut core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glReadPixels(x : i32, y : i32, width : i32, height : i32, format : u32, r#type : u32, pixels : *mut core::ffi::c_void));
    glReadPixels(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(width), core::mem::transmute(height), core::mem::transmute(format), core::mem::transmute(r#type), core::mem::transmute(pixels))
}
#[inline]
pub unsafe fn glRectd(x1: f64, y1: f64, x2: f64, y2: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glRectd(x1 : f64, y1 : f64, x2 : f64, y2 : f64));
    glRectd(core::mem::transmute(x1), core::mem::transmute(y1), core::mem::transmute(x2), core::mem::transmute(y2))
}
#[inline]
pub unsafe fn glRectdv(v1: *const f64, v2: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glRectdv(v1 : *const f64, v2 : *const f64));
    glRectdv(core::mem::transmute(v1), core::mem::transmute(v2))
}
#[inline]
pub unsafe fn glRectf(x1: f32, y1: f32, x2: f32, y2: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glRectf(x1 : f32, y1 : f32, x2 : f32, y2 : f32));
    glRectf(core::mem::transmute(x1), core::mem::transmute(y1), core::mem::transmute(x2), core::mem::transmute(y2))
}
#[inline]
pub unsafe fn glRectfv(v1: *const f32, v2: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glRectfv(v1 : *const f32, v2 : *const f32));
    glRectfv(core::mem::transmute(v1), core::mem::transmute(v2))
}
#[inline]
pub unsafe fn glRecti(x1: i32, y1: i32, x2: i32, y2: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glRecti(x1 : i32, y1 : i32, x2 : i32, y2 : i32));
    glRecti(core::mem::transmute(x1), core::mem::transmute(y1), core::mem::transmute(x2), core::mem::transmute(y2))
}
#[inline]
pub unsafe fn glRectiv(v1: *const i32, v2: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glRectiv(v1 : *const i32, v2 : *const i32));
    glRectiv(core::mem::transmute(v1), core::mem::transmute(v2))
}
#[inline]
pub unsafe fn glRects(x1: i16, y1: i16, x2: i16, y2: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glRects(x1 : i16, y1 : i16, x2 : i16, y2 : i16));
    glRects(core::mem::transmute(x1), core::mem::transmute(y1), core::mem::transmute(x2), core::mem::transmute(y2))
}
#[inline]
pub unsafe fn glRectsv(v1: *const i16, v2: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glRectsv(v1 : *const i16, v2 : *const i16));
    glRectsv(core::mem::transmute(v1), core::mem::transmute(v2))
}
#[inline]
pub unsafe fn glRenderMode(mode: u32) -> i32 {
    windows_targets::link!("opengl32.dll" "system" fn glRenderMode(mode : u32) -> i32);
    glRenderMode(core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glRotated(angle: f64, x: f64, y: f64, z: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glRotated(angle : f64, x : f64, y : f64, z : f64));
    glRotated(core::mem::transmute(angle), core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glRotatef(angle: f32, x: f32, y: f32, z: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glRotatef(angle : f32, x : f32, y : f32, z : f32));
    glRotatef(core::mem::transmute(angle), core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glScaled(x: f64, y: f64, z: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glScaled(x : f64, y : f64, z : f64));
    glScaled(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glScalef(x: f32, y: f32, z: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glScalef(x : f32, y : f32, z : f32));
    glScalef(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glScissor(x: i32, y: i32, width: i32, height: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glScissor(x : i32, y : i32, width : i32, height : i32));
    glScissor(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(width), core::mem::transmute(height))
}
#[inline]
pub unsafe fn glSelectBuffer(size: i32, buffer: *mut u32) {
    windows_targets::link!("opengl32.dll" "system" fn glSelectBuffer(size : i32, buffer : *mut u32));
    glSelectBuffer(core::mem::transmute(size), core::mem::transmute(buffer))
}
#[inline]
pub unsafe fn glShadeModel(mode: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glShadeModel(mode : u32));
    glShadeModel(core::mem::transmute(mode))
}
#[inline]
pub unsafe fn glStencilFunc(func: u32, r#ref: i32, mask: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glStencilFunc(func : u32, r#ref : i32, mask : u32));
    glStencilFunc(core::mem::transmute(func), core::mem::transmute(r#ref), core::mem::transmute(mask))
}
#[inline]
pub unsafe fn glStencilMask(mask: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glStencilMask(mask : u32));
    glStencilMask(core::mem::transmute(mask))
}
#[inline]
pub unsafe fn glStencilOp(fail: u32, zfail: u32, zpass: u32) {
    windows_targets::link!("opengl32.dll" "system" fn glStencilOp(fail : u32, zfail : u32, zpass : u32));
    glStencilOp(core::mem::transmute(fail), core::mem::transmute(zfail), core::mem::transmute(zpass))
}
#[inline]
pub unsafe fn glTexCoord1d(s: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord1d(s : f64));
    glTexCoord1d(core::mem::transmute(s))
}
#[inline]
pub unsafe fn glTexCoord1dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord1dv(v : *const f64));
    glTexCoord1dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord1f(s: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord1f(s : f32));
    glTexCoord1f(core::mem::transmute(s))
}
#[inline]
pub unsafe fn glTexCoord1fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord1fv(v : *const f32));
    glTexCoord1fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord1i(s: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord1i(s : i32));
    glTexCoord1i(core::mem::transmute(s))
}
#[inline]
pub unsafe fn glTexCoord1iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord1iv(v : *const i32));
    glTexCoord1iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord1s(s: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord1s(s : i16));
    glTexCoord1s(core::mem::transmute(s))
}
#[inline]
pub unsafe fn glTexCoord1sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord1sv(v : *const i16));
    glTexCoord1sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord2d(s: f64, t: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord2d(s : f64, t : f64));
    glTexCoord2d(core::mem::transmute(s), core::mem::transmute(t))
}
#[inline]
pub unsafe fn glTexCoord2dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord2dv(v : *const f64));
    glTexCoord2dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord2f(s: f32, t: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord2f(s : f32, t : f32));
    glTexCoord2f(core::mem::transmute(s), core::mem::transmute(t))
}
#[inline]
pub unsafe fn glTexCoord2fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord2fv(v : *const f32));
    glTexCoord2fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord2i(s: i32, t: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord2i(s : i32, t : i32));
    glTexCoord2i(core::mem::transmute(s), core::mem::transmute(t))
}
#[inline]
pub unsafe fn glTexCoord2iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord2iv(v : *const i32));
    glTexCoord2iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord2s(s: i16, t: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord2s(s : i16, t : i16));
    glTexCoord2s(core::mem::transmute(s), core::mem::transmute(t))
}
#[inline]
pub unsafe fn glTexCoord2sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord2sv(v : *const i16));
    glTexCoord2sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord3d(s: f64, t: f64, r: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord3d(s : f64, t : f64, r : f64));
    glTexCoord3d(core::mem::transmute(s), core::mem::transmute(t), core::mem::transmute(r))
}
#[inline]
pub unsafe fn glTexCoord3dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord3dv(v : *const f64));
    glTexCoord3dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord3f(s: f32, t: f32, r: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord3f(s : f32, t : f32, r : f32));
    glTexCoord3f(core::mem::transmute(s), core::mem::transmute(t), core::mem::transmute(r))
}
#[inline]
pub unsafe fn glTexCoord3fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord3fv(v : *const f32));
    glTexCoord3fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord3i(s: i32, t: i32, r: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord3i(s : i32, t : i32, r : i32));
    glTexCoord3i(core::mem::transmute(s), core::mem::transmute(t), core::mem::transmute(r))
}
#[inline]
pub unsafe fn glTexCoord3iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord3iv(v : *const i32));
    glTexCoord3iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord3s(s: i16, t: i16, r: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord3s(s : i16, t : i16, r : i16));
    glTexCoord3s(core::mem::transmute(s), core::mem::transmute(t), core::mem::transmute(r))
}
#[inline]
pub unsafe fn glTexCoord3sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord3sv(v : *const i16));
    glTexCoord3sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord4d(s: f64, t: f64, r: f64, q: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord4d(s : f64, t : f64, r : f64, q : f64));
    glTexCoord4d(core::mem::transmute(s), core::mem::transmute(t), core::mem::transmute(r), core::mem::transmute(q))
}
#[inline]
pub unsafe fn glTexCoord4dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord4dv(v : *const f64));
    glTexCoord4dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord4f(s: f32, t: f32, r: f32, q: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord4f(s : f32, t : f32, r : f32, q : f32));
    glTexCoord4f(core::mem::transmute(s), core::mem::transmute(t), core::mem::transmute(r), core::mem::transmute(q))
}
#[inline]
pub unsafe fn glTexCoord4fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord4fv(v : *const f32));
    glTexCoord4fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord4i(s: i32, t: i32, r: i32, q: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord4i(s : i32, t : i32, r : i32, q : i32));
    glTexCoord4i(core::mem::transmute(s), core::mem::transmute(t), core::mem::transmute(r), core::mem::transmute(q))
}
#[inline]
pub unsafe fn glTexCoord4iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord4iv(v : *const i32));
    glTexCoord4iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoord4s(s: i16, t: i16, r: i16, q: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord4s(s : i16, t : i16, r : i16, q : i16));
    glTexCoord4s(core::mem::transmute(s), core::mem::transmute(t), core::mem::transmute(r), core::mem::transmute(q))
}
#[inline]
pub unsafe fn glTexCoord4sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoord4sv(v : *const i16));
    glTexCoord4sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glTexCoordPointer(size: i32, r#type: u32, stride: i32, pointer: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glTexCoordPointer(size : i32, r#type : u32, stride : i32, pointer : *const core::ffi::c_void));
    glTexCoordPointer(core::mem::transmute(size), core::mem::transmute(r#type), core::mem::transmute(stride), core::mem::transmute(pointer))
}
#[inline]
pub unsafe fn glTexEnvf(target: u32, pname: u32, param2: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexEnvf(target : u32, pname : u32, param2 : f32));
    glTexEnvf(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(param2))
}
#[inline]
pub unsafe fn glTexEnvfv(target: u32, pname: u32, params: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexEnvfv(target : u32, pname : u32, params : *const f32));
    glTexEnvfv(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glTexEnvi(target: u32, pname: u32, param2: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexEnvi(target : u32, pname : u32, param2 : i32));
    glTexEnvi(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(param2))
}
#[inline]
pub unsafe fn glTexEnviv(target: u32, pname: u32, params: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexEnviv(target : u32, pname : u32, params : *const i32));
    glTexEnviv(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glTexGend(coord: u32, pname: u32, param2: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glTexGend(coord : u32, pname : u32, param2 : f64));
    glTexGend(core::mem::transmute(coord), core::mem::transmute(pname), core::mem::transmute(param2))
}
#[inline]
pub unsafe fn glTexGendv(coord: u32, pname: u32, params: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glTexGendv(coord : u32, pname : u32, params : *const f64));
    glTexGendv(core::mem::transmute(coord), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glTexGenf(coord: u32, pname: u32, param2: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexGenf(coord : u32, pname : u32, param2 : f32));
    glTexGenf(core::mem::transmute(coord), core::mem::transmute(pname), core::mem::transmute(param2))
}
#[inline]
pub unsafe fn glTexGenfv(coord: u32, pname: u32, params: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexGenfv(coord : u32, pname : u32, params : *const f32));
    glTexGenfv(core::mem::transmute(coord), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glTexGeni(coord: u32, pname: u32, param2: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexGeni(coord : u32, pname : u32, param2 : i32));
    glTexGeni(core::mem::transmute(coord), core::mem::transmute(pname), core::mem::transmute(param2))
}
#[inline]
pub unsafe fn glTexGeniv(coord: u32, pname: u32, params: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexGeniv(coord : u32, pname : u32, params : *const i32));
    glTexGeniv(core::mem::transmute(coord), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glTexImage1D(target: u32, level: i32, internalformat: i32, width: i32, border: i32, format: u32, r#type: u32, pixels: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glTexImage1D(target : u32, level : i32, internalformat : i32, width : i32, border : i32, format : u32, r#type : u32, pixels : *const core::ffi::c_void));
    glTexImage1D(core::mem::transmute(target), core::mem::transmute(level), core::mem::transmute(internalformat), core::mem::transmute(width), core::mem::transmute(border), core::mem::transmute(format), core::mem::transmute(r#type), core::mem::transmute(pixels))
}
#[inline]
pub unsafe fn glTexImage2D(target: u32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: u32, r#type: u32, pixels: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glTexImage2D(target : u32, level : i32, internalformat : i32, width : i32, height : i32, border : i32, format : u32, r#type : u32, pixels : *const core::ffi::c_void));
    glTexImage2D(core::mem::transmute(target), core::mem::transmute(level), core::mem::transmute(internalformat), core::mem::transmute(width), core::mem::transmute(height), core::mem::transmute(border), core::mem::transmute(format), core::mem::transmute(r#type), core::mem::transmute(pixels))
}
#[inline]
pub unsafe fn glTexParameterf(target: u32, pname: u32, param2: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexParameterf(target : u32, pname : u32, param2 : f32));
    glTexParameterf(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(param2))
}
#[inline]
pub unsafe fn glTexParameterfv(target: u32, pname: u32, params: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexParameterfv(target : u32, pname : u32, params : *const f32));
    glTexParameterfv(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glTexParameteri(target: u32, pname: u32, param2: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexParameteri(target : u32, pname : u32, param2 : i32));
    glTexParameteri(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(param2))
}
#[inline]
pub unsafe fn glTexParameteriv(target: u32, pname: u32, params: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glTexParameteriv(target : u32, pname : u32, params : *const i32));
    glTexParameteriv(core::mem::transmute(target), core::mem::transmute(pname), core::mem::transmute(params))
}
#[inline]
pub unsafe fn glTexSubImage1D(target: u32, level: i32, xoffset: i32, width: i32, format: u32, r#type: u32, pixels: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glTexSubImage1D(target : u32, level : i32, xoffset : i32, width : i32, format : u32, r#type : u32, pixels : *const core::ffi::c_void));
    glTexSubImage1D(core::mem::transmute(target), core::mem::transmute(level), core::mem::transmute(xoffset), core::mem::transmute(width), core::mem::transmute(format), core::mem::transmute(r#type), core::mem::transmute(pixels))
}
#[inline]
pub unsafe fn glTexSubImage2D(target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, r#type: u32, pixels: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glTexSubImage2D(target : u32, level : i32, xoffset : i32, yoffset : i32, width : i32, height : i32, format : u32, r#type : u32, pixels : *const core::ffi::c_void));
    glTexSubImage2D(core::mem::transmute(target), core::mem::transmute(level), core::mem::transmute(xoffset), core::mem::transmute(yoffset), core::mem::transmute(width), core::mem::transmute(height), core::mem::transmute(format), core::mem::transmute(r#type), core::mem::transmute(pixels))
}
#[inline]
pub unsafe fn glTranslated(x: f64, y: f64, z: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glTranslated(x : f64, y : f64, z : f64));
    glTranslated(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glTranslatef(x: f32, y: f32, z: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glTranslatef(x : f32, y : f32, z : f32));
    glTranslatef(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glVertex2d(x: f64, y: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex2d(x : f64, y : f64));
    glVertex2d(core::mem::transmute(x), core::mem::transmute(y))
}
#[inline]
pub unsafe fn glVertex2dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex2dv(v : *const f64));
    glVertex2dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertex2f(x: f32, y: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex2f(x : f32, y : f32));
    glVertex2f(core::mem::transmute(x), core::mem::transmute(y))
}
#[inline]
pub unsafe fn glVertex2fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex2fv(v : *const f32));
    glVertex2fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertex2i(x: i32, y: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex2i(x : i32, y : i32));
    glVertex2i(core::mem::transmute(x), core::mem::transmute(y))
}
#[inline]
pub unsafe fn glVertex2iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex2iv(v : *const i32));
    glVertex2iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertex2s(x: i16, y: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex2s(x : i16, y : i16));
    glVertex2s(core::mem::transmute(x), core::mem::transmute(y))
}
#[inline]
pub unsafe fn glVertex2sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex2sv(v : *const i16));
    glVertex2sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertex3d(x: f64, y: f64, z: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex3d(x : f64, y : f64, z : f64));
    glVertex3d(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glVertex3dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex3dv(v : *const f64));
    glVertex3dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertex3f(x: f32, y: f32, z: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex3f(x : f32, y : f32, z : f32));
    glVertex3f(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glVertex3fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex3fv(v : *const f32));
    glVertex3fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertex3i(x: i32, y: i32, z: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex3i(x : i32, y : i32, z : i32));
    glVertex3i(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glVertex3iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex3iv(v : *const i32));
    glVertex3iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertex3s(x: i16, y: i16, z: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex3s(x : i16, y : i16, z : i16));
    glVertex3s(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn glVertex3sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex3sv(v : *const i16));
    glVertex3sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertex4d(x: f64, y: f64, z: f64, w: f64) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex4d(x : f64, y : f64, z : f64, w : f64));
    glVertex4d(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z), core::mem::transmute(w))
}
#[inline]
pub unsafe fn glVertex4dv(v: *const f64) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex4dv(v : *const f64));
    glVertex4dv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertex4f(x: f32, y: f32, z: f32, w: f32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex4f(x : f32, y : f32, z : f32, w : f32));
    glVertex4f(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z), core::mem::transmute(w))
}
#[inline]
pub unsafe fn glVertex4fv(v: *const f32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex4fv(v : *const f32));
    glVertex4fv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertex4i(x: i32, y: i32, z: i32, w: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex4i(x : i32, y : i32, z : i32, w : i32));
    glVertex4i(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z), core::mem::transmute(w))
}
#[inline]
pub unsafe fn glVertex4iv(v: *const i32) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex4iv(v : *const i32));
    glVertex4iv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertex4s(x: i16, y: i16, z: i16, w: i16) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex4s(x : i16, y : i16, z : i16, w : i16));
    glVertex4s(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z), core::mem::transmute(w))
}
#[inline]
pub unsafe fn glVertex4sv(v: *const i16) {
    windows_targets::link!("opengl32.dll" "system" fn glVertex4sv(v : *const i16));
    glVertex4sv(core::mem::transmute(v))
}
#[inline]
pub unsafe fn glVertexPointer(size: i32, r#type: u32, stride: i32, pointer: *const core::ffi::c_void) {
    windows_targets::link!("opengl32.dll" "system" fn glVertexPointer(size : i32, r#type : u32, stride : i32, pointer : *const core::ffi::c_void));
    glVertexPointer(core::mem::transmute(size), core::mem::transmute(r#type), core::mem::transmute(stride), core::mem::transmute(pointer))
}
#[inline]
pub unsafe fn glViewport(x: i32, y: i32, width: i32, height: i32) {
    windows_targets::link!("opengl32.dll" "system" fn glViewport(x : i32, y : i32, width : i32, height : i32));
    glViewport(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(width), core::mem::transmute(height))
}
#[inline]
pub unsafe fn gluBeginCurve(nobj: *mut GLUnurbs) {
    windows_targets::link!("glu32.dll" "system" fn gluBeginCurve(nobj : *mut GLUnurbs));
    gluBeginCurve(core::mem::transmute(nobj))
}
#[inline]
pub unsafe fn gluBeginPolygon(tess: *mut GLUtesselator) {
    windows_targets::link!("glu32.dll" "system" fn gluBeginPolygon(tess : *mut GLUtesselator));
    gluBeginPolygon(core::mem::transmute(tess))
}
#[inline]
pub unsafe fn gluBeginSurface(nobj: *mut GLUnurbs) {
    windows_targets::link!("glu32.dll" "system" fn gluBeginSurface(nobj : *mut GLUnurbs));
    gluBeginSurface(core::mem::transmute(nobj))
}
#[inline]
pub unsafe fn gluBeginTrim(nobj: *mut GLUnurbs) {
    windows_targets::link!("glu32.dll" "system" fn gluBeginTrim(nobj : *mut GLUnurbs));
    gluBeginTrim(core::mem::transmute(nobj))
}
#[inline]
pub unsafe fn gluBuild1DMipmaps(target: u32, components: i32, width: i32, format: u32, r#type: u32, data: *const core::ffi::c_void) -> i32 {
    windows_targets::link!("glu32.dll" "system" fn gluBuild1DMipmaps(target : u32, components : i32, width : i32, format : u32, r#type : u32, data : *const core::ffi::c_void) -> i32);
    gluBuild1DMipmaps(core::mem::transmute(target), core::mem::transmute(components), core::mem::transmute(width), core::mem::transmute(format), core::mem::transmute(r#type), core::mem::transmute(data))
}
#[inline]
pub unsafe fn gluBuild2DMipmaps(target: u32, components: i32, width: i32, height: i32, format: u32, r#type: u32, data: *const core::ffi::c_void) -> i32 {
    windows_targets::link!("glu32.dll" "system" fn gluBuild2DMipmaps(target : u32, components : i32, width : i32, height : i32, format : u32, r#type : u32, data : *const core::ffi::c_void) -> i32);
    gluBuild2DMipmaps(core::mem::transmute(target), core::mem::transmute(components), core::mem::transmute(width), core::mem::transmute(height), core::mem::transmute(format), core::mem::transmute(r#type), core::mem::transmute(data))
}
#[inline]
pub unsafe fn gluCylinder(qobj: *mut GLUquadric, baseradius: f64, topradius: f64, height: f64, slices: i32, stacks: i32) {
    windows_targets::link!("glu32.dll" "system" fn gluCylinder(qobj : *mut GLUquadric, baseradius : f64, topradius : f64, height : f64, slices : i32, stacks : i32));
    gluCylinder(core::mem::transmute(qobj), core::mem::transmute(baseradius), core::mem::transmute(topradius), core::mem::transmute(height), core::mem::transmute(slices), core::mem::transmute(stacks))
}
#[inline]
pub unsafe fn gluDeleteNurbsRenderer(nobj: *mut GLUnurbs) {
    windows_targets::link!("glu32.dll" "system" fn gluDeleteNurbsRenderer(nobj : *mut GLUnurbs));
    gluDeleteNurbsRenderer(core::mem::transmute(nobj))
}
#[inline]
pub unsafe fn gluDeleteQuadric(state: *mut GLUquadric) {
    windows_targets::link!("glu32.dll" "system" fn gluDeleteQuadric(state : *mut GLUquadric));
    gluDeleteQuadric(core::mem::transmute(state))
}
#[inline]
pub unsafe fn gluDeleteTess(tess: *mut GLUtesselator) {
    windows_targets::link!("glu32.dll" "system" fn gluDeleteTess(tess : *mut GLUtesselator));
    gluDeleteTess(core::mem::transmute(tess))
}
#[inline]
pub unsafe fn gluDisk(qobj: *mut GLUquadric, innerradius: f64, outerradius: f64, slices: i32, loops: i32) {
    windows_targets::link!("glu32.dll" "system" fn gluDisk(qobj : *mut GLUquadric, innerradius : f64, outerradius : f64, slices : i32, loops : i32));
    gluDisk(core::mem::transmute(qobj), core::mem::transmute(innerradius), core::mem::transmute(outerradius), core::mem::transmute(slices), core::mem::transmute(loops))
}
#[inline]
pub unsafe fn gluEndCurve(nobj: *mut GLUnurbs) {
    windows_targets::link!("glu32.dll" "system" fn gluEndCurve(nobj : *mut GLUnurbs));
    gluEndCurve(core::mem::transmute(nobj))
}
#[inline]
pub unsafe fn gluEndPolygon(tess: *mut GLUtesselator) {
    windows_targets::link!("glu32.dll" "system" fn gluEndPolygon(tess : *mut GLUtesselator));
    gluEndPolygon(core::mem::transmute(tess))
}
#[inline]
pub unsafe fn gluEndSurface(nobj: *mut GLUnurbs) {
    windows_targets::link!("glu32.dll" "system" fn gluEndSurface(nobj : *mut GLUnurbs));
    gluEndSurface(core::mem::transmute(nobj))
}
#[inline]
pub unsafe fn gluEndTrim(nobj: *mut GLUnurbs) {
    windows_targets::link!("glu32.dll" "system" fn gluEndTrim(nobj : *mut GLUnurbs));
    gluEndTrim(core::mem::transmute(nobj))
}
#[inline]
pub unsafe fn gluErrorString(errcode: u32) -> *mut u8 {
    windows_targets::link!("glu32.dll" "system" fn gluErrorString(errcode : u32) -> *mut u8);
    gluErrorString(core::mem::transmute(errcode))
}
#[inline]
pub unsafe fn gluErrorUnicodeStringEXT(errcode: u32) -> windows_core::PCWSTR {
    windows_targets::link!("glu32.dll" "system" fn gluErrorUnicodeStringEXT(errcode : u32) -> windows_core::PCWSTR);
    gluErrorUnicodeStringEXT(core::mem::transmute(errcode))
}
#[inline]
pub unsafe fn gluGetNurbsProperty(nobj: *mut GLUnurbs, property: u32, value: *mut f32) {
    windows_targets::link!("glu32.dll" "system" fn gluGetNurbsProperty(nobj : *mut GLUnurbs, property : u32, value : *mut f32));
    gluGetNurbsProperty(core::mem::transmute(nobj), core::mem::transmute(property), core::mem::transmute(value))
}
#[inline]
pub unsafe fn gluGetString(name: u32) -> *mut u8 {
    windows_targets::link!("glu32.dll" "system" fn gluGetString(name : u32) -> *mut u8);
    gluGetString(core::mem::transmute(name))
}
#[inline]
pub unsafe fn gluGetTessProperty(tess: *mut GLUtesselator, which: u32, value: *mut f64) {
    windows_targets::link!("glu32.dll" "system" fn gluGetTessProperty(tess : *mut GLUtesselator, which : u32, value : *mut f64));
    gluGetTessProperty(core::mem::transmute(tess), core::mem::transmute(which), core::mem::transmute(value))
}
#[inline]
pub unsafe fn gluLoadSamplingMatrices(nobj: *mut GLUnurbs, modelmatrix: *const f32, projmatrix: *const f32, viewport: *const i32) {
    windows_targets::link!("glu32.dll" "system" fn gluLoadSamplingMatrices(nobj : *mut GLUnurbs, modelmatrix : *const f32, projmatrix : *const f32, viewport : *const i32));
    gluLoadSamplingMatrices(core::mem::transmute(nobj), core::mem::transmute(modelmatrix), core::mem::transmute(projmatrix), core::mem::transmute(viewport))
}
#[inline]
pub unsafe fn gluLookAt(eyex: f64, eyey: f64, eyez: f64, centerx: f64, centery: f64, centerz: f64, upx: f64, upy: f64, upz: f64) {
    windows_targets::link!("glu32.dll" "system" fn gluLookAt(eyex : f64, eyey : f64, eyez : f64, centerx : f64, centery : f64, centerz : f64, upx : f64, upy : f64, upz : f64));
    gluLookAt(core::mem::transmute(eyex), core::mem::transmute(eyey), core::mem::transmute(eyez), core::mem::transmute(centerx), core::mem::transmute(centery), core::mem::transmute(centerz), core::mem::transmute(upx), core::mem::transmute(upy), core::mem::transmute(upz))
}
#[inline]
pub unsafe fn gluNewNurbsRenderer() -> *mut GLUnurbs {
    windows_targets::link!("glu32.dll" "system" fn gluNewNurbsRenderer() -> *mut GLUnurbs);
    gluNewNurbsRenderer()
}
#[inline]
pub unsafe fn gluNewQuadric() -> *mut GLUquadric {
    windows_targets::link!("glu32.dll" "system" fn gluNewQuadric() -> *mut GLUquadric);
    gluNewQuadric()
}
#[inline]
pub unsafe fn gluNewTess() -> *mut GLUtesselator {
    windows_targets::link!("glu32.dll" "system" fn gluNewTess() -> *mut GLUtesselator);
    gluNewTess()
}
#[inline]
pub unsafe fn gluNextContour(tess: *mut GLUtesselator, r#type: u32) {
    windows_targets::link!("glu32.dll" "system" fn gluNextContour(tess : *mut GLUtesselator, r#type : u32));
    gluNextContour(core::mem::transmute(tess), core::mem::transmute(r#type))
}
#[inline]
pub unsafe fn gluNurbsCallback(nobj: *mut GLUnurbs, which: u32, r#fn: isize) {
    windows_targets::link!("glu32.dll" "system" fn gluNurbsCallback(nobj : *mut GLUnurbs, which : u32, r#fn : isize));
    gluNurbsCallback(core::mem::transmute(nobj), core::mem::transmute(which), core::mem::transmute(r#fn))
}
#[inline]
pub unsafe fn gluNurbsCurve(nobj: *mut GLUnurbs, nknots: i32, knot: *mut f32, stride: i32, ctlarray: *mut f32, order: i32, r#type: u32) {
    windows_targets::link!("glu32.dll" "system" fn gluNurbsCurve(nobj : *mut GLUnurbs, nknots : i32, knot : *mut f32, stride : i32, ctlarray : *mut f32, order : i32, r#type : u32));
    gluNurbsCurve(core::mem::transmute(nobj), core::mem::transmute(nknots), core::mem::transmute(knot), core::mem::transmute(stride), core::mem::transmute(ctlarray), core::mem::transmute(order), core::mem::transmute(r#type))
}
#[inline]
pub unsafe fn gluNurbsProperty(nobj: *mut GLUnurbs, property: u32, value: f32) {
    windows_targets::link!("glu32.dll" "system" fn gluNurbsProperty(nobj : *mut GLUnurbs, property : u32, value : f32));
    gluNurbsProperty(core::mem::transmute(nobj), core::mem::transmute(property), core::mem::transmute(value))
}
#[inline]
pub unsafe fn gluNurbsSurface(nobj: *mut GLUnurbs, sknot_count: i32, sknot: *mut f32, tknot_count: i32, tknot: *mut f32, s_stride: i32, t_stride: i32, ctlarray: *mut f32, sorder: i32, torder: i32, r#type: u32) {
    windows_targets::link!("glu32.dll" "system" fn gluNurbsSurface(nobj : *mut GLUnurbs, sknot_count : i32, sknot : *mut f32, tknot_count : i32, tknot : *mut f32, s_stride : i32, t_stride : i32, ctlarray : *mut f32, sorder : i32, torder : i32, r#type : u32));
    gluNurbsSurface(core::mem::transmute(nobj), core::mem::transmute(sknot_count), core::mem::transmute(sknot), core::mem::transmute(tknot_count), core::mem::transmute(tknot), core::mem::transmute(s_stride), core::mem::transmute(t_stride), core::mem::transmute(ctlarray), core::mem::transmute(sorder), core::mem::transmute(torder), core::mem::transmute(r#type))
}
#[inline]
pub unsafe fn gluOrtho2D(left: f64, right: f64, bottom: f64, top: f64) {
    windows_targets::link!("glu32.dll" "system" fn gluOrtho2D(left : f64, right : f64, bottom : f64, top : f64));
    gluOrtho2D(core::mem::transmute(left), core::mem::transmute(right), core::mem::transmute(bottom), core::mem::transmute(top))
}
#[inline]
pub unsafe fn gluPartialDisk(qobj: *mut GLUquadric, innerradius: f64, outerradius: f64, slices: i32, loops: i32, startangle: f64, sweepangle: f64) {
    windows_targets::link!("glu32.dll" "system" fn gluPartialDisk(qobj : *mut GLUquadric, innerradius : f64, outerradius : f64, slices : i32, loops : i32, startangle : f64, sweepangle : f64));
    gluPartialDisk(core::mem::transmute(qobj), core::mem::transmute(innerradius), core::mem::transmute(outerradius), core::mem::transmute(slices), core::mem::transmute(loops), core::mem::transmute(startangle), core::mem::transmute(sweepangle))
}
#[inline]
pub unsafe fn gluPerspective(fovy: f64, aspect: f64, znear: f64, zfar: f64) {
    windows_targets::link!("glu32.dll" "system" fn gluPerspective(fovy : f64, aspect : f64, znear : f64, zfar : f64));
    gluPerspective(core::mem::transmute(fovy), core::mem::transmute(aspect), core::mem::transmute(znear), core::mem::transmute(zfar))
}
#[inline]
pub unsafe fn gluPickMatrix(x: f64, y: f64, width: f64, height: f64, viewport: *mut i32) {
    windows_targets::link!("glu32.dll" "system" fn gluPickMatrix(x : f64, y : f64, width : f64, height : f64, viewport : *mut i32));
    gluPickMatrix(core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(width), core::mem::transmute(height), core::mem::transmute(viewport))
}
#[inline]
pub unsafe fn gluProject(objx: f64, objy: f64, objz: f64, modelmatrix: *const f64, projmatrix: *const f64, viewport: *const i32, winx: *mut f64, winy: *mut f64, winz: *mut f64) -> i32 {
    windows_targets::link!("glu32.dll" "system" fn gluProject(objx : f64, objy : f64, objz : f64, modelmatrix : *const f64, projmatrix : *const f64, viewport : *const i32, winx : *mut f64, winy : *mut f64, winz : *mut f64) -> i32);
    gluProject(core::mem::transmute(objx), core::mem::transmute(objy), core::mem::transmute(objz), core::mem::transmute(modelmatrix), core::mem::transmute(projmatrix), core::mem::transmute(viewport), core::mem::transmute(winx), core::mem::transmute(winy), core::mem::transmute(winz))
}
#[inline]
pub unsafe fn gluPwlCurve(nobj: *mut GLUnurbs, count: i32, array: *mut f32, stride: i32, r#type: u32) {
    windows_targets::link!("glu32.dll" "system" fn gluPwlCurve(nobj : *mut GLUnurbs, count : i32, array : *mut f32, stride : i32, r#type : u32));
    gluPwlCurve(core::mem::transmute(nobj), core::mem::transmute(count), core::mem::transmute(array), core::mem::transmute(stride), core::mem::transmute(r#type))
}
#[inline]
pub unsafe fn gluQuadricCallback(qobj: *mut GLUquadric, which: u32, r#fn: isize) {
    windows_targets::link!("glu32.dll" "system" fn gluQuadricCallback(qobj : *mut GLUquadric, which : u32, r#fn : isize));
    gluQuadricCallback(core::mem::transmute(qobj), core::mem::transmute(which), core::mem::transmute(r#fn))
}
#[inline]
pub unsafe fn gluQuadricDrawStyle(quadobject: *mut GLUquadric, drawstyle: u32) {
    windows_targets::link!("glu32.dll" "system" fn gluQuadricDrawStyle(quadobject : *mut GLUquadric, drawstyle : u32));
    gluQuadricDrawStyle(core::mem::transmute(quadobject), core::mem::transmute(drawstyle))
}
#[inline]
pub unsafe fn gluQuadricNormals(quadobject: *mut GLUquadric, normals: u32) {
    windows_targets::link!("glu32.dll" "system" fn gluQuadricNormals(quadobject : *mut GLUquadric, normals : u32));
    gluQuadricNormals(core::mem::transmute(quadobject), core::mem::transmute(normals))
}
#[inline]
pub unsafe fn gluQuadricOrientation(quadobject: *mut GLUquadric, orientation: u32) {
    windows_targets::link!("glu32.dll" "system" fn gluQuadricOrientation(quadobject : *mut GLUquadric, orientation : u32));
    gluQuadricOrientation(core::mem::transmute(quadobject), core::mem::transmute(orientation))
}
#[inline]
pub unsafe fn gluQuadricTexture(quadobject: *mut GLUquadric, texturecoords: u8) {
    windows_targets::link!("glu32.dll" "system" fn gluQuadricTexture(quadobject : *mut GLUquadric, texturecoords : u8));
    gluQuadricTexture(core::mem::transmute(quadobject), core::mem::transmute(texturecoords))
}
#[inline]
pub unsafe fn gluScaleImage(format: u32, widthin: i32, heightin: i32, typein: u32, datain: *const core::ffi::c_void, widthout: i32, heightout: i32, typeout: u32, dataout: *mut core::ffi::c_void) -> i32 {
    windows_targets::link!("glu32.dll" "system" fn gluScaleImage(format : u32, widthin : i32, heightin : i32, typein : u32, datain : *const core::ffi::c_void, widthout : i32, heightout : i32, typeout : u32, dataout : *mut core::ffi::c_void) -> i32);
    gluScaleImage(core::mem::transmute(format), core::mem::transmute(widthin), core::mem::transmute(heightin), core::mem::transmute(typein), core::mem::transmute(datain), core::mem::transmute(widthout), core::mem::transmute(heightout), core::mem::transmute(typeout), core::mem::transmute(dataout))
}
#[inline]
pub unsafe fn gluSphere(qobj: *mut GLUquadric, radius: f64, slices: i32, stacks: i32) {
    windows_targets::link!("glu32.dll" "system" fn gluSphere(qobj : *mut GLUquadric, radius : f64, slices : i32, stacks : i32));
    gluSphere(core::mem::transmute(qobj), core::mem::transmute(radius), core::mem::transmute(slices), core::mem::transmute(stacks))
}
#[inline]
pub unsafe fn gluTessBeginContour(tess: *mut GLUtesselator) {
    windows_targets::link!("glu32.dll" "system" fn gluTessBeginContour(tess : *mut GLUtesselator));
    gluTessBeginContour(core::mem::transmute(tess))
}
#[inline]
pub unsafe fn gluTessBeginPolygon(tess: *mut GLUtesselator, polygon_data: *mut core::ffi::c_void) {
    windows_targets::link!("glu32.dll" "system" fn gluTessBeginPolygon(tess : *mut GLUtesselator, polygon_data : *mut core::ffi::c_void));
    gluTessBeginPolygon(core::mem::transmute(tess), core::mem::transmute(polygon_data))
}
#[inline]
pub unsafe fn gluTessCallback(tess: *mut GLUtesselator, which: u32, r#fn: isize) {
    windows_targets::link!("glu32.dll" "system" fn gluTessCallback(tess : *mut GLUtesselator, which : u32, r#fn : isize));
    gluTessCallback(core::mem::transmute(tess), core::mem::transmute(which), core::mem::transmute(r#fn))
}
#[inline]
pub unsafe fn gluTessEndContour(tess: *mut GLUtesselator) {
    windows_targets::link!("glu32.dll" "system" fn gluTessEndContour(tess : *mut GLUtesselator));
    gluTessEndContour(core::mem::transmute(tess))
}
#[inline]
pub unsafe fn gluTessEndPolygon(tess: *mut GLUtesselator) {
    windows_targets::link!("glu32.dll" "system" fn gluTessEndPolygon(tess : *mut GLUtesselator));
    gluTessEndPolygon(core::mem::transmute(tess))
}
#[inline]
pub unsafe fn gluTessNormal(tess: *mut GLUtesselator, x: f64, y: f64, z: f64) {
    windows_targets::link!("glu32.dll" "system" fn gluTessNormal(tess : *mut GLUtesselator, x : f64, y : f64, z : f64));
    gluTessNormal(core::mem::transmute(tess), core::mem::transmute(x), core::mem::transmute(y), core::mem::transmute(z))
}
#[inline]
pub unsafe fn gluTessProperty(tess: *mut GLUtesselator, which: u32, value: f64) {
    windows_targets::link!("glu32.dll" "system" fn gluTessProperty(tess : *mut GLUtesselator, which : u32, value : f64));
    gluTessProperty(core::mem::transmute(tess), core::mem::transmute(which), core::mem::transmute(value))
}
#[inline]
pub unsafe fn gluTessVertex(tess: *mut GLUtesselator, coords: *mut f64, data: *mut core::ffi::c_void) {
    windows_targets::link!("glu32.dll" "system" fn gluTessVertex(tess : *mut GLUtesselator, coords : *mut f64, data : *mut core::ffi::c_void));
    gluTessVertex(core::mem::transmute(tess), core::mem::transmute(coords), core::mem::transmute(data))
}
#[inline]
pub unsafe fn gluUnProject(winx: f64, winy: f64, winz: f64, modelmatrix: *const f64, projmatrix: *const f64, viewport: *const i32, objx: *mut f64, objy: *mut f64, objz: *mut f64) -> i32 {
    windows_targets::link!("glu32.dll" "system" fn gluUnProject(winx : f64, winy : f64, winz : f64, modelmatrix : *const f64, projmatrix : *const f64, viewport : *const i32, objx : *mut f64, objy : *mut f64, objz : *mut f64) -> i32);
    gluUnProject(core::mem::transmute(winx), core::mem::transmute(winy), core::mem::transmute(winz), core::mem::transmute(modelmatrix), core::mem::transmute(projmatrix), core::mem::transmute(viewport), core::mem::transmute(objx), core::mem::transmute(objy), core::mem::transmute(objz))
}
#[inline]
pub unsafe fn wglCopyContext(param0: HGLRC, param1: HGLRC, param2: u32) -> windows_core::Result<()> {
    windows_targets::link!("opengl32.dll" "system" fn wglCopyContext(param0 : HGLRC, param1 : HGLRC, param2 : u32) -> super::super::Foundation:: BOOL);
    wglCopyContext(core::mem::transmute(param0), core::mem::transmute(param1), core::mem::transmute(param2)).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglCreateContext(param0: super::Gdi::HDC) -> windows_core::Result<HGLRC> {
    windows_targets::link!("opengl32.dll" "system" fn wglCreateContext(param0 : super::Gdi:: HDC) -> HGLRC);
    let result__ = wglCreateContext(core::mem::transmute(param0));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglCreateLayerContext(param0: super::Gdi::HDC, param1: i32) -> windows_core::Result<HGLRC> {
    windows_targets::link!("opengl32.dll" "system" fn wglCreateLayerContext(param0 : super::Gdi:: HDC, param1 : i32) -> HGLRC);
    let result__ = wglCreateLayerContext(core::mem::transmute(param0), core::mem::transmute(param1));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn wglDeleteContext(param0: HGLRC) -> windows_core::Result<()> {
    windows_targets::link!("opengl32.dll" "system" fn wglDeleteContext(param0 : HGLRC) -> super::super::Foundation:: BOOL);
    wglDeleteContext(core::mem::transmute(param0)).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglDescribeLayerPlane(param0: super::Gdi::HDC, param1: i32, param2: i32, param3: u32, param4: *mut LAYERPLANEDESCRIPTOR) -> super::super::Foundation::BOOL {
    windows_targets::link!("opengl32.dll" "system" fn wglDescribeLayerPlane(param0 : super::Gdi:: HDC, param1 : i32, param2 : i32, param3 : u32, param4 : *mut LAYERPLANEDESCRIPTOR) -> super::super::Foundation:: BOOL);
    wglDescribeLayerPlane(core::mem::transmute(param0), core::mem::transmute(param1), core::mem::transmute(param2), core::mem::transmute(param3), core::mem::transmute(param4))
}
#[inline]
pub unsafe fn wglGetCurrentContext() -> HGLRC {
    windows_targets::link!("opengl32.dll" "system" fn wglGetCurrentContext() -> HGLRC);
    wglGetCurrentContext()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglGetCurrentDC() -> super::Gdi::HDC {
    windows_targets::link!("opengl32.dll" "system" fn wglGetCurrentDC() -> super::Gdi:: HDC);
    wglGetCurrentDC()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglGetLayerPaletteEntries(param0: super::Gdi::HDC, param1: i32, param2: i32, param3: i32, param4: *mut super::super::Foundation::COLORREF) -> i32 {
    windows_targets::link!("opengl32.dll" "system" fn wglGetLayerPaletteEntries(param0 : super::Gdi:: HDC, param1 : i32, param2 : i32, param3 : i32, param4 : *mut super::super::Foundation:: COLORREF) -> i32);
    wglGetLayerPaletteEntries(core::mem::transmute(param0), core::mem::transmute(param1), core::mem::transmute(param2), core::mem::transmute(param3), core::mem::transmute(param4))
}
#[inline]
pub unsafe fn wglGetProcAddress<P0>(param0: P0) -> super::super::Foundation::PROC
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("opengl32.dll" "system" fn wglGetProcAddress(param0 : windows_core::PCSTR) -> super::super::Foundation:: PROC);
    wglGetProcAddress(param0.param().abi())
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglMakeCurrent(param0: super::Gdi::HDC, param1: HGLRC) -> windows_core::Result<()> {
    windows_targets::link!("opengl32.dll" "system" fn wglMakeCurrent(param0 : super::Gdi:: HDC, param1 : HGLRC) -> super::super::Foundation:: BOOL);
    wglMakeCurrent(core::mem::transmute(param0), core::mem::transmute(param1)).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglRealizeLayerPalette(param0: super::Gdi::HDC, param1: i32, param2: bool) -> windows_core::Result<()> {
    windows_targets::link!("opengl32.dll" "system" fn wglRealizeLayerPalette(param0 : super::Gdi:: HDC, param1 : i32, param2 : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    wglRealizeLayerPalette(core::mem::transmute(param0), core::mem::transmute(param1), param2.into()).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglSetLayerPaletteEntries(param0: super::Gdi::HDC, param1: i32, param2: i32, param3: i32, param4: *const super::super::Foundation::COLORREF) -> i32 {
    windows_targets::link!("opengl32.dll" "system" fn wglSetLayerPaletteEntries(param0 : super::Gdi:: HDC, param1 : i32, param2 : i32, param3 : i32, param4 : *const super::super::Foundation:: COLORREF) -> i32);
    wglSetLayerPaletteEntries(core::mem::transmute(param0), core::mem::transmute(param1), core::mem::transmute(param2), core::mem::transmute(param3), core::mem::transmute(param4))
}
#[inline]
pub unsafe fn wglShareLists(param0: HGLRC, param1: HGLRC) -> windows_core::Result<()> {
    windows_targets::link!("opengl32.dll" "system" fn wglShareLists(param0 : HGLRC, param1 : HGLRC) -> super::super::Foundation:: BOOL);
    wglShareLists(core::mem::transmute(param0), core::mem::transmute(param1)).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglSwapLayerBuffers(param0: super::Gdi::HDC, param1: u32) -> windows_core::Result<()> {
    windows_targets::link!("opengl32.dll" "system" fn wglSwapLayerBuffers(param0 : super::Gdi:: HDC, param1 : u32) -> super::super::Foundation:: BOOL);
    wglSwapLayerBuffers(core::mem::transmute(param0), core::mem::transmute(param1)).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglUseFontBitmapsA(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32) -> windows_core::Result<()> {
    windows_targets::link!("opengl32.dll" "system" fn wglUseFontBitmapsA(param0 : super::Gdi:: HDC, param1 : u32, param2 : u32, param3 : u32) -> super::super::Foundation:: BOOL);
    wglUseFontBitmapsA(core::mem::transmute(param0), core::mem::transmute(param1), core::mem::transmute(param2), core::mem::transmute(param3)).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglUseFontBitmapsW(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32) -> windows_core::Result<()> {
    windows_targets::link!("opengl32.dll" "system" fn wglUseFontBitmapsW(param0 : super::Gdi:: HDC, param1 : u32, param2 : u32, param3 : u32) -> super::super::Foundation:: BOOL);
    wglUseFontBitmapsW(core::mem::transmute(param0), core::mem::transmute(param1), core::mem::transmute(param2), core::mem::transmute(param3)).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglUseFontOutlinesA(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32, param4: f32, param5: f32, param6: i32, param7: *mut GLYPHMETRICSFLOAT) -> windows_core::Result<()> {
    windows_targets::link!("opengl32.dll" "system" fn wglUseFontOutlinesA(param0 : super::Gdi:: HDC, param1 : u32, param2 : u32, param3 : u32, param4 : f32, param5 : f32, param6 : i32, param7 : *mut GLYPHMETRICSFLOAT) -> super::super::Foundation:: BOOL);
    wglUseFontOutlinesA(core::mem::transmute(param0), core::mem::transmute(param1), core::mem::transmute(param2), core::mem::transmute(param3), core::mem::transmute(param4), core::mem::transmute(param5), core::mem::transmute(param6), core::mem::transmute(param7)).ok()
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn wglUseFontOutlinesW(param0: super::Gdi::HDC, param1: u32, param2: u32, param3: u32, param4: f32, param5: f32, param6: i32, param7: *mut GLYPHMETRICSFLOAT) -> windows_core::Result<()> {
    windows_targets::link!("opengl32.dll" "system" fn wglUseFontOutlinesW(param0 : super::Gdi:: HDC, param1 : u32, param2 : u32, param3 : u32, param4 : f32, param5 : f32, param6 : i32, param7 : *mut GLYPHMETRICSFLOAT) -> super::super::Foundation:: BOOL);
    wglUseFontOutlinesW(core::mem::transmute(param0), core::mem::transmute(param1), core::mem::transmute(param2), core::mem::transmute(param3), core::mem::transmute(param4), core::mem::transmute(param5), core::mem::transmute(param6), core::mem::transmute(param7)).ok()
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EMRPIXELFORMAT {
    pub emr: super::Gdi::EMR,
    pub pfd: PIXELFORMATDESCRIPTOR,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for EMRPIXELFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GLU_AUTO_LOAD_MATRIX: u32 = 100200u32;
pub const GLU_BEGIN: u32 = 100100u32;
pub const GLU_CCW: u32 = 100121u32;
pub const GLU_CULLING: u32 = 100201u32;
pub const GLU_CW: u32 = 100120u32;
pub const GLU_DISPLAY_MODE: u32 = 100204u32;
pub const GLU_DOMAIN_DISTANCE: u32 = 100217u32;
pub const GLU_EDGE_FLAG: u32 = 100104u32;
pub const GLU_END: u32 = 100102u32;
pub const GLU_ERROR: u32 = 100103u32;
pub const GLU_EXTENSIONS: u32 = 100801u32;
pub const GLU_EXTERIOR: u32 = 100123u32;
pub const GLU_FALSE: u32 = 0u32;
pub const GLU_FILL: u32 = 100012u32;
pub const GLU_FLAT: u32 = 100001u32;
pub const GLU_INCOMPATIBLE_GL_VERSION: u32 = 100903u32;
pub const GLU_INSIDE: u32 = 100021u32;
pub const GLU_INTERIOR: u32 = 100122u32;
pub const GLU_INVALID_ENUM: u32 = 100900u32;
pub const GLU_INVALID_VALUE: u32 = 100901u32;
pub const GLU_LINE: u32 = 100011u32;
pub const GLU_MAP1_TRIM_2: u32 = 100210u32;
pub const GLU_MAP1_TRIM_3: u32 = 100211u32;
pub const GLU_NONE: u32 = 100002u32;
pub const GLU_NURBS_ERROR1: u32 = 100251u32;
pub const GLU_NURBS_ERROR10: u32 = 100260u32;
pub const GLU_NURBS_ERROR11: u32 = 100261u32;
pub const GLU_NURBS_ERROR12: u32 = 100262u32;
pub const GLU_NURBS_ERROR13: u32 = 100263u32;
pub const GLU_NURBS_ERROR14: u32 = 100264u32;
pub const GLU_NURBS_ERROR15: u32 = 100265u32;
pub const GLU_NURBS_ERROR16: u32 = 100266u32;
pub const GLU_NURBS_ERROR17: u32 = 100267u32;
pub const GLU_NURBS_ERROR18: u32 = 100268u32;
pub const GLU_NURBS_ERROR19: u32 = 100269u32;
pub const GLU_NURBS_ERROR2: u32 = 100252u32;
pub const GLU_NURBS_ERROR20: u32 = 100270u32;
pub const GLU_NURBS_ERROR21: u32 = 100271u32;
pub const GLU_NURBS_ERROR22: u32 = 100272u32;
pub const GLU_NURBS_ERROR23: u32 = 100273u32;
pub const GLU_NURBS_ERROR24: u32 = 100274u32;
pub const GLU_NURBS_ERROR25: u32 = 100275u32;
pub const GLU_NURBS_ERROR26: u32 = 100276u32;
pub const GLU_NURBS_ERROR27: u32 = 100277u32;
pub const GLU_NURBS_ERROR28: u32 = 100278u32;
pub const GLU_NURBS_ERROR29: u32 = 100279u32;
pub const GLU_NURBS_ERROR3: u32 = 100253u32;
pub const GLU_NURBS_ERROR30: u32 = 100280u32;
pub const GLU_NURBS_ERROR31: u32 = 100281u32;
pub const GLU_NURBS_ERROR32: u32 = 100282u32;
pub const GLU_NURBS_ERROR33: u32 = 100283u32;
pub const GLU_NURBS_ERROR34: u32 = 100284u32;
pub const GLU_NURBS_ERROR35: u32 = 100285u32;
pub const GLU_NURBS_ERROR36: u32 = 100286u32;
pub const GLU_NURBS_ERROR37: u32 = 100287u32;
pub const GLU_NURBS_ERROR4: u32 = 100254u32;
pub const GLU_NURBS_ERROR5: u32 = 100255u32;
pub const GLU_NURBS_ERROR6: u32 = 100256u32;
pub const GLU_NURBS_ERROR7: u32 = 100257u32;
pub const GLU_NURBS_ERROR8: u32 = 100258u32;
pub const GLU_NURBS_ERROR9: u32 = 100259u32;
pub const GLU_OUTLINE_PATCH: u32 = 100241u32;
pub const GLU_OUTLINE_POLYGON: u32 = 100240u32;
pub const GLU_OUTSIDE: u32 = 100020u32;
pub const GLU_OUT_OF_MEMORY: u32 = 100902u32;
pub const GLU_PARAMETRIC_ERROR: u32 = 100216u32;
pub const GLU_PARAMETRIC_TOLERANCE: u32 = 100202u32;
pub const GLU_PATH_LENGTH: u32 = 100215u32;
pub const GLU_POINT: u32 = 100010u32;
pub const GLU_SAMPLING_METHOD: u32 = 100205u32;
pub const GLU_SAMPLING_TOLERANCE: u32 = 100203u32;
pub const GLU_SILHOUETTE: u32 = 100013u32;
pub const GLU_SMOOTH: u32 = 100000u32;
pub const GLU_TESS_BEGIN: u32 = 100100u32;
pub const GLU_TESS_BEGIN_DATA: u32 = 100106u32;
pub const GLU_TESS_BOUNDARY_ONLY: u32 = 100141u32;
pub const GLU_TESS_COMBINE: u32 = 100105u32;
pub const GLU_TESS_COMBINE_DATA: u32 = 100111u32;
pub const GLU_TESS_COORD_TOO_LARGE: u32 = 100155u32;
pub const GLU_TESS_EDGE_FLAG: u32 = 100104u32;
pub const GLU_TESS_EDGE_FLAG_DATA: u32 = 100110u32;
pub const GLU_TESS_END: u32 = 100102u32;
pub const GLU_TESS_END_DATA: u32 = 100108u32;
pub const GLU_TESS_ERROR: u32 = 100103u32;
pub const GLU_TESS_ERROR1: u32 = 100151u32;
pub const GLU_TESS_ERROR2: u32 = 100152u32;
pub const GLU_TESS_ERROR3: u32 = 100153u32;
pub const GLU_TESS_ERROR4: u32 = 100154u32;
pub const GLU_TESS_ERROR5: u32 = 100155u32;
pub const GLU_TESS_ERROR6: u32 = 100156u32;
pub const GLU_TESS_ERROR7: u32 = 100157u32;
pub const GLU_TESS_ERROR8: u32 = 100158u32;
pub const GLU_TESS_ERROR_DATA: u32 = 100109u32;
pub const GLU_TESS_MISSING_BEGIN_CONTOUR: u32 = 100152u32;
pub const GLU_TESS_MISSING_BEGIN_POLYGON: u32 = 100151u32;
pub const GLU_TESS_MISSING_END_CONTOUR: u32 = 100154u32;
pub const GLU_TESS_MISSING_END_POLYGON: u32 = 100153u32;
pub const GLU_TESS_NEED_COMBINE_CALLBACK: u32 = 100156u32;
pub const GLU_TESS_TOLERANCE: u32 = 100142u32;
pub const GLU_TESS_VERTEX: u32 = 100101u32;
pub const GLU_TESS_VERTEX_DATA: u32 = 100107u32;
pub const GLU_TESS_WINDING_ABS_GEQ_TWO: u32 = 100134u32;
pub const GLU_TESS_WINDING_NEGATIVE: u32 = 100133u32;
pub const GLU_TESS_WINDING_NONZERO: u32 = 100131u32;
pub const GLU_TESS_WINDING_ODD: u32 = 100130u32;
pub const GLU_TESS_WINDING_POSITIVE: u32 = 100132u32;
pub const GLU_TESS_WINDING_RULE: u32 = 100140u32;
pub const GLU_TRUE: u32 = 1u32;
pub const GLU_UNKNOWN: u32 = 100124u32;
pub const GLU_U_STEP: u32 = 100206u32;
pub const GLU_VERSION: u32 = 100800u32;
pub const GLU_VERSION_1_1: u32 = 1u32;
pub const GLU_VERSION_1_2: u32 = 1u32;
pub const GLU_VERTEX: u32 = 100101u32;
pub const GLU_V_STEP: u32 = 100207u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLUnurbs(pub isize);
pub type GLUnurbsErrorProc = Option<unsafe extern "system" fn(param0: u32)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLUquadric(pub isize);
pub type GLUquadricErrorProc = Option<unsafe extern "system" fn(param0: u32)>;
pub type GLUtessBeginDataProc = Option<unsafe extern "system" fn(param0: u32, param1: *mut core::ffi::c_void)>;
pub type GLUtessBeginProc = Option<unsafe extern "system" fn(param0: u32)>;
pub type GLUtessCombineDataProc = Option<unsafe extern "system" fn(param0: *mut f64, param1: *mut *mut core::ffi::c_void, param2: *mut f32, param3: *mut *mut core::ffi::c_void, param4: *mut core::ffi::c_void)>;
pub type GLUtessCombineProc = Option<unsafe extern "system" fn(param0: *mut f64, param1: *mut *mut core::ffi::c_void, param2: *mut f32, param3: *mut *mut core::ffi::c_void)>;
pub type GLUtessEdgeFlagDataProc = Option<unsafe extern "system" fn(param0: u8, param1: *mut core::ffi::c_void)>;
pub type GLUtessEdgeFlagProc = Option<unsafe extern "system" fn(param0: u8)>;
pub type GLUtessEndDataProc = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void)>;
pub type GLUtessEndProc = Option<unsafe extern "system" fn()>;
pub type GLUtessErrorDataProc = Option<unsafe extern "system" fn(param0: u32, param1: *mut core::ffi::c_void)>;
pub type GLUtessErrorProc = Option<unsafe extern "system" fn(param0: u32)>;
pub type GLUtessVertexDataProc = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void)>;
pub type GLUtessVertexProc = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct GLUtesselator(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GLYPHMETRICSFLOAT {
    pub gmfBlackBoxX: f32,
    pub gmfBlackBoxY: f32,
    pub gmfptGlyphOrigin: POINTFLOAT,
    pub gmfCellIncX: f32,
    pub gmfCellIncY: f32,
}
impl Default for GLYPHMETRICSFLOAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GL_2D: u32 = 1536u32;
pub const GL_2_BYTES: u32 = 5127u32;
pub const GL_3D: u32 = 1537u32;
pub const GL_3D_COLOR: u32 = 1538u32;
pub const GL_3D_COLOR_TEXTURE: u32 = 1539u32;
pub const GL_3_BYTES: u32 = 5128u32;
pub const GL_4D_COLOR_TEXTURE: u32 = 1540u32;
pub const GL_4_BYTES: u32 = 5129u32;
pub const GL_ACCUM: u32 = 256u32;
pub const GL_ACCUM_ALPHA_BITS: u32 = 3419u32;
pub const GL_ACCUM_BLUE_BITS: u32 = 3418u32;
pub const GL_ACCUM_BUFFER_BIT: u32 = 512u32;
pub const GL_ACCUM_CLEAR_VALUE: u32 = 2944u32;
pub const GL_ACCUM_GREEN_BITS: u32 = 3417u32;
pub const GL_ACCUM_RED_BITS: u32 = 3416u32;
pub const GL_ADD: u32 = 260u32;
pub const GL_ALL_ATTRIB_BITS: u32 = 1048575u32;
pub const GL_ALPHA: u32 = 6406u32;
pub const GL_ALPHA12: u32 = 32829u32;
pub const GL_ALPHA16: u32 = 32830u32;
pub const GL_ALPHA4: u32 = 32827u32;
pub const GL_ALPHA8: u32 = 32828u32;
pub const GL_ALPHA_BIAS: u32 = 3357u32;
pub const GL_ALPHA_BITS: u32 = 3413u32;
pub const GL_ALPHA_SCALE: u32 = 3356u32;
pub const GL_ALPHA_TEST: u32 = 3008u32;
pub const GL_ALPHA_TEST_FUNC: u32 = 3009u32;
pub const GL_ALPHA_TEST_REF: u32 = 3010u32;
pub const GL_ALWAYS: u32 = 519u32;
pub const GL_AMBIENT: u32 = 4608u32;
pub const GL_AMBIENT_AND_DIFFUSE: u32 = 5634u32;
pub const GL_AND: u32 = 5377u32;
pub const GL_AND_INVERTED: u32 = 5380u32;
pub const GL_AND_REVERSE: u32 = 5378u32;
pub const GL_ATTRIB_STACK_DEPTH: u32 = 2992u32;
pub const GL_AUTO_NORMAL: u32 = 3456u32;
pub const GL_AUX0: u32 = 1033u32;
pub const GL_AUX1: u32 = 1034u32;
pub const GL_AUX2: u32 = 1035u32;
pub const GL_AUX3: u32 = 1036u32;
pub const GL_AUX_BUFFERS: u32 = 3072u32;
pub const GL_BACK: u32 = 1029u32;
pub const GL_BACK_LEFT: u32 = 1026u32;
pub const GL_BACK_RIGHT: u32 = 1027u32;
pub const GL_BGRA_EXT: u32 = 32993u32;
pub const GL_BGR_EXT: u32 = 32992u32;
pub const GL_BITMAP: u32 = 6656u32;
pub const GL_BITMAP_TOKEN: u32 = 1796u32;
pub const GL_BLEND: u32 = 3042u32;
pub const GL_BLEND_DST: u32 = 3040u32;
pub const GL_BLEND_SRC: u32 = 3041u32;
pub const GL_BLUE: u32 = 6405u32;
pub const GL_BLUE_BIAS: u32 = 3355u32;
pub const GL_BLUE_BITS: u32 = 3412u32;
pub const GL_BLUE_SCALE: u32 = 3354u32;
pub const GL_BYTE: u32 = 5120u32;
pub const GL_C3F_V3F: u32 = 10788u32;
pub const GL_C4F_N3F_V3F: u32 = 10790u32;
pub const GL_C4UB_V2F: u32 = 10786u32;
pub const GL_C4UB_V3F: u32 = 10787u32;
pub const GL_CCW: u32 = 2305u32;
pub const GL_CLAMP: u32 = 10496u32;
pub const GL_CLEAR: u32 = 5376u32;
pub const GL_CLIENT_ALL_ATTRIB_BITS: u32 = 4294967295u32;
pub const GL_CLIENT_ATTRIB_STACK_DEPTH: u32 = 2993u32;
pub const GL_CLIENT_PIXEL_STORE_BIT: u32 = 1u32;
pub const GL_CLIENT_VERTEX_ARRAY_BIT: u32 = 2u32;
pub const GL_CLIP_PLANE0: u32 = 12288u32;
pub const GL_CLIP_PLANE1: u32 = 12289u32;
pub const GL_CLIP_PLANE2: u32 = 12290u32;
pub const GL_CLIP_PLANE3: u32 = 12291u32;
pub const GL_CLIP_PLANE4: u32 = 12292u32;
pub const GL_CLIP_PLANE5: u32 = 12293u32;
pub const GL_COEFF: u32 = 2560u32;
pub const GL_COLOR: u32 = 6144u32;
pub const GL_COLOR_ARRAY: u32 = 32886u32;
pub const GL_COLOR_ARRAY_COUNT_EXT: u32 = 32900u32;
pub const GL_COLOR_ARRAY_EXT: u32 = 32886u32;
pub const GL_COLOR_ARRAY_POINTER: u32 = 32912u32;
pub const GL_COLOR_ARRAY_POINTER_EXT: u32 = 32912u32;
pub const GL_COLOR_ARRAY_SIZE: u32 = 32897u32;
pub const GL_COLOR_ARRAY_SIZE_EXT: u32 = 32897u32;
pub const GL_COLOR_ARRAY_STRIDE: u32 = 32899u32;
pub const GL_COLOR_ARRAY_STRIDE_EXT: u32 = 32899u32;
pub const GL_COLOR_ARRAY_TYPE: u32 = 32898u32;
pub const GL_COLOR_ARRAY_TYPE_EXT: u32 = 32898u32;
pub const GL_COLOR_BUFFER_BIT: u32 = 16384u32;
pub const GL_COLOR_CLEAR_VALUE: u32 = 3106u32;
pub const GL_COLOR_INDEX: u32 = 6400u32;
pub const GL_COLOR_INDEX12_EXT: u32 = 32998u32;
pub const GL_COLOR_INDEX16_EXT: u32 = 32999u32;
pub const GL_COLOR_INDEX1_EXT: u32 = 32994u32;
pub const GL_COLOR_INDEX2_EXT: u32 = 32995u32;
pub const GL_COLOR_INDEX4_EXT: u32 = 32996u32;
pub const GL_COLOR_INDEX8_EXT: u32 = 32997u32;
pub const GL_COLOR_INDEXES: u32 = 5635u32;
pub const GL_COLOR_LOGIC_OP: u32 = 3058u32;
pub const GL_COLOR_MATERIAL: u32 = 2903u32;
pub const GL_COLOR_MATERIAL_FACE: u32 = 2901u32;
pub const GL_COLOR_MATERIAL_PARAMETER: u32 = 2902u32;
pub const GL_COLOR_TABLE_ALPHA_SIZE_EXT: u32 = 32989u32;
pub const GL_COLOR_TABLE_BLUE_SIZE_EXT: u32 = 32988u32;
pub const GL_COLOR_TABLE_FORMAT_EXT: u32 = 32984u32;
pub const GL_COLOR_TABLE_GREEN_SIZE_EXT: u32 = 32987u32;
pub const GL_COLOR_TABLE_INTENSITY_SIZE_EXT: u32 = 32991u32;
pub const GL_COLOR_TABLE_LUMINANCE_SIZE_EXT: u32 = 32990u32;
pub const GL_COLOR_TABLE_RED_SIZE_EXT: u32 = 32986u32;
pub const GL_COLOR_TABLE_WIDTH_EXT: u32 = 32985u32;
pub const GL_COLOR_WRITEMASK: u32 = 3107u32;
pub const GL_COMPILE: u32 = 4864u32;
pub const GL_COMPILE_AND_EXECUTE: u32 = 4865u32;
pub const GL_CONSTANT_ATTENUATION: u32 = 4615u32;
pub const GL_COPY: u32 = 5379u32;
pub const GL_COPY_INVERTED: u32 = 5388u32;
pub const GL_COPY_PIXEL_TOKEN: u32 = 1798u32;
pub const GL_CULL_FACE: u32 = 2884u32;
pub const GL_CULL_FACE_MODE: u32 = 2885u32;
pub const GL_CURRENT_BIT: u32 = 1u32;
pub const GL_CURRENT_COLOR: u32 = 2816u32;
pub const GL_CURRENT_INDEX: u32 = 2817u32;
pub const GL_CURRENT_NORMAL: u32 = 2818u32;
pub const GL_CURRENT_RASTER_COLOR: u32 = 2820u32;
pub const GL_CURRENT_RASTER_DISTANCE: u32 = 2825u32;
pub const GL_CURRENT_RASTER_INDEX: u32 = 2821u32;
pub const GL_CURRENT_RASTER_POSITION: u32 = 2823u32;
pub const GL_CURRENT_RASTER_POSITION_VALID: u32 = 2824u32;
pub const GL_CURRENT_RASTER_TEXTURE_COORDS: u32 = 2822u32;
pub const GL_CURRENT_TEXTURE_COORDS: u32 = 2819u32;
pub const GL_CW: u32 = 2304u32;
pub const GL_DECAL: u32 = 8449u32;
pub const GL_DECR: u32 = 7683u32;
pub const GL_DEPTH: u32 = 6145u32;
pub const GL_DEPTH_BIAS: u32 = 3359u32;
pub const GL_DEPTH_BITS: u32 = 3414u32;
pub const GL_DEPTH_BUFFER_BIT: u32 = 256u32;
pub const GL_DEPTH_CLEAR_VALUE: u32 = 2931u32;
pub const GL_DEPTH_COMPONENT: u32 = 6402u32;
pub const GL_DEPTH_FUNC: u32 = 2932u32;
pub const GL_DEPTH_RANGE: u32 = 2928u32;
pub const GL_DEPTH_SCALE: u32 = 3358u32;
pub const GL_DEPTH_TEST: u32 = 2929u32;
pub const GL_DEPTH_WRITEMASK: u32 = 2930u32;
pub const GL_DIFFUSE: u32 = 4609u32;
pub const GL_DITHER: u32 = 3024u32;
pub const GL_DOMAIN: u32 = 2562u32;
pub const GL_DONT_CARE: u32 = 4352u32;
pub const GL_DOUBLE: u32 = 5130u32;
pub const GL_DOUBLEBUFFER: u32 = 3122u32;
pub const GL_DOUBLE_EXT: u32 = 5130u32;
pub const GL_DRAW_BUFFER: u32 = 3073u32;
pub const GL_DRAW_PIXEL_TOKEN: u32 = 1797u32;
pub const GL_DST_ALPHA: u32 = 772u32;
pub const GL_DST_COLOR: u32 = 774u32;
pub const GL_EDGE_FLAG: u32 = 2883u32;
pub const GL_EDGE_FLAG_ARRAY: u32 = 32889u32;
pub const GL_EDGE_FLAG_ARRAY_COUNT_EXT: u32 = 32909u32;
pub const GL_EDGE_FLAG_ARRAY_EXT: u32 = 32889u32;
pub const GL_EDGE_FLAG_ARRAY_POINTER: u32 = 32915u32;
pub const GL_EDGE_FLAG_ARRAY_POINTER_EXT: u32 = 32915u32;
pub const GL_EDGE_FLAG_ARRAY_STRIDE: u32 = 32908u32;
pub const GL_EDGE_FLAG_ARRAY_STRIDE_EXT: u32 = 32908u32;
pub const GL_EMISSION: u32 = 5632u32;
pub const GL_ENABLE_BIT: u32 = 8192u32;
pub const GL_EQUAL: u32 = 514u32;
pub const GL_EQUIV: u32 = 5385u32;
pub const GL_EVAL_BIT: u32 = 65536u32;
pub const GL_EXP: u32 = 2048u32;
pub const GL_EXP2: u32 = 2049u32;
pub const GL_EXTENSIONS: u32 = 7939u32;
pub const GL_EXT_bgra: u32 = 1u32;
pub const GL_EXT_paletted_texture: u32 = 1u32;
pub const GL_EXT_vertex_array: u32 = 1u32;
pub const GL_EYE_LINEAR: u32 = 9216u32;
pub const GL_EYE_PLANE: u32 = 9474u32;
pub const GL_FALSE: u32 = 0u32;
pub const GL_FASTEST: u32 = 4353u32;
pub const GL_FEEDBACK: u32 = 7169u32;
pub const GL_FEEDBACK_BUFFER_POINTER: u32 = 3568u32;
pub const GL_FEEDBACK_BUFFER_SIZE: u32 = 3569u32;
pub const GL_FEEDBACK_BUFFER_TYPE: u32 = 3570u32;
pub const GL_FILL: u32 = 6914u32;
pub const GL_FLAT: u32 = 7424u32;
pub const GL_FLOAT: u32 = 5126u32;
pub const GL_FOG: u32 = 2912u32;
pub const GL_FOG_BIT: u32 = 128u32;
pub const GL_FOG_COLOR: u32 = 2918u32;
pub const GL_FOG_DENSITY: u32 = 2914u32;
pub const GL_FOG_END: u32 = 2916u32;
pub const GL_FOG_HINT: u32 = 3156u32;
pub const GL_FOG_INDEX: u32 = 2913u32;
pub const GL_FOG_MODE: u32 = 2917u32;
pub const GL_FOG_SPECULAR_TEXTURE_WIN: u32 = 33004u32;
pub const GL_FOG_START: u32 = 2915u32;
pub const GL_FRONT: u32 = 1028u32;
pub const GL_FRONT_AND_BACK: u32 = 1032u32;
pub const GL_FRONT_FACE: u32 = 2886u32;
pub const GL_FRONT_LEFT: u32 = 1024u32;
pub const GL_FRONT_RIGHT: u32 = 1025u32;
pub const GL_GEQUAL: u32 = 518u32;
pub const GL_GREATER: u32 = 516u32;
pub const GL_GREEN: u32 = 6404u32;
pub const GL_GREEN_BIAS: u32 = 3353u32;
pub const GL_GREEN_BITS: u32 = 3411u32;
pub const GL_GREEN_SCALE: u32 = 3352u32;
pub const GL_HINT_BIT: u32 = 32768u32;
pub const GL_INCR: u32 = 7682u32;
pub const GL_INDEX_ARRAY: u32 = 32887u32;
pub const GL_INDEX_ARRAY_COUNT_EXT: u32 = 32903u32;
pub const GL_INDEX_ARRAY_EXT: u32 = 32887u32;
pub const GL_INDEX_ARRAY_POINTER: u32 = 32913u32;
pub const GL_INDEX_ARRAY_POINTER_EXT: u32 = 32913u32;
pub const GL_INDEX_ARRAY_STRIDE: u32 = 32902u32;
pub const GL_INDEX_ARRAY_STRIDE_EXT: u32 = 32902u32;
pub const GL_INDEX_ARRAY_TYPE: u32 = 32901u32;
pub const GL_INDEX_ARRAY_TYPE_EXT: u32 = 32901u32;
pub const GL_INDEX_BITS: u32 = 3409u32;
pub const GL_INDEX_CLEAR_VALUE: u32 = 3104u32;
pub const GL_INDEX_LOGIC_OP: u32 = 3057u32;
pub const GL_INDEX_MODE: u32 = 3120u32;
pub const GL_INDEX_OFFSET: u32 = 3347u32;
pub const GL_INDEX_SHIFT: u32 = 3346u32;
pub const GL_INDEX_WRITEMASK: u32 = 3105u32;
pub const GL_INT: u32 = 5124u32;
pub const GL_INTENSITY: u32 = 32841u32;
pub const GL_INTENSITY12: u32 = 32844u32;
pub const GL_INTENSITY16: u32 = 32845u32;
pub const GL_INTENSITY4: u32 = 32842u32;
pub const GL_INTENSITY8: u32 = 32843u32;
pub const GL_INVALID_ENUM: u32 = 1280u32;
pub const GL_INVALID_OPERATION: u32 = 1282u32;
pub const GL_INVALID_VALUE: u32 = 1281u32;
pub const GL_INVERT: u32 = 5386u32;
pub const GL_KEEP: u32 = 7680u32;
pub const GL_LEFT: u32 = 1030u32;
pub const GL_LEQUAL: u32 = 515u32;
pub const GL_LESS: u32 = 513u32;
pub const GL_LIGHT0: u32 = 16384u32;
pub const GL_LIGHT1: u32 = 16385u32;
pub const GL_LIGHT2: u32 = 16386u32;
pub const GL_LIGHT3: u32 = 16387u32;
pub const GL_LIGHT4: u32 = 16388u32;
pub const GL_LIGHT5: u32 = 16389u32;
pub const GL_LIGHT6: u32 = 16390u32;
pub const GL_LIGHT7: u32 = 16391u32;
pub const GL_LIGHTING: u32 = 2896u32;
pub const GL_LIGHTING_BIT: u32 = 64u32;
pub const GL_LIGHT_MODEL_AMBIENT: u32 = 2899u32;
pub const GL_LIGHT_MODEL_LOCAL_VIEWER: u32 = 2897u32;
pub const GL_LIGHT_MODEL_TWO_SIDE: u32 = 2898u32;
pub const GL_LINE: u32 = 6913u32;
pub const GL_LINEAR: u32 = 9729u32;
pub const GL_LINEAR_ATTENUATION: u32 = 4616u32;
pub const GL_LINEAR_MIPMAP_LINEAR: u32 = 9987u32;
pub const GL_LINEAR_MIPMAP_NEAREST: u32 = 9985u32;
pub const GL_LINES: u32 = 1u32;
pub const GL_LINE_BIT: u32 = 4u32;
pub const GL_LINE_LOOP: u32 = 2u32;
pub const GL_LINE_RESET_TOKEN: u32 = 1799u32;
pub const GL_LINE_SMOOTH: u32 = 2848u32;
pub const GL_LINE_SMOOTH_HINT: u32 = 3154u32;
pub const GL_LINE_STIPPLE: u32 = 2852u32;
pub const GL_LINE_STIPPLE_PATTERN: u32 = 2853u32;
pub const GL_LINE_STIPPLE_REPEAT: u32 = 2854u32;
pub const GL_LINE_STRIP: u32 = 3u32;
pub const GL_LINE_TOKEN: u32 = 1794u32;
pub const GL_LINE_WIDTH: u32 = 2849u32;
pub const GL_LINE_WIDTH_GRANULARITY: u32 = 2851u32;
pub const GL_LINE_WIDTH_RANGE: u32 = 2850u32;
pub const GL_LIST_BASE: u32 = 2866u32;
pub const GL_LIST_BIT: u32 = 131072u32;
pub const GL_LIST_INDEX: u32 = 2867u32;
pub const GL_LIST_MODE: u32 = 2864u32;
pub const GL_LOAD: u32 = 257u32;
pub const GL_LOGIC_OP: u32 = 3057u32;
pub const GL_LOGIC_OP_MODE: u32 = 3056u32;
pub const GL_LUMINANCE: u32 = 6409u32;
pub const GL_LUMINANCE12: u32 = 32833u32;
pub const GL_LUMINANCE12_ALPHA12: u32 = 32839u32;
pub const GL_LUMINANCE12_ALPHA4: u32 = 32838u32;
pub const GL_LUMINANCE16: u32 = 32834u32;
pub const GL_LUMINANCE16_ALPHA16: u32 = 32840u32;
pub const GL_LUMINANCE4: u32 = 32831u32;
pub const GL_LUMINANCE4_ALPHA4: u32 = 32835u32;
pub const GL_LUMINANCE6_ALPHA2: u32 = 32836u32;
pub const GL_LUMINANCE8: u32 = 32832u32;
pub const GL_LUMINANCE8_ALPHA8: u32 = 32837u32;
pub const GL_LUMINANCE_ALPHA: u32 = 6410u32;
pub const GL_MAP1_COLOR_4: u32 = 3472u32;
pub const GL_MAP1_GRID_DOMAIN: u32 = 3536u32;
pub const GL_MAP1_GRID_SEGMENTS: u32 = 3537u32;
pub const GL_MAP1_INDEX: u32 = 3473u32;
pub const GL_MAP1_NORMAL: u32 = 3474u32;
pub const GL_MAP1_TEXTURE_COORD_1: u32 = 3475u32;
pub const GL_MAP1_TEXTURE_COORD_2: u32 = 3476u32;
pub const GL_MAP1_TEXTURE_COORD_3: u32 = 3477u32;
pub const GL_MAP1_TEXTURE_COORD_4: u32 = 3478u32;
pub const GL_MAP1_VERTEX_3: u32 = 3479u32;
pub const GL_MAP1_VERTEX_4: u32 = 3480u32;
pub const GL_MAP2_COLOR_4: u32 = 3504u32;
pub const GL_MAP2_GRID_DOMAIN: u32 = 3538u32;
pub const GL_MAP2_GRID_SEGMENTS: u32 = 3539u32;
pub const GL_MAP2_INDEX: u32 = 3505u32;
pub const GL_MAP2_NORMAL: u32 = 3506u32;
pub const GL_MAP2_TEXTURE_COORD_1: u32 = 3507u32;
pub const GL_MAP2_TEXTURE_COORD_2: u32 = 3508u32;
pub const GL_MAP2_TEXTURE_COORD_3: u32 = 3509u32;
pub const GL_MAP2_TEXTURE_COORD_4: u32 = 3510u32;
pub const GL_MAP2_VERTEX_3: u32 = 3511u32;
pub const GL_MAP2_VERTEX_4: u32 = 3512u32;
pub const GL_MAP_COLOR: u32 = 3344u32;
pub const GL_MAP_STENCIL: u32 = 3345u32;
pub const GL_MATRIX_MODE: u32 = 2976u32;
pub const GL_MAX_ATTRIB_STACK_DEPTH: u32 = 3381u32;
pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: u32 = 3387u32;
pub const GL_MAX_CLIP_PLANES: u32 = 3378u32;
pub const GL_MAX_ELEMENTS_INDICES_WIN: u32 = 33001u32;
pub const GL_MAX_ELEMENTS_VERTICES_WIN: u32 = 33000u32;
pub const GL_MAX_EVAL_ORDER: u32 = 3376u32;
pub const GL_MAX_LIGHTS: u32 = 3377u32;
pub const GL_MAX_LIST_NESTING: u32 = 2865u32;
pub const GL_MAX_MODELVIEW_STACK_DEPTH: u32 = 3382u32;
pub const GL_MAX_NAME_STACK_DEPTH: u32 = 3383u32;
pub const GL_MAX_PIXEL_MAP_TABLE: u32 = 3380u32;
pub const GL_MAX_PROJECTION_STACK_DEPTH: u32 = 3384u32;
pub const GL_MAX_TEXTURE_SIZE: u32 = 3379u32;
pub const GL_MAX_TEXTURE_STACK_DEPTH: u32 = 3385u32;
pub const GL_MAX_VIEWPORT_DIMS: u32 = 3386u32;
pub const GL_MODELVIEW: u32 = 5888u32;
pub const GL_MODELVIEW_MATRIX: u32 = 2982u32;
pub const GL_MODELVIEW_STACK_DEPTH: u32 = 2979u32;
pub const GL_MODULATE: u32 = 8448u32;
pub const GL_MULT: u32 = 259u32;
pub const GL_N3F_V3F: u32 = 10789u32;
pub const GL_NAME_STACK_DEPTH: u32 = 3440u32;
pub const GL_NAND: u32 = 5390u32;
pub const GL_NEAREST: u32 = 9728u32;
pub const GL_NEAREST_MIPMAP_LINEAR: u32 = 9986u32;
pub const GL_NEAREST_MIPMAP_NEAREST: u32 = 9984u32;
pub const GL_NEVER: u32 = 512u32;
pub const GL_NICEST: u32 = 4354u32;
pub const GL_NONE: u32 = 0u32;
pub const GL_NOOP: u32 = 5381u32;
pub const GL_NOR: u32 = 5384u32;
pub const GL_NORMALIZE: u32 = 2977u32;
pub const GL_NORMAL_ARRAY: u32 = 32885u32;
pub const GL_NORMAL_ARRAY_COUNT_EXT: u32 = 32896u32;
pub const GL_NORMAL_ARRAY_EXT: u32 = 32885u32;
pub const GL_NORMAL_ARRAY_POINTER: u32 = 32911u32;
pub const GL_NORMAL_ARRAY_POINTER_EXT: u32 = 32911u32;
pub const GL_NORMAL_ARRAY_STRIDE: u32 = 32895u32;
pub const GL_NORMAL_ARRAY_STRIDE_EXT: u32 = 32895u32;
pub const GL_NORMAL_ARRAY_TYPE: u32 = 32894u32;
pub const GL_NORMAL_ARRAY_TYPE_EXT: u32 = 32894u32;
pub const GL_NOTEQUAL: u32 = 517u32;
pub const GL_NO_ERROR: u32 = 0u32;
pub const GL_OBJECT_LINEAR: u32 = 9217u32;
pub const GL_OBJECT_PLANE: u32 = 9473u32;
pub const GL_ONE: u32 = 1u32;
pub const GL_ONE_MINUS_DST_ALPHA: u32 = 773u32;
pub const GL_ONE_MINUS_DST_COLOR: u32 = 775u32;
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 771u32;
pub const GL_ONE_MINUS_SRC_COLOR: u32 = 769u32;
pub const GL_OR: u32 = 5383u32;
pub const GL_ORDER: u32 = 2561u32;
pub const GL_OR_INVERTED: u32 = 5389u32;
pub const GL_OR_REVERSE: u32 = 5387u32;
pub const GL_OUT_OF_MEMORY: u32 = 1285u32;
pub const GL_PACK_ALIGNMENT: u32 = 3333u32;
pub const GL_PACK_LSB_FIRST: u32 = 3329u32;
pub const GL_PACK_ROW_LENGTH: u32 = 3330u32;
pub const GL_PACK_SKIP_PIXELS: u32 = 3332u32;
pub const GL_PACK_SKIP_ROWS: u32 = 3331u32;
pub const GL_PACK_SWAP_BYTES: u32 = 3328u32;
pub const GL_PASS_THROUGH_TOKEN: u32 = 1792u32;
pub const GL_PERSPECTIVE_CORRECTION_HINT: u32 = 3152u32;
pub const GL_PHONG_HINT_WIN: u32 = 33003u32;
pub const GL_PHONG_WIN: u32 = 33002u32;
pub const GL_PIXEL_MAP_A_TO_A: u32 = 3193u32;
pub const GL_PIXEL_MAP_A_TO_A_SIZE: u32 = 3257u32;
pub const GL_PIXEL_MAP_B_TO_B: u32 = 3192u32;
pub const GL_PIXEL_MAP_B_TO_B_SIZE: u32 = 3256u32;
pub const GL_PIXEL_MAP_G_TO_G: u32 = 3191u32;
pub const GL_PIXEL_MAP_G_TO_G_SIZE: u32 = 3255u32;
pub const GL_PIXEL_MAP_I_TO_A: u32 = 3189u32;
pub const GL_PIXEL_MAP_I_TO_A_SIZE: u32 = 3253u32;
pub const GL_PIXEL_MAP_I_TO_B: u32 = 3188u32;
pub const GL_PIXEL_MAP_I_TO_B_SIZE: u32 = 3252u32;
pub const GL_PIXEL_MAP_I_TO_G: u32 = 3187u32;
pub const GL_PIXEL_MAP_I_TO_G_SIZE: u32 = 3251u32;
pub const GL_PIXEL_MAP_I_TO_I: u32 = 3184u32;
pub const GL_PIXEL_MAP_I_TO_I_SIZE: u32 = 3248u32;
pub const GL_PIXEL_MAP_I_TO_R: u32 = 3186u32;
pub const GL_PIXEL_MAP_I_TO_R_SIZE: u32 = 3250u32;
pub const GL_PIXEL_MAP_R_TO_R: u32 = 3190u32;
pub const GL_PIXEL_MAP_R_TO_R_SIZE: u32 = 3254u32;
pub const GL_PIXEL_MAP_S_TO_S: u32 = 3185u32;
pub const GL_PIXEL_MAP_S_TO_S_SIZE: u32 = 3249u32;
pub const GL_PIXEL_MODE_BIT: u32 = 32u32;
pub const GL_POINT: u32 = 6912u32;
pub const GL_POINTS: u32 = 0u32;
pub const GL_POINT_BIT: u32 = 2u32;
pub const GL_POINT_SIZE: u32 = 2833u32;
pub const GL_POINT_SIZE_GRANULARITY: u32 = 2835u32;
pub const GL_POINT_SIZE_RANGE: u32 = 2834u32;
pub const GL_POINT_SMOOTH: u32 = 2832u32;
pub const GL_POINT_SMOOTH_HINT: u32 = 3153u32;
pub const GL_POINT_TOKEN: u32 = 1793u32;
pub const GL_POLYGON: u32 = 9u32;
pub const GL_POLYGON_BIT: u32 = 8u32;
pub const GL_POLYGON_MODE: u32 = 2880u32;
pub const GL_POLYGON_OFFSET_FACTOR: u32 = 32824u32;
pub const GL_POLYGON_OFFSET_FILL: u32 = 32823u32;
pub const GL_POLYGON_OFFSET_LINE: u32 = 10754u32;
pub const GL_POLYGON_OFFSET_POINT: u32 = 10753u32;
pub const GL_POLYGON_OFFSET_UNITS: u32 = 10752u32;
pub const GL_POLYGON_SMOOTH: u32 = 2881u32;
pub const GL_POLYGON_SMOOTH_HINT: u32 = 3155u32;
pub const GL_POLYGON_STIPPLE: u32 = 2882u32;
pub const GL_POLYGON_STIPPLE_BIT: u32 = 16u32;
pub const GL_POLYGON_TOKEN: u32 = 1795u32;
pub const GL_POSITION: u32 = 4611u32;
pub const GL_PROJECTION: u32 = 5889u32;
pub const GL_PROJECTION_MATRIX: u32 = 2983u32;
pub const GL_PROJECTION_STACK_DEPTH: u32 = 2980u32;
pub const GL_PROXY_TEXTURE_1D: u32 = 32867u32;
pub const GL_PROXY_TEXTURE_2D: u32 = 32868u32;
pub const GL_Q: u32 = 8195u32;
pub const GL_QUADRATIC_ATTENUATION: u32 = 4617u32;
pub const GL_QUADS: u32 = 7u32;
pub const GL_QUAD_STRIP: u32 = 8u32;
pub const GL_R: u32 = 8194u32;
pub const GL_R3_G3_B2: u32 = 10768u32;
pub const GL_READ_BUFFER: u32 = 3074u32;
pub const GL_RED: u32 = 6403u32;
pub const GL_RED_BIAS: u32 = 3349u32;
pub const GL_RED_BITS: u32 = 3410u32;
pub const GL_RED_SCALE: u32 = 3348u32;
pub const GL_RENDER: u32 = 7168u32;
pub const GL_RENDERER: u32 = 7937u32;
pub const GL_RENDER_MODE: u32 = 3136u32;
pub const GL_REPEAT: u32 = 10497u32;
pub const GL_REPLACE: u32 = 7681u32;
pub const GL_RETURN: u32 = 258u32;
pub const GL_RGB: u32 = 6407u32;
pub const GL_RGB10: u32 = 32850u32;
pub const GL_RGB10_A2: u32 = 32857u32;
pub const GL_RGB12: u32 = 32851u32;
pub const GL_RGB16: u32 = 32852u32;
pub const GL_RGB4: u32 = 32847u32;
pub const GL_RGB5: u32 = 32848u32;
pub const GL_RGB5_A1: u32 = 32855u32;
pub const GL_RGB8: u32 = 32849u32;
pub const GL_RGBA: u32 = 6408u32;
pub const GL_RGBA12: u32 = 32858u32;
pub const GL_RGBA16: u32 = 32859u32;
pub const GL_RGBA2: u32 = 32853u32;
pub const GL_RGBA4: u32 = 32854u32;
pub const GL_RGBA8: u32 = 32856u32;
pub const GL_RGBA_MODE: u32 = 3121u32;
pub const GL_RIGHT: u32 = 1031u32;
pub const GL_S: u32 = 8192u32;
pub const GL_SCISSOR_BIT: u32 = 524288u32;
pub const GL_SCISSOR_BOX: u32 = 3088u32;
pub const GL_SCISSOR_TEST: u32 = 3089u32;
pub const GL_SELECT: u32 = 7170u32;
pub const GL_SELECTION_BUFFER_POINTER: u32 = 3571u32;
pub const GL_SELECTION_BUFFER_SIZE: u32 = 3572u32;
pub const GL_SET: u32 = 5391u32;
pub const GL_SHADE_MODEL: u32 = 2900u32;
pub const GL_SHININESS: u32 = 5633u32;
pub const GL_SHORT: u32 = 5122u32;
pub const GL_SMOOTH: u32 = 7425u32;
pub const GL_SPECULAR: u32 = 4610u32;
pub const GL_SPHERE_MAP: u32 = 9218u32;
pub const GL_SPOT_CUTOFF: u32 = 4614u32;
pub const GL_SPOT_DIRECTION: u32 = 4612u32;
pub const GL_SPOT_EXPONENT: u32 = 4613u32;
pub const GL_SRC_ALPHA: u32 = 770u32;
pub const GL_SRC_ALPHA_SATURATE: u32 = 776u32;
pub const GL_SRC_COLOR: u32 = 768u32;
pub const GL_STACK_OVERFLOW: u32 = 1283u32;
pub const GL_STACK_UNDERFLOW: u32 = 1284u32;
pub const GL_STENCIL: u32 = 6146u32;
pub const GL_STENCIL_BITS: u32 = 3415u32;
pub const GL_STENCIL_BUFFER_BIT: u32 = 1024u32;
pub const GL_STENCIL_CLEAR_VALUE: u32 = 2961u32;
pub const GL_STENCIL_FAIL: u32 = 2964u32;
pub const GL_STENCIL_FUNC: u32 = 2962u32;
pub const GL_STENCIL_INDEX: u32 = 6401u32;
pub const GL_STENCIL_PASS_DEPTH_FAIL: u32 = 2965u32;
pub const GL_STENCIL_PASS_DEPTH_PASS: u32 = 2966u32;
pub const GL_STENCIL_REF: u32 = 2967u32;
pub const GL_STENCIL_TEST: u32 = 2960u32;
pub const GL_STENCIL_VALUE_MASK: u32 = 2963u32;
pub const GL_STENCIL_WRITEMASK: u32 = 2968u32;
pub const GL_STEREO: u32 = 3123u32;
pub const GL_SUBPIXEL_BITS: u32 = 3408u32;
pub const GL_T: u32 = 8193u32;
pub const GL_T2F_C3F_V3F: u32 = 10794u32;
pub const GL_T2F_C4F_N3F_V3F: u32 = 10796u32;
pub const GL_T2F_C4UB_V3F: u32 = 10793u32;
pub const GL_T2F_N3F_V3F: u32 = 10795u32;
pub const GL_T2F_V3F: u32 = 10791u32;
pub const GL_T4F_C4F_N3F_V4F: u32 = 10797u32;
pub const GL_T4F_V4F: u32 = 10792u32;
pub const GL_TEXTURE: u32 = 5890u32;
pub const GL_TEXTURE_1D: u32 = 3552u32;
pub const GL_TEXTURE_2D: u32 = 3553u32;
pub const GL_TEXTURE_ALPHA_SIZE: u32 = 32863u32;
pub const GL_TEXTURE_BINDING_1D: u32 = 32872u32;
pub const GL_TEXTURE_BINDING_2D: u32 = 32873u32;
pub const GL_TEXTURE_BIT: u32 = 262144u32;
pub const GL_TEXTURE_BLUE_SIZE: u32 = 32862u32;
pub const GL_TEXTURE_BORDER: u32 = 4101u32;
pub const GL_TEXTURE_BORDER_COLOR: u32 = 4100u32;
pub const GL_TEXTURE_COMPONENTS: u32 = 4099u32;
pub const GL_TEXTURE_COORD_ARRAY: u32 = 32888u32;
pub const GL_TEXTURE_COORD_ARRAY_COUNT_EXT: u32 = 32907u32;
pub const GL_TEXTURE_COORD_ARRAY_EXT: u32 = 32888u32;
pub const GL_TEXTURE_COORD_ARRAY_POINTER: u32 = 32914u32;
pub const GL_TEXTURE_COORD_ARRAY_POINTER_EXT: u32 = 32914u32;
pub const GL_TEXTURE_COORD_ARRAY_SIZE: u32 = 32904u32;
pub const GL_TEXTURE_COORD_ARRAY_SIZE_EXT: u32 = 32904u32;
pub const GL_TEXTURE_COORD_ARRAY_STRIDE: u32 = 32906u32;
pub const GL_TEXTURE_COORD_ARRAY_STRIDE_EXT: u32 = 32906u32;
pub const GL_TEXTURE_COORD_ARRAY_TYPE: u32 = 32905u32;
pub const GL_TEXTURE_COORD_ARRAY_TYPE_EXT: u32 = 32905u32;
pub const GL_TEXTURE_ENV: u32 = 8960u32;
pub const GL_TEXTURE_ENV_COLOR: u32 = 8705u32;
pub const GL_TEXTURE_ENV_MODE: u32 = 8704u32;
pub const GL_TEXTURE_GEN_MODE: u32 = 9472u32;
pub const GL_TEXTURE_GEN_Q: u32 = 3171u32;
pub const GL_TEXTURE_GEN_R: u32 = 3170u32;
pub const GL_TEXTURE_GEN_S: u32 = 3168u32;
pub const GL_TEXTURE_GEN_T: u32 = 3169u32;
pub const GL_TEXTURE_GREEN_SIZE: u32 = 32861u32;
pub const GL_TEXTURE_HEIGHT: u32 = 4097u32;
pub const GL_TEXTURE_INTENSITY_SIZE: u32 = 32865u32;
pub const GL_TEXTURE_INTERNAL_FORMAT: u32 = 4099u32;
pub const GL_TEXTURE_LUMINANCE_SIZE: u32 = 32864u32;
pub const GL_TEXTURE_MAG_FILTER: u32 = 10240u32;
pub const GL_TEXTURE_MATRIX: u32 = 2984u32;
pub const GL_TEXTURE_MIN_FILTER: u32 = 10241u32;
pub const GL_TEXTURE_PRIORITY: u32 = 32870u32;
pub const GL_TEXTURE_RED_SIZE: u32 = 32860u32;
pub const GL_TEXTURE_RESIDENT: u32 = 32871u32;
pub const GL_TEXTURE_STACK_DEPTH: u32 = 2981u32;
pub const GL_TEXTURE_WIDTH: u32 = 4096u32;
pub const GL_TEXTURE_WRAP_S: u32 = 10242u32;
pub const GL_TEXTURE_WRAP_T: u32 = 10243u32;
pub const GL_TRANSFORM_BIT: u32 = 4096u32;
pub const GL_TRIANGLES: u32 = 4u32;
pub const GL_TRIANGLE_FAN: u32 = 6u32;
pub const GL_TRIANGLE_STRIP: u32 = 5u32;
pub const GL_TRUE: u32 = 1u32;
pub const GL_UNPACK_ALIGNMENT: u32 = 3317u32;
pub const GL_UNPACK_LSB_FIRST: u32 = 3313u32;
pub const GL_UNPACK_ROW_LENGTH: u32 = 3314u32;
pub const GL_UNPACK_SKIP_PIXELS: u32 = 3316u32;
pub const GL_UNPACK_SKIP_ROWS: u32 = 3315u32;
pub const GL_UNPACK_SWAP_BYTES: u32 = 3312u32;
pub const GL_UNSIGNED_BYTE: u32 = 5121u32;
pub const GL_UNSIGNED_INT: u32 = 5125u32;
pub const GL_UNSIGNED_SHORT: u32 = 5123u32;
pub const GL_V2F: u32 = 10784u32;
pub const GL_V3F: u32 = 10785u32;
pub const GL_VENDOR: u32 = 7936u32;
pub const GL_VERSION: u32 = 7938u32;
pub const GL_VERSION_1_1: u32 = 1u32;
pub const GL_VERTEX_ARRAY: u32 = 32884u32;
pub const GL_VERTEX_ARRAY_COUNT_EXT: u32 = 32893u32;
pub const GL_VERTEX_ARRAY_EXT: u32 = 32884u32;
pub const GL_VERTEX_ARRAY_POINTER: u32 = 32910u32;
pub const GL_VERTEX_ARRAY_POINTER_EXT: u32 = 32910u32;
pub const GL_VERTEX_ARRAY_SIZE: u32 = 32890u32;
pub const GL_VERTEX_ARRAY_SIZE_EXT: u32 = 32890u32;
pub const GL_VERTEX_ARRAY_STRIDE: u32 = 32892u32;
pub const GL_VERTEX_ARRAY_STRIDE_EXT: u32 = 32892u32;
pub const GL_VERTEX_ARRAY_TYPE: u32 = 32891u32;
pub const GL_VERTEX_ARRAY_TYPE_EXT: u32 = 32891u32;
pub const GL_VIEWPORT: u32 = 2978u32;
pub const GL_VIEWPORT_BIT: u32 = 2048u32;
pub const GL_WIN_draw_range_elements: u32 = 1u32;
pub const GL_WIN_swap_hint: u32 = 1u32;
pub const GL_XOR: u32 = 5382u32;
pub const GL_ZERO: u32 = 0u32;
pub const GL_ZOOM_X: u32 = 3350u32;
pub const GL_ZOOM_Y: u32 = 3351u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HGLRC(pub *mut core::ffi::c_void);
impl HGLRC {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 as _ || self.0 == 0 as _
    }
}
impl windows_core::Free for HGLRC {
    #[inline]
    unsafe fn free(&mut self) {
        if !self.is_invalid() {
            windows_targets::link!("opengl32.dll" "system" fn wglDeleteContext(param0 : *mut core::ffi::c_void) -> i32);
            wglDeleteContext(self.0);
        }
    }
}
impl Default for HGLRC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LAYERPLANEDESCRIPTOR {
    pub nSize: u16,
    pub nVersion: u16,
    pub dwFlags: u32,
    pub iPixelType: u8,
    pub cColorBits: u8,
    pub cRedBits: u8,
    pub cRedShift: u8,
    pub cGreenBits: u8,
    pub cGreenShift: u8,
    pub cBlueBits: u8,
    pub cBlueShift: u8,
    pub cAlphaBits: u8,
    pub cAlphaShift: u8,
    pub cAccumBits: u8,
    pub cAccumRedBits: u8,
    pub cAccumGreenBits: u8,
    pub cAccumBlueBits: u8,
    pub cAccumAlphaBits: u8,
    pub cDepthBits: u8,
    pub cStencilBits: u8,
    pub cAuxBuffers: u8,
    pub iLayerPlane: u8,
    pub bReserved: u8,
    pub crTransparent: super::super::Foundation::COLORREF,
}
impl Default for LAYERPLANEDESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PFD_DEPTH_DONTCARE: PFD_FLAGS = PFD_FLAGS(536870912u32);
pub const PFD_DIRECT3D_ACCELERATED: PFD_FLAGS = PFD_FLAGS(16384u32);
pub const PFD_DOUBLEBUFFER: PFD_FLAGS = PFD_FLAGS(1u32);
pub const PFD_DOUBLEBUFFER_DONTCARE: PFD_FLAGS = PFD_FLAGS(1073741824u32);
pub const PFD_DRAW_TO_BITMAP: PFD_FLAGS = PFD_FLAGS(8u32);
pub const PFD_DRAW_TO_WINDOW: PFD_FLAGS = PFD_FLAGS(4u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PFD_FLAGS(pub u32);
impl PFD_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PFD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PFD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PFD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PFD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PFD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PFD_GENERIC_ACCELERATED: PFD_FLAGS = PFD_FLAGS(4096u32);
pub const PFD_GENERIC_FORMAT: PFD_FLAGS = PFD_FLAGS(64u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PFD_LAYER_TYPE(pub i8);
pub const PFD_MAIN_PLANE: PFD_LAYER_TYPE = PFD_LAYER_TYPE(0i8);
pub const PFD_NEED_PALETTE: PFD_FLAGS = PFD_FLAGS(128u32);
pub const PFD_NEED_SYSTEM_PALETTE: PFD_FLAGS = PFD_FLAGS(256u32);
pub const PFD_OVERLAY_PLANE: PFD_LAYER_TYPE = PFD_LAYER_TYPE(1i8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PFD_PIXEL_TYPE(pub u8);
pub const PFD_STEREO: PFD_FLAGS = PFD_FLAGS(2u32);
pub const PFD_STEREO_DONTCARE: PFD_FLAGS = PFD_FLAGS(2147483648u32);
pub const PFD_SUPPORT_COMPOSITION: PFD_FLAGS = PFD_FLAGS(32768u32);
pub const PFD_SUPPORT_DIRECTDRAW: PFD_FLAGS = PFD_FLAGS(8192u32);
pub const PFD_SUPPORT_GDI: PFD_FLAGS = PFD_FLAGS(16u32);
pub const PFD_SUPPORT_OPENGL: PFD_FLAGS = PFD_FLAGS(32u32);
pub const PFD_SWAP_COPY: PFD_FLAGS = PFD_FLAGS(1024u32);
pub const PFD_SWAP_EXCHANGE: PFD_FLAGS = PFD_FLAGS(512u32);
pub const PFD_SWAP_LAYER_BUFFERS: PFD_FLAGS = PFD_FLAGS(2048u32);
pub const PFD_TYPE_COLORINDEX: PFD_PIXEL_TYPE = PFD_PIXEL_TYPE(1u8);
pub const PFD_TYPE_RGBA: PFD_PIXEL_TYPE = PFD_PIXEL_TYPE(0u8);
pub const PFD_UNDERLAY_PLANE: PFD_LAYER_TYPE = PFD_LAYER_TYPE(-1i8);
pub type PFNGLADDSWAPHINTRECTWINPROC = Option<unsafe extern "system" fn(x: i32, y: i32, width: i32, height: i32)>;
pub type PFNGLARRAYELEMENTARRAYEXTPROC = Option<unsafe extern "system" fn(mode: u32, count: i32, pi: *const core::ffi::c_void)>;
pub type PFNGLARRAYELEMENTEXTPROC = Option<unsafe extern "system" fn(i: i32)>;
pub type PFNGLCOLORPOINTEREXTPROC = Option<unsafe extern "system" fn(size: i32, r#type: u32, stride: i32, count: i32, pointer: *const core::ffi::c_void)>;
pub type PFNGLCOLORSUBTABLEEXTPROC = Option<unsafe extern "system" fn(target: u32, start: i32, count: i32, format: u32, r#type: u32, data: *const core::ffi::c_void)>;
pub type PFNGLCOLORTABLEEXTPROC = Option<unsafe extern "system" fn(target: u32, internalformat: u32, width: i32, format: u32, r#type: u32, data: *const core::ffi::c_void)>;
pub type PFNGLDRAWARRAYSEXTPROC = Option<unsafe extern "system" fn(mode: u32, first: i32, count: i32)>;
pub type PFNGLDRAWRANGEELEMENTSWINPROC = Option<unsafe extern "system" fn(mode: u32, start: u32, end: u32, count: i32, r#type: u32, indices: *const core::ffi::c_void)>;
pub type PFNGLEDGEFLAGPOINTEREXTPROC = Option<unsafe extern "system" fn(stride: i32, count: i32, pointer: *const u8)>;
pub type PFNGLGETCOLORTABLEEXTPROC = Option<unsafe extern "system" fn(target: u32, format: u32, r#type: u32, data: *mut core::ffi::c_void)>;
pub type PFNGLGETCOLORTABLEPARAMETERFVEXTPROC = Option<unsafe extern "system" fn(target: u32, pname: u32, params: *mut f32)>;
pub type PFNGLGETCOLORTABLEPARAMETERIVEXTPROC = Option<unsafe extern "system" fn(target: u32, pname: u32, params: *mut i32)>;
pub type PFNGLGETPOINTERVEXTPROC = Option<unsafe extern "system" fn(pname: u32, params: *mut *mut core::ffi::c_void)>;
pub type PFNGLINDEXPOINTEREXTPROC = Option<unsafe extern "system" fn(r#type: u32, stride: i32, count: i32, pointer: *const core::ffi::c_void)>;
pub type PFNGLNORMALPOINTEREXTPROC = Option<unsafe extern "system" fn(r#type: u32, stride: i32, count: i32, pointer: *const core::ffi::c_void)>;
pub type PFNGLTEXCOORDPOINTEREXTPROC = Option<unsafe extern "system" fn(size: i32, r#type: u32, stride: i32, count: i32, pointer: *const core::ffi::c_void)>;
pub type PFNGLVERTEXPOINTEREXTPROC = Option<unsafe extern "system" fn(size: i32, r#type: u32, stride: i32, count: i32, pointer: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PIXELFORMATDESCRIPTOR {
    pub nSize: u16,
    pub nVersion: u16,
    pub dwFlags: PFD_FLAGS,
    pub iPixelType: PFD_PIXEL_TYPE,
    pub cColorBits: u8,
    pub cRedBits: u8,
    pub cRedShift: u8,
    pub cGreenBits: u8,
    pub cGreenShift: u8,
    pub cBlueBits: u8,
    pub cBlueShift: u8,
    pub cAlphaBits: u8,
    pub cAlphaShift: u8,
    pub cAccumBits: u8,
    pub cAccumRedBits: u8,
    pub cAccumGreenBits: u8,
    pub cAccumBlueBits: u8,
    pub cAccumAlphaBits: u8,
    pub cDepthBits: u8,
    pub cStencilBits: u8,
    pub cAuxBuffers: u8,
    pub iLayerType: u8,
    pub bReserved: u8,
    pub dwLayerMask: u32,
    pub dwVisibleMask: u32,
    pub dwDamageMask: u32,
}
impl Default for PIXELFORMATDESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POINTFLOAT {
    pub x: f32,
    pub y: f32,
}
impl Default for POINTFLOAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
