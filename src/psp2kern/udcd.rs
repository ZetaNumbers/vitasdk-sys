/* automatically generated by rust-bindgen 0.65.1 */

#[allow(unused_imports)]
use crate::psp2common::types::*;
#[allow(unused_imports)]
use crate::psp2kern::types::*;

pub const USB_DT_DEVICE_SIZE: u32 = 18;
pub const USB_DT_CONFIG_SIZE: u32 = 9;
pub const USB_DT_INTERFACE_SIZE: u32 = 9;
pub const USB_DT_ENDPOINT_SIZE: u32 = 7;
pub const USB_DT_ENDPOINT_AUDIO_SIZE: u32 = 9;
pub const USB_DT_HUB_NONVAR_SIZE: u32 = 7;
pub const USB_CTRLTYPE_DIR_MASK: u32 = 128;
pub const USB_CTRLTYPE_DIR_HOST2DEVICE: u32 = 0;
pub const USB_CTRLTYPE_DIR_DEVICE2HOST: u32 = 128;
pub const USB_CTRLTYPE_TYPE_MASK: u32 = 96;
pub const USB_CTRLTYPE_TYPE_STANDARD: u32 = 0;
pub const USB_CTRLTYPE_TYPE_CLASS: u32 = 32;
pub const USB_CTRLTYPE_TYPE_VENDOR: u32 = 64;
pub const USB_CTRLTYPE_TYPE_RESERVED: u32 = 96;
pub const USB_CTRLTYPE_REC_MASK: u32 = 31;
pub const USB_CTRLTYPE_REC_DEVICE: u32 = 0;
pub const USB_CTRLTYPE_REC_INTERFACE: u32 = 1;
pub const USB_CTRLTYPE_REC_ENDPOINT: u32 = 2;
pub const USB_CTRLTYPE_REC_OTHER: u32 = 3;
pub const USB_ENDPOINT_ADDRESS_MASK: u32 = 15;
pub const USB_ENDPOINT_DIR_MASK: u32 = 128;
pub const USB_FEATURE_ENDPOINT_HALT: u32 = 0;
pub const USB_ENDPOINT_IN: u32 = 128;
pub const USB_ENDPOINT_OUT: u32 = 0;
pub const USB_ENDPOINT_TYPE_MASK: u32 = 3;
pub const USB_ENDPOINT_TYPE_CONTROL: u32 = 0;
pub const USB_ENDPOINT_TYPE_ISOCHRONOUS: u32 = 1;
pub const USB_ENDPOINT_TYPE_BULK: u32 = 2;
pub const USB_ENDPOINT_TYPE_INTERRUPT: u32 = 3;
pub const HID_INTF: u32 = 3;
pub const BOOT_INTF_SUBCLASS: u32 = 1;
pub const SCE_UDCD_MAX_INTERFACES: u32 = 8;
pub const SCE_UDCD_MAX_ENDPOINTS: u32 = 9;
pub const SCE_UDCD_MAX_ALTERNATE: u32 = 2;
pub mod SceUdcdUsbClass {
    pub type Type = crate::ctypes::c_uint;
    pub const USB_CLASS_PER_INTERFACE: Type = 0;
    pub const USB_CLASS_AUDIO: Type = 1;
    pub const USB_CLASS_COMM: Type = 2;
    pub const USB_CLASS_HID: Type = 3;
    pub const USB_CLASS_PTP: Type = 6;
    pub const USB_CLASS_PRINTER: Type = 7;
    pub const USB_CLASS_MASS_STORAGE: Type = 8;
    pub const USB_CLASS_HUB: Type = 9;
    pub const USB_CLASS_DATA: Type = 10;
    pub const USB_CLASS_VIDEO: Type = 14;
    pub const USB_CLASS_VENDOR_SPEC: Type = 255;
}
pub mod SceUdcdUsbDt {
    pub type Type = crate::ctypes::c_uint;
    pub const USB_DT_DEVICE: Type = 1;
    pub const USB_DT_CONFIG: Type = 2;
    pub const USB_DT_STRING: Type = 3;
    pub const USB_DT_INTERFACE: Type = 4;
    pub const USB_DT_ENDPOINT: Type = 5;
}
pub mod SceUdcdUsbReq {
    pub type Type = crate::ctypes::c_uint;
    pub const USB_REQ_GET_STATUS: Type = 0;
    pub const USB_REQ_CLEAR_FEATURE: Type = 1;
    pub const USB_REQ_SET_FEATURE: Type = 3;
    pub const USB_REQ_SET_ADDRESS: Type = 5;
    pub const USB_REQ_GET_DESCRIPTOR: Type = 6;
    pub const USB_REQ_SET_DESCRIPTOR: Type = 7;
    pub const USB_REQ_GET_CONFIG: Type = 8;
    pub const USB_REQ_SET_CONFIG: Type = 9;
    pub const USB_REQ_GET_INTERFACE: Type = 10;
    pub const USB_REQ_SET_INTERFACE: Type = 11;
    pub const USB_REQ_SYNC_FRAME: Type = 12;
}
pub mod SceUdcdHidRequest {
    pub type Type = crate::ctypes::c_uint;
    pub const HID_REQUEST_GET_REPORT: Type = 1;
    pub const HID_REQUEST_GET_IDLE: Type = 2;
    pub const HID_REQUEST_GET_PROTOCOL: Type = 3;
    pub const HID_REQUEST_SET_REPORT: Type = 9;
    pub const HID_REQUEST_SET_IDLE: Type = 10;
    pub const HID_REQUEST_SET_PROTOCOL: Type = 11;
}
pub mod SceUdcdHidDescriptor {
    pub type Type = crate::ctypes::c_uint;
    pub const HID_DESCRIPTOR_HID: Type = 33;
    pub const HID_DESCRIPTOR_REPORT: Type = 34;
    pub const HID_DESRIPTOR_PHY: Type = 35;
}
pub mod SceUdcdProtocol {
    pub type Type = crate::ctypes::c_uint;
    pub const BOOT_PROTOCOL: Type = 0;
    pub const RPT_PROTOCOL: Type = 1;
}
pub mod SceUdcdHidProtocol {
    pub type Type = crate::ctypes::c_uint;
    pub const HID_PROTOCOL_NONE: Type = 0;
    pub const HID_PROTOCOL_KEYBOARD: Type = 1;
    pub const HID_PROTOCOL_MOUSE: Type = 2;
}
pub mod SceUdcdStatus {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_UDCD_STATUS_CONNECTION_NEW: Type = 1;
    pub const SCE_UDCD_STATUS_CONNECTION_ESTABLISHED: Type = 2;
    pub const SCE_UDCD_STATUS_CONNECTION_SUSPENDED: Type = 4;
    pub const SCE_UDCD_STATUS_CABLE_DISCONNECTED: Type = 16;
    pub const SCE_UDCD_STATUS_CABLE_CONNECTED: Type = 32;
    pub const SCE_UDCD_STATUS_DEACTIVATED: Type = 256;
    pub const SCE_UDCD_STATUS_ACTIVATED: Type = 512;
    pub const SCE_UDCD_STATUS_IS_CHARGING: Type = 1024;
    pub const SCE_UDCD_STATUS_USE_USB_CHARGING: Type = 2048;
    pub const SCE_UDCD_STATUS_UNKNOWN_1000: Type = 4096;
    pub const SCE_UDCD_STATUS_UNKNOWN_2000: Type = 8192;
}
pub mod SceUdcdStatusDriver {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_UDCD_STATUS_DRIVER_STARTED: Type = 1;
    pub const SCE_UDCD_STATUS_DRIVER_REGISTERED: Type = 2;
}
pub mod SceUdcdRetcode {
    pub type Type = crate::ctypes::c_int;
    pub const SCE_UDCD_RETCODE_CANCEL: Type = -1;
    pub const SCE_UDCD_RETCODE_CANCEL_ALL: Type = -2;
    pub const SCE_UDCD_RETCODE_CANCEL_TRANSMISSION: Type = -3;
    pub const SCE_UDCD_RETCODE_SUCCESS: Type = 0;
    pub const SCE_UDCD_RETCODE_SEND: Type = 1;
    pub const SCE_UDCD_RETCODE_RECV: Type = 2;
}
pub mod SceUdcdErrorCode {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_UDCD_ERROR_ILLEGAL_CONTEXT: Type = 2147483696;
    pub const SCE_UDCD_ERROR_INVALID_POINTER: Type = 2147483907;
    pub const SCE_UDCD_ERROR_INVALID_FLAG: Type = 2147483909;
    pub const SCE_UDCD_ERROR_INVALID_VALUE: Type = 2147484158;
    pub const SCE_UDCD_ERROR_ALREADY_DONE: Type = 2149855233;
    pub const SCE_UDCD_ERROR_INVALID_ARGUMENT: Type = 2149855234;
    pub const SCE_UDCD_ERROR_ARGUMENT_EXCEEDED_LIMIT: Type = 2149855235;
    pub const SCE_UDCD_ERROR_MEMORY_EXHAUSTED: Type = 2149855236;
    pub const SCE_UDCD_ERROR_DRIVER_NOT_FOUND: Type = 2149855237;
    pub const SCE_UDCD_ERROR_DRIVER_IN_PROGRESS: Type = 2149855238;
    pub const SCE_UDCD_ERROR_BUS_DRIVER_NOT_STARTED: Type = 2149855239;
    pub const SCE_UDCD_ERROR_WAIT_TIMEOUT: Type = 2149855240;
    pub const SCE_UDCD_ERROR_WAIT_CANCEL: Type = 2149855241;
    pub const SCE_UDCD_ERROR_USBDRIVER_INVALID_DRIVER: Type = 2149855744;
    pub const SCE_UDCD_ERROR_USBDRIVER_INVALID_NAME: Type = 2149855745;
    pub const SCE_UDCD_ERROR_USBDRIVER_INVALID_FUNCS: Type = 2149855746;
}
pub mod SceUdcdDeviceRequestAttr {
    pub type Type = crate::ctypes::c_uint;
    pub const SCE_UDCD_DEVICE_REQUEST_ATTR_PHYCONT: Type = 1;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdStringDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub bString: [crate::ctypes::c_short; 31usize],
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDeviceDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub bcdUSB: crate::ctypes::c_ushort,
    pub bDeviceClass: crate::ctypes::c_uchar,
    pub bDeviceSubClass: crate::ctypes::c_uchar,
    pub bDeviceProtocol: crate::ctypes::c_uchar,
    pub bMaxPacketSize0: crate::ctypes::c_uchar,
    pub idVendor: crate::ctypes::c_ushort,
    pub idProduct: crate::ctypes::c_ushort,
    pub bcdDevice: crate::ctypes::c_ushort,
    pub iManufacturer: crate::ctypes::c_uchar,
    pub iProduct: crate::ctypes::c_uchar,
    pub iSerialNumber: crate::ctypes::c_uchar,
    pub bNumConfigurations: crate::ctypes::c_uchar,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDeviceQualifierDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub bcdUSB: crate::ctypes::c_ushort,
    pub bDeviceClass: crate::ctypes::c_uchar,
    pub bDeviceSubClass: crate::ctypes::c_uchar,
    pub bDeviceProtocol: crate::ctypes::c_uchar,
    pub bMaxPacketSize0: crate::ctypes::c_uchar,
    pub bNumConfigurations: crate::ctypes::c_uchar,
    pub bReserved: crate::ctypes::c_uchar,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdConfigDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub wTotalLength: crate::ctypes::c_ushort,
    pub bNumInterfaces: crate::ctypes::c_uchar,
    pub bConfigurationValue: crate::ctypes::c_uchar,
    pub iConfiguration: crate::ctypes::c_uchar,
    pub bmAttributes: crate::ctypes::c_uchar,
    pub bMaxPower: crate::ctypes::c_uchar,
    pub settings: *mut SceUdcdInterfaceSettings,
    pub extra: *mut crate::ctypes::c_uchar,
    pub extraLength: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdInterfaceSettings {
    pub descriptors: *mut SceUdcdInterfaceDescriptor,
    pub alternateSetting: crate::ctypes::c_uint,
    pub numDescriptors: crate::ctypes::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdInterfaceDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub bInterfaceNumber: crate::ctypes::c_uchar,
    pub bAlternateSetting: crate::ctypes::c_uchar,
    pub bNumEndpoints: crate::ctypes::c_uchar,
    pub bInterfaceClass: crate::ctypes::c_uchar,
    pub bInterfaceSubClass: crate::ctypes::c_uchar,
    pub bInterfaceProtocol: crate::ctypes::c_uchar,
    pub iInterface: crate::ctypes::c_uchar,
    pub endpoints: *mut SceUdcdEndpointDescriptor,
    pub extra: *mut crate::ctypes::c_uchar,
    pub extraLength: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdEndpointDescriptor {
    pub bLength: crate::ctypes::c_uchar,
    pub bDescriptorType: crate::ctypes::c_uchar,
    pub bEndpointAddress: crate::ctypes::c_uchar,
    pub bmAttributes: crate::ctypes::c_uchar,
    pub wMaxPacketSize: crate::ctypes::c_ushort,
    pub bInterval: crate::ctypes::c_uchar,
    pub extra: *mut crate::ctypes::c_uchar,
    pub extraLength: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdInterface {
    pub expectNumber: crate::ctypes::c_int,
    pub interfaceNumber: crate::ctypes::c_int,
    pub numInterfaces: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdEndpoint {
    pub direction: crate::ctypes::c_int,
    pub driverEndpointNumber: crate::ctypes::c_int,
    pub endpointNumber: crate::ctypes::c_int,
    pub transmittedBytes: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdConfiguration {
    pub configDescriptors: *mut SceUdcdConfigDescriptor,
    pub settings: *mut SceUdcdInterfaceSettings,
    pub interfaceDescriptors: *mut SceUdcdInterfaceDescriptor,
    pub endpointDescriptors: *mut SceUdcdEndpointDescriptor,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdEP0DeviceRequest {
    pub bmRequestType: crate::ctypes::c_uchar,
    pub bRequest: crate::ctypes::c_uchar,
    pub wValue: crate::ctypes::c_ushort,
    pub wIndex: crate::ctypes::c_ushort,
    pub wLength: crate::ctypes::c_ushort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDriver {
    pub driverName: *const crate::ctypes::c_char,
    pub numEndpoints: crate::ctypes::c_int,
    pub endpoints: *mut SceUdcdEndpoint,
    pub interface: *mut SceUdcdInterface,
    pub descriptor_hi: *mut SceUdcdDeviceDescriptor,
    pub configuration_hi: *mut SceUdcdConfiguration,
    pub descriptor: *mut SceUdcdDeviceDescriptor,
    pub configuration: *mut SceUdcdConfiguration,
    pub stringDescriptors: *mut SceUdcdStringDescriptor,
    pub stringDescriptorProduct: *mut SceUdcdStringDescriptor,
    pub stringDescriptorSerial: *mut SceUdcdStringDescriptor,
    pub processRequest: ::core::option::Option<
        unsafe extern "C" fn(
            recipient: crate::ctypes::c_int,
            arg: crate::ctypes::c_int,
            req: *mut SceUdcdEP0DeviceRequest,
            user_data: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub changeSetting: ::core::option::Option<
        unsafe extern "C" fn(
            interfaceNumber: crate::ctypes::c_int,
            alternateSetting: crate::ctypes::c_int,
            bus: crate::ctypes::c_int,
        ) -> crate::ctypes::c_int,
    >,
    pub attach: ::core::option::Option<
        unsafe extern "C" fn(
            usb_version: crate::ctypes::c_int,
            user_data: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub detach: ::core::option::Option<unsafe extern "C" fn(user_data: *mut crate::ctypes::c_void)>,
    pub configure: ::core::option::Option<
        unsafe extern "C" fn(
            usb_version: crate::ctypes::c_int,
            desc_count: crate::ctypes::c_int,
            settings: *mut SceUdcdInterfaceSettings,
            user_data: *mut crate::ctypes::c_void,
        ),
    >,
    pub start: ::core::option::Option<
        unsafe extern "C" fn(
            size: crate::ctypes::c_int,
            args: *mut crate::ctypes::c_void,
            user_data: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub stop: ::core::option::Option<
        unsafe extern "C" fn(
            size: crate::ctypes::c_int,
            args: *mut crate::ctypes::c_void,
            user_data: *mut crate::ctypes::c_void,
        ) -> crate::ctypes::c_int,
    >,
    pub user_data: *mut crate::ctypes::c_void,
    pub bus: crate::ctypes::c_int,
    pub link: *mut SceUdcdDriver,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDeviceRequest {
    pub endpoint: *mut SceUdcdEndpoint,
    pub data: *mut crate::ctypes::c_void,
    pub attributes: crate::ctypes::c_uint,
    pub size: crate::ctypes::c_int,
    pub isControlRequest: crate::ctypes::c_int,
    pub onComplete: ::core::option::Option<unsafe extern "C" fn(req: *mut SceUdcdDeviceRequest)>,
    pub transmitted: crate::ctypes::c_int,
    pub returnCode: crate::ctypes::c_int,
    pub next: *mut SceUdcdDeviceRequest,
    pub unused: *mut crate::ctypes::c_void,
    pub physicalAddress: *mut crate::ctypes::c_void,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDriverName {
    pub size: crate::ctypes::c_int,
    pub name: [crate::ctypes::c_char; 32usize],
    pub flags: crate::ctypes::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdDeviceInfo {
    pub info: [crate::ctypes::c_uchar; 64usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SceUdcdWaitParam {
    pub unk_00: crate::ctypes::c_int,
    pub status: crate::ctypes::c_int,
    pub unk_08: crate::ctypes::c_int,
    pub unk_0C: crate::ctypes::c_int,
    pub unk_10: crate::ctypes::c_int,
    pub driverName: *const crate::ctypes::c_char,
}
extern "C" {
    pub fn ksceUdcdWaitBusInitialized(
        timeout: crate::ctypes::c_uint,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdStart(
        driverName: *const crate::ctypes::c_char,
        size: crate::ctypes::c_int,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdStartInternal(
        driverName: *const crate::ctypes::c_char,
        size: crate::ctypes::c_int,
        args: *mut crate::ctypes::c_void,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdStartCurrentInternal(
        unused: crate::ctypes::c_int,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdStop(
        driverName: *const crate::ctypes::c_char,
        size: crate::ctypes::c_int,
        args: *mut crate::ctypes::c_void,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdStopInternal(
        driverName: *const crate::ctypes::c_char,
        size: crate::ctypes::c_int,
        args: *mut crate::ctypes::c_void,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdStopCurrentInternal(bus: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdActivate(productId: crate::ctypes::c_uint) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdActivateInternal(
        productId: crate::ctypes::c_uint,
        bus_powered: crate::ctypes::c_uint,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdDeactivate() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdDeactivateInternal(bus: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdGetDeviceState() -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdGetDeviceStateInternal(bus: crate::ctypes::c_int) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdGetDeviceInfo(devInfo: *mut SceUdcdDeviceInfo) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdGetDeviceInfoInternal(
        devInfo: *mut SceUdcdDeviceInfo,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdGetDrvState(driverName: *const crate::ctypes::c_char) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdGetDrvStateInternal(
        driverName: *const crate::ctypes::c_char,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdWaitState(
        waitParam: *mut SceUdcdWaitParam,
        timeout: crate::ctypes::c_uint,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdWaitStateInternal(
        waitParam: *mut SceUdcdWaitParam,
        timeout: crate::ctypes::c_uint,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdRegister(drv: *mut SceUdcdDriver) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdRegisterInternal(
        drv: *mut SceUdcdDriver,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdUnregister(drv: *mut SceUdcdDriver) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdUnregisterInternal(
        drv: *mut SceUdcdDriver,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdClearFIFO(endp: *mut SceUdcdEndpoint) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdClearFIFOInternal(
        endp: *mut SceUdcdEndpoint,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdReqCancelAll(endp: *mut SceUdcdEndpoint) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdStall(endp: *mut SceUdcdEndpoint) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdStallInternal(
        endp: *mut SceUdcdEndpoint,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdReqSend(req: *mut SceUdcdDeviceRequest) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdReqSendInternal(
        req: *mut SceUdcdDeviceRequest,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdReqRecv(req: *mut SceUdcdDeviceRequest) -> crate::ctypes::c_int;
}
extern "C" {
    pub fn ksceUdcdReqRecvInternal(
        req: *mut SceUdcdDeviceRequest,
        bus: crate::ctypes::c_int,
    ) -> crate::ctypes::c_int;
}
