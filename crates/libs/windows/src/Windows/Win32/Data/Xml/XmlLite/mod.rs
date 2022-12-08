#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlReader<P0>(riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IMalloc>>,
{
    ::windows::core::link ! ( "xmllite.dll""system" fn CreateXmlReader ( riid : *const :: windows::core::GUID , ppvobject : *mut *mut ::core::ffi::c_void , pmalloc : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CreateXmlReader(riid, ppvobject, pmalloc.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingCodePage<P0, P1, P2, P3>(pinputstream: P0, pmalloc: P1, nencodingcodepage: u32, fencodinghint: P2, pwszbaseuri: P3) -> ::windows::core::Result<::windows::core::IUnknown>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IMalloc>>,
    P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "xmllite.dll""system" fn CreateXmlReaderInputWithEncodingCodePage ( pinputstream : * mut::core::ffi::c_void , pmalloc : * mut::core::ffi::c_void , nencodingcodepage : u32 , fencodinghint : super::super::super::Foundation:: BOOL , pwszbaseuri : :: windows::core::PCWSTR , ppinput : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateXmlReaderInputWithEncodingCodePage(pinputstream.into().abi(), pmalloc.into().abi(), nencodingcodepage, fencodinghint.into(), pwszbaseuri.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn CreateXmlReaderInputWithEncodingName<P0, P1, P2, P3, P4>(pinputstream: P0, pmalloc: P1, pwszencodingname: P2, fencodinghint: P3, pwszbaseuri: P4) -> ::windows::core::Result<::windows::core::IUnknown>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IMalloc>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P3: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "xmllite.dll""system" fn CreateXmlReaderInputWithEncodingName ( pinputstream : * mut::core::ffi::c_void , pmalloc : * mut::core::ffi::c_void , pwszencodingname : :: windows::core::PCWSTR , fencodinghint : super::super::super::Foundation:: BOOL , pwszbaseuri : :: windows::core::PCWSTR , ppinput : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateXmlReaderInputWithEncodingName(pinputstream.into().abi(), pmalloc.into().abi(), pwszencodingname.into().abi(), fencodinghint.into(), pwszbaseuri.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriter<P0>(riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, pmalloc: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IMalloc>>,
{
    ::windows::core::link ! ( "xmllite.dll""system" fn CreateXmlWriter ( riid : *const :: windows::core::GUID , ppvobject : *mut *mut ::core::ffi::c_void , pmalloc : * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    CreateXmlWriter(riid, ppvobject, pmalloc.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingCodePage<P0, P1>(poutputstream: P0, pmalloc: P1, nencodingcodepage: u32) -> ::windows::core::Result<::windows::core::IUnknown>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IMalloc>>,
{
    ::windows::core::link ! ( "xmllite.dll""system" fn CreateXmlWriterOutputWithEncodingCodePage ( poutputstream : * mut::core::ffi::c_void , pmalloc : * mut::core::ffi::c_void , nencodingcodepage : u32 , ppoutput : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateXmlWriterOutputWithEncodingCodePage(poutputstream.into().abi(), pmalloc.into().abi(), nencodingcodepage, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn CreateXmlWriterOutputWithEncodingName<P0, P1, P2>(poutputstream: P0, pmalloc: P1, pwszencodingname: P2) -> ::windows::core::Result<::windows::core::IUnknown>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IMalloc>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "xmllite.dll""system" fn CreateXmlWriterOutputWithEncodingName ( poutputstream : * mut::core::ffi::c_void , pmalloc : * mut::core::ffi::c_void , pwszencodingname : :: windows::core::PCWSTR , ppoutput : *mut * mut::core::ffi::c_void ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateXmlWriterOutputWithEncodingName(poutputstream.into().abi(), pmalloc.into().abi(), pwszencodingname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
pub struct IXmlReader(::windows::core::IUnknown);
impl IXmlReader {
    pub unsafe fn SetInput<P0>(&self, pinput: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetInput)(::windows::core::Vtable::as_raw(self), pinput.into().abi()).ok()
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProperty)(::windows::core::Vtable::as_raw(self), nproperty, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetProperty)(::windows::core::Vtable::as_raw(self), nproperty, pvalue).ok()
    }
    pub unsafe fn Read(&self, pnodetype: ::core::option::Option<*mut XmlNodeType>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).Read)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pnodetype.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GetNodeType(&self) -> ::windows::core::Result<XmlNodeType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetNodeType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MoveToFirstAttribute(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).MoveToFirstAttribute)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn MoveToNextAttribute(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).MoveToNextAttribute)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn MoveToAttributeByName<P0, P1>(&self, pwszlocalname: P0, pwsznamespaceuri: P1) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).MoveToAttributeByName)(::windows::core::Vtable::as_raw(self), pwszlocalname.into().abi(), pwsznamespaceuri.into().abi())
    }
    pub unsafe fn MoveToElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).MoveToElement)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetQualifiedName(&self, ppwszqualifiedname: ::core::option::Option<*mut ::windows::core::PWSTR>, pcwchqualifiedname: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetQualifiedName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppwszqualifiedname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcwchqualifiedname.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetNamespaceUri(&self, ppwsznamespaceuri: ::core::option::Option<*mut ::windows::core::PWSTR>, pcwchnamespaceuri: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetNamespaceUri)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppwsznamespaceuri.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcwchnamespaceuri.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetLocalName(&self, ppwszlocalname: ::core::option::Option<*mut ::windows::core::PWSTR>, pcwchlocalname: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetLocalName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppwszlocalname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcwchlocalname.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetPrefix(&self, ppwszprefix: ::core::option::Option<*mut ::windows::core::PWSTR>, pcwchprefix: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetPrefix)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppwszprefix.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcwchprefix.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetValue(&self, ppwszvalue: ::core::option::Option<*mut ::windows::core::PWSTR>, pcwchvalue: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetValue)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppwszvalue.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcwchvalue.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn ReadValueChunk(&self, pwchbuffer: &mut [u16], pcwchread: *mut u32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).ReadValueChunk)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwchbuffer.as_ptr()), pwchbuffer.len() as _, pcwchread)
    }
    pub unsafe fn GetBaseUri(&self, ppwszbaseuri: ::core::option::Option<*mut ::windows::core::PWSTR>, pcwchbaseuri: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetBaseUri)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppwszbaseuri.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcwchbaseuri.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDefault(&self) -> super::super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsDefault)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEmptyElement(&self) -> super::super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsEmptyElement)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetLineNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLineNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLinePosition(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLinePosition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAttributeCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAttributeCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDepth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDepth)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEOF(&self) -> super::super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).IsEOF)(::windows::core::Vtable::as_raw(self))
    }
}
::windows::core::interface_hierarchy!(IXmlReader, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXmlReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlReader {}
impl ::core::fmt::Debug for IXmlReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXmlReader {
    type Vtable = IXmlReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc81_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    pub Read: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT,
    pub GetNodeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows::core::HRESULT,
    pub MoveToFirstAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveToNextAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub MoveToAttributeByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub MoveToElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetQualifiedName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszqualifiedname: *mut ::windows::core::PWSTR, pcwchqualifiedname: *mut u32) -> ::windows::core::HRESULT,
    pub GetNamespaceUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwsznamespaceuri: *mut ::windows::core::PWSTR, pcwchnamespaceuri: *mut u32) -> ::windows::core::HRESULT,
    pub GetLocalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszlocalname: *mut ::windows::core::PWSTR, pcwchlocalname: *mut u32) -> ::windows::core::HRESULT,
    pub GetPrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszprefix: *mut ::windows::core::PWSTR, pcwchprefix: *mut u32) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszvalue: *mut ::windows::core::PWSTR, pcwchvalue: *mut u32) -> ::windows::core::HRESULT,
    pub ReadValueChunk: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwchbuffer: ::windows::core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows::core::HRESULT,
    pub GetBaseUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppwszbaseuri: *mut ::windows::core::PWSTR, pcwchbaseuri: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDefault: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEmptyElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEmptyElement: usize,
    pub GetLineNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlinenumber: *mut u32) -> ::windows::core::HRESULT,
    pub GetLinePosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlineposition: *mut u32) -> ::windows::core::HRESULT,
    pub GetAttributeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnattributecount: *mut u32) -> ::windows::core::HRESULT,
    pub GetDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pndepth: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsEOF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsEOF: usize,
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
pub struct IXmlResolver(::windows::core::IUnknown);
impl IXmlResolver {
    pub unsafe fn ResolveUri<P0, P1, P2>(&self, pwszbaseuri: P0, pwszpublicidentifier: P1, pwszsystemidentifier: P2) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ResolveUri)(::windows::core::Vtable::as_raw(self), pwszbaseuri.into().abi(), pwszpublicidentifier.into().abi(), pwszsystemidentifier.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXmlResolver, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXmlResolver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlResolver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlResolver {}
impl ::core::fmt::Debug for IXmlResolver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlResolver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXmlResolver {
    type Vtable = IXmlResolver_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlResolver {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc82_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlResolver_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ResolveUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszbaseuri: ::windows::core::PCWSTR, pwszpublicidentifier: ::windows::core::PCWSTR, pwszsystemidentifier: ::windows::core::PCWSTR, ppresolvedinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
pub struct IXmlWriter(::windows::core::IUnknown);
impl IXmlWriter {
    pub unsafe fn SetOutput<P0>(&self, poutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetOutput)(::windows::core::Vtable::as_raw(self), poutput.into().abi()).ok()
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProperty)(::windows::core::Vtable::as_raw(self), nproperty, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetProperty)(::windows::core::Vtable::as_raw(self), nproperty, pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributes<P0, P1>(&self, preader: P0, fwritedefaultattributes: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXmlReader>>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).WriteAttributes)(::windows::core::Vtable::as_raw(self), preader.into().abi(), fwritedefaultattributes.into()).ok()
    }
    pub unsafe fn WriteAttributeString<P0, P1, P2, P3>(&self, pwszprefix: P0, pwszlocalname: P1, pwsznamespaceuri: P2, pwszvalue: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteAttributeString)(::windows::core::Vtable::as_raw(self), pwszprefix.into().abi(), pwszlocalname.into().abi(), pwsznamespaceuri.into().abi(), pwszvalue.into().abi()).ok()
    }
    pub unsafe fn WriteCData<P0>(&self, pwsztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteCData)(::windows::core::Vtable::as_raw(self), pwsztext.into().abi()).ok()
    }
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteCharEntity)(::windows::core::Vtable::as_raw(self), wch).ok()
    }
    pub unsafe fn WriteChars(&self, pwch: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteChars)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwch.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pwch.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn WriteComment<P0>(&self, pwszcomment: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteComment)(::windows::core::Vtable::as_raw(self), pwszcomment.into().abi()).ok()
    }
    pub unsafe fn WriteDocType<P0, P1, P2, P3>(&self, pwszname: P0, pwszpublicid: P1, pwszsystemid: P2, pwszsubset: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteDocType)(::windows::core::Vtable::as_raw(self), pwszname.into().abi(), pwszpublicid.into().abi(), pwszsystemid.into().abi(), pwszsubset.into().abi()).ok()
    }
    pub unsafe fn WriteElementString<P0, P1, P2, P3>(&self, pwszprefix: P0, pwszlocalname: P1, pwsznamespaceuri: P2, pwszvalue: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteElementString)(::windows::core::Vtable::as_raw(self), pwszprefix.into().abi(), pwszlocalname.into().abi(), pwsznamespaceuri.into().abi(), pwszvalue.into().abi()).ok()
    }
    pub unsafe fn WriteEndDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteEndDocument)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn WriteEndElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteEndElement)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn WriteEntityRef<P0>(&self, pwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteEntityRef)(::windows::core::Vtable::as_raw(self), pwszname.into().abi()).ok()
    }
    pub unsafe fn WriteFullEndElement(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteFullEndElement)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn WriteName<P0>(&self, pwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteName)(::windows::core::Vtable::as_raw(self), pwszname.into().abi()).ok()
    }
    pub unsafe fn WriteNmToken<P0>(&self, pwsznmtoken: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteNmToken)(::windows::core::Vtable::as_raw(self), pwsznmtoken.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNode<P0, P1>(&self, preader: P0, fwritedefaultattributes: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXmlReader>>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).WriteNode)(::windows::core::Vtable::as_raw(self), preader.into().abi(), fwritedefaultattributes.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNodeShallow<P0, P1>(&self, preader: P0, fwritedefaultattributes: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXmlReader>>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).WriteNodeShallow)(::windows::core::Vtable::as_raw(self), preader.into().abi(), fwritedefaultattributes.into()).ok()
    }
    pub unsafe fn WriteProcessingInstruction<P0, P1>(&self, pwszname: P0, pwsztext: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteProcessingInstruction)(::windows::core::Vtable::as_raw(self), pwszname.into().abi(), pwsztext.into().abi()).ok()
    }
    pub unsafe fn WriteQualifiedName<P0, P1>(&self, pwszlocalname: P0, pwsznamespaceuri: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteQualifiedName)(::windows::core::Vtable::as_raw(self), pwszlocalname.into().abi(), pwsznamespaceuri.into().abi()).ok()
    }
    pub unsafe fn WriteRaw<P0>(&self, pwszdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteRaw)(::windows::core::Vtable::as_raw(self), pwszdata.into().abi()).ok()
    }
    pub unsafe fn WriteRawChars(&self, pwch: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteRawChars)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwch.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pwch.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteStartDocument)(::windows::core::Vtable::as_raw(self), standalone).ok()
    }
    pub unsafe fn WriteStartElement<P0, P1, P2>(&self, pwszprefix: P0, pwszlocalname: P1, pwsznamespaceuri: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteStartElement)(::windows::core::Vtable::as_raw(self), pwszprefix.into().abi(), pwszlocalname.into().abi(), pwsznamespaceuri.into().abi()).ok()
    }
    pub unsafe fn WriteString<P0>(&self, pwsztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteString)(::windows::core::Vtable::as_raw(self), pwsztext.into().abi()).ok()
    }
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteSurrogateCharEntity)(::windows::core::Vtable::as_raw(self), wchlow, wchhigh).ok()
    }
    pub unsafe fn WriteWhitespace<P0>(&self, pwszwhitespace: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteWhitespace)(::windows::core::Vtable::as_raw(self), pwszwhitespace.into().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Flush)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IXmlWriter, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXmlWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlWriter {}
