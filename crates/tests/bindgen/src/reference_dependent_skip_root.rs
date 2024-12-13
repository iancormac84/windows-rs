#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub mod Windows {
    pub mod Foundation {
        windows_core::imp::define_interface!(
            IMemoryBuffer,
            IMemoryBuffer_Vtbl,
            0xfbc4dd2a_245b_11e4_af98_689423260cf8
        );
        impl windows_core::RuntimeType for IMemoryBuffer {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::for_interface::<Self>();
        }
        windows_core::imp::interface_hierarchy!(
            IMemoryBuffer,
            windows_core::IUnknown,
            windows_core::IInspectable
        );
        windows_core::imp::required_hierarchy!(
            IMemoryBuffer,
            crate::reference_dependency_skip_root::Windows::Foundation::IClosable
        );
        impl IMemoryBuffer {
            pub fn CreateReference(
                &self,
            ) -> windows_core::Result<
                crate::reference_dependency_skip_root::Windows::Foundation::IMemoryBufferReference,
            > {
                let this = self;
                unsafe {
                    let mut result__ = core::mem::zeroed();
                    (windows_core::Interface::vtable(this).CreateReference)(
                        windows_core::Interface::as_raw(this),
                        &mut result__,
                    )
                    .and_then(|| windows_core::Type::from_abi(result__))
                }
            }
            pub fn Close(&self) -> windows_core::Result<()> {
                let this = &windows_core::Interface::cast::<
                    crate::reference_dependency_skip_root::Windows::Foundation::IClosable,
                >(self)?;
                unsafe {
                    (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(
                        this,
                    ))
                    .ok()
                }
            }
        }
        impl windows_core::RuntimeName for IMemoryBuffer {
            const NAME: &'static str = "Windows.Foundation.IMemoryBuffer";
        }
        pub trait IMemoryBuffer_Impl:
            crate::reference_dependency_skip_root::Windows::Foundation::IClosable_Impl
        {
            fn CreateReference(
                &self,
            ) -> windows_core::Result<
                crate::reference_dependency_skip_root::Windows::Foundation::IMemoryBufferReference,
            >;
        }
        impl IMemoryBuffer_Vtbl {
            pub const fn new<Identity: IMemoryBuffer_Impl, const OFFSET: isize>() -> Self {
                unsafe extern "system" fn CreateReference<
                    Identity: IMemoryBuffer_Impl,
                    const OFFSET: isize,
                >(
                    this: *mut core::ffi::c_void,
                    result__: *mut *mut core::ffi::c_void,
                ) -> windows_core::HRESULT {
                    unsafe {
                        let this: &Identity =
                            &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                        match IMemoryBuffer_Impl::CreateReference(this) {
                            Ok(ok__) => {
                                result__.write(core::mem::transmute_copy(&ok__));
                                core::mem::forget(ok__);
                                windows_core::HRESULT(0)
                            }
                            Err(err) => err.into(),
                        }
                    }
                }
                Self {
                    base__: windows_core::IInspectable_Vtbl::new::<Identity, IMemoryBuffer, OFFSET>(
                    ),
                    CreateReference: CreateReference::<Identity, OFFSET>,
                }
            }
            pub fn matches(iid: &windows_core::GUID) -> bool {
                iid == &<IMemoryBuffer as windows_core::Interface>::IID
            }
        }
        #[repr(C)]
        pub struct IMemoryBuffer_Vtbl {
            pub base__: windows_core::IInspectable_Vtbl,
            pub CreateReference: unsafe extern "system" fn(
                *mut core::ffi::c_void,
                *mut *mut core::ffi::c_void,
            ) -> windows_core::HRESULT,
        }
    }
}
