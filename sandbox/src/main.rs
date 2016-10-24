extern crate winapi;
extern crate advapi32;
extern crate kernel32;

use std::ffi::CString;
use std::ptr::{null, null_mut};
use winapi::LUID;
use winapi::winnt::{self, TOKEN_ADJUST_PRIVILEGES, TOKEN_QUERY};
use kernel32::GetCurrentProcess;
use advapi32::{OpenProcessToken, LookupPrivilegeValueA};


fn enable_priviledges(name: &str, enable: bool) -> bool {
  // 1. retrieve the process token of current process.
  let mut h_token = null_mut();
  let ret = unsafe {
    OpenProcessToken(GetCurrentProcess(),
                     TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY,
                     &mut h_token)
  };
  if ret == 0 {
    println!("[debug] failed to retrieve process token");
    return false;
  }

  // 2. retrieve a LUID for given priviledge
  let mut symlink_luid = LUID {
    LowPart: 0,
    HighPart: 0,
  };
  let ret = unsafe {
  let name = CString::new(name).unwrap();
    LookupPrivilegeValueA(null(), name.as_ptr(), &mut symlink_luid)
  };
  if ret == 0 {
    println!("[debug] failed to retrieve a LUID");
    unsafe { kernel32::CloseHandle(h_token) };
    return false;
  }

  let token_priviledges = vec![0u8; std::mem::size_of::<winnt::TOKEN_PRIVILEGES>() + 1];
  let mut p = token_priviledges.as_ptr() as *mut winnt::TOKEN_PRIVILEGES;
  unsafe {
    (*p).PrivilegeCount = 1;
    (*p).Privileges[0].Luid = symlink_luid;
    (*p).Privileges[0].Attributes = if enable {
      winnt::SE_PRIVILEGE_ENABLED
    } else {
      0
    };
  }

  unsafe {
    advapi32::AdjustTokenPrivileges(h_token,
                                    0,
                                    token_priviledges.as_ptr() as *mut winnt::TOKEN_PRIVILEGES,
                                    0,
                                    null_mut(),
                                    null_mut());
  }

  if unsafe { kernel32::GetLastError() } == winapi::winerror::ERROR_SUCCESS {
    unsafe { kernel32::CloseHandle(h_token) };
    true
  } else {
    println!("[debug] failed to adjust priviledge");
    unsafe { kernel32::CloseHandle(h_token) };
    false
  }
}


fn main() {
  let enabled = enable_priviledges("SeCreateSymbolicLinkPriviledge", true);
  if enabled {
    println!("success to enable SeCreateSymbolicLinkPriviledge");
  } else {
    println!("failed");
  }
}
