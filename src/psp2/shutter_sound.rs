/* automatically generated by rust-bindgen 0.58.1 */

pub mod SceShutterSoundErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SHUTTER_SOUND_ERROR_INVALID_ARGUMENT: Type = 2148553217;
    pub const SCE_SHUTTER_SOUND_ERROR_INTERNAL: Type = 2148553218;
    pub const SCE_SHUTTER_SOUND_ERROR_FATAL: Type = 2148553219;
}
pub mod SceShutterSoundType {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_SHUTTER_SOUND_TYPE_SAVE_IMAGE: Type = 0;
    pub const SCE_SHUTTER_SOUND_TYPE_SAVE_VIDEO_START: Type = 1;
    pub const SCE_SHUTTER_SOUND_TYPE_SAVE_VIDEO_END: Type = 2;
}
extern "C" {
    pub fn sceShutterSoundPlay(type_: u32) -> crate::ctypes::c_int;
}
