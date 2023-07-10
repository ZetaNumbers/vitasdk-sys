/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

pub const SCE_KERNEL_DMAC_CMD_OP_COPY: u32 = 0;
pub const SCE_KERNEL_DMAC_CMD_OP_SET: u32 = 12;
pub const SCE_KERNEL_DMAC_CMD_OP_RNG: u32 = 4;
pub const SCE_KERNEL_DMAC_CMD_OP_HASH_SHA1: u32 = 3;
pub const SCE_KERNEL_DMAC_CMD_OP_HASH_SHA224: u32 = 11;
pub const SCE_KERNEL_DMAC_CMD_OP_HASH_SHA256: u32 = 19;
pub const SCE_KERNEL_DMAC_CMD_OP_HMAC_SHA1: u32 = 35;
pub const SCE_KERNEL_DMAC_CMD_OP_HMAC_SHA224: u32 = 43;
pub const SCE_KERNEL_DMAC_CMD_OP_HMAC_SHA256: u32 = 51;
pub const SCE_KERNEL_DMAC_CMD_OP_ENCRYPT_AES_ECB: u32 = 1;
pub const SCE_KERNEL_DMAC_CMD_OP_ENCRYPT_AES_CBC: u32 = 9;
pub const SCE_KERNEL_DMAC_CMD_OP_ENCRYPT_AES_CTR: u32 = 17;
pub const SCE_KERNEL_DMAC_CMD_OP_ENCRYPT_DES_ECB: u32 = 65;
pub const SCE_KERNEL_DMAC_CMD_OP_ENCRYPT_DES_CBC: u32 = 73;
pub const SCE_KERNEL_DMAC_CMD_OP_DECRYPT_AES_ECB: u32 = 2;
pub const SCE_KERNEL_DMAC_CMD_OP_DECRYPT_AES_CBC: u32 = 10;
pub const SCE_KERNEL_DMAC_CMD_OP_DECRYPT_AES_CTR: u32 = 18;
pub const SCE_KERNEL_DMAC_CMD_OP_DECRYPT_DES_ECB: u32 = 66;
pub const SCE_KERNEL_DMAC_CMD_OP_DECRYPT_DES_CBC: u32 = 74;
pub const SCE_KERNEL_DMAC_CMD_USE_EXTERNAL_KEY: u32 = 128;
pub const SCE_KERNEL_DMAC_CMD_KEYSIZE_64BIT: u32 = 0;
pub const SCE_KERNEL_DMAC_CMD_KEYSIZE_128BIT: u32 = 256;
pub const SCE_KERNEL_DMAC_CMD_KEYSIZE_192BIT: u32 = 512;
pub const SCE_KERNEL_DMAC_CMD_KEYSIZE_256BIT: u32 = 768;
pub const SCE_KERNEL_DMAC_CMD_HASH_UPDATE: u32 = 1024;
pub const SCE_KERNEL_DMAC_CMD_HASH_FINALIZE: u32 = 2048;
pub const SCE_KERNEL_DMAC_CMD_COHERENT_SRC: u32 = 16777216;
pub const SCE_KERNEL_DMAC_CMD_COHERENT_DST: u32 = 33554432;
pub const SCE_KERNEL_DMAC_CMD_COHERENT_IV: u32 = 201326592;
pub const SCE_KERNEL_DMAC_BLOCKSIZE_SRC_SHIFT: u32 = 0;
pub const SCE_KERNEL_DMAC_BLOCKSIZE_SRC_MASK: u32 = 65535;
pub const SCE_KERNEL_DMAC_BLOCKSIZE_DST_SHIFT: u32 = 16;
pub const SCE_KERNEL_DMAC_BLOCKSIZE_DST_MASK: u32 = 4294901760;
pub const SCE_KERNEL_DMAC_STAT_BUSY: u32 = 1;
pub const SCE_KERNEL_DMAC_STAT_ABORTED: u32 = 2;
pub const SCE_KERNEL_DMAC_STAT_ERROR_READ: u32 = 65536;
pub const SCE_KERNEL_DMAC_STAT_ERROR_WRITE: u32 = 131072;
pub const SCE_KERNEL_DMAC_STAT_ERROR_ILLEGAL_CONFIG: u32 = 262144;
pub const SCE_KERNEL_DMAC_STAT_ERROR_TAG: u32 = 524288;
pub const SCE_KERNEL_DMAC_STAT_ERROR_ZERO_BYTE: u32 = 1048576;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_SRC_SHIFT: u32 = 0;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_SRC_MASK: u32 = 511;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_DST_SHIFT: u32 = 9;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_DST_MASK: u32 = 261120;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_SRC_DST_MASK: u32 = 262143;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_UNK_SHIFT: u32 = 18;
pub const SCE_KERNEL_DMAC_COHERENCY_MSK_UNK_MASK: u32 = 133955584;
extern "C" {
    pub fn ksceDmacMemcpy(
        dst: *mut crate::ctypes::c_void,
        src: *const crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceDmacMemset(
        dst: *mut crate::ctypes::c_void,
        c: crate::ctypes::c_int,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
pub type SceKernelDmaOpId = SceInt32;
pub mod SceKernelDmacId {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_DMAC_ID_DMAC01: Type = 16;
    pub const SCE_KERNEL_DMAC_ID_DMAC23: Type = 17;
    pub const SCE_KERNEL_DMAC_ID_DMAC4: Type = 18;
    pub const SCE_KERNEL_DMAC_ID_DMAC5: Type = 19;
    pub const SCE_KERNEL_DMAC_ID_DMAC6: Type = 20;
}
pub mod SceKernelDmaOpFlag {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_DMA_OP_PHYSICAL_ADDR: Type = 0;
    pub const SCE_KERNEL_DMA_OP_VIRTUAL_SRC_ADDR: Type = 1;
    pub const SCE_KERNEL_DMA_OP_VIRTUAL_DST_ADDR: Type = 16;
    pub const SCE_KERNEL_DMA_OP_VIRTUAL_ADDR: Type = 17;
    pub const SCE_KERNEL_DMA_OP_COMPLETE_CHAIN: Type = 256;
}
pub mod SceKernelDmaOpSyncMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_KERNEL_DMA_OP_SYNC_POLL: Type = 1;
    pub const SCE_KERNEL_DMA_OP_SYNC_WAIT: Type = 2;
    pub const SCE_KERNEL_DMA_OP_SYNC_TIMED_WAIT: Type = 3;
}
#[repr(C)]
pub struct SceKernelDmaOpTag {
    pub src: *const crate::ctypes::c_void,
    pub dst: *mut crate::ctypes::c_void,
    pub len: SceUInt32,
    pub cmd: SceUInt32,
    pub keyring: SceUInt32,
    pub iv: *mut crate::ctypes::c_void,
    pub blockSize: SceUInt32,
    pub pNext: *mut SceKernelDmaOpTag,
}
#[repr(C)]
pub struct SceKernelDmaOpEncDec {
    pub keyring: SceUInt32,
    pub iv: *mut crate::ctypes::c_void,
    pub unk_0x8: SceUInt32,
    pub reserved: SceUInt32,
    pub key: [SceUInt8; 64usize],
}
#[repr(C)]
pub struct SceKernelDmaOpChainParam {
    pub size: SceSize,
    pub coherencyMask: SceUInt32,
    pub setValue: SceUInt32,
}
#[repr(C)]
pub struct SceKernelDmaOpEncDecChainParam {
    pub header: SceKernelDmaOpChainParam,
    pub encDec: SceKernelDmaOpEncDec,
}
#[repr(C)]
pub struct SceKernelDmaOpDirectParam {
    pub src: *const crate::ctypes::c_void,
    pub dst: *mut crate::ctypes::c_void,
    pub len: SceUInt32,
    pub cmd: SceUInt32,
    pub blockSize: SceUInt32,
    pub coherencyMask: SceUInt32,
    pub setValue: SceUInt32,
    pub encDec: SceKernelDmaOpEncDec,
}
pub type SceKernelDmaOpCallback = ::core::option::Option<
    unsafe extern "C" fn(
        opid: SceKernelDmaOpId,
        stat: SceUInt32,
        pUserData: *mut crate::ctypes::c_void,
        pTag: *mut SceKernelDmaOpTag,
    ),
>;
extern "C" {
    pub fn ksceKernelDmaOpAlloc(name: *const crate::ctypes::c_char) -> SceKernelDmaOpId;
}
extern "C" {
    pub fn ksceKernelDmaOpFree(opid: SceKernelDmaOpId) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDmaOpEnQueue(opid: SceKernelDmaOpId) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDmaOpDeQueue(opid: SceKernelDmaOpId) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDmaOpQuit(opid: SceKernelDmaOpId) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDmaOpSync(
        opid: SceKernelDmaOpId,
        syncMode: SceKernelDmaOpSyncMode::Type,
        pTimeout: *mut SceUInt32,
        ppErrorTag: *mut *mut SceKernelDmaOpTag,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDmaOpAssign(
        opid: SceKernelDmaOpId,
        dmac: SceKernelDmacId::Type,
        channel: SceUInt32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDmaOpSetupDirect(
        opid: SceKernelDmaOpId,
        pParam: *mut SceKernelDmaOpDirectParam,
        flag: SceKernelDmaOpFlag::Type,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDmaOpSetupChain(
        opid: SceKernelDmaOpId,
        pTag: *mut SceKernelDmaOpTag,
        pParam: *mut SceKernelDmaOpChainParam,
        flag: SceKernelDmaOpFlag::Type,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDmaOpConcatenate(
        opid: SceKernelDmaOpId,
        pTag: *mut SceKernelDmaOpTag,
        flag: SceKernelDmaOpFlag::Type,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceKernelDmaOpSetCallback(
        opid: SceKernelDmaOpId,
        callback: SceKernelDmaOpCallback,
        pUserData: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
