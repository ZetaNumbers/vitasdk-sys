/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::kernel::threadmgr::*;
#[allow(unused_imports)]
use crate::psp2::types::*;

pub mod SceUsbdErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_USBD_ERROR_IO: Type = 2149842945;
    pub const SCE_USBD_ERROR_INVALID_ARG: Type = 2149842946;
    pub const SCE_USBD_ERROR_ACCESS: Type = 2149842947;
    pub const SCE_USBD_ERROR_NO_DEVICE: Type = 2149842948;
    pub const SCE_USBD_ERROR_NOT_FOUND: Type = 2149842949;
    pub const SCE_USBD_ERROR_BUSY: Type = 2149842950;
    pub const SCE_USBD_ERROR_TIMEOUT: Type = 2149842951;
    pub const SCE_USBD_ERROR_OVERFLOW: Type = 2149842952;
    pub const SCE_USBD_ERROR_PIPE: Type = 2149842953;
    pub const SCE_USBD_ERROR_INTERRUPTED: Type = 2149842954;
    pub const SCE_USBD_ERROR_NO_MEM: Type = 2149842955;
    pub const SCE_USBD_ERROR_NOT_SUPPORTED: Type = 2149842956;
    pub const SCE_USBD_ERROR_FATAL: Type = 2149843199;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDeviceInfo {
    pub unk0: crate::ctypes::c_uint,
    pub device_id: crate::ctypes::c_uint,
    pub unk2: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdDeviceAddress {
    pub unk0: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdTransferData {
    pub pipe: crate::ctypes::c_uint,
    pub buff1: *const crate::ctypes::c_void,
    pub size1: crate::ctypes::c_uint,
    pub buff2: *mut crate::ctypes::c_void,
    pub size2: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUsbdReceiveEvent {
    pub unk0: crate::ctypes::c_uint,
    pub unk1: crate::ctypes::c_uint,
    pub unk2: crate::ctypes::c_uint,
    pub unk3: crate::ctypes::c_uint,
    pub unk4: crate::ctypes::c_uint,
    pub unk5: crate::ctypes::c_uint,
    pub transfer_id: crate::ctypes::c_uint,
}
extern "C" {
    pub fn sceUsbdInit(uid: *mut SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdEnd(uid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdRegisterCallback(
        cbid: SceUID,
        arg1: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdUnregisterCallback(cbid: SceUID) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdResetDevice(
        uid: SceUID,
        device_id: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdAttach(
        uid: SceUID,
        arg1: crate::ctypes::c_int,
        arg2: crate::ctypes::c_int,
        arg3: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetDeviceList(
        uid: SceUID,
        num: crate::ctypes::c_uint,
        info: *mut SceUsbdDeviceInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetDescriptor(
        uid: SceUID,
        device_id: crate::ctypes::c_uint,
        descriptor: *mut crate::ctypes::c_uchar,
        size: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetDescriptorSize(
        uid: SceUID,
        device_id: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetDeviceAddress(
        uid: SceUID,
        device_id: crate::ctypes::c_uint,
        addr: *mut SceUsbdDeviceAddress,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetDeviceSpeed(
        uid: SceUID,
        device_id: crate::ctypes::c_uint,
        speed: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetTransferStatus(
        uid: SceUID,
        buff: *mut crate::ctypes::c_uchar,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdGetIsochTransferStatus(
        uid: SceUID,
        buff: *mut crate::ctypes::c_uchar,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdOpenDefaultPipe(
        uid: SceUID,
        device_id: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdOpenPipe(uid: SceUID, unk: *mut crate::ctypes::c_uchar) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdClosePipe(uid: SceUID, device_id: crate::ctypes::c_uint) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdTransferData(uid: SceUID, data: *mut SceUsbdTransferData)
        -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdIsochTransferData(
        uid: SceUID,
        unk: crate::ctypes::c_int,
        buff: *mut crate::ctypes::c_uchar,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdReceiveEvent(
        uid: SceUID,
        event: *mut SceUsbdReceiveEvent,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdRegisterLdd(
        uid: SceUID,
        str_: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdUnregisterLdd(
        uid: SceUID,
        str_: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdRegisterCompositeLdd(
        uid: SceUID,
        str_: *const crate::ctypes::c_char,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceUsbdAttachCompositeLdd(
        arg1: SceUID,
        unk: *mut crate::ctypes::c_uchar,
    ) -> crate::ctypes::c_int;
}
