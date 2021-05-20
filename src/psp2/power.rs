/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;

pub mod ScePowerErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_POWER_ERROR_INVALID_VALUE: Type = 2150301696;
    pub const SCE_POWER_ERROR_ALREADY_REGISTERED: Type = 2150301697;
    pub const SCE_POWER_ERROR_CALLBACK_NOT_REGISTERED: Type = 2150301698;
    pub const SCE_POWER_ERROR_CANT_SUSPEND: Type = 2150301699;
    pub const SCE_POWER_ERROR_NO_BATTERY: Type = 2150301952;
    pub const SCE_POWER_ERROR_DETECTING: Type = 2150301953;
}
pub mod ScePowerCallbackType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_POWER_CB_BATTERY_MODE: Type = 0;
    pub const SCE_POWER_CB_LOW_BATTERY: Type = 256;
    pub const SCE_POWER_CB_AC_POWER_MODE: Type = 4096;
    pub const SCE_POWER_CB_SHUTDOWN: Type = 65536;
    pub const SCE_POWER_CB_RESUME_LIVEAREA: Type = 2097152;
    pub const SCE_POWER_CB_SUSPENDING: Type = 4194304;
    pub const SCE_POWER_CB_RESUMING: Type = 8388608;
    pub const SCE_POWER_CB_SCREENSHOT_TRIGGER: Type = 67108864;
    pub const SCE_POWER_CB_QUICK_MENU_TRIGGER: Type = 268435456;
    pub const SCE_POWER_CB_PS_BUTTON_PRESS: Type = 536870912;
    pub const SCE_POWER_CB_SHUTDOWN_MENU_TRIGGER: Type = 1073741824;
    pub const SCE_POWER_CB_UNLOCK_MENU_TRIGGER: Type = 2147483648;
}
pub mod ScePowerConfigurationMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_POWER_CONFIGURATION_MODE_A: Type = 128;
    pub const SCE_POWER_CONFIGURATION_MODE_B: Type = 2048;
    pub const SCE_POWER_CONFIGURATION_MODE_C: Type = 67712;
}
pub type ScePowerCallback = ::core::option::Option<
    unsafe extern "C" fn(
        notifyId: crate::ctypes::c_int,
        notifyCount: crate::ctypes::c_int,
        powerInfo: crate::ctypes::c_int,
        userData: *mut crate::ctypes::c_void,
    ),
>;
extern "C" {
    pub fn scePowerRegisterCallback(cbid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerUnregisterCallback(cbid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerIsBatteryCharging() -> SceBool;
}
extern "C" {
    pub fn scePowerGetBatteryLifePercent() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerSetConfigurationMode(conf: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerIsSuspendRequired() -> SceBool;
}
extern "C" {
    pub fn scePowerIsPowerOnline() -> SceBool;
}
extern "C" {
    pub fn scePowerGetBatteryLifeTime() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerGetBatteryRemainCapacity() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerIsLowBattery() -> SceBool;
}
extern "C" {
    pub fn scePowerGetBatteryFullCapacity() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerGetBatteryTemp() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerGetBatteryVolt() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerGetBatterySOH() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerGetBatteryCycleCount() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerGetArmClockFrequency() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerGetBusClockFrequency() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerGetGpuClockFrequency() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerGetGpuXbarClockFrequency() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerRequestColdReset() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerRequestStandby() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerRequestSuspend() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerRequestDisplayOn() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerRequestDisplayOff() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerSetArmClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerSetBusClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerSetGpuClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerSetGpuXbarClockFrequency(freq: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerSetUsingWireless(enabled: SceBool) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn scePowerGetUsingWireless() -> crate::ctypes::c_int;
}
