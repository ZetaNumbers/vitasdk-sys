/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

#[repr(C)]
pub struct SceRtcTick {
    pub tick: SceUInt64,
}
extern "C" {
    pub fn ksceRtcSetCurrentTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceRtcGetCurrentTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceRtcSetCurrentNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceRtcGetCurrentNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceRtcSetCurrentSecureTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceRtcGetCurrentSecureTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceRtcSetCurrentDebugNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceRtcGetCurrentDebugNetworkTick(tick: *mut SceRtcTick) -> crate::ctypes::c_int;
}
