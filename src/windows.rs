use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr::{null, null_mut};
use winapi::LUID;
use winapi::winerror::ERROR_SUCCESS;
use winapi::winnt;
use kernel32;
use advapi32;


#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
struct TOKEN_ELEVATION {
  TokenIsElevated: u32,
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum TOKEN_INFORMATION_CLASS {
  TokenUser = 1,
  TokenGroups,
  TokenPrivileges,
  TokenOwner,
  TokenPrimaryGroup,
  TokenDefaultDacl,
  TokenSource,
  TokenType,
  TokenImpersonationLevel,
  TokenStatistics,
  TokenRestrictedSids,
  TokenSessionId,
  TokenGroupsAndPrivileges,
  TokenSessionReference,
  TokenSandBoxInert,
  TokenAuditPolicy,
  TokenOrigin,
  TokenElevationType,
  TokenLinkedToken,
  TokenElevation,
  TokenHasRestrictions,
  TokenAccessInformation,
  TokenVirtualizationAllowed,
  TokenVirtualizationEnabled,
  TokenIntegrityLevel,
  TokenUIAccess,
  TokenMandatoryPolicy,
  TokenLogonSid,
  TokenIsAppContainer,
  TokenCapabilities,
  TokenAppContainerSid,
  TokenAppContainerNumber,
  TokenUserClaimAttributes,
  TokenDeviceClaimAttributes,
  TokenRestrictedUserClaimAttributes,
  TokenRestrictedDeviceClaimAttributes,
  TokenDeviceGroups,
  TokenRestrictedDeviceGroups,
  TokenSecurityAttributes,
  TokenIsRestricted,
  MaxTokenInfoClass,
}

extern "system" {
  fn GetTokenInformation(TokenHandle: winnt::HANDLE,
                         TokenInformationClass: u32,
                         TokenInformation: *mut c_void,
                         TokenInformationLength: u32,
                         ReturnLength: *mut u32)
                         -> i32;

  fn IsUserAnAdmin() -> i32;
}



struct Handle(winnt::HANDLE);

impl Handle {
  fn new(h: winnt::HANDLE) -> Handle {
    Handle(h)
  }

  fn as_raw(&self) -> winnt::HANDLE {
    self.0
  }
}

impl Drop for Handle {
  fn drop(&mut self) {
    unsafe { kernel32::CloseHandle(self.0) };
    self.0 = null_mut();
  }
}


fn open_process_token(token_type: u32) -> Option<Handle> {
  let mut h_token = null_mut();
  let ret =
    unsafe { advapi32::OpenProcessToken(kernel32::GetCurrentProcess(), token_type, &mut h_token) };
  if ret == 0 {
    return None;
  }

  Some(Handle::new(h_token))
}


pub fn enable_privilege(name: &str) -> bool {
  // 1. retrieve the process token of current process.
  let h_token = match open_process_token(winnt::TOKEN_ADJUST_PRIVILEGES | winnt::TOKEN_QUERY) {
    Some(h) => h,
    None => return false,
  };

  // 2. retrieve a LUID for given priviledge
  let mut luid = LUID {
    LowPart: 0,
    HighPart: 0,
  };
  let ret = unsafe {
    let name = CString::new(name).unwrap();
    advapi32::LookupPrivilegeValueA(null(), name.as_ptr(), &mut luid)
  };
  if ret == 0 {
    return false;
  }

  let token_privileges = vec![0u8; mem::size_of::<winnt::TOKEN_PRIVILEGES>() + 1];
  unsafe {
    let mut p = token_privileges.as_ptr() as *mut winnt::TOKEN_PRIVILEGES;
    let mut la = (*p).Privileges.as_ptr() as *mut winnt::LUID_AND_ATTRIBUTES;
    (*p).PrivilegeCount = 1;
    (*la).Luid = luid;
    (*la).Attributes = winnt::SE_PRIVILEGE_ENABLED;
  }

  unsafe {
    advapi32::AdjustTokenPrivileges(h_token.as_raw(),
                                    0,
                                    token_privileges.as_ptr() as *mut winnt::TOKEN_PRIVILEGES,
                                    0,
                                    null_mut(),
                                    null_mut());
  }

  (unsafe { kernel32::GetLastError() }) == ERROR_SUCCESS
}

pub fn is_elevated() -> bool {
  let token = match open_process_token(winnt::TOKEN_QUERY) {
    Some(h) => h,
    None => return false,
  };

  let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
  let mut cb_size: u32 = mem::size_of_val(&elevation) as u32;
  let ret = unsafe {
    GetTokenInformation(token.as_raw(),
                        mem::transmute::<_, u8>(TOKEN_INFORMATION_CLASS::TokenElevation) as u32,
                        mem::transmute(&mut elevation),
                        mem::size_of_val(&elevation) as u32,
                        &mut cb_size)
  };
  if ret == 0 {
    return false;
  }

  elevation.TokenIsElevated != 0
}

pub fn is_user_an_admin() -> bool {
  (unsafe { IsUserAnAdmin() }) != 0
}
