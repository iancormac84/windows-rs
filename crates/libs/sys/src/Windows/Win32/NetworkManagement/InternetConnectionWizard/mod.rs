// Bindings generated by `riddle` 0.0.1

#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_ALREADYRUN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_CHECKSTATUS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_FULLPRESENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_FULL_SMARTSTART: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_LAUNCHEDFULL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_LAUNCHEDMANUAL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_LAUNCHFULL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_LAUNCHMANUAL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_MANUALPRESENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_MAX_ACCTNAME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_MAX_EMAILADDR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_MAX_EMAILNAME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_MAX_LOGONNAME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_MAX_PASSWORD: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_MAX_RASNAME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_MAX_SERVERNAME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_REGKEYCOMPLETED: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("Completed");
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_REGPATHSETTINGS: ::windows_sys::core::PCSTR = ::windows_sys::core::s!("Software\\Microsoft\\Internet Connection Wizard");
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_USEDEFAULTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub const ICW_USE_SHELLNEXT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub type PFNCHECKCONNECTIONWIZARD = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_InternetConnectionWizard\"`*"]
pub type PFNSETSHELLNEXT = ::core::option::Option<unsafe extern "system" fn(param0: ::windows_sys::core::PCSTR) -> u32>;
