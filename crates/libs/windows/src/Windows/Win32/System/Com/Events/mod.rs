windows_core::imp::define_interface!(IDontSupportEventSubscription, IDontSupportEventSubscription_Vtbl, 0x784121f1_62a6_4b89_855f_d65f296de83a);
impl core::ops::Deref for IDontSupportEventSubscription {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDontSupportEventSubscription, windows_core::IUnknown);
impl IDontSupportEventSubscription {}
#[repr(C)]
pub struct IDontSupportEventSubscription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IDontSupportEventSubscription_Impl: Sized + windows_core::IUnknownImpl {}
impl windows_core::RuntimeName for IDontSupportEventSubscription {}
impl IDontSupportEventSubscription_Vtbl {
    pub const fn new<Identity: IDontSupportEventSubscription_Impl, const OFFSET: isize>() -> IDontSupportEventSubscription_Vtbl {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDontSupportEventSubscription as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEnumEventObject, IEnumEventObject_Vtbl, 0xf4a07d63_2e25_11d1_9964_00c04fbbb345);
impl core::ops::Deref for IEnumEventObject {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEnumEventObject, windows_core::IUnknown);
impl IEnumEventObject {
    pub unsafe fn Clone(&self) -> windows_core::Result<IEnumEventObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Next(&self, ppinterface: &mut [Option<windows_core::IUnknown>], cretelem: *mut u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), ppinterface.len().try_into().unwrap(), core::mem::transmute(ppinterface.as_ptr()), cretelem).ok()
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self))
    }
    pub unsafe fn Skip(&self, cskipelem: u32) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), cskipelem).ok()
    }
}
#[repr(C)]
pub struct IEnumEventObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IEnumEventObject_Impl: Sized + windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IEnumEventObject>;
    fn Next(&self, creqelem: u32, ppinterface: *mut Option<windows_core::IUnknown>, cretelem: *mut u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::HRESULT;
    fn Skip(&self, cskipelem: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnumEventObject {}
impl IEnumEventObject_Vtbl {
    pub const fn new<Identity: IEnumEventObject_Impl, const OFFSET: isize>() -> IEnumEventObject_Vtbl {
        unsafe extern "system" fn Clone<Identity: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumEventObject_Impl::Clone(this) {
                Ok(ok__) => {
                    ppinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut core::ffi::c_void, cretelem: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumEventObject_Impl::Next(this, core::mem::transmute_copy(&creqelem), core::mem::transmute_copy(&ppinterface), core::mem::transmute_copy(&cretelem)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumEventObject_Impl::Reset(this)
        }
        unsafe extern "system" fn Skip<Identity: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cskipelem: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumEventObject_Impl::Skip(this, core::mem::transmute_copy(&cskipelem)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumEventObject as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEventClass, IEventClass_Vtbl, 0xfb2b72a0_7a68_11d1_88f9_0080c7d771bf);
impl core::ops::Deref for IEventClass {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventClass, windows_core::IUnknown, super::IDispatch);
impl IEventClass {
    pub unsafe fn EventClassID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventClassID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventClassID<P0>(&self, bstreventclassid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEventClassID)(windows_core::Interface::as_raw(self), bstreventclassid.param().abi()).ok()
    }
    pub unsafe fn EventClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventClassName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventClassName<P0>(&self, bstreventclassname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEventClassName)(windows_core::Interface::as_raw(self), bstreventclassname.param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OwnerSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOwnerSID)(windows_core::Interface::as_raw(self), bstrownersid.param().abi()).ok()
    }
    pub unsafe fn FiringInterfaceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FiringInterfaceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetFiringInterfaceID<P0>(&self, bstrfiringinterfaceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetFiringInterfaceID)(windows_core::Interface::as_raw(self), bstrfiringinterfaceid.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn CustomConfigCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CustomConfigCLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetCustomConfigCLSID<P0>(&self, bstrcustomconfigclsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetCustomConfigCLSID)(windows_core::Interface::as_raw(self), bstrcustomconfigclsid.param().abi()).ok()
    }
    pub unsafe fn TypeLib(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).TypeLib)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetTypeLib<P0>(&self, bstrtypelib: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetTypeLib)(windows_core::Interface::as_raw(self), bstrtypelib.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventClass_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub EventClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EventClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEventClassName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub FiringInterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetFiringInterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub CustomConfigCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetCustomConfigCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub TypeLib: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetTypeLib: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventClass_Impl: Sized + super::IDispatch_Impl {
    fn EventClassID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEventClassID(&self, bstreventclassid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EventClassName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEventClassName(&self, bstreventclassname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FiringInterfaceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFiringInterfaceID(&self, bstrfiringinterfaceid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CustomConfigCLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCustomConfigCLSID(&self, bstrcustomconfigclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TypeLib(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetTypeLib(&self, bstrtypelib: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEventClass {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEventClass_Vtbl {
    pub const fn new<Identity: IEventClass_Impl, const OFFSET: isize>() -> IEventClass_Vtbl {
        unsafe extern "system" fn EventClassID<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::EventClassID(this) {
                Ok(ok__) => {
                    pbstreventclassid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstreventclassid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetEventClassID(this, core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn EventClassName<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::EventClassName(this) {
                Ok(ok__) => {
                    pbstreventclassname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassName<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstreventclassname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetEventClassName(this, core::mem::transmute(&bstreventclassname)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrownersid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::OwnerSID(this) {
                Ok(ok__) => {
                    pbstrownersid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrownersid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetOwnerSID(this, core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn FiringInterfaceID<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfiringinterfaceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::FiringInterfaceID(this) {
                Ok(ok__) => {
                    pbstrfiringinterfaceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFiringInterfaceID<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrfiringinterfaceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetFiringInterfaceID(this, core::mem::transmute(&bstrfiringinterfaceid)).into()
        }
        unsafe extern "system" fn Description<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::Description(this) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn CustomConfigCLSID<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcustomconfigclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::CustomConfigCLSID(this) {
                Ok(ok__) => {
                    pbstrcustomconfigclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomConfigCLSID<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcustomconfigclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetCustomConfigCLSID(this, core::mem::transmute(&bstrcustomconfigclsid)).into()
        }
        unsafe extern "system" fn TypeLib<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtypelib: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass_Impl::TypeLib(this) {
                Ok(ok__) => {
                    pbstrtypelib.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeLib<Identity: IEventClass_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtypelib: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass_Impl::SetTypeLib(this, core::mem::transmute(&bstrtypelib)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EventClassID: EventClassID::<Identity, OFFSET>,
            SetEventClassID: SetEventClassID::<Identity, OFFSET>,
            EventClassName: EventClassName::<Identity, OFFSET>,
            SetEventClassName: SetEventClassName::<Identity, OFFSET>,
            OwnerSID: OwnerSID::<Identity, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, OFFSET>,
            FiringInterfaceID: FiringInterfaceID::<Identity, OFFSET>,
            SetFiringInterfaceID: SetFiringInterfaceID::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            CustomConfigCLSID: CustomConfigCLSID::<Identity, OFFSET>,
            SetCustomConfigCLSID: SetCustomConfigCLSID::<Identity, OFFSET>,
            TypeLib: TypeLib::<Identity, OFFSET>,
            SetTypeLib: SetTypeLib::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventClass as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEventClass2, IEventClass2_Vtbl, 0xfb2b72a1_7a68_11d1_88f9_0080c7d771bf);
impl core::ops::Deref for IEventClass2 {
    type Target = IEventClass;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventClass2, windows_core::IUnknown, super::IDispatch, IEventClass);
impl IEventClass2 {
    pub unsafe fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublisherID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPublisherID)(windows_core::Interface::as_raw(self), bstrpublisherid.param().abi()).ok()
    }
    pub unsafe fn MultiInterfacePublisherFilterCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MultiInterfacePublisherFilterCLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMultiInterfacePublisherFilterCLSID<P0>(&self, bstrpubfilclsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMultiInterfacePublisherFilterCLSID)(windows_core::Interface::as_raw(self), bstrpubfilclsid.param().abi()).ok()
    }
    pub unsafe fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowInprocActivation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowInprocActivation)(windows_core::Interface::as_raw(self), fallowinprocactivation.param().abi()).ok()
    }
    pub unsafe fn FireInParallel(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FireInParallel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFireInParallel<P0>(&self, ffireinparallel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFireInParallel)(windows_core::Interface::as_raw(self), ffireinparallel.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventClass2_Vtbl {
    pub base__: IEventClass_Vtbl,
    pub PublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub FireInParallel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetFireInParallel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventClass2_Impl: Sized + IEventClass_Impl {
    fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MultiInterfacePublisherFilterCLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMultiInterfacePublisherFilterCLSID(&self, bstrpubfilclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn FireInParallel(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(&self, ffireinparallel: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEventClass2 {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEventClass2_Vtbl {
    pub const fn new<Identity: IEventClass2_Impl, const OFFSET: isize>() -> IEventClass2_Vtbl {
        unsafe extern "system" fn PublisherID<Identity: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublisherid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass2_Impl::PublisherID(this) {
                Ok(ok__) => {
                    pbstrpublisherid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass2_Impl::SetPublisherID(this, core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn MultiInterfacePublisherFilterCLSID<Identity: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpubfilclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass2_Impl::MultiInterfacePublisherFilterCLSID(this) {
                Ok(ok__) => {
                    pbstrpubfilclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiInterfacePublisherFilterCLSID<Identity: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpubfilclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass2_Impl::SetMultiInterfacePublisherFilterCLSID(this, core::mem::transmute(&bstrpubfilclsid)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass2_Impl::AllowInprocActivation(this) {
                Ok(ok__) => {
                    pfallowinprocactivation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass2_Impl::SetAllowInprocActivation(this, core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Identity: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventClass2_Impl::FireInParallel(this) {
                Ok(ok__) => {
                    pffireinparallel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Identity: IEventClass2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventClass2_Impl::SetFireInParallel(this, core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base__: IEventClass_Vtbl::new::<Identity, OFFSET>(),
            PublisherID: PublisherID::<Identity, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, OFFSET>,
            MultiInterfacePublisherFilterCLSID: MultiInterfacePublisherFilterCLSID::<Identity, OFFSET>,
            SetMultiInterfacePublisherFilterCLSID: SetMultiInterfacePublisherFilterCLSID::<Identity, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, OFFSET>,
            FireInParallel: FireInParallel::<Identity, OFFSET>,
            SetFireInParallel: SetFireInParallel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventClass2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IEventClass as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEventControl, IEventControl_Vtbl, 0x0343e2f4_86f6_11d1_b760_00c04fb926af);
impl core::ops::Deref for IEventControl {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventControl, windows_core::IUnknown, super::IDispatch);
impl IEventControl {
    pub unsafe fn SetPublisherFilter<P0, P1>(&self, methodname: P0, ppublisherfilter: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IPublisherFilter>,
    {
        (windows_core::Interface::vtable(self).SetPublisherFilter)(windows_core::Interface::as_raw(self), methodname.param().abi(), ppublisherfilter.param().abi()).ok()
    }
    pub unsafe fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowInprocActivation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowInprocActivation)(windows_core::Interface::as_raw(self), fallowinprocactivation.param().abi()).ok()
    }
    pub unsafe fn GetSubscriptions<P0, P1>(&self, methodname: P0, optionalcriteria: P1, optionalerrorindex: *const i32) -> windows_core::Result<IEventObjectCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubscriptions)(windows_core::Interface::as_raw(self), methodname.param().abi(), optionalcriteria.param().abi(), optionalerrorindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefaultQuery<P0, P1>(&self, methodname: P0, criteria: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetDefaultQuery)(windows_core::Interface::as_raw(self), methodname.param().abi(), criteria.param().abi(), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IEventControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SetPublisherFilter: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub GetSubscriptions: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventControl_Impl: Sized + super::IDispatch_Impl {
    fn SetPublisherFilter(&self, methodname: &windows_core::BSTR, ppublisherfilter: Option<&IPublisherFilter>) -> windows_core::Result<()>;
    fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetSubscriptions(&self, methodname: &windows_core::BSTR, optionalcriteria: &windows_core::BSTR, optionalerrorindex: *const i32) -> windows_core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(&self, methodname: &windows_core::BSTR, criteria: &windows_core::BSTR) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEventControl {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEventControl_Vtbl {
    pub const fn new<Identity: IEventControl_Impl, const OFFSET: isize>() -> IEventControl_Vtbl {
        unsafe extern "system" fn SetPublisherFilter<Identity: IEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, ppublisherfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventControl_Impl::SetPublisherFilter(this, core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&ppublisherfilter)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: IEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventControl_Impl::AllowInprocActivation(this) {
                Ok(ok__) => {
                    pfallowinprocactivation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: IEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventControl_Impl::SetAllowInprocActivation(this, core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Identity: IEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, optionalcriteria: core::mem::MaybeUninit<windows_core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventControl_Impl::GetSubscriptions(this, core::mem::transmute(&methodname), core::mem::transmute(&optionalcriteria), core::mem::transmute_copy(&optionalerrorindex)) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: IEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, criteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventControl_Impl::SetDefaultQuery(this, core::mem::transmute(&methodname), core::mem::transmute(&criteria)) {
                Ok(ok__) => {
                    errorindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetPublisherFilter: SetPublisherFilter::<Identity, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, OFFSET>,
            GetSubscriptions: GetSubscriptions::<Identity, OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventControl as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEventObjectChange, IEventObjectChange_Vtbl, 0xf4a07d70_2e25_11d1_9964_00c04fbbb345);
impl core::ops::Deref for IEventObjectChange {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventObjectChange, windows_core::IUnknown);
impl IEventObjectChange {
    pub unsafe fn ChangedSubscription<P0>(&self, changetype: EOC_ChangeType, bstrsubscriptionid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ChangedSubscription)(windows_core::Interface::as_raw(self), changetype, bstrsubscriptionid.param().abi()).ok()
    }
    pub unsafe fn ChangedEventClass<P0>(&self, changetype: EOC_ChangeType, bstreventclassid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ChangedEventClass)(windows_core::Interface::as_raw(self), changetype, bstreventclassid.param().abi()).ok()
    }
    pub unsafe fn ChangedPublisher<P0>(&self, changetype: EOC_ChangeType, bstrpublisherid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).ChangedPublisher)(windows_core::Interface::as_raw(self), changetype, bstrpublisherid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventObjectChange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ChangedSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, EOC_ChangeType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(*mut core::ffi::c_void, EOC_ChangeType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub ChangedPublisher: unsafe extern "system" fn(*mut core::ffi::c_void, EOC_ChangeType, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
pub trait IEventObjectChange_Impl: Sized + windows_core::IUnknownImpl {
    fn ChangedSubscription(&self, changetype: EOC_ChangeType, bstrsubscriptionid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangedEventClass(&self, changetype: EOC_ChangeType, bstreventclassid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangedPublisher(&self, changetype: EOC_ChangeType, bstrpublisherid: &windows_core::BSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEventObjectChange {}
impl IEventObjectChange_Vtbl {
    pub const fn new<Identity: IEventObjectChange_Impl, const OFFSET: isize>() -> IEventObjectChange_Vtbl {
        unsafe extern "system" fn ChangedSubscription<Identity: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectChange_Impl::ChangedSubscription(this, core::mem::transmute_copy(&changetype), core::mem::transmute(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Identity: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectChange_Impl::ChangedEventClass(this, core::mem::transmute_copy(&changetype), core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn ChangedPublisher<Identity: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectChange_Impl::ChangedPublisher(this, core::mem::transmute_copy(&changetype), core::mem::transmute(&bstrpublisherid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Identity, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, OFFSET>,
            ChangedPublisher: ChangedPublisher::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventObjectChange as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEventObjectChange2, IEventObjectChange2_Vtbl, 0x7701a9c3_bd68_438f_83e0_67bf4f53a422);
impl core::ops::Deref for IEventObjectChange2 {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventObjectChange2, windows_core::IUnknown);
impl IEventObjectChange2 {
    pub unsafe fn ChangedSubscription(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChangedSubscription)(windows_core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn ChangedEventClass(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).ChangedEventClass)(windows_core::Interface::as_raw(self), pinfo).ok()
    }
}
#[repr(C)]
pub struct IEventObjectChange2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ChangedSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMEVENTSYSCHANGEINFO) -> windows_core::HRESULT,
    pub ChangedEventClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const COMEVENTSYSCHANGEINFO) -> windows_core::HRESULT,
}
pub trait IEventObjectChange2_Impl: Sized + windows_core::IUnknownImpl {
    fn ChangedSubscription(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::Result<()>;
    fn ChangedEventClass(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEventObjectChange2 {}
impl IEventObjectChange2_Vtbl {
    pub const fn new<Identity: IEventObjectChange2_Impl, const OFFSET: isize>() -> IEventObjectChange2_Vtbl {
        unsafe extern "system" fn ChangedSubscription<Identity: IEventObjectChange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectChange2_Impl::ChangedSubscription(this, core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Identity: IEventObjectChange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectChange2_Impl::ChangedEventClass(this, core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Identity, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventObjectChange2 as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEventObjectCollection, IEventObjectCollection_Vtbl, 0xf89ac270_d4eb_11d1_b682_00805fc79216);
impl core::ops::Deref for IEventObjectCollection {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventObjectCollection, windows_core::IUnknown, super::IDispatch);
impl IEventObjectCollection {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item<P0>(&self, objectid: P0) -> windows_core::Result<super::super::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), objectid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn NewEnum(&self) -> windows_core::Result<IEnumEventObject> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add<P0>(&self, item: *const super::super::Variant::VARIANT, objectid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), core::mem::transmute(item), objectid.param().abi()).ok()
    }
    pub unsafe fn Remove<P0>(&self, objectid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), objectid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventObjectCollection_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    pub NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<super::super::Variant::VARIANT>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventObjectCollection_Impl: Sized + super::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, objectid: &windows_core::BSTR) -> windows_core::Result<super::super::Variant::VARIANT>;
    fn NewEnum(&self) -> windows_core::Result<IEnumEventObject>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn Add(&self, item: *const super::super::Variant::VARIANT, objectid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Remove(&self, objectid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEventObjectCollection {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEventObjectCollection_Vtbl {
    pub const fn new<Identity: IEventObjectCollection_Impl, const OFFSET: isize>() -> IEventObjectCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunkenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventObjectCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppunkenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: core::mem::MaybeUninit<windows_core::BSTR>, pitem: *mut core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventObjectCollection_Impl::get_Item(this, core::mem::transmute(&objectid)) {
                Ok(ok__) => {
                    pitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewEnum<Identity: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventObjectCollection_Impl::NewEnum(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventObjectCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *const core::mem::MaybeUninit<super::super::Variant::VARIANT>, objectid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectCollection_Impl::Add(this, core::mem::transmute_copy(&item), core::mem::transmute(&objectid)).into()
        }
        unsafe extern "system" fn Remove<Identity: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventObjectCollection_Impl::Remove(this, core::mem::transmute(&objectid)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            NewEnum: NewEnum::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventObjectCollection as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEventProperty, IEventProperty_Vtbl, 0xda538ee2_f4de_11d1_b6bb_00805fc79216);
impl core::ops::Deref for IEventProperty {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventProperty, windows_core::IUnknown, super::IDispatch);
impl IEventProperty {
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetName<P0>(&self, propertyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), propertyname.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Value(&self) -> windows_core::Result<super::super::Variant::VARIANT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetValue(&self, propertyvalue: *const super::super::Variant::VARIANT) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute(propertyvalue)).ok()
    }
}
#[repr(C)]
pub struct IEventProperty_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Value: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetValue: usize,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventProperty_Impl: Sized + super::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetName(&self, propertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<super::super::Variant::VARIANT>;
    fn SetValue(&self, propertyvalue: *const super::super::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEventProperty {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEventProperty_Vtbl {
    pub const fn new<Identity: IEventProperty_Impl, const OFFSET: isize>() -> IEventProperty_Vtbl {
        unsafe extern "system" fn Name<Identity: IEventProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventProperty_Impl::Name(this) {
                Ok(ok__) => {
                    propertyname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: IEventProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventProperty_Impl::SetName(this, core::mem::transmute(&propertyname)).into()
        }
        unsafe extern "system" fn Value<Identity: IEventProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyvalue: *mut core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventProperty_Impl::Value(this) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: IEventProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyvalue: *const core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventProperty_Impl::SetValue(this, core::mem::transmute_copy(&propertyvalue)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventProperty as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEventPublisher, IEventPublisher_Vtbl, 0xe341516b_2e32_11d1_9964_00c04fbbb345);
impl core::ops::Deref for IEventPublisher {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventPublisher, windows_core::IUnknown, super::IDispatch);
impl IEventPublisher {
    pub unsafe fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublisherID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPublisherID)(windows_core::Interface::as_raw(self), bstrpublisherid.param().abi()).ok()
    }
    pub unsafe fn PublisherName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublisherName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherName<P0>(&self, bstrpublishername: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPublisherName)(windows_core::Interface::as_raw(self), bstrpublishername.param().abi()).ok()
    }
    pub unsafe fn PublisherType(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublisherType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherType<P0>(&self, bstrpublishertype: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPublisherType)(windows_core::Interface::as_raw(self), bstrpublishertype.param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OwnerSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOwnerSID)(windows_core::Interface::as_raw(self), bstrownersid.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetDefaultProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<super::super::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutDefaultProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const super::super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PutDefaultProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemoveDefaultProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveDefaultProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi()).ok()
    }
    pub unsafe fn GetDefaultPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetDefaultPropertyCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IEventPublisher_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub PublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PublisherName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPublisherName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PublisherType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPublisherType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetDefaultProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetDefaultProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutDefaultProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutDefaultProperty: usize,
    pub RemoveDefaultProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetDefaultPropertyCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventPublisher_Impl: Sized + super::IDispatch_Impl {
    fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PublisherName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPublisherName(&self, bstrpublishername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PublisherType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPublisherType(&self, bstrpublishertype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDefaultProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<super::super::Variant::VARIANT>;
    fn PutDefaultProperty(&self, bstrpropertyname: &windows_core::BSTR, propertyvalue: *const super::super::Variant::VARIANT) -> windows_core::Result<()>;
    fn RemoveDefaultProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDefaultPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEventPublisher {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEventPublisher_Vtbl {
    pub const fn new<Identity: IEventPublisher_Impl, const OFFSET: isize>() -> IEventPublisher_Vtbl {
        unsafe extern "system" fn PublisherID<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublisherid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::PublisherID(this) {
                Ok(ok__) => {
                    pbstrpublisherid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::SetPublisherID(this, core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn PublisherName<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublishername: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::PublisherName(this) {
                Ok(ok__) => {
                    pbstrpublishername.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherName<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublishername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::SetPublisherName(this, core::mem::transmute(&bstrpublishername)).into()
        }
        unsafe extern "system" fn PublisherType<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublishertype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::PublisherType(this) {
                Ok(ok__) => {
                    pbstrpublishertype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherType<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublishertype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::SetPublisherType(this, core::mem::transmute(&bstrpublishertype)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrownersid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::OwnerSID(this) {
                Ok(ok__) => {
                    pbstrownersid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrownersid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::SetOwnerSID(this, core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn Description<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::Description(this) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn GetDefaultProperty<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::GetDefaultProperty(this, core::mem::transmute(&bstrpropertyname)) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutDefaultProperty<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *const core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::PutDefaultProperty(this, core::mem::transmute(&bstrpropertyname), core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveDefaultProperty<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventPublisher_Impl::RemoveDefaultProperty(this, core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetDefaultPropertyCollection<Identity: IEventPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventPublisher_Impl::GetDefaultPropertyCollection(this) {
                Ok(ok__) => {
                    collection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            PublisherID: PublisherID::<Identity, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, OFFSET>,
            PublisherName: PublisherName::<Identity, OFFSET>,
            SetPublisherName: SetPublisherName::<Identity, OFFSET>,
            PublisherType: PublisherType::<Identity, OFFSET>,
            SetPublisherType: SetPublisherType::<Identity, OFFSET>,
            OwnerSID: OwnerSID::<Identity, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            GetDefaultProperty: GetDefaultProperty::<Identity, OFFSET>,
            PutDefaultProperty: PutDefaultProperty::<Identity, OFFSET>,
            RemoveDefaultProperty: RemoveDefaultProperty::<Identity, OFFSET>,
            GetDefaultPropertyCollection: GetDefaultPropertyCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventPublisher as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEventSubscription, IEventSubscription_Vtbl, 0x4a6b0e15_2e38_11d1_9965_00c04fbbb345);
impl core::ops::Deref for IEventSubscription {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventSubscription, windows_core::IUnknown, super::IDispatch);
impl IEventSubscription {
    pub unsafe fn SubscriptionID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubscriptionID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriptionID<P0>(&self, bstrsubscriptionid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubscriptionID)(windows_core::Interface::as_raw(self), bstrsubscriptionid.param().abi()).ok()
    }
    pub unsafe fn SubscriptionName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubscriptionName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriptionName<P0>(&self, bstrsubscriptionname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubscriptionName)(windows_core::Interface::as_raw(self), bstrsubscriptionname.param().abi()).ok()
    }
    pub unsafe fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PublisherID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetPublisherID<P0>(&self, bstrpublisherid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetPublisherID)(windows_core::Interface::as_raw(self), bstrpublisherid.param().abi()).ok()
    }
    pub unsafe fn EventClassID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventClassID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetEventClassID<P0>(&self, bstreventclassid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetEventClassID)(windows_core::Interface::as_raw(self), bstreventclassid.param().abi()).ok()
    }
    pub unsafe fn MethodName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MethodName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMethodName<P0>(&self, bstrmethodname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMethodName)(windows_core::Interface::as_raw(self), bstrmethodname.param().abi()).ok()
    }
    pub unsafe fn SubscriberCLSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubscriberCLSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriberCLSID<P0>(&self, bstrsubscriberclsid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetSubscriberCLSID)(windows_core::Interface::as_raw(self), bstrsubscriberclsid.param().abi()).ok()
    }
    pub unsafe fn SubscriberInterface(&self) -> windows_core::Result<windows_core::IUnknown> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SubscriberInterface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetSubscriberInterface<P0>(&self, psubscriberinterface: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).SetSubscriberInterface)(windows_core::Interface::as_raw(self), psubscriberinterface.param().abi()).ok()
    }
    pub unsafe fn PerUser(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).PerUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetPerUser<P0>(&self, fperuser: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetPerUser)(windows_core::Interface::as_raw(self), fperuser.param().abi()).ok()
    }
    pub unsafe fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).OwnerSID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetOwnerSID<P0>(&self, bstrownersid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetOwnerSID)(windows_core::Interface::as_raw(self), bstrownersid.param().abi()).ok()
    }
    pub unsafe fn Enabled(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Enabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetEnabled<P0>(&self, fenabled: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), fenabled.param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDescription<P0>(&self, bstrdescription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), bstrdescription.param().abi()).ok()
    }
    pub unsafe fn MachineName(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).MachineName)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetMachineName<P0>(&self, bstrmachinename: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetMachineName)(windows_core::Interface::as_raw(self), bstrmachinename.param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetPublisherProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<super::super::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPublisherProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutPublisherProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const super::super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PutPublisherProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemovePublisherProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemovePublisherProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi()).ok()
    }
    pub unsafe fn GetPublisherPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetPublisherPropertyCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetSubscriberProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<super::super::Variant::VARIANT>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubscriberProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PutSubscriberProperty<P0>(&self, bstrpropertyname: P0, propertyvalue: *const super::super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).PutSubscriberProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi(), core::mem::transmute(propertyvalue)).ok()
    }
    pub unsafe fn RemoveSubscriberProperty<P0>(&self, bstrpropertyname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveSubscriberProperty)(windows_core::Interface::as_raw(self), bstrpropertyname.param().abi()).ok()
    }
    pub unsafe fn GetSubscriberPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubscriberPropertyCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).InterfaceID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetInterfaceID<P0>(&self, bstrinterfaceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).SetInterfaceID)(windows_core::Interface::as_raw(self), bstrinterfaceid.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventSubscription_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub SubscriptionID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSubscriptionID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SubscriptionName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSubscriptionName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub PublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetPublisherID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub EventClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetEventClassID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MethodName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMethodName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SubscriberCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetSubscriberCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SubscriberInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSubscriberInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PerUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetPerUser: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub OwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetOwnerSID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub Enabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub MachineName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetMachineName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetPublisherProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetPublisherProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutPublisherProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutPublisherProperty: usize,
    pub RemovePublisherProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetPublisherPropertyCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetSubscriberProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetSubscriberProperty: usize,
    #[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PutSubscriberProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *const core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PutSubscriberProperty: usize,
    pub RemoveSubscriberProperty: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub GetSubscriberPropertyCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub SetInterfaceID: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventSubscription_Impl: Sized + super::IDispatch_Impl {
    fn SubscriptionID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubscriptionID(&self, bstrsubscriptionid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SubscriptionName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubscriptionName(&self, bstrsubscriptionname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PublisherID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EventClassID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetEventClassID(&self, bstreventclassid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MethodName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMethodName(&self, bstrmethodname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SubscriberCLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSubscriberCLSID(&self, bstrsubscriberclsid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SubscriberInterface(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn SetSubscriberInterface(&self, psubscriberinterface: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn PerUser(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetPerUser(&self, fperuser: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn OwnerSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enabled(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetEnabled(&self, fenabled: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, bstrdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MachineName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMachineName(&self, bstrmachinename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPublisherProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<super::super::Variant::VARIANT>;
    fn PutPublisherProperty(&self, bstrpropertyname: &windows_core::BSTR, propertyvalue: *const super::super::Variant::VARIANT) -> windows_core::Result<()>;
    fn RemovePublisherProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPublisherPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection>;
    fn GetSubscriberProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<super::super::Variant::VARIANT>;
    fn PutSubscriberProperty(&self, bstrpropertyname: &windows_core::BSTR, propertyvalue: *const super::super::Variant::VARIANT) -> windows_core::Result<()>;
    fn RemoveSubscriberProperty(&self, bstrpropertyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSubscriberPropertyCollection(&self) -> windows_core::Result<IEventObjectCollection>;
    fn InterfaceID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetInterfaceID(&self, bstrinterfaceid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEventSubscription {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEventSubscription_Vtbl {
    pub const fn new<Identity: IEventSubscription_Impl, const OFFSET: isize>() -> IEventSubscription_Vtbl {
        unsafe extern "system" fn SubscriptionID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubscriptionid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::SubscriptionID(this) {
                Ok(ok__) => {
                    pbstrsubscriptionid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubscriptionid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetSubscriptionID(this, core::mem::transmute(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn SubscriptionName<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubscriptionname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::SubscriptionName(this) {
                Ok(ok__) => {
                    pbstrsubscriptionname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionName<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubscriptionname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetSubscriptionName(this, core::mem::transmute(&bstrsubscriptionname)).into()
        }
        unsafe extern "system" fn PublisherID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpublisherid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::PublisherID(this) {
                Ok(ok__) => {
                    pbstrpublisherid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpublisherid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetPublisherID(this, core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn EventClassID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::EventClassID(this) {
                Ok(ok__) => {
                    pbstreventclassid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstreventclassid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetEventClassID(this, core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn MethodName<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmethodname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::MethodName(this) {
                Ok(ok__) => {
                    pbstrmethodname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMethodName<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmethodname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetMethodName(this, core::mem::transmute(&bstrmethodname)).into()
        }
        unsafe extern "system" fn SubscriberCLSID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsubscriberclsid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::SubscriberCLSID(this) {
                Ok(ok__) => {
                    pbstrsubscriberclsid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberCLSID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsubscriberclsid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetSubscriberCLSID(this, core::mem::transmute(&bstrsubscriberclsid)).into()
        }
        unsafe extern "system" fn SubscriberInterface<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubscriberinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::SubscriberInterface(this) {
                Ok(ok__) => {
                    ppsubscriberinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberInterface<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubscriberinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetSubscriberInterface(this, windows_core::from_raw_borrowed(&psubscriberinterface)).into()
        }
        unsafe extern "system" fn PerUser<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::PerUser(this) {
                Ok(ok__) => {
                    pfperuser.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPerUser<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetPerUser(this, core::mem::transmute_copy(&fperuser)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrownersid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::OwnerSID(this) {
                Ok(ok__) => {
                    pbstrownersid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrownersid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetOwnerSID(this, core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn Enabled<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::Enabled(this) {
                Ok(ok__) => {
                    pfenabled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetEnabled(this, core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn Description<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::Description(this) {
                Ok(ok__) => {
                    pbstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetDescription(this, core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn MachineName<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrmachinename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::MachineName(this) {
                Ok(ok__) => {
                    pbstrmachinename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachineName<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrmachinename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetMachineName(this, core::mem::transmute(&bstrmachinename)).into()
        }
        unsafe extern "system" fn GetPublisherProperty<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::GetPublisherProperty(this, core::mem::transmute(&bstrpropertyname)) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutPublisherProperty<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *const core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::PutPublisherProperty(this, core::mem::transmute(&bstrpropertyname), core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemovePublisherProperty<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::RemovePublisherProperty(this, core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetPublisherPropertyCollection<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::GetPublisherPropertyCollection(this) {
                Ok(ok__) => {
                    collection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriberProperty<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *mut core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::GetSubscriberProperty(this, core::mem::transmute(&bstrpropertyname)) {
                Ok(ok__) => {
                    propertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutSubscriberProperty<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertyvalue: *const core::mem::MaybeUninit<super::super::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::PutSubscriberProperty(this, core::mem::transmute(&bstrpropertyname), core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveSubscriberProperty<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropertyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::RemoveSubscriberProperty(this, core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetSubscriberPropertyCollection<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, collection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::GetSubscriberPropertyCollection(this) {
                Ok(ok__) => {
                    collection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrinterfaceid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSubscription_Impl::InterfaceID(this) {
                Ok(ok__) => {
                    pbstrinterfaceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceID<Identity: IEventSubscription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrinterfaceid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSubscription_Impl::SetInterfaceID(this, core::mem::transmute(&bstrinterfaceid)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SubscriptionID: SubscriptionID::<Identity, OFFSET>,
            SetSubscriptionID: SetSubscriptionID::<Identity, OFFSET>,
            SubscriptionName: SubscriptionName::<Identity, OFFSET>,
            SetSubscriptionName: SetSubscriptionName::<Identity, OFFSET>,
            PublisherID: PublisherID::<Identity, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, OFFSET>,
            EventClassID: EventClassID::<Identity, OFFSET>,
            SetEventClassID: SetEventClassID::<Identity, OFFSET>,
            MethodName: MethodName::<Identity, OFFSET>,
            SetMethodName: SetMethodName::<Identity, OFFSET>,
            SubscriberCLSID: SubscriberCLSID::<Identity, OFFSET>,
            SetSubscriberCLSID: SetSubscriberCLSID::<Identity, OFFSET>,
            SubscriberInterface: SubscriberInterface::<Identity, OFFSET>,
            SetSubscriberInterface: SetSubscriberInterface::<Identity, OFFSET>,
            PerUser: PerUser::<Identity, OFFSET>,
            SetPerUser: SetPerUser::<Identity, OFFSET>,
            OwnerSID: OwnerSID::<Identity, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, OFFSET>,
            Enabled: Enabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
            MachineName: MachineName::<Identity, OFFSET>,
            SetMachineName: SetMachineName::<Identity, OFFSET>,
            GetPublisherProperty: GetPublisherProperty::<Identity, OFFSET>,
            PutPublisherProperty: PutPublisherProperty::<Identity, OFFSET>,
            RemovePublisherProperty: RemovePublisherProperty::<Identity, OFFSET>,
            GetPublisherPropertyCollection: GetPublisherPropertyCollection::<Identity, OFFSET>,
            GetSubscriberProperty: GetSubscriberProperty::<Identity, OFFSET>,
            PutSubscriberProperty: PutSubscriberProperty::<Identity, OFFSET>,
            RemoveSubscriberProperty: RemoveSubscriberProperty::<Identity, OFFSET>,
            GetSubscriberPropertyCollection: GetSubscriberPropertyCollection::<Identity, OFFSET>,
            InterfaceID: InterfaceID::<Identity, OFFSET>,
            SetInterfaceID: SetInterfaceID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventSubscription as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IEventSystem, IEventSystem_Vtbl, 0x4e14fb9f_2e22_11d1_9964_00c04fbbb345);
impl core::ops::Deref for IEventSystem {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEventSystem, windows_core::IUnknown, super::IDispatch);
impl IEventSystem {
    pub unsafe fn Query<P0, P1>(&self, progid: P0, querycriteria: P1, errorindex: *mut i32) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), progid.param().abi(), querycriteria.param().abi(), errorindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn Store<P0, P1>(&self, progid: P0, pinterface: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        (windows_core::Interface::vtable(self).Store)(windows_core::Interface::as_raw(self), progid.param().abi(), pinterface.param().abi()).ok()
    }
    pub unsafe fn Remove<P0, P1>(&self, progid: P0, querycriteria: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), progid.param().abi(), querycriteria.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn EventObjectChangeEventClassID(&self) -> windows_core::Result<windows_core::BSTR> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).EventObjectChangeEventClassID)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn QueryS<P0, P1>(&self, progid: P0, querycriteria: P1) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryS)(windows_core::Interface::as_raw(self), progid.param().abi(), querycriteria.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn RemoveS<P0, P1>(&self, progid: P0, querycriteria: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        (windows_core::Interface::vtable(self).RemoveS)(windows_core::Interface::as_raw(self), progid.param().abi(), querycriteria.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IEventSystem_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Store: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub EventObjectChangeEventClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
    pub QueryS: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveS: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventSystem_Impl: Sized + super::IDispatch_Impl {
    fn Query(&self, progid: &windows_core::BSTR, querycriteria: &windows_core::BSTR, errorindex: *mut i32) -> windows_core::Result<windows_core::IUnknown>;
    fn Store(&self, progid: &windows_core::BSTR, pinterface: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Remove(&self, progid: &windows_core::BSTR, querycriteria: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn EventObjectChangeEventClassID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn QueryS(&self, progid: &windows_core::BSTR, querycriteria: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn RemoveS(&self, progid: &windows_core::BSTR, querycriteria: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEventSystem {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEventSystem_Vtbl {
    pub const fn new<Identity: IEventSystem_Impl, const OFFSET: isize>() -> IEventSystem_Vtbl {
        unsafe extern "system" fn Query<Identity: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSystem_Impl::Query(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria), core::mem::transmute_copy(&errorindex)) {
                Ok(ok__) => {
                    ppinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Store<Identity: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, pinterface: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSystem_Impl::Store(this, core::mem::transmute(&progid), windows_core::from_raw_borrowed(&pinterface)).into()
        }
        unsafe extern "system" fn Remove<Identity: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSystem_Impl::Remove(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria)) {
                Ok(ok__) => {
                    errorindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventObjectChangeEventClassID<Identity: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstreventclassid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSystem_Impl::EventObjectChangeEventClassID(this) {
                Ok(ok__) => {
                    pbstreventclassid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryS<Identity: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEventSystem_Impl::QueryS(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria)) {
                Ok(ok__) => {
                    ppinterface.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveS<Identity: IEventSystem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, progid: core::mem::MaybeUninit<windows_core::BSTR>, querycriteria: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEventSystem_Impl::RemoveS(this, core::mem::transmute(&progid), core::mem::transmute(&querycriteria)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Query: Query::<Identity, OFFSET>,
            Store: Store::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            EventObjectChangeEventClassID: EventObjectChangeEventClassID::<Identity, OFFSET>,
            QueryS: QueryS::<Identity, OFFSET>,
            RemoveS: RemoveS::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEventSystem as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IFiringControl, IFiringControl_Vtbl, 0xe0498c93_4efe_11d1_9971_00c04fbbb345);
impl core::ops::Deref for IFiringControl {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFiringControl, windows_core::IUnknown, super::IDispatch);
impl IFiringControl {
    pub unsafe fn FireSubscription<P0>(&self, subscription: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IEventSubscription>,
    {
        (windows_core::Interface::vtable(self).FireSubscription)(windows_core::Interface::as_raw(self), subscription.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IFiringControl_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub FireSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFiringControl_Impl: Sized + super::IDispatch_Impl {
    fn FireSubscription(&self, subscription: Option<&IEventSubscription>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFiringControl {}
#[cfg(all(feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFiringControl_Vtbl {
    pub const fn new<Identity: IFiringControl_Impl, const OFFSET: isize>() -> IFiringControl_Vtbl {
        unsafe extern "system" fn FireSubscription<Identity: IFiringControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subscription: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFiringControl_Impl::FireSubscription(this, windows_core::from_raw_borrowed(&subscription)).into()
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), FireSubscription: FireSubscription::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFiringControl as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IMultiInterfaceEventControl, IMultiInterfaceEventControl_Vtbl, 0x0343e2f5_86f6_11d1_b760_00c04fb926af);
impl core::ops::Deref for IMultiInterfaceEventControl {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultiInterfaceEventControl, windows_core::IUnknown);
impl IMultiInterfaceEventControl {
    pub unsafe fn SetMultiInterfacePublisherFilter<P0>(&self, classfilter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMultiInterfacePublisherFilter>,
    {
        (windows_core::Interface::vtable(self).SetMultiInterfacePublisherFilter)(windows_core::Interface::as_raw(self), classfilter.param().abi()).ok()
    }
    pub unsafe fn GetSubscriptions<P0, P1>(&self, eventiid: *const windows_core::GUID, bstrmethodname: P0, optionalcriteria: P1, optionalerrorindex: *const i32) -> windows_core::Result<IEventObjectCollection>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).GetSubscriptions)(windows_core::Interface::as_raw(self), eventiid, bstrmethodname.param().abi(), optionalcriteria.param().abi(), optionalerrorindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
    pub unsafe fn SetDefaultQuery<P0, P1>(&self, eventiid: *const windows_core::GUID, bstrmethodname: P0, bstrcriteria: P1) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<windows_core::BSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).SetDefaultQuery)(windows_core::Interface::as_raw(self), eventiid, bstrmethodname.param().abi(), bstrcriteria.param().abi(), &mut result__).map(|| result__)
    }
    pub unsafe fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).AllowInprocActivation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetAllowInprocActivation<P0>(&self, fallowinprocactivation: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetAllowInprocActivation)(windows_core::Interface::as_raw(self), fallowinprocactivation.param().abi()).ok()
    }
    pub unsafe fn FireInParallel(&self) -> windows_core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).FireInParallel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn SetFireInParallel<P0>(&self, ffireinparallel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::BOOL>,
    {
        (windows_core::Interface::vtable(self).SetFireInParallel)(windows_core::Interface::as_raw(self), ffireinparallel.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMultiInterfaceEventControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMultiInterfacePublisherFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubscriptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultQuery: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, core::mem::MaybeUninit<windows_core::BSTR>, core::mem::MaybeUninit<windows_core::BSTR>, *mut i32) -> windows_core::HRESULT,
    pub AllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetAllowInprocActivation: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub FireInParallel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
    pub SetFireInParallel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::BOOL) -> windows_core::HRESULT,
}
pub trait IMultiInterfaceEventControl_Impl: Sized + windows_core::IUnknownImpl {
    fn SetMultiInterfacePublisherFilter(&self, classfilter: Option<&IMultiInterfacePublisherFilter>) -> windows_core::Result<()>;
    fn GetSubscriptions(&self, eventiid: *const windows_core::GUID, bstrmethodname: &windows_core::BSTR, optionalcriteria: &windows_core::BSTR, optionalerrorindex: *const i32) -> windows_core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(&self, eventiid: *const windows_core::GUID, bstrmethodname: &windows_core::BSTR, bstrcriteria: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn AllowInprocActivation(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn FireInParallel(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(&self, ffireinparallel: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMultiInterfaceEventControl {}
impl IMultiInterfaceEventControl_Vtbl {
    pub const fn new<Identity: IMultiInterfaceEventControl_Impl, const OFFSET: isize>() -> IMultiInterfaceEventControl_Vtbl {
        unsafe extern "system" fn SetMultiInterfacePublisherFilter<Identity: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classfilter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiInterfaceEventControl_Impl::SetMultiInterfacePublisherFilter(this, windows_core::from_raw_borrowed(&classfilter)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Identity: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventiid: *const windows_core::GUID, bstrmethodname: core::mem::MaybeUninit<windows_core::BSTR>, optionalcriteria: core::mem::MaybeUninit<windows_core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiInterfaceEventControl_Impl::GetSubscriptions(this, core::mem::transmute_copy(&eventiid), core::mem::transmute(&bstrmethodname), core::mem::transmute(&optionalcriteria), core::mem::transmute_copy(&optionalerrorindex)) {
                Ok(ok__) => {
                    ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventiid: *const windows_core::GUID, bstrmethodname: core::mem::MaybeUninit<windows_core::BSTR>, bstrcriteria: core::mem::MaybeUninit<windows_core::BSTR>, errorindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiInterfaceEventControl_Impl::SetDefaultQuery(this, core::mem::transmute_copy(&eventiid), core::mem::transmute(&bstrmethodname), core::mem::transmute(&bstrcriteria)) {
                Ok(ok__) => {
                    errorindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiInterfaceEventControl_Impl::AllowInprocActivation(this) {
                Ok(ok__) => {
                    pfallowinprocactivation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiInterfaceEventControl_Impl::SetAllowInprocActivation(this, core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Identity: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultiInterfaceEventControl_Impl::FireInParallel(this) {
                Ok(ok__) => {
                    pffireinparallel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Identity: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiInterfaceEventControl_Impl::SetFireInParallel(this, core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMultiInterfacePublisherFilter: SetMultiInterfacePublisherFilter::<Identity, OFFSET>,
            GetSubscriptions: GetSubscriptions::<Identity, OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Identity, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, OFFSET>,
            FireInParallel: FireInParallel::<Identity, OFFSET>,
            SetFireInParallel: SetFireInParallel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiInterfaceEventControl as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IMultiInterfacePublisherFilter, IMultiInterfacePublisherFilter_Vtbl, 0x465e5cc1_7b26_11d1_88fb_0080c7d771bf);
impl core::ops::Deref for IMultiInterfacePublisherFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMultiInterfacePublisherFilter, windows_core::IUnknown);
impl IMultiInterfacePublisherFilter {
    pub unsafe fn Initialize<P0>(&self, peic: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMultiInterfaceEventControl>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), peic.param().abi()).ok()
    }
    pub unsafe fn PrepareToFire<P0, P1>(&self, iid: *const windows_core::GUID, methodname: P0, firingcontrol: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IFiringControl>,
    {
        (windows_core::Interface::vtable(self).PrepareToFire)(windows_core::Interface::as_raw(self), iid, methodname.param().abi(), firingcontrol.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IMultiInterfacePublisherFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMultiInterfacePublisherFilter_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, peic: Option<&IMultiInterfaceEventControl>) -> windows_core::Result<()>;
    fn PrepareToFire(&self, iid: *const windows_core::GUID, methodname: &windows_core::BSTR, firingcontrol: Option<&IFiringControl>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMultiInterfacePublisherFilter {}
impl IMultiInterfacePublisherFilter_Vtbl {
    pub const fn new<Identity: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>() -> IMultiInterfacePublisherFilter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peic: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiInterfacePublisherFilter_Impl::Initialize(this, windows_core::from_raw_borrowed(&peic)).into()
        }
        unsafe extern "system" fn PrepareToFire<Identity: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, methodname: core::mem::MaybeUninit<windows_core::BSTR>, firingcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultiInterfacePublisherFilter_Impl::PrepareToFire(this, core::mem::transmute_copy(&iid), core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&firingcontrol)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultiInterfacePublisherFilter as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IPublisherFilter, IPublisherFilter_Vtbl, 0x465e5cc0_7b26_11d1_88fb_0080c7d771bf);
impl core::ops::Deref for IPublisherFilter {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPublisherFilter, windows_core::IUnknown);
impl IPublisherFilter {
    pub unsafe fn Initialize<P0, P1>(&self, methodname: P0, dispuserdefined: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<super::IDispatch>,
    {
        (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), methodname.param().abi(), dispuserdefined.param().abi()).ok()
    }
    pub unsafe fn PrepareToFire<P0, P1>(&self, methodname: P0, firingcontrol: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::BSTR>,
        P1: windows_core::Param<IFiringControl>,
    {
        (windows_core::Interface::vtable(self).PrepareToFire)(windows_core::Interface::as_raw(self), methodname.param().abi(), firingcontrol.param().abi()).ok()
    }
}
#[repr(C)]
pub struct IPublisherFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PrepareToFire: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::BSTR>, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPublisherFilter_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self, methodname: &windows_core::BSTR, dispuserdefined: Option<&super::IDispatch>) -> windows_core::Result<()>;
    fn PrepareToFire(&self, methodname: &windows_core::BSTR, firingcontrol: Option<&IFiringControl>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPublisherFilter {}
impl IPublisherFilter_Vtbl {
    pub const fn new<Identity: IPublisherFilter_Impl, const OFFSET: isize>() -> IPublisherFilter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IPublisherFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, dispuserdefined: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPublisherFilter_Impl::Initialize(this, core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&dispuserdefined)).into()
        }
        unsafe extern "system" fn PrepareToFire<Identity: IPublisherFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, methodname: core::mem::MaybeUninit<windows_core::BSTR>, firingcontrol: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPublisherFilter_Impl::PrepareToFire(this, core::mem::transmute(&methodname), windows_core::from_raw_borrowed(&firingcontrol)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPublisherFilter as windows_core::Interface>::IID
    }
}
pub const EOC_DeletedObject: EOC_ChangeType = EOC_ChangeType(2i32);
pub const EOC_ModifiedObject: EOC_ChangeType = EOC_ChangeType(1i32);
pub const EOC_NewObject: EOC_ChangeType = EOC_ChangeType(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct EOC_ChangeType(pub i32);
impl windows_core::TypeKind for EOC_ChangeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for EOC_ChangeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EOC_ChangeType").field(&self.0).finish()
    }
}
pub const CEventClass: windows_core::GUID = windows_core::GUID::from_u128(0xcdbec9c0_7a68_11d1_88f9_0080c7d771bf);
pub const CEventPublisher: windows_core::GUID = windows_core::GUID::from_u128(0xab944620_79c6_11d1_88f9_0080c7d771bf);
pub const CEventSubscription: windows_core::GUID = windows_core::GUID::from_u128(0x7542e960_79c7_11d1_88f9_0080c7d771bf);
pub const CEventSystem: windows_core::GUID = windows_core::GUID::from_u128(0x4e14fba2_2e22_11d1_9964_00c04fbbb345);
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub struct COMEVENTSYSCHANGEINFO {
    pub cbSize: u32,
    pub changeType: EOC_ChangeType,
    pub objectId: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub partitionId: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub applicationId: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub reserved: [windows_core::GUID; 10],
}
impl Clone for COMEVENTSYSCHANGEINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl windows_core::TypeKind for COMEVENTSYSCHANGEINFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for COMEVENTSYSCHANGEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EventObjectChange: windows_core::GUID = windows_core::GUID::from_u128(0xd0565000_9df4_11d1_a281_00c04fca0aa7);
pub const EventObjectChange2: windows_core::GUID = windows_core::GUID::from_u128(0xbb07bacd_cd56_4e63_a8ff_cbf0355fb9f4);
