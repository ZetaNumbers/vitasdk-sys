/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;

pub type SceJpegEncoderContext = *mut crate::ctypes::c_void;
pub mod SceJpegEncErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_JPEGENC_ERROR_IMAGE_SIZE: Type = 2154103296;
    pub const SCE_JPEGENC_ERROR_INSUFFICIENT_BUFFER: Type = 2154103297;
    pub const SCE_JPEGENC_ERROR_INVALID_COMPRATIO: Type = 2154103298;
    pub const SCE_JPEGENC_ERROR_INVALID_PIXELFORMAT: Type = 2154103299;
    pub const SCE_JPEGENC_ERROR_INVALID_HEADER_MODE: Type = 2154103300;
    pub const SCE_JPEGENC_ERROR_INVALID_POINTER: Type = 2154103301;
    pub const SCE_JPEGENC_ERROR_NOT_PHY_CONTINUOUS_MEMORY: Type = 2154103302;
}
pub mod SceJpegEncoderPixelFormat {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_JPEGENC_PIXELFORMAT_ARGB8888: Type = 0;
    pub const SCE_JPEGENC_PIXELFORMAT_YCBCR420: Type = 8;
    pub const SCE_JPEGENC_PIXELFORMAT_YCBCR422: Type = 9;
    pub const SCE_JPEGENC_PIXELFORMAT_CSC_ARGB_YCBCR: Type = 16;
}
pub mod SceJpegEncoderHeaderMode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_JPEGENC_HEADER_MODE_JPEG: Type = 0;
    pub const SCE_JPEGENC_HEADER_MODE_MJPEG: Type = 1;
}
pub mod SceJpegEncoderInitParamOption {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_JPEGENC_INIT_PARAM_OPTION_NONE: Type = 0;
    pub const SCE_JPEGENC_INIT_PARAM_OPTION_LPDDR2_MEMORY: Type = 1;
}
#[repr(C)]
pub struct SceJpegEncoderInitParam {
    pub size: SceSize,
    pub inWidth: crate::ctypes::c_int,
    pub inHeight: crate::ctypes::c_int,
    pub pixelFormat: crate::ctypes::c_int,
    pub outBuffer: *mut crate::ctypes::c_void,
    pub outSize: SceSize,
    pub option: crate::ctypes::c_int,
}
extern "C" {
    pub fn sceJpegEncoderInit(
        context: SceJpegEncoderContext,
        inWidth: crate::ctypes::c_int,
        inHeight: crate::ctypes::c_int,
        pixelformat: SceJpegEncoderPixelFormat::Type,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegEncoderInitWithParam(
        context: SceJpegEncoderContext,
        initParam: *const SceJpegEncoderInitParam,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegEncoderEnd(context: SceJpegEncoderContext) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegEncoderEncode(
        context: SceJpegEncoderContext,
        inBuffer: *const crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegEncoderSetCompressionRatio(
        context: SceJpegEncoderContext,
        ratio: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegEncoderSetOutputAddr(
        context: SceJpegEncoderContext,
        outBuffer: *mut crate::ctypes::c_void,
        outSize: SceSize,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegEncoderCsc(
        context: SceJpegEncoderContext,
        outBuffer: *mut crate::ctypes::c_void,
        inBuffer: *const crate::ctypes::c_void,
        inPitch: crate::ctypes::c_int,
        inPixelFormat: SceJpegEncoderPixelFormat::Type,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegEncoderGetContextSize() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegEncoderSetValidRegion(
        context: SceJpegEncoderContext,
        inWidth: crate::ctypes::c_int,
        inHeight: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceJpegEncoderSetHeaderMode(
        context: SceJpegEncoderContext,
        mode: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
