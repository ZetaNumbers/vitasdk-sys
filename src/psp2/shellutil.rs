/* automatically generated by rust-bindgen 0.58.1 */

pub mod SceShellUtilLockType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_PS_BTN: Type = 1;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_QUICK_MENU: Type = 2;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_POWEROFF_MENU: Type = 4;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_UNK8: Type = 8;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_USB_CONNECTION: Type = 16;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_MC_INSERTED: Type = 32;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_MC_REMOVED: Type = 64;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_UNK80: Type = 128;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_UNK100: Type = 256;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_UNK200: Type = 512;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_MUSIC_PLAYER: Type = 1024;
    pub const SCE_SHELL_UTIL_LOCK_TYPE_PS_BTN_2: Type = 2048;
}
pub mod SceShellUtilLockMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SHELL_UTIL_LOCK_MODE_LOCK: Type = 1;
    pub const SCE_SHELL_UTIL_LOCK_MODE_UNLOCK: Type = 2;
}
pub type SceShellUtilEventHandler = ::core::option::Option<
    unsafe extern "C" fn(
        result: crate::ctypes::c_int,
        mode: SceShellUtilLockMode::Type,
        type_: SceShellUtilLockType::Type,
        userData: *mut crate::ctypes::c_void,
    ),
>;
extern "C" {
    pub fn sceShellUtilInitEvents(unk: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceShellUtilRegisterEventHandler(
        handler: SceShellUtilEventHandler,
        userData: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceShellUtilLock(type_: SceShellUtilLockType::Type) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceShellUtilUnlock(type_: SceShellUtilLockType::Type) -> crate::ctypes::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceShellUtilLaunchAppParam {
    pub cmd: *const crate::ctypes::c_char,
}
extern "C" {
    pub fn sceShellUtilRequestLaunchApp(
        param: *mut SceShellUtilLaunchAppParam,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceShellUtilLaunchAppRequestLaunchApp(
        param: *mut SceShellUtilLaunchAppParam,
    ) -> crate::ctypes::c_int;
}
