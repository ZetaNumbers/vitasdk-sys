/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2common::types::*;

pub const SCE_HID_MAX_REPORT: u32 = 16;
pub const SCE_HID_MAX_DEVICE_COUNT: u32 = 8;
#[repr(C)]
pub struct SceHidKeyboardReport {
    pub reserved: SceUInt8,
    pub modifiers: [SceUInt8; 2usize],
    pub keycodes: [SceUInt8; 6usize],
    pub unk1: [SceUInt8; 15usize],
}
#[repr(C)]
pub struct SceHidMouseReport {
    pub buttons: SceUInt8,
    pub reserved: SceUInt8,
    pub rel_x: SceInt16,
    pub rel_y: SceInt16,
    pub unk: [SceInt8; 10usize],
}
extern "C" {
    pub fn sceHidKeyboardEnumerate(
        handle: *mut crate::ctypes::c_int,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceHidKeyboardRead(
        handle: SceUInt32,
        reports: *mut *mut SceHidKeyboardReport,
        nReports: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceHidMouseEnumerate(
        handle: *mut crate::ctypes::c_int,
        count: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceHidMouseRead(
        handle: SceUInt32,
        reports: *mut *mut SceHidMouseReport,
        nReports: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
