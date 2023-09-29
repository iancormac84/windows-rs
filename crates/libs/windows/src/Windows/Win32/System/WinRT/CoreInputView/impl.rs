#[doc = "Required features: `Win32_Foundation`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICoreFrameworkInputViewInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ICoreFrameworkInputViewInterop {}
#[cfg(feature = "Win32_Foundation")]
impl ICoreFrameworkInputViewInterop_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreFrameworkInputViewInterop_Impl, const OFFSET: isize>() -> ICoreFrameworkInputViewInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreFrameworkInputViewInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&coreframeworkinputview)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ICoreFrameworkInputViewInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ICoreFrameworkInputViewInterop as ::windows_core::ComInterface>::IID
    }
}
