// Bindings generated by `windows-bindgen` 0.58.0

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
windows_core::imp::define_interface!(
    IJsonValidator,
    IJsonValidator_Vtbl,
    0xe09cb12b_b13c_5139_8c99_6140bf80deb9
);
impl windows_core::RuntimeType for IJsonValidator {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJsonValidator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Validate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        core::mem::MaybeUninit<windows_core::HSTRING>,
        *mut core::mem::MaybeUninit<windows_core::HSTRING>,
    ) -> windows_core::HRESULT,
}
pub trait IJsonValidator_Impl: Sized + windows_core::IUnknownImpl {
    fn Validate(
        &self,
        value: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IJsonValidator {
    const NAME: &'static str = "Sample.IJsonValidator";
}
impl IJsonValidator_Vtbl {
    pub const fn new<Identity: IJsonValidator_Impl, const OFFSET: isize>() -> IJsonValidator_Vtbl {
        unsafe extern "system" fn Validate<Identity: IJsonValidator_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            value: core::mem::MaybeUninit<windows_core::HSTRING>,
            result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>,
        ) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IJsonValidator_Impl::Validate(this, core::mem::transmute(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IJsonValidator, OFFSET>(),
            Validate: Validate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsonValidator as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(
    IJsonValidatorFactory,
    IJsonValidatorFactory_Vtbl,
    0x1cf4464e_ae9e_55d5_9539_0af4d8fc35aa
);
impl windows_core::RuntimeType for IJsonValidatorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IJsonValidatorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        core::mem::MaybeUninit<windows_core::HSTRING>,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IJsonValidatorFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn CreateInstance(&self, schema: &windows_core::HSTRING)
        -> windows_core::Result<JsonValidator>;
}
impl windows_core::RuntimeName for IJsonValidatorFactory {
    const NAME: &'static str = "Sample.IJsonValidatorFactory";
}
impl IJsonValidatorFactory_Vtbl {
    pub const fn new<Identity: IJsonValidatorFactory_Impl, const OFFSET: isize>(
    ) -> IJsonValidatorFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<
            Identity: IJsonValidatorFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            schema: core::mem::MaybeUninit<windows_core::HSTRING>,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IJsonValidatorFactory_Impl::CreateInstance(this, core::mem::transmute(&schema)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IJsonValidatorFactory, OFFSET>(
            ),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJsonValidatorFactory as windows_core::Interface>::IID
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct JsonValidator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(
    JsonValidator,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl JsonValidator {
    pub fn Validate(
        &self,
        value: &windows_core::HSTRING,
    ) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Validate)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(value),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateInstance(schema: &windows_core::HSTRING) -> windows_core::Result<JsonValidator> {
        Self::IJsonValidatorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(
                windows_core::Interface::as_raw(this),
                core::mem::transmute_copy(schema),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IJsonValidatorFactory<R, F: FnOnce(&IJsonValidatorFactory) -> windows_core::Result<R>>(
        callback: F,
    ) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<JsonValidator, IJsonValidatorFactory> =
            windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for JsonValidator {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_class::<Self, IJsonValidator>();
}
unsafe impl windows_core::Interface for JsonValidator {
    type Vtable = IJsonValidator_Vtbl;
    const IID: windows_core::GUID = <IJsonValidator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for JsonValidator {
    const NAME: &'static str = "Sample.JsonValidator";
}
unsafe impl Send for JsonValidator {}
unsafe impl Sync for JsonValidator {}