impl ::core::fmt::Debug for IXmlWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXmlWriter {
    type Vtable = IXmlWriter_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc88_709d_4095_b63d_69fe4b0d9030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteAttributes: usize,
    pub WriteAttributeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: ::windows::core::PCWSTR, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteCData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT,
    pub WriteChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT,
    pub WriteComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomment: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteDocType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwszpublicid: ::windows::core::PCWSTR, pwszsystemid: ::windows::core::PCWSTR, pwszsubset: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteElementString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: ::windows::core::PCWSTR, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteEndDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteEntityRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteFullEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteNmToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznmtoken: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNodeShallow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNodeShallow: usize,
    pub WriteProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteQualifiedName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteRawChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT,
    pub WriteStartDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT,
    pub WriteStartElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszprefix: ::windows::core::PCWSTR, pwszlocalname: ::windows::core::PCWSTR, pwsznamespaceuri: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteSurrogateCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT,
    pub WriteWhitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszwhitespace: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
pub struct IXmlWriterLite(::windows::core::IUnknown);
impl IXmlWriterLite {
    pub unsafe fn SetOutput<P0>(&self, poutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).SetOutput)(::windows::core::Vtable::as_raw(self), poutput.into().abi()).ok()
    }
    pub unsafe fn GetProperty(&self, nproperty: u32) -> ::windows::core::Result<isize> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProperty)(::windows::core::Vtable::as_raw(self), nproperty, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProperty(&self, nproperty: u32, pvalue: isize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetProperty)(::windows::core::Vtable::as_raw(self), nproperty, pvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteAttributes<P0, P1>(&self, preader: P0, fwritedefaultattributes: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXmlReader>>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).WriteAttributes)(::windows::core::Vtable::as_raw(self), preader.into().abi(), fwritedefaultattributes.into()).ok()
    }
    pub unsafe fn WriteAttributeString(&self, pwszqname: &[u16], pwszvalue: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteAttributeString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszqname.as_ptr()), pwszqname.len() as _, ::core::mem::transmute(pwszvalue.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pwszvalue.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn WriteCData<P0>(&self, pwsztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteCData)(::windows::core::Vtable::as_raw(self), pwsztext.into().abi()).ok()
    }
    pub unsafe fn WriteCharEntity(&self, wch: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteCharEntity)(::windows::core::Vtable::as_raw(self), wch).ok()
    }
    pub unsafe fn WriteChars(&self, pwch: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteChars)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwch.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pwch.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn WriteComment<P0>(&self, pwszcomment: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteComment)(::windows::core::Vtable::as_raw(self), pwszcomment.into().abi()).ok()
    }
    pub unsafe fn WriteDocType<P0, P1, P2, P3>(&self, pwszname: P0, pwszpublicid: P1, pwszsystemid: P2, pwszsubset: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteDocType)(::windows::core::Vtable::as_raw(self), pwszname.into().abi(), pwszpublicid.into().abi(), pwszsystemid.into().abi(), pwszsubset.into().abi()).ok()
    }
    pub unsafe fn WriteElementString<P0>(&self, pwszqname: &[u16], pwszvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteElementString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszqname.as_ptr()), pwszqname.len() as _, pwszvalue.into().abi()).ok()
    }
    pub unsafe fn WriteEndDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteEndDocument)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn WriteEndElement(&self, pwszqname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteEndElement)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszqname.as_ptr()), pwszqname.len() as _).ok()
    }
    pub unsafe fn WriteEntityRef<P0>(&self, pwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteEntityRef)(::windows::core::Vtable::as_raw(self), pwszname.into().abi()).ok()
    }
    pub unsafe fn WriteFullEndElement(&self, pwszqname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteFullEndElement)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszqname.as_ptr()), pwszqname.len() as _).ok()
    }
    pub unsafe fn WriteName<P0>(&self, pwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteName)(::windows::core::Vtable::as_raw(self), pwszname.into().abi()).ok()
    }
    pub unsafe fn WriteNmToken<P0>(&self, pwsznmtoken: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteNmToken)(::windows::core::Vtable::as_raw(self), pwsznmtoken.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNode<P0, P1>(&self, preader: P0, fwritedefaultattributes: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXmlReader>>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).WriteNode)(::windows::core::Vtable::as_raw(self), preader.into().abi(), fwritedefaultattributes.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteNodeShallow<P0, P1>(&self, preader: P0, fwritedefaultattributes: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXmlReader>>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).WriteNodeShallow)(::windows::core::Vtable::as_raw(self), preader.into().abi(), fwritedefaultattributes.into()).ok()
    }
    pub unsafe fn WriteProcessingInstruction<P0, P1>(&self, pwszname: P0, pwsztext: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteProcessingInstruction)(::windows::core::Vtable::as_raw(self), pwszname.into().abi(), pwsztext.into().abi()).ok()
    }
    pub unsafe fn WriteRaw<P0>(&self, pwszdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteRaw)(::windows::core::Vtable::as_raw(self), pwszdata.into().abi()).ok()
    }
    pub unsafe fn WriteRawChars(&self, pwch: ::core::option::Option<&[u16]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteRawChars)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwch.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pwch.as_deref().map_or(0, |slice| slice.len() as _)).ok()
    }
    pub unsafe fn WriteStartDocument(&self, standalone: XmlStandalone) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteStartDocument)(::windows::core::Vtable::as_raw(self), standalone).ok()
    }
    pub unsafe fn WriteStartElement(&self, pwszqname: &[u16]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteStartElement)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszqname.as_ptr()), pwszqname.len() as _).ok()
    }
    pub unsafe fn WriteString<P0>(&self, pwsztext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteString)(::windows::core::Vtable::as_raw(self), pwsztext.into().abi()).ok()
    }
    pub unsafe fn WriteSurrogateCharEntity(&self, wchlow: u16, wchhigh: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).WriteSurrogateCharEntity)(::windows::core::Vtable::as_raw(self), wchlow, wchhigh).ok()
    }
    pub unsafe fn WriteWhitespace<P0>(&self, pwszwhitespace: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).WriteWhitespace)(::windows::core::Vtable::as_raw(self), pwszwhitespace.into().abi()).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Flush)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IXmlWriterLite, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXmlWriterLite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXmlWriterLite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXmlWriterLite {}
