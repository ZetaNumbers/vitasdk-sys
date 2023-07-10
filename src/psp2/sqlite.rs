/* automatically generated by rust-bindgen 0.65.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceSqliteMallocMethods {
    pub xMalloc: ::core::option::Option<
        unsafe extern "C" fn(arg1: crate::ctypes::c_int) -> *mut crate::ctypes::c_void,
    >,
    pub xRealloc: ::core::option::Option<
        unsafe extern "C" fn(
            arg1: *mut crate::ctypes::c_void,
            arg2: crate::ctypes::c_int,
        ) -> *mut crate::ctypes::c_void,
    >,
    pub xFree: ::core::option::Option<unsafe extern "C" fn(arg1: *mut crate::ctypes::c_void)>,
}
extern "C" {
    pub fn sceSqliteConfigMallocMethods(
        methods: *mut SceSqliteMallocMethods,
    ) -> crate::ctypes::c_int;
}
