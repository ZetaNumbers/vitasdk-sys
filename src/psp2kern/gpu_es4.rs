/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

extern "C" {
    pub fn PVRSRVGetMiscInfoKM(info: *mut crate::ctypes::c_void) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceGpuGetRegisterDump(
        dst: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
