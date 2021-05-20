/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;

pub const SCE_KERNEL_PROCESS_ID_SELF: u32 = 0;
pub const SCE_KERNEL_START_SUCCESS: u32 = 0;
pub const SCE_KERNEL_START_RESIDENT: u32 = 0;
pub const SCE_KERNEL_START_NO_RESIDENT: u32 = 1;
pub const SCE_KERNEL_START_FAILED: u32 = 2;
pub const SCE_KERNEL_STOP_SUCCESS: u32 = 0;
pub const SCE_KERNEL_STOP_FAIL: u32 = 1;
pub const SCE_KERNEL_STOP_CANCEL: u32 = 1;
pub mod SceKernelModuleState {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_MODULE_STATE_READY: Type = 2;
    pub const SCE_KERNEL_MODULE_STATE_STARTED: Type = 6;
    pub const SCE_KERNEL_MODULE_STATE_ENDED: Type = 9;
}
pub mod SceKernelPreloadInhibit {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_PRELOAD_INHIBIT_NONE: Type = 0;
    pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBC: Type = 65536;
    pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBDBG: Type = 131072;
    pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBSHELLSVC: Type = 524288;
    pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBCDLG: Type = 1048576;
    pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBFIOS2: Type = 2097152;
    pub const SCE_KERNEL_PRELOAD_INHIBIT_APPUTIL: Type = 4194304;
    pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBSCEFT2: Type = 8388608;
    pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBPVF: Type = 16777216;
    pub const SCE_KERNEL_PRELOAD_INHIBIT_LIBPERF: Type = 33554432;
}
#[repr(C)]
pub struct SceKernelSegmentInfo {
    pub size: SceSize,
    pub perms: SceUInt,
    pub vaddr: *mut crate::ctypes::c_void,
    pub memsz: SceSize,
    pub filesz: SceSize,
    pub res: SceUInt,
}
#[repr(C)]
pub struct SceKernelModuleInfo {
    pub size: SceSize,
    pub modid: SceUID,
    pub modattr: u16,
    pub modver: [u8; 2usize],
    pub module_name: [crate::ctypes::c_char; 28usize],
    pub unk28: SceUInt,
    pub start_entry: *mut crate::ctypes::c_void,
    pub stop_entry: *mut crate::ctypes::c_void,
    pub exit_entry: *mut crate::ctypes::c_void,
    pub exidx_top: *mut crate::ctypes::c_void,
    pub exidx_btm: *mut crate::ctypes::c_void,
    pub extab_top: *mut crate::ctypes::c_void,
    pub extab_btm: *mut crate::ctypes::c_void,
    pub tlsInit: *mut crate::ctypes::c_void,
    pub tlsInitSize: SceSize,
    pub tlsAreaSize: SceSize,
    pub path: [crate::ctypes::c_char; 256usize],
    pub segments: [SceKernelSegmentInfo; 4usize],
    pub state: SceUInt,
}
#[repr(C)]
pub struct SceKernelLMOption {
    pub size: SceSize,
}
#[repr(C)]
pub struct SceKernelULMOption {
    pub size: SceSize,
}
extern "C" {
    pub fn sceKernelGetModuleList(
        flags: crate::ctypes::c_int,
        modids: *mut SceUID,
        num: *mut SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelGetModuleInfo(
        modid: SceUID,
        info: *mut SceKernelModuleInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelLoadModule(
        path: *const crate::ctypes::c_char,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelLMOption,
    ) -> SceUID;
}
extern "C" {
    pub fn sceKernelUnloadModule(
        modid: SceUID,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelULMOption,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelStartModule(
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut crate::ctypes::c_void,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelStopModule(
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut crate::ctypes::c_void,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceKernelLoadStartModule(
        path: *const crate::ctypes::c_char,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelLMOption,
        status: *mut crate::ctypes::c_int,
    ) -> SceUID;
}
extern "C" {
    pub fn sceKernelStopUnloadModule(
        modid: SceUID,
        args: SceSize,
        argp: *mut crate::ctypes::c_void,
        flags: crate::ctypes::c_int,
        option: *mut SceKernelULMOption,
        status: *mut crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
#[repr(C)]
pub struct SceKernelFwInfo {
    pub size: SceSize,
    pub versionString: [crate::ctypes::c_char; 28usize],
    pub version: SceUInt,
    pub unk_24: SceUInt,
}
extern "C" {
    pub fn sceKernelGetSystemSwVersion(data: *mut SceKernelFwInfo) -> crate::ctypes::c_int;
}