impl ::core::fmt::Debug for IXmlWriterLite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXmlWriterLite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IXmlWriterLite {
    type Vtable = IXmlWriterLite_Vtbl;
}
unsafe impl ::windows::core::Interface for IXmlWriterLite {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x862494c6_1310_4aad_b3cd_2dbeebf670d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXmlWriterLite_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows::core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteAttributes: usize,
    pub WriteAttributeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32, pwszvalue: ::windows::core::PCWSTR, cwszvalue: u32) -> ::windows::core::HRESULT,
    pub WriteCData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows::core::HRESULT,
    pub WriteChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT,
    pub WriteComment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszcomment: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteDocType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwszpublicid: ::windows::core::PCWSTR, pwszsystemid: ::windows::core::PCWSTR, pwszsubset: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteElementString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteEndDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WriteEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    pub WriteEntityRef: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteFullEndElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    pub WriteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteNmToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsznmtoken: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub WriteNodeShallow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WriteNodeShallow: usize,
    pub WriteProcessingInstruction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteRaw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteRawChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwch: ::windows::core::PCWSTR, cwch: u32) -> ::windows::core::HRESULT,
    pub WriteStartDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows::core::HRESULT,
    pub WriteStartElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszqname: ::windows::core::PCWSTR, cwszqname: u32) -> ::windows::core::HRESULT,
    pub WriteString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwsztext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub WriteSurrogateCharEntity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows::core::HRESULT,
    pub WriteWhitespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszwhitespace: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Flush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
