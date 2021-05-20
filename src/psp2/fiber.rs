/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::vitasdk::align::*;

pub mod SceFiberErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_FIBER_ERROR_NULL: Type = 2153316353;
    pub const SCE_FIBER_ERROR_ALIGNMENT: Type = 2153316354;
    pub const SCE_FIBER_ERROR_RANGE: Type = 2153316355;
    pub const SCE_FIBER_ERROR_INVALID: Type = 2153316356;
    pub const SCE_FIBER_ERROR_PERMISSION: Type = 2153316357;
    pub const SCE_FIBER_ERROR_STATE: Type = 2153316358;
    pub const SCE_FIBER_ERROR_BUSY: Type = 2153316359;
    pub const SCE_FIBER_ERROR_AGAIN: Type = 2153316360;
    pub const SCE_FIBER_ERROR_FATAL: Type = 2153316361;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub struct SceFiber {
    pub reserved: [crate::ctypes::c_char; 128usize],
}
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone)]
pub struct SceFiberOptParam {
    pub reserved: [crate::ctypes::c_char; 128usize],
}
pub type SceFiberEntry =
    ::core::option::Option<unsafe extern "C" fn(argOnInitialize: SceUInt32, argOnRun: SceUInt32)>;
#[repr(C)]
#[repr(align(8))]
pub struct SceFiberInfo {
    pub entry: SceFiberEntry,
    pub argOnInitialize: SceUInt32,
    pub addrContext: *mut crate::ctypes::c_void,
    pub sizeContext: SceSize,
    pub name: [crate::ctypes::c_char; 32usize],
    pub padding: [crate::ctypes::c_uint; 80usize],
}
extern "C" {
    pub fn _sceFiberInitializeImpl(
        fiber: *mut SceFiber,
        name: *mut crate::ctypes::c_char,
        entry: SceFiberEntry,
        argOnInitialize: SceUInt32,
        addrContext: *mut crate::ctypes::c_void,
        sizeContext: SceSize,
        params: *mut SceFiberOptParam,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceFiberOptParamInitialize(optParam: *mut SceFiberOptParam) -> SceInt32;
}
extern "C" {
    pub fn sceFiberFinalize(fiber: *mut SceFiber) -> SceInt32;
}
extern "C" {
    pub fn sceFiberRun(
        fiber: *mut SceFiber,
        argOnRunTo: SceUInt32,
        argOnRun: *mut SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceFiberSwitch(
        fiber: *mut SceFiber,
        argOnRunTo: SceUInt32,
        argOnRun: *mut SceUInt32,
    ) -> SceInt32;
}
extern "C" {
    pub fn sceFiberGetSelf(fiber: *mut SceFiber) -> SceInt32;
}
extern "C" {
    pub fn sceFiberReturnToThread(argOnReturn: SceUInt32, argOnRun: *mut SceUInt32) -> SceInt32;
}
extern "C" {
    pub fn sceFiberGetInfo(fiber: *mut SceFiber, fiberInfo: *mut SceFiberInfo) -> SceInt32;
}
