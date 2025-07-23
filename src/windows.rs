//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use advapi32;
use kernel32;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr::{null, null_mut};
use winapi::winerror::ERROR_SUCCESS;
use winapi::winnt;
use winapi::LUID;

type BYTE = u8;
type BOOL = i32;
type DWORD = u32;

#[repr(C)]
struct TOKEN_ELEVATION {
    TokenIsElevated: DWORD,
}

type TOKEN_ELEVATION_TYPE = u32;

#[repr(C)]
struct TOKEN_GROUPS {
    GroupCount: DWORD,
    Groups: [SID_AND_ATTRIBUTES; 0],
}

#[repr(C)]
struct SID_AND_ATTRIBUTES {
    Sid: PSID,
    Attributes: DWORD,
}

#[repr(C)]
struct SID_IDENTIFIER_AUTHORITY {
    Value: [BYTE; 6],
}

#[repr(C)]
#[allow(improper_ctypes)]
struct SID;

type PSID = *mut SID;
type PSID_IDENTIFIER_AUTHORITY = *mut SID_IDENTIFIER_AUTHORITY;

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
    fn GetTokenInformation(
        TokenHandle: winnt::HANDLE,
        TokenInformationClass: DWORD,
        TokenInformation: *mut c_void,
        TokenInformationLength: DWORD,
        ReturnLength: *mut DWORD,
    ) -> BOOL;

    fn IsUserAnAdmin() -> BOOL;

    fn AllocateAndInitializeSid(
        pIdentifierAuthority: PSID_IDENTIFIER_AUTHORITY,
        nSubAuthorityCount: BYTE,
        dwSubAuthority0: DWORD,
        dwSubAuthority1: DWORD,
        dwSubAuthority2: DWORD,
        dwSubAuthority3: DWORD,
        dwSubAuthority4: DWORD,
        dwSubAuthority5: DWORD,
        dwSubAuthority6: DWORD,
        dwSubAuthority7: DWORD,
        pSid: *mut PSID,
    ) -> BOOL;
    fn FreeSid(pSid: PSID) -> *mut c_void;

    fn CheckTokenMembership(
        TokenHandle: winnt::HANDLE,
        SidToCheck: PSID,
        IsMember: *mut BOOL,
    ) -> BOOL;
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

struct Sid(PSID);

impl Sid {
    fn as_raw(&self) -> PSID {
        self.0
    }
}

impl Drop for Sid {
    fn drop(&mut self) {
        unsafe { FreeSid(self.0) };
        self.0 = null_mut();
    }
}

pub fn enable_privilege(name: &str) -> Result<(), &'static str> {
    // 1. retrieve the process token of current process.
    let token = open_process_token(winnt::TOKEN_ADJUST_PRIVILEGES | winnt::TOKEN_QUERY)?;

    // 2. retrieve a LUID for given priviledge
    let luid = lookup_privilege_value(name)?;

    let len = mem::size_of::<winnt::TOKEN_PRIVILEGES>()
        + 1 * mem::size_of::<winnt::LUID_AND_ATTRIBUTES>();
    let token_privileges = vec![0u8; len];
    unsafe {
        let mut p = token_privileges.as_ptr() as *mut winnt::TOKEN_PRIVILEGES;
        let mut la = (*p).Privileges.as_ptr() as *mut winnt::LUID_AND_ATTRIBUTES;
        (*p).PrivilegeCount = 1;
        (*la).Luid = luid;
        (*la).Attributes = winnt::SE_PRIVILEGE_ENABLED;
    }

    unsafe {
        advapi32::AdjustTokenPrivileges(
            token.as_raw(),
            0,
            token_privileges.as_ptr() as *mut winnt::TOKEN_PRIVILEGES,
            0,
            null_mut(),
            null_mut(),
        );
    }

    match unsafe { kernel32::GetLastError() } {
        ERROR_SUCCESS => Ok(()),
        _ => Err("failed to adjust token privilege"),
    }
}

pub fn is_elevated() -> Result<bool, &'static str> {
    let token = open_process_token(winnt::TOKEN_QUERY)?;

    let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
    let mut cb_size: u32 = mem::size_of_val(&elevation) as u32;
    let ret = unsafe {
        GetTokenInformation(
            token.as_raw(),
            mem::transmute::<_, u8>(TOKEN_INFORMATION_CLASS::TokenElevation) as u32,
            mem::transmute(&mut elevation),
            mem::size_of_val(&elevation) as u32,
            &mut cb_size,
        )
    };
    if ret == 0 {
        return Err("failed to get token information");
    }

    Ok(elevation.TokenIsElevated != 0)
}

#[derive(Debug, PartialEq)]
pub enum ElevationType {
    Default = 1,
    Full,
    Limited,
}

pub fn get_elevation_type() -> Result<ElevationType, &'static str> {
    let token = open_process_token(winnt::TOKEN_QUERY)?;

    let mut elev_type = 0;
    let mut cb_size = mem::size_of_val(&elev_type) as u32;
    let ret = unsafe {
        GetTokenInformation(
            token.as_raw(),
            mem::transmute::<_, u8>(TOKEN_INFORMATION_CLASS::TokenElevationType) as u32,
            mem::transmute(&mut elev_type),
            mem::size_of_val(&elev_type) as u32,
            &mut cb_size,
        )
    };
    if ret == 0 {
        return Err("failed to get token information");
    }

    match elev_type {
        1 => Ok(ElevationType::Default), // default (standard user/ administrator without UAC)
        2 => Ok(ElevationType::Full),    // full access (administrator, not elevated)
        3 => Ok(ElevationType::Limited), // limited access (administrator, not elevated)
        _ => Err("unknown elevation type"),
    }
}

fn open_process_token(token_type: u32) -> Result<Handle, &'static str> {
    let mut h_token = null_mut();
    let ret = unsafe {
        advapi32::OpenProcessToken(kernel32::GetCurrentProcess(), token_type, &mut h_token)
    };
    match ret {
        0 => Err("failed to get process token"),
        _ => Ok(Handle::new(h_token)),
    }
}

fn lookup_privilege_value(name: &str) -> Result<LUID, &'static str> {
    let mut luid = LUID {
        LowPart: 0,
        HighPart: 0,
    };
    let ret = unsafe {
        let name = CString::new(name).unwrap();
        advapi32::LookupPrivilegeValueA(null(), name.as_ptr(), &mut luid)
    };
    match ret {
        0 => Err("failed to get the privilege value"),
        _ => Ok(luid),
    }
}