pub const _IID_IXmlReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc81_709d_4095_b63d_69fe4b0d9030);
pub const _IID_IXmlResolver: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc82_709d_4095_b63d_69fe4b0d9030);
pub const _IID_IXmlWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7279fc88_709d_4095_b63d_69fe4b0d9030);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DtdProcessing(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const DtdProcessing_Prohibit: DtdProcessing = DtdProcessing(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const DtdProcessing_Parse: DtdProcessing = DtdProcessing(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _DtdProcessing_Last: DtdProcessing = DtdProcessing(1i32);
impl ::core::marker::Copy for DtdProcessing {}
impl ::core::clone::Clone for DtdProcessing {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DtdProcessing {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DtdProcessing {
    type Abi = Self;
}
impl ::core::fmt::Debug for DtdProcessing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DtdProcessing").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XmlConformanceLevel(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlConformanceLevel_Auto: XmlConformanceLevel = XmlConformanceLevel(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlConformanceLevel_Fragment: XmlConformanceLevel = XmlConformanceLevel(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlConformanceLevel_Document: XmlConformanceLevel = XmlConformanceLevel(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlConformanceLevel_Last: XmlConformanceLevel = XmlConformanceLevel(2i32);
impl ::core::marker::Copy for XmlConformanceLevel {}
impl ::core::clone::Clone for XmlConformanceLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlConformanceLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlConformanceLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlConformanceLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlConformanceLevel").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XmlError(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_MX: XmlError = XmlError(-1072894464i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_INPUTEND: XmlError = XmlError(-1072894463i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_ENCODING: XmlError = XmlError(-1072894462i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_ENCODINGSWITCH: XmlError = XmlError(-1072894461i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const MX_E_ENCODINGSIGNATURE: XmlError = XmlError(-1072894460i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_WC: XmlError = XmlError(-1072894432i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_WHITESPACE: XmlError = XmlError(-1072894431i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SEMICOLON: XmlError = XmlError(-1072894430i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_GREATERTHAN: XmlError = XmlError(-1072894429i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_QUOTE: XmlError = XmlError(-1072894428i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_EQUAL: XmlError = XmlError(-1072894427i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LESSTHAN: XmlError = XmlError(-1072894426i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_HEXDIGIT: XmlError = XmlError(-1072894425i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DIGIT: XmlError = XmlError(-1072894424i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LEFTBRACKET: XmlError = XmlError(-1072894423i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LEFTPAREN: XmlError = XmlError(-1072894422i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_XMLCHARACTER: XmlError = XmlError(-1072894421i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NAMECHARACTER: XmlError = XmlError(-1072894420i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SYNTAX: XmlError = XmlError(-1072894419i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_CDSECT: XmlError = XmlError(-1072894418i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_COMMENT: XmlError = XmlError(-1072894417i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_CONDSECT: XmlError = XmlError(-1072894416i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLATTLIST: XmlError = XmlError(-1072894415i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLDOCTYPE: XmlError = XmlError(-1072894414i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLELEMENT: XmlError = XmlError(-1072894413i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLENTITY: XmlError = XmlError(-1072894412i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DECLNOTATION: XmlError = XmlError(-1072894411i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NDATA: XmlError = XmlError(-1072894410i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PUBLIC: XmlError = XmlError(-1072894409i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SYSTEM: XmlError = XmlError(-1072894408i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NAME: XmlError = XmlError(-1072894407i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ROOTELEMENT: XmlError = XmlError(-1072894406i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ELEMENTMATCH: XmlError = XmlError(-1072894405i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_UNIQUEATTRIBUTE: XmlError = XmlError(-1072894404i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_TEXTXMLDECL: XmlError = XmlError(-1072894403i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_LEADINGXML: XmlError = XmlError(-1072894402i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_TEXTDECL: XmlError = XmlError(-1072894401i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_XMLDECL: XmlError = XmlError(-1072894400i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ENCNAME: XmlError = XmlError(-1072894399i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PUBLICID: XmlError = XmlError(-1072894398i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PESINTERNALSUBSET: XmlError = XmlError(-1072894397i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PESBETWEENDECLS: XmlError = XmlError(-1072894396i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NORECURSION: XmlError = XmlError(-1072894395i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_ENTITYCONTENT: XmlError = XmlError(-1072894394i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_UNDECLAREDENTITY: XmlError = XmlError(-1072894393i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PARSEDENTITY: XmlError = XmlError(-1072894392i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_NOEXTERNALENTITYREF: XmlError = XmlError(-1072894391i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_PI: XmlError = XmlError(-1072894390i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_SYSTEMID: XmlError = XmlError(-1072894389i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_QUESTIONMARK: XmlError = XmlError(-1072894388i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_CDSECTEND: XmlError = XmlError(-1072894387i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_MOREDATA: XmlError = XmlError(-1072894386i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_DTDPROHIBITED: XmlError = XmlError(-1072894385i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WC_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894384i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_NC: XmlError = XmlError(-1072894368i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_QNAMECHARACTER: XmlError = XmlError(-1072894367i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_QNAMECOLON: XmlError = XmlError(-1072894366i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_NAMECOLON: XmlError = XmlError(-1072894365i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_DECLAREDPREFIX: XmlError = XmlError(-1072894364i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_UNDECLAREDPREFIX: XmlError = XmlError(-1072894363i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_EMPTYURI: XmlError = XmlError(-1072894362i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLPREFIXRESERVED: XmlError = XmlError(-1072894361i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLNSPREFIXRESERVED: XmlError = XmlError(-1072894360i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLURIRESERVED: XmlError = XmlError(-1072894359i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const NC_E_XMLNSURIRESERVED: XmlError = XmlError(-1072894358i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const SC_E_SC: XmlError = XmlError(-1072894336i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const SC_E_MAXELEMENTDEPTH: XmlError = XmlError(-1072894335i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const SC_E_MAXENTITYEXPANSION: XmlError = XmlError(-1072894334i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_WR: XmlError = XmlError(-1072894208i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NONWHITESPACE: XmlError = XmlError(-1072894207i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NSPREFIXDECLARED: XmlError = XmlError(-1072894206i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NSPREFIXWITHEMPTYNSURI: XmlError = XmlError(-1072894205i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_DUPLICATEATTRIBUTE: XmlError = XmlError(-1072894204i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLNSPREFIXDECLARATION: XmlError = XmlError(-1072894203i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLPREFIXDECLARATION: XmlError = XmlError(-1072894202i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLURIDECLARATION: XmlError = XmlError(-1072894201i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_XMLNSURIDECLARATION: XmlError = XmlError(-1072894200i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_NAMESPACEUNDECLARED: XmlError = XmlError(-1072894199i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_INVALIDXMLSPACE: XmlError = XmlError(-1072894198i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_INVALIDACTION: XmlError = XmlError(-1072894197i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const WR_E_INVALIDSURROGATEPAIR: XmlError = XmlError(-1072894196i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALID_DECIMAL: XmlError = XmlError(-1072898019i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALID_HEXIDECIMAL: XmlError = XmlError(-1072898018i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALID_UNICODE: XmlError = XmlError(-1072898017i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XML_E_INVALIDENCODING: XmlError = XmlError(-1072897938i32);
impl ::core::marker::Copy for XmlError {}
impl ::core::clone::Clone for XmlError {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlError {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlError {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlError").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XmlNodeType(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_None: XmlNodeType = XmlNodeType(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Element: XmlNodeType = XmlNodeType(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Attribute: XmlNodeType = XmlNodeType(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Text: XmlNodeType = XmlNodeType(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_CDATA: XmlNodeType = XmlNodeType(4i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_ProcessingInstruction: XmlNodeType = XmlNodeType(7i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Comment: XmlNodeType = XmlNodeType(8i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_DocumentType: XmlNodeType = XmlNodeType(10i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_Whitespace: XmlNodeType = XmlNodeType(13i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_EndElement: XmlNodeType = XmlNodeType(15i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlNodeType_XmlDeclaration: XmlNodeType = XmlNodeType(17i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlNodeType_Last: XmlNodeType = XmlNodeType(17i32);
impl ::core::marker::Copy for XmlNodeType {}
impl ::core::clone::Clone for XmlNodeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlNodeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlNodeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlNodeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlNodeType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XmlReadState(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Initial: XmlReadState = XmlReadState(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Interactive: XmlReadState = XmlReadState(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Error: XmlReadState = XmlReadState(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_EndOfFile: XmlReadState = XmlReadState(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReadState_Closed: XmlReadState = XmlReadState(4i32);
impl ::core::marker::Copy for XmlReadState {}
impl ::core::clone::Clone for XmlReadState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlReadState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlReadState {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlReadState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlReadState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XmlReaderProperty(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_MultiLanguage: XmlReaderProperty = XmlReaderProperty(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_ConformanceLevel: XmlReaderProperty = XmlReaderProperty(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_RandomAccess: XmlReaderProperty = XmlReaderProperty(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_XmlResolver: XmlReaderProperty = XmlReaderProperty(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_DtdProcessing: XmlReaderProperty = XmlReaderProperty(4i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_ReadState: XmlReaderProperty = XmlReaderProperty(5i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_MaxElementDepth: XmlReaderProperty = XmlReaderProperty(6i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlReaderProperty_MaxEntityExpansion: XmlReaderProperty = XmlReaderProperty(7i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlReaderProperty_Last: XmlReaderProperty = XmlReaderProperty(7i32);
impl ::core::marker::Copy for XmlReaderProperty {}
impl ::core::clone::Clone for XmlReaderProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlReaderProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlReaderProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlReaderProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlReaderProperty").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XmlStandalone(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlStandalone_Omit: XmlStandalone = XmlStandalone(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlStandalone_Yes: XmlStandalone = XmlStandalone(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlStandalone_No: XmlStandalone = XmlStandalone(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlStandalone_Last: XmlStandalone = XmlStandalone(2i32);
impl ::core::marker::Copy for XmlStandalone {}
impl ::core::clone::Clone for XmlStandalone {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlStandalone {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlStandalone {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlStandalone {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlStandalone").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XmlWriterProperty(pub i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_MultiLanguage: XmlWriterProperty = XmlWriterProperty(0i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_Indent: XmlWriterProperty = XmlWriterProperty(1i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_ByteOrderMark: XmlWriterProperty = XmlWriterProperty(2i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_OmitXmlDeclaration: XmlWriterProperty = XmlWriterProperty(3i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_ConformanceLevel: XmlWriterProperty = XmlWriterProperty(4i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const XmlWriterProperty_CompactEmptyElement: XmlWriterProperty = XmlWriterProperty(5i32);
#[doc = "*Required features: `\"Win32_Data_Xml_XmlLite\"`*"]
pub const _XmlWriterProperty_Last: XmlWriterProperty = XmlWriterProperty(5i32);
impl ::core::marker::Copy for XmlWriterProperty {}
impl ::core::clone::Clone for XmlWriterProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XmlWriterProperty {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XmlWriterProperty {
    type Abi = Self;
}
impl ::core::fmt::Debug for XmlWriterProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XmlWriterProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
