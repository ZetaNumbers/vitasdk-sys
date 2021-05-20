/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;

pub type SceFontLibHandle = *mut crate::ctypes::c_void;
pub type SceFontHandle = *mut crate::ctypes::c_void;
pub mod SceFontErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_FONT_ERROR_OUT_OF_MEMORY: Type = 2152071169;
    pub const SCE_FONT_ERROR_INVALID_LIBID: Type = 2152071170;
    pub const SCE_FONT_ERROR_INVALID_PARAMETER: Type = 2152071171;
    pub const SCE_FONT_ERROR_HANDLER_OPEN_FAILED: Type = 2152071173;
    pub const SCE_FONT_ERROR_TOO_MANY_OPEN_FONTS: Type = 2152071177;
    pub const SCE_FONT_ERROR_INVALID_FONT_DATA: Type = 2152071178;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSceFontNewLibParams {
    pub userData: *mut crate::ctypes::c_void,
    pub numFonts: crate::ctypes::c_uint,
    pub cacheData: *mut crate::ctypes::c_void,
    pub allocFunc: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut crate::ctypes::c_void,
            arg2: crate::ctypes::c_uint,
        ) -> *mut crate::ctypes::c_void,
    >,
    pub freeFunc: ::core::option::Option<
        unsafe extern "C" fn(arg1: *mut crate::ctypes::c_void, arg2: *mut crate::ctypes::c_void),
    >,
    pub openFunc: *mut crate::ctypes::c_void,
    pub closeFunc: *mut crate::ctypes::c_void,
    pub readFunc: *mut crate::ctypes::c_void,
    pub seekFunc: *mut crate::ctypes::c_void,
    pub errorFunc: *mut crate::ctypes::c_void,
    pub ioFinishFunc: *mut crate::ctypes::c_void,
}
pub type SceFontNewLibParams = SceSceFontNewLibParams;
pub mod SceFontFamilyCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_FONT_FAMILY_DEFAULT: Type = 0;
    pub const SCE_FONT_FAMILY_SANS_SERIF: Type = 1;
    pub const SCE_FONT_FAMILY_SERIF: Type = 2;
    pub const SCE_FONT_FAMILY_ROUNDED: Type = 3;
}
pub mod SceFontStyleCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_FONT_STYLE_DEFAULT: Type = 0;
    pub const SCE_FONT_STYLE_REGULAR: Type = 1;
    pub const SCE_FONT_STYLE_ITALIC: Type = 2;
    pub const SCE_FONT_STYLE_NARROW: Type = 3;
    pub const SCE_FONT_STYLE_NARROW_ITALIC: Type = 4;
    pub const SCE_FONT_STYLE_BOLD: Type = 5;
    pub const SCE_FONT_STYLE_BOLD_ITALIC: Type = 6;
    pub const SCE_FONT_STYLE_BLACK: Type = 7;
    pub const SCE_FONT_STYLE_BLACK_ITALIC: Type = 8;
    pub const SCE_FONT_STYLE_L: Type = 101;
    pub const SCE_FONT_STYLE_M: Type = 102;
    pub const SCE_FONT_STYLE_DB: Type = 103;
    pub const SCE_FONT_STYLE_B: Type = 104;
    pub const SCE_FONT_STYLE_EB: Type = 105;
    pub const SCE_FONT_STYLE_UB: Type = 106;
}
pub mod SceFontLanguageCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_FONT_LANGUAGE_DEFAULT: Type = 0;
    pub const SCE_FONT_LANGUAGE_JAPANESE: Type = 1;
    pub const SCE_FONT_LANGUAGE_LATIN: Type = 2;
    pub const SCE_FONT_LANGUAGE_KOREAN: Type = 3;
    pub const SCE_FONT_LANGUAGE_CHINESE: Type = 4;
    pub const SCE_FONT_LANGUAGE_CJK: Type = 5;
}
pub mod SceFontPixelFormatCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_FONT_PIXELFORMAT_4: Type = 0;
    pub const SCE_FONT_PIXELFORMAT_4_REV: Type = 1;
    pub const SCE_FONT_PIXELFORMAT_8: Type = 2;
    pub const SCE_FONT_PIXELFORMAT_24: Type = 3;
    pub const SCE_FONT_PIXELFORMAT_32: Type = 4;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFontImageRect {
    pub width: crate::ctypes::c_ushort,
    pub height: crate::ctypes::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFontGlyphImage {
    pub pixelFormat: crate::ctypes::c_uint,
    pub xPos64: crate::ctypes::c_int,
    pub yPos64: crate::ctypes::c_int,
    pub bufWidth: crate::ctypes::c_ushort,
    pub bufHeight: crate::ctypes::c_ushort,
    pub bytesPerLine: crate::ctypes::c_ushort,
    pub pad: crate::ctypes::c_ushort,
    pub bufferPtr: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceFontStyle {
    pub fontH: f32,
    pub fontV: f32,
    pub fontHRes: f32,
    pub fontVRes: f32,
    pub fontWeight: f32,
    pub fontFamily: crate::ctypes::c_ushort,
    pub fontStyle: crate::ctypes::c_ushort,
    pub fontStyleSub: crate::ctypes::c_ushort,
    pub fontLanguage: crate::ctypes::c_ushort,
    pub fontRegion: crate::ctypes::c_ushort,
    pub fontCountry: crate::ctypes::c_ushort,
    pub fontName: [crate::ctypes::c_char; 64usize],
    pub fontFileName: [crate::ctypes::c_char; 64usize],
    pub fontAttributes: crate::ctypes::c_uint,
    pub fontExpire: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceFontCharInfo {
    pub bitmapWidth: crate::ctypes::c_uint,
    pub bitmapHeight: crate::ctypes::c_uint,
    pub bitmapLeft: crate::ctypes::c_uint,
    pub bitmapTop: crate::ctypes::c_uint,
    pub sfp26Width: crate::ctypes::c_uint,
    pub sfp26Height: crate::ctypes::c_uint,
    pub sfp26Ascender: crate::ctypes::c_int,
    pub sfp26Descender: crate::ctypes::c_int,
    pub sfp26BearingHX: crate::ctypes::c_int,
    pub sfp26BearingHY: crate::ctypes::c_int,
    pub sfp26BearingVX: crate::ctypes::c_int,
    pub sfp26BearingVY: crate::ctypes::c_int,
    pub sfp26AdvanceH: crate::ctypes::c_int,
    pub sfp26AdvanceV: crate::ctypes::c_int,
    pub shadowFlags: crate::ctypes::c_short,
    pub shadowId: crate::ctypes::c_short,
}
#[repr(C)]
pub struct SceFontInfo {
    pub maxGlyphWidthI: crate::ctypes::c_uint,
    pub maxGlyphHeightI: crate::ctypes::c_uint,
    pub maxGlyphAscenderI: crate::ctypes::c_uint,
    pub maxGlyphDescenderI: crate::ctypes::c_uint,
    pub maxGlyphLeftXI: crate::ctypes::c_uint,
    pub maxGlyphBaseYI: crate::ctypes::c_uint,
    pub minGlyphCenterXI: crate::ctypes::c_uint,
    pub maxGlyphTopYI: crate::ctypes::c_uint,
    pub maxGlyphAdvanceXI: crate::ctypes::c_uint,
    pub maxGlyphAdvanceYI: crate::ctypes::c_uint,
    pub maxGlyphWidthF: f32,
    pub maxGlyphHeightF: f32,
    pub maxGlyphAscenderF: f32,
    pub maxGlyphDescenderF: f32,
    pub maxGlyphLeftXF: f32,
    pub maxGlyphBaseYF: f32,
    pub minGlyphCenterXF: f32,
    pub maxGlyphTopYF: f32,
    pub maxGlyphAdvanceXF: f32,
    pub maxGlyphAdvanceYF: f32,
    pub maxGlyphWidth: crate::ctypes::c_short,
    pub maxGlyphHeight: crate::ctypes::c_short,
    pub charMapLength: crate::ctypes::c_uint,
    pub shadowMapLength: crate::ctypes::c_uint,
    pub fontStyle: SceFontStyle,
    pub BPP: u8,
    pub pad: [u8; 3usize],
}
extern "C" {
    pub fn sceFontNewLib(
        params: *mut SceFontNewLibParams,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> SceFontLibHandle;
}
extern "C" {
    pub fn sceFontDoneLib(libHandle: SceFontLibHandle) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontOpen(
        libHandle: SceFontLibHandle,
        index: crate::ctypes::c_int,
        mode: crate::ctypes::c_int,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> SceFontHandle;
}
extern "C" {
    pub fn sceFontOpenUserMemory(
        libHandle: SceFontLibHandle,
        pMemoryFont: *mut crate::ctypes::c_void,
        pMemoryFontSize: SceSize,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> SceFontHandle;
}
extern "C" {
    pub fn sceFontOpenUserFile(
        libHandle: SceFontLibHandle,
        file: *mut crate::ctypes::c_char,
        mode: crate::ctypes::c_int,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> SceFontHandle;
}
extern "C" {
    pub fn sceFontClose(fontHandle: SceFontHandle) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontGetNumFontList(
        libHandle: SceFontLibHandle,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontFindOptimumFont(
        libHandle: SceFontLibHandle,
        fontStyle: *mut SceFontStyle,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontFindFont(
        libHandle: SceFontLibHandle,
        fontStyle: *mut SceFontStyle,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontGetFontInfo(
        fontHandle: SceFontHandle,
        fontInfo: *mut SceFontInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontGetFontInfoByIndexNumber(
        libHandle: SceFontLibHandle,
        fontStyle: *mut SceFontStyle,
        unknown: crate::ctypes::c_int,
        fontIndex: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontSetResolution(
        libHandle: SceFontLibHandle,
        hRes: f32,
        vRes: f32,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontGetFontList(
        libHandle: SceFontLibHandle,
        fontStyle: *mut SceFontStyle,
        numFonts: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontGetCharInfo(
        fontHandle: SceFontHandle,
        charCode: crate::ctypes::c_uint,
        charInfo: *mut SceFontCharInfo,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontGetCharImageRect(
        fontHandle: SceFontHandle,
        charCode: crate::ctypes::c_uint,
        charRect: *mut SceFontImageRect,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontGetCharGlyphImage(
        fontHandle: SceFontHandle,
        charCode: crate::ctypes::c_uint,
        glyphImage: *mut SceFontGlyphImage,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontGetCharGlyphImage_Clip(
        fontHandle: SceFontHandle,
        charCode: crate::ctypes::c_uint,
        glyphImage: *mut SceFontGlyphImage,
        clipXPos: crate::ctypes::c_int,
        clipYPos: crate::ctypes::c_int,
        clipWidth: crate::ctypes::c_int,
        clipHeight: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontPixelToPointH(
        libHandle: SceFontLibHandle,
        fontPixelsH: f32,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> f32;
}
extern "C" {
    pub fn sceFontPixelToPointV(
        libHandle: SceFontLibHandle,
        fontPixelsV: f32,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> f32;
}
extern "C" {
    pub fn sceFontPointToPixelH(
        libHandle: SceFontLibHandle,
        fontPointsH: f32,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> f32;
}
extern "C" {
    pub fn sceFontPointToPixelV(
        libHandle: SceFontLibHandle,
        fontPointsV: f32,
        errorCode: *mut crate::ctypes::c_uint,
    ) -> f32;
}
extern "C" {
    pub fn sceFontSetAltCharacterCode(
        libHandle: SceFontLibHandle,
        charCode: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn sceFontFlush(fontHandle: SceFontHandle) -> crate::ctypes::c_int;
}
