windows_core::imp::define_interface!(IWsbApplicationAsync, IWsbApplicationAsync_Vtbl, 0x0843f6f7_895c_44a6_b0c2_05a5022aa3a1);
impl core::ops::Deref for IWsbApplicationAsync {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWsbApplicationAsync, windows_core::IUnknown);
impl IWsbApplicationAsync {
    pub unsafe fn QueryStatus(&self) -> windows_core::Result<windows_core::HRESULT> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).QueryStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok()
    }
}
#[repr(C)]
pub struct IWsbApplicationAsync_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWsbApplicationAsync_Impl: Sized + windows_core::IUnknownImpl {
    fn QueryStatus(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn Abort(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWsbApplicationAsync {}
impl IWsbApplicationAsync_Vtbl {
    pub const fn new<Identity: IWsbApplicationAsync_Impl, const OFFSET: isize>() -> IWsbApplicationAsync_Vtbl {
        unsafe extern "system" fn QueryStatus<Identity: IWsbApplicationAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phrresult: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWsbApplicationAsync_Impl::QueryStatus(this) {
                Ok(ok__) => {
                    phrresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Identity: IWsbApplicationAsync_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWsbApplicationAsync_Impl::Abort(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), QueryStatus: QueryStatus::<Identity, OFFSET>, Abort: Abort::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWsbApplicationAsync as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IWsbApplicationBackupSupport, IWsbApplicationBackupSupport_Vtbl, 0x1eff3510_4a27_46ad_b9e0_08332f0f4f6d);
impl core::ops::Deref for IWsbApplicationBackupSupport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWsbApplicationBackupSupport, windows_core::IUnknown);
impl IWsbApplicationBackupSupport {
    pub unsafe fn CheckConsistency<P0, P1, P2>(&self, wszwritermetadata: P0, wszcomponentname: P1, wszcomponentlogicalpath: P2, cvolumes: u32, rgwszsourcevolumepath: *const windows_core::PCWSTR, rgwszsnapshotvolumepath: *const windows_core::PCWSTR) -> windows_core::Result<IWsbApplicationAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).CheckConsistency)(windows_core::Interface::as_raw(self), wszwritermetadata.param().abi(), wszcomponentname.param().abi(), wszcomponentlogicalpath.param().abi(), cvolumes, rgwszsourcevolumepath, rgwszsnapshotvolumepath, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct IWsbApplicationBackupSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CheckConsistency: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32, *const windows_core::PCWSTR, *const windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWsbApplicationBackupSupport_Impl: Sized + windows_core::IUnknownImpl {
    fn CheckConsistency(&self, wszwritermetadata: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcomponentlogicalpath: &windows_core::PCWSTR, cvolumes: u32, rgwszsourcevolumepath: *const windows_core::PCWSTR, rgwszsnapshotvolumepath: *const windows_core::PCWSTR) -> windows_core::Result<IWsbApplicationAsync>;
}
impl windows_core::RuntimeName for IWsbApplicationBackupSupport {}
impl IWsbApplicationBackupSupport_Vtbl {
    pub const fn new<Identity: IWsbApplicationBackupSupport_Impl, const OFFSET: isize>() -> IWsbApplicationBackupSupport_Vtbl {
        unsafe extern "system" fn CheckConsistency<Identity: IWsbApplicationBackupSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszwritermetadata: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcomponentlogicalpath: windows_core::PCWSTR, cvolumes: u32, rgwszsourcevolumepath: *const windows_core::PCWSTR, rgwszsnapshotvolumepath: *const windows_core::PCWSTR, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWsbApplicationBackupSupport_Impl::CheckConsistency(this, core::mem::transmute(&wszwritermetadata), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcomponentlogicalpath), core::mem::transmute_copy(&cvolumes), core::mem::transmute_copy(&rgwszsourcevolumepath), core::mem::transmute_copy(&rgwszsnapshotvolumepath)) {
                Ok(ok__) => {
                    ppasync.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CheckConsistency: CheckConsistency::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWsbApplicationBackupSupport as windows_core::Interface>::IID
    }
}
windows_core::imp::define_interface!(IWsbApplicationRestoreSupport, IWsbApplicationRestoreSupport_Vtbl, 0x8d3bdb38_4ee8_4718_85f9_c7dbc4ab77aa);
impl core::ops::Deref for IWsbApplicationRestoreSupport {
    type Target = windows_core::IUnknown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWsbApplicationRestoreSupport, windows_core::IUnknown);
impl IWsbApplicationRestoreSupport {
    pub unsafe fn PreRestore<P0, P1, P2, P3>(&self, wszwritermetadata: P0, wszcomponentname: P1, wszcomponentlogicalpath: P2, bnorollforward: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::Foundation::BOOLEAN>,
    {
        (windows_core::Interface::vtable(self).PreRestore)(windows_core::Interface::as_raw(self), wszwritermetadata.param().abi(), wszcomponentname.param().abi(), wszcomponentlogicalpath.param().abi(), bnorollforward.param().abi()).ok()
    }
    pub unsafe fn PostRestore<P0, P1, P2, P3>(&self, wszwritermetadata: P0, wszcomponentname: P1, wszcomponentlogicalpath: P2, bnorollforward: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::super::Foundation::BOOLEAN>,
    {
        (windows_core::Interface::vtable(self).PostRestore)(windows_core::Interface::as_raw(self), wszwritermetadata.param().abi(), wszcomponentname.param().abi(), wszcomponentlogicalpath.param().abi(), bnorollforward.param().abi()).ok()
    }
    pub unsafe fn OrderComponents(&self, ccomponents: u32, rgcomponentname: *const windows_core::PCWSTR, rgcomponentlogicalpaths: *const windows_core::PCWSTR, prgcomponentname: *mut *mut windows_core::PWSTR, prgcomponentlogicalpath: *mut *mut windows_core::PWSTR) -> windows_core::Result<()> {
        (windows_core::Interface::vtable(self).OrderComponents)(windows_core::Interface::as_raw(self), ccomponents, rgcomponentname, rgcomponentlogicalpaths, prgcomponentname, prgcomponentlogicalpath).ok()
    }
    pub unsafe fn IsRollForwardSupported(&self) -> windows_core::Result<u8> {
        let mut result__ = core::mem::zeroed();
        (windows_core::Interface::vtable(self).IsRollForwardSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IWsbApplicationRestoreSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PreRestore: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, super::super::Foundation::BOOLEAN) -> windows_core::HRESULT,
    pub PostRestore: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, super::super::Foundation::BOOLEAN) -> windows_core::HRESULT,
    pub OrderComponents: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::PCWSTR, *const windows_core::PCWSTR, *mut *mut windows_core::PWSTR, *mut *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsRollForwardSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
}
pub trait IWsbApplicationRestoreSupport_Impl: Sized + windows_core::IUnknownImpl {
    fn PreRestore(&self, wszwritermetadata: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcomponentlogicalpath: &windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> windows_core::Result<()>;
    fn PostRestore(&self, wszwritermetadata: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcomponentlogicalpath: &windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> windows_core::Result<()>;
    fn OrderComponents(&self, ccomponents: u32, rgcomponentname: *const windows_core::PCWSTR, rgcomponentlogicalpaths: *const windows_core::PCWSTR, prgcomponentname: *mut *mut windows_core::PWSTR, prgcomponentlogicalpath: *mut *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn IsRollForwardSupported(&self) -> windows_core::Result<u8>;
}
impl windows_core::RuntimeName for IWsbApplicationRestoreSupport {}
impl IWsbApplicationRestoreSupport_Vtbl {
    pub const fn new<Identity: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>() -> IWsbApplicationRestoreSupport_Vtbl {
        unsafe extern "system" fn PreRestore<Identity: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszwritermetadata: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcomponentlogicalpath: windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWsbApplicationRestoreSupport_Impl::PreRestore(this, core::mem::transmute(&wszwritermetadata), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcomponentlogicalpath), core::mem::transmute_copy(&bnorollforward)).into()
        }
        unsafe extern "system" fn PostRestore<Identity: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszwritermetadata: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcomponentlogicalpath: windows_core::PCWSTR, bnorollforward: super::super::Foundation::BOOLEAN) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWsbApplicationRestoreSupport_Impl::PostRestore(this, core::mem::transmute(&wszwritermetadata), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcomponentlogicalpath), core::mem::transmute_copy(&bnorollforward)).into()
        }
        unsafe extern "system" fn OrderComponents<Identity: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ccomponents: u32, rgcomponentname: *const windows_core::PCWSTR, rgcomponentlogicalpaths: *const windows_core::PCWSTR, prgcomponentname: *mut *mut windows_core::PWSTR, prgcomponentlogicalpath: *mut *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWsbApplicationRestoreSupport_Impl::OrderComponents(this, core::mem::transmute_copy(&ccomponents), core::mem::transmute_copy(&rgcomponentname), core::mem::transmute_copy(&rgcomponentlogicalpaths), core::mem::transmute_copy(&prgcomponentname), core::mem::transmute_copy(&prgcomponentlogicalpath)).into()
        }
        unsafe extern "system" fn IsRollForwardSupported<Identity: IWsbApplicationRestoreSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrollforwardsupported: *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWsbApplicationRestoreSupport_Impl::IsRollForwardSupported(this) {
                Ok(ok__) => {
                    pbrollforwardsupported.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreRestore: PreRestore::<Identity, OFFSET>,
            PostRestore: PostRestore::<Identity, OFFSET>,
            OrderComponents: OrderComponents::<Identity, OFFSET>,
            IsRollForwardSupported: IsRollForwardSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWsbApplicationRestoreSupport as windows_core::Interface>::IID
    }
}
pub const WSBAPP_ASYNC_IN_PROGRESS: windows_core::HRESULT = windows_core::HRESULT(0x7A0004_u32 as _);
pub const WSB_MAX_OB_STATUS_ENTRY: u32 = 5u32;
pub const WSB_MAX_OB_STATUS_VALUE_TYPE_PAIR: u32 = 5u32;
pub const WSB_OB_ET_DATETIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(3i32);
pub const WSB_OB_ET_MAX: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(6i32);
pub const WSB_OB_ET_NUMBER: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(2i32);
pub const WSB_OB_ET_SIZE: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(5i32);
pub const WSB_OB_ET_STRING: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(1i32);
pub const WSB_OB_ET_TIME: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(4i32);
pub const WSB_OB_ET_UNDEFINED: WSB_OB_STATUS_ENTRY_PAIR_TYPE = WSB_OB_STATUS_ENTRY_PAIR_TYPE(0i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct WSB_OB_STATUS_ENTRY_PAIR_TYPE(pub i32);
impl windows_core::TypeKind for WSB_OB_STATUS_ENTRY_PAIR_TYPE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for WSB_OB_STATUS_ENTRY_PAIR_TYPE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WSB_OB_STATUS_ENTRY_PAIR_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSB_OB_REGISTRATION_INFO {
    pub m_wszResourceDLL: windows_core::PWSTR,
    pub m_guidSnapinId: windows_core::GUID,
    pub m_dwProviderName: u32,
    pub m_dwProviderIcon: u32,
    pub m_bSupportsRemoting: super::super::Foundation::BOOLEAN,
}
impl windows_core::TypeKind for WSB_OB_REGISTRATION_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WSB_OB_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSB_OB_STATUS_ENTRY {
    pub m_dwIcon: u32,
    pub m_dwStatusEntryName: u32,
    pub m_dwStatusEntryValue: u32,
    pub m_cValueTypePair: u32,
    pub m_rgValueTypePair: *mut WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR,
}
impl windows_core::TypeKind for WSB_OB_STATUS_ENTRY {
    type TypeKind = windows_core::CopyType;
}
impl Default for WSB_OB_STATUS_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    pub m_wszObStatusEntryPairValue: windows_core::PWSTR,
    pub m_ObStatusEntryPairType: WSB_OB_STATUS_ENTRY_PAIR_TYPE,
}
impl windows_core::TypeKind for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    type TypeKind = windows_core::CopyType;
}
impl Default for WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSB_OB_STATUS_INFO {
    pub m_guidSnapinId: windows_core::GUID,
    pub m_cStatusEntry: u32,
    pub m_rgStatusEntry: *mut WSB_OB_STATUS_ENTRY,
}
impl windows_core::TypeKind for WSB_OB_STATUS_INFO {
    type TypeKind = windows_core::CopyType;
}
impl Default for WSB_OB_STATUS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
