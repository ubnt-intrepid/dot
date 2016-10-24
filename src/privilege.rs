use std::ffi::CString;
use std::mem::size_of;
use std::ptr::{null, null_mut};
use winapi::LUID;
use winapi::winerror::ERROR_SUCCESS;
use winapi::winnt;
use kernel32;
use advapi32;


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


pub fn enable_privilege(name: &str) -> bool {
  // 1. retrieve the process token of current process.
  let mut h_token = null_mut();
  let ret = unsafe {
    advapi32::OpenProcessToken(kernel32::GetCurrentProcess(),
                               winnt::TOKEN_ADJUST_PRIVILEGES | winnt::TOKEN_QUERY,
                               &mut h_token)
  };
  if ret == 0 {
    return false;
  }
  let h_token = Handle::new(h_token);

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

  let token_privileges = vec![0u8; size_of::<winnt::TOKEN_PRIVILEGES>() + 1];
  unsafe {
    let mut p = token_privileges.as_ptr() as *mut winnt::TOKEN_PRIVILEGES;
    (*p).PrivilegeCount = 1;
    (*((*p).Privileges.as_ptr() as *mut winnt::LUID_AND_ATTRIBUTES)).Luid = luid;
    (*((*p).Privileges.as_ptr() as *mut winnt::LUID_AND_ATTRIBUTES)).Attributes =
      winnt::SE_PRIVILEGE_ENABLED;
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
