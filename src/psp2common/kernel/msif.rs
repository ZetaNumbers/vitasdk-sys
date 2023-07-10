/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;

#[repr(C)]
pub struct SceMsInfo {
    pub unk_0x00: crate::ctypes::c_int,
    pub unk_0x04: crate::ctypes::c_int,
    pub nbytes: SceUInt64,
    pub nbytes2: SceUInt64,
    pub sector_size: SceUInt32,
    pub unk_0x1C: crate::ctypes::c_int,
    pub fs_offset: SceUInt32,
    pub unk_0x24: SceUInt32,
    pub unk_0x28: SceUInt32,
    pub unk_0x2C: SceUInt32,
    pub id: [SceUInt8; 16usize],
}
