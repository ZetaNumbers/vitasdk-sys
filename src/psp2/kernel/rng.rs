/* automatically generated by rust-bindgen 0.58.1 */

#[allow(unused_imports)]
use crate::psp2::types::*;

pub const SCE_KERNEL_PROCESS_ID_SELF: u32 = 0;
extern "C" {
    pub fn sceKernelGetRandomNumber(
        output: *mut crate::ctypes::c_void,
        size: SceSize,
    ) -> crate::ctypes::c_int;
}
