#![allow(non_camel_case_types,unused_assignments,unused_unsafe,non_snake_case,non_upper_case_globals,unused_must_use)]

use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn libusb_get_device_descriptor(
        dev: *mut libusb_device,
        desc: *mut libusb_device_descriptor,
    ) -> libc::c_int;
    #[no_mangle]
    fn libusb_get_active_config_descriptor(
        dev: *mut libusb_device,
        config: *mut *mut libusb_config_descriptor,
    ) -> libc::c_int;
    #[no_mangle]
    fn libusb_free_config_descriptor(config: *mut libusb_config_descriptor);
    #[no_mangle]
    fn libusb_get_ss_endpoint_companion_descriptor(
        ctx: *mut libusb_context,
        endpoint: *const libusb_endpoint_descriptor,
        ep_comp: *mut *mut libusb_ss_endpoint_companion_descriptor,
    ) -> libc::c_int;
    #[no_mangle]
    fn libusb_free_ss_endpoint_companion_descriptor(
        ep_comp: *mut libusb_ss_endpoint_companion_descriptor,
    );
    /* sync I/O */
    #[no_mangle]
    fn libusb_control_transfer(
        dev_handle: *mut libusb_device_handle,
        request_type: uint8_t,
        bRequest: uint8_t,
        wValue: uint16_t,
        wIndex: uint16_t,
        data: *mut libc::c_uchar,
        wLength: uint16_t,
        timeout: libc::c_uint,
    ) -> libc::c_int;
    #[no_mangle]
    fn libusb_lock_events(ctx: *mut libusb_context);
    #[no_mangle]
    fn libusb_unlock_events(ctx: *mut libusb_context);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn usbi_get_tid() -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn libusb_handle_events_timeout(ctx: *mut libusb_context, tv: *mut timeval) -> libc::c_int;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    #[no_mangle]
    fn usbi_io_exit(ctx: *mut libusb_context);
    #[no_mangle]
    fn usbi_io_init(ctx: *mut libusb_context) -> libc::c_int;
    #[no_mangle]
    static usbi_backend: usbi_os_backend;
    #[no_mangle]
    fn usbi_hotplug_deregister(ctx: *mut libusb_context, forced: libc::c_int);
    #[no_mangle]
    fn usbi_hotplug_notification(
        ctx: *mut libusb_context,
        dev: *mut libusb_device,
        event: libusb_hotplug_event,
    );
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    #[no_mangle]
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub c2rust_unnamed: C2RustUnnamed_1,
    pub c2rust_unnamed_0: C2RustUnnamed,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_key_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type intptr_t = libc::c_long;
pub type uintptr_t = libc::c_ulong;
/* * \ingroup libusb_desc
 * Descriptor types as defined by the USB specification. */
pub type libusb_descriptor_type = libc::c_uint;
/* * SuperSpeed Endpoint Companion descriptor */
pub const LIBUSB_DT_SS_ENDPOINT_COMPANION: libusb_descriptor_type = 48;
/* * SuperSpeed Hub descriptor */
pub const LIBUSB_DT_SUPERSPEED_HUB: libusb_descriptor_type = 42;
/* * Hub descriptor */
pub const LIBUSB_DT_HUB: libusb_descriptor_type = 41;
/* * Physical descriptor */
pub const LIBUSB_DT_PHYSICAL: libusb_descriptor_type = 35;
/* * HID report descriptor */
pub const LIBUSB_DT_REPORT: libusb_descriptor_type = 34;
/* * HID descriptor */
pub const LIBUSB_DT_HID: libusb_descriptor_type = 33;
/* * Device Capability descriptor */
pub const LIBUSB_DT_DEVICE_CAPABILITY: libusb_descriptor_type = 16;
/* * BOS descriptor */
pub const LIBUSB_DT_BOS: libusb_descriptor_type = 15;
/* * Endpoint descriptor. See libusb_endpoint_descriptor. */
pub const LIBUSB_DT_ENDPOINT: libusb_descriptor_type = 5;
/* * Interface descriptor. See libusb_interface_descriptor. */
pub const LIBUSB_DT_INTERFACE: libusb_descriptor_type = 4;
/* * String descriptor */
pub const LIBUSB_DT_STRING: libusb_descriptor_type = 3;
/* * Configuration descriptor. See libusb_config_descriptor. */
pub const LIBUSB_DT_CONFIG: libusb_descriptor_type = 2;
/* * Device descriptor. See libusb_device_descriptor. */
pub const LIBUSB_DT_DEVICE: libusb_descriptor_type = 1;
/* Audio extension */
/* BOS descriptor sizes */
/* We unwrap the BOS => define its max size */
/* in bEndpointAddress */
/* * \ingroup libusb_desc
 * Endpoint direction. Values for bit 7 of the
 * \ref libusb_endpoint_descriptor::bEndpointAddress "endpoint address" scheme.
 */
pub type libusb_endpoint_direction = libc::c_uint;
/* * In: device-to-host */
pub const LIBUSB_ENDPOINT_IN: libusb_endpoint_direction = 128;
/* * Out: host-to-device */
pub const LIBUSB_ENDPOINT_OUT: libusb_endpoint_direction = 0;
/* in bmAttributes */
/* * \ingroup libusb_desc
 * Endpoint transfer type. Values for bits 0:1 of the
 * \ref libusb_endpoint_descriptor::bmAttributes "endpoint attributes" field.
 */
pub type libusb_endpoint_transfer_type = libc::c_uint;
/* * Interrupt endpoint */
pub const LIBUSB_ENDPOINT_TRANSFER_TYPE_INTERRUPT: libusb_endpoint_transfer_type = 3;
/* * Bulk endpoint */
pub const LIBUSB_ENDPOINT_TRANSFER_TYPE_BULK: libusb_endpoint_transfer_type = 2;
/* * Isochronous endpoint */
pub const LIBUSB_ENDPOINT_TRANSFER_TYPE_ISOCHRONOUS: libusb_endpoint_transfer_type = 1;
/* * Control endpoint */
pub const LIBUSB_ENDPOINT_TRANSFER_TYPE_CONTROL: libusb_endpoint_transfer_type = 0;
/* * \ingroup libusb_misc
 * Standard requests, as defined in table 9-5 of the USB 3.0 specifications */
pub type libusb_standard_request = libc::c_uint;
/* * Delay from the time a host transmits a packet to the time it is
 * received by the device. */
pub const LIBUSB_SET_ISOCH_DELAY: libusb_standard_request = 49;
/* * Sets both the U1 and U2 Exit Latency */
pub const LIBUSB_REQUEST_SET_SEL: libusb_standard_request = 48;
/* * Set then report an endpoint's synchronization frame */
pub const LIBUSB_REQUEST_SYNCH_FRAME: libusb_standard_request = 12;
/* * Select an alternate interface for the specified interface */
pub const LIBUSB_REQUEST_SET_INTERFACE: libusb_standard_request = 11;
/* * Return the selected alternate setting for the specified interface */
pub const LIBUSB_REQUEST_GET_INTERFACE: libusb_standard_request = 10;
/* * Set device configuration */
pub const LIBUSB_REQUEST_SET_CONFIGURATION: libusb_standard_request = 9;
/* * Get the current device configuration value */
pub const LIBUSB_REQUEST_GET_CONFIGURATION: libusb_standard_request = 8;
/* * Used to update existing descriptors or add new descriptors */
pub const LIBUSB_REQUEST_SET_DESCRIPTOR: libusb_standard_request = 7;
/* * Get the specified descriptor */
pub const LIBUSB_REQUEST_GET_DESCRIPTOR: libusb_standard_request = 6;
/* 0x04 is reserved */
/* * Set device address for all future accesses */
pub const LIBUSB_REQUEST_SET_ADDRESS: libusb_standard_request = 5;
/* 0x02 is reserved */
/* * Set or enable a specific feature */
pub const LIBUSB_REQUEST_SET_FEATURE: libusb_standard_request = 3;
/* * Clear or disable a specific feature */
pub const LIBUSB_REQUEST_CLEAR_FEATURE: libusb_standard_request = 1;
/* * Request status of the specific recipient */
pub const LIBUSB_REQUEST_GET_STATUS: libusb_standard_request = 0;
/* * \ingroup libusb_desc
 * A structure representing the standard USB device descriptor. This
 * descriptor is documented in section 9.6.1 of the USB 3.0 specification.
 * All multiple-byte fields are represented in host-endian format.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_device_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub bcdUSB: uint16_t,
    pub bDeviceClass: uint8_t,
    pub bDeviceSubClass: uint8_t,
    pub bDeviceProtocol: uint8_t,
    pub bMaxPacketSize0: uint8_t,
    pub idVendor: uint16_t,
    pub idProduct: uint16_t,
    pub bcdDevice: uint16_t,
    pub iManufacturer: uint8_t,
    pub iProduct: uint8_t,
    pub iSerialNumber: uint8_t,
    pub bNumConfigurations: uint8_t,
}
/* * \ingroup libusb_desc
 * A structure representing the standard USB endpoint descriptor. This
 * descriptor is documented in section 9.6.6 of the USB 3.0 specification.
 * All multiple-byte fields are represented in host-endian format.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_endpoint_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub bEndpointAddress: uint8_t,
    pub bmAttributes: uint8_t,
    pub wMaxPacketSize: uint16_t,
    pub bInterval: uint8_t,
    pub bRefresh: uint8_t,
    pub bSynchAddress: uint8_t,
    pub extra: *const libc::c_uchar,
    pub extra_length: libc::c_int,
}
/* * \ingroup libusb_desc
 * A structure representing the standard USB interface descriptor. This
 * descriptor is documented in section 9.6.5 of the USB 3.0 specification.
 * All multiple-byte fields are represented in host-endian format.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_interface_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub bInterfaceNumber: uint8_t,
    pub bAlternateSetting: uint8_t,
    pub bNumEndpoints: uint8_t,
    pub bInterfaceClass: uint8_t,
    pub bInterfaceSubClass: uint8_t,
    pub bInterfaceProtocol: uint8_t,
    pub iInterface: uint8_t,
    pub endpoint: *const libusb_endpoint_descriptor,
    pub extra: *const libc::c_uchar,
    pub extra_length: libc::c_int,
}
/* * \ingroup libusb_desc
 * A collection of alternate settings for a particular USB interface.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_interface {
    pub altsetting: *const libusb_interface_descriptor,
    pub num_altsetting: libc::c_int,
}
/* * \ingroup libusb_desc
 * A structure representing the standard USB configuration descriptor. This
 * descriptor is documented in section 9.6.3 of the USB 3.0 specification.
 * All multiple-byte fields are represented in host-endian format.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_config_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub wTotalLength: uint16_t,
    pub bNumInterfaces: uint8_t,
    pub bConfigurationValue: uint8_t,
    pub iConfiguration: uint8_t,
    pub bmAttributes: uint8_t,
    pub MaxPower: uint8_t,
    pub interface: *const libusb_interface,
    pub extra: *const libc::c_uchar,
    pub extra_length: libc::c_int,
}
/* * \ingroup libusb_desc
 * A structure representing the superspeed endpoint companion
 * descriptor. This descriptor is documented in section 9.6.7 of
 * the USB 3.0 specification. All multiple-byte fields are represented in
 * host-endian format.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_ss_endpoint_companion_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub bMaxBurst: uint8_t,
    pub bmAttributes: uint8_t,
    pub wBytesPerInterval: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_context {
    pub debug: libusb_log_level,
    pub debug_fixed: libc::c_int,
    pub log_handler: libusb_log_cb,
    pub event_pipe: [libc::c_int; 2],
    pub usb_devs: list_head,
    pub usb_devs_lock: usbi_mutex_t,
    pub open_devs: list_head,
    pub open_devs_lock: usbi_mutex_t,
    pub hotplug_cbs: list_head,
    pub next_hotplug_cb_handle: libusb_hotplug_callback_handle,
    pub hotplug_cbs_lock: usbi_mutex_t,
    pub flying_transfers: list_head,
    pub flying_transfers_lock: usbi_mutex_t,
    pub fd_added_cb: libusb_pollfd_added_cb,
    pub fd_removed_cb: libusb_pollfd_removed_cb,
    pub fd_cb_user_data: *mut libc::c_void,
    pub events_lock: usbi_mutex_t,
    pub event_handler_active: libc::c_int,
    pub event_handling_key: usbi_tls_key_t,
    pub event_waiters_lock: usbi_mutex_t,
    pub event_waiters_cond: usbi_cond_t,
    pub event_data_lock: usbi_mutex_t,
    pub event_flags: libc::c_uint,
    pub device_close: libc::c_uint,
    pub ipollfds: list_head,
    pub removed_ipollfds: list_head,
    pub pollfds: *mut pollfd,
    pub pollfds_cnt: usbi_nfds_t,
    pub hotplug_msgs: list_head,
    pub completed_transfers: list_head,
    pub timerfd: libc::c_int,
    pub list: list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list_head {
    pub prev: *mut list_head,
    pub next: *mut list_head,
}
pub type usbi_nfds_t = nfds_t;
pub type nfds_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
/*
 * libusb synchronization using POSIX Threads
 *
 * Copyright © 2010 Peter Stuge <peter@stuge.se>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */
pub type usbi_mutex_t = pthread_mutex_t;
pub type usbi_cond_t = pthread_cond_t;
pub type usbi_tls_key_t = pthread_key_t;
/* * \ingroup libusb_poll
 * File descriptor for polling
 */
/* * Numeric file descriptor */
/* * Event flags to poll for from <poll.h>. POLLIN indicates that you
 * should monitor this file descriptor for becoming ready to read from,
 * and POLLOUT indicates that you should monitor this file descriptor for
 * nonblocking write readiness. */
/* * \ingroup libusb_poll
 * Callback function, invoked when a new file descriptor should be added
 * to the set of file descriptors monitored for events.
 * \param fd the new file descriptor
 * \param events events to monitor for, see \ref libusb_pollfd for a
 * description
 * \param user_data User data pointer specified in
 * libusb_set_pollfd_notifiers() call
 * \see libusb_set_pollfd_notifiers()
 */
/* * \ingroup libusb_poll
 * Callback function, invoked when a file descriptor should be removed from
 * the set of file descriptors being monitored for events. After returning
 * from this callback, do not use that file descriptor again.
 * \param fd the file descriptor to stop monitoring
 * \param user_data User data pointer specified in
 * libusb_set_pollfd_notifiers() call
 * \see libusb_set_pollfd_notifiers()
 */
pub type libusb_pollfd_removed_cb =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_void) -> ()>;
pub type libusb_pollfd_added_cb =
    Option<unsafe extern "C" fn(_: libc::c_int, _: libc::c_short, _: *mut libc::c_void) -> ()>;
/* * \ingroup libusb_hotplug
 * Callback handle.
 *
 * Callbacks handles are generated by libusb_hotplug_register_callback()
 * and can be used to deregister callbacks. Callback handles are unique
 * per libusb_context and it is safe to call libusb_hotplug_deregister_callback()
 * on an already deregisted callback.
 *
 * Since version 1.0.16, \ref LIBUSB_API_VERSION >= 0x01000102
 *
 * For more information, see \ref libusb_hotplug.
 */
pub type libusb_hotplug_callback_handle = libc::c_int;
/* * \ingroup libusb_lib
 * Callback function for handling log messages.
 * \param ctx the context which is related to the log message, or NULL if it
 * is a global log message
 * \param level the log level, see \ref libusb_log_level for a description
 * \param str the log message
 * \see libusb_set_log_cb()
 */
pub type libusb_log_cb = Option<
    unsafe extern "C" fn(_: *mut libusb_context, _: libusb_log_level, _: *const libc::c_char) -> (),
>;
/* * \ingroup libusb_lib
 *  Log message levels.
 */
pub type libusb_log_level = libc::c_uint;
/* * (4) : All messages are emitted */
pub const LIBUSB_LOG_LEVEL_DEBUG: libusb_log_level = 4;
/* * (3) : Informational, warning and error messages are emitted */
pub const LIBUSB_LOG_LEVEL_INFO: libusb_log_level = 3;
/* * (2) : Warning and error messages are emitted */
pub const LIBUSB_LOG_LEVEL_WARNING: libusb_log_level = 2;
/* * (1) : Error messages are emitted */
pub const LIBUSB_LOG_LEVEL_ERROR: libusb_log_level = 1;
/* * (0) : No messages ever emitted by the library (default) */
pub const LIBUSB_LOG_LEVEL_NONE: libusb_log_level = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_device {
    pub lock: usbi_mutex_t,
    pub refcnt: libc::c_int,
    pub ctx: *mut libusb_context,
    pub parent_dev: *mut libusb_device,
    pub bus_number: uint8_t,
    pub port_number: uint8_t,
    pub device_address: uint8_t,
    pub speed: libusb_speed,
    pub list: list_head,
    pub session_data: libc::c_ulong,
    pub device_descriptor: libusb_device_descriptor,
    pub attached: libc::c_int,
}
/* * \ingroup libusb_dev
 * Speed codes. Indicates the speed at which the device is operating.
 */
pub type libusb_speed = libc::c_uint;
/* * The device is operating at super speed plus (10000MBit/s). */
pub const LIBUSB_SPEED_SUPER_PLUS: libusb_speed = 5;
/* * The device is operating at super speed (5000MBit/s). */
pub const LIBUSB_SPEED_SUPER: libusb_speed = 4;
/* * The device is operating at high speed (480MBit/s). */
pub const LIBUSB_SPEED_HIGH: libusb_speed = 3;
/* * The device is operating at full speed (12MBit/s). */
pub const LIBUSB_SPEED_FULL: libusb_speed = 2;
/* * The device is operating at low speed (1.5MBit/s). */
pub const LIBUSB_SPEED_LOW: libusb_speed = 1;
/* * The OS doesn't report or know the device speed. */
pub const LIBUSB_SPEED_UNKNOWN: libusb_speed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_device_handle {
    pub lock: usbi_mutex_t,
    pub claimed_interfaces: libc::c_ulong,
    pub list: list_head,
    pub dev: *mut libusb_device,
    pub auto_detach_kernel_driver: libc::c_int,
}
/* libusb */
/* * \ingroup libusb_lib
 * Structure providing the version of the libusb runtime
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_version {
    pub major: uint16_t,
    pub minor: uint16_t,
    pub micro: uint16_t,
    pub nano: uint16_t,
    pub rc: *const libc::c_char,
    pub describe: *const libc::c_char,
}
/* * \ingroup libusb_misc
 * Error codes. Most libusb functions return 0 on success or one of these
 * codes on failure.
 * You can call libusb_error_name() to retrieve a string representation of an
 * error code or libusb_strerror() to get an end-user suitable description of
 * an error code.
 */
pub type libusb_error = libc::c_int;
/* NB: Remember to update LIBUSB_ERROR_COUNT below as well as the
message strings in strerror.c when adding new error codes here. */
/* * Other error */
pub const LIBUSB_ERROR_OTHER: libusb_error = -99;
/* * Operation not supported or unimplemented on this platform */
pub const LIBUSB_ERROR_NOT_SUPPORTED: libusb_error = -12;
/* * Insufficient memory */
pub const LIBUSB_ERROR_NO_MEM: libusb_error = -11;
/* * System call interrupted (perhaps due to signal) */
pub const LIBUSB_ERROR_INTERRUPTED: libusb_error = -10;
/* * Pipe error */
pub const LIBUSB_ERROR_PIPE: libusb_error = -9;
/* * Overflow */
pub const LIBUSB_ERROR_OVERFLOW: libusb_error = -8;
/* * Operation timed out */
pub const LIBUSB_ERROR_TIMEOUT: libusb_error = -7;
/* * Resource busy */
pub const LIBUSB_ERROR_BUSY: libusb_error = -6;
/* * Entity not found */
pub const LIBUSB_ERROR_NOT_FOUND: libusb_error = -5;
/* * No such device (it may have been disconnected) */
pub const LIBUSB_ERROR_NO_DEVICE: libusb_error = -4;
/* * Access denied (insufficient permissions) */
pub const LIBUSB_ERROR_ACCESS: libusb_error = -3;
/* * Invalid parameter */
pub const LIBUSB_ERROR_INVALID_PARAM: libusb_error = -2;
/* * Input/output error */
pub const LIBUSB_ERROR_IO: libusb_error = -1;
/* * Success (no error) */
pub const LIBUSB_SUCCESS: libusb_error = 0;
/* * \ingroup libusb_asyncio
 * Transfer status codes */
pub type libusb_transfer_status = libc::c_uint;
/* NB! Remember to update libusb_error_name()
when adding new status codes here. */
/* * Device sent more data than requested */
pub const LIBUSB_TRANSFER_OVERFLOW: libusb_transfer_status = 6;
/* * Device was disconnected */
pub const LIBUSB_TRANSFER_NO_DEVICE: libusb_transfer_status = 5;
/* * For bulk/interrupt endpoints: halt condition detected (endpoint
 * stalled). For control endpoints: control request not supported. */
pub const LIBUSB_TRANSFER_STALL: libusb_transfer_status = 4;
/* * Transfer was cancelled */
pub const LIBUSB_TRANSFER_CANCELLED: libusb_transfer_status = 3;
/* * Transfer timed out */
pub const LIBUSB_TRANSFER_TIMED_OUT: libusb_transfer_status = 2;
/* * Transfer failed */
pub const LIBUSB_TRANSFER_ERROR: libusb_transfer_status = 1;
/* * Transfer completed without error. Note that this does not indicate
 * that the entire amount of requested data was transferred. */
pub const LIBUSB_TRANSFER_COMPLETED: libusb_transfer_status = 0;
/* * \ingroup libusb_asyncio
 * Isochronous packet descriptor. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_iso_packet_descriptor {
    pub length: libc::c_uint,
    pub actual_length: libc::c_uint,
    pub status: libusb_transfer_status,
}
/* * \ingroup libusb_asyncio
 * The generic USB transfer structure. The user populates this structure and
 * then submits it in order to request a transfer. After the transfer has
 * completed, the library populates the transfer with the results and passes
 * it back to the user.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_transfer {
    pub dev_handle: *mut libusb_device_handle,
    pub flags: uint8_t,
    pub endpoint: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub timeout: libc::c_uint,
    pub status: libusb_transfer_status,
    pub length: libc::c_int,
    pub actual_length: libc::c_int,
    pub callback: libusb_transfer_cb_fn,
    pub user_data: *mut libc::c_void,
    pub buffer: *mut libc::c_uchar,
    pub num_iso_packets: libc::c_int,
    pub iso_packet_desc: [libusb_iso_packet_descriptor; 0],
}
/* * \ingroup libusb_asyncio
 * Asynchronous transfer callback function type. When submitting asynchronous
 * transfers, you pass a pointer to a callback function of this type via the
 * \ref libusb_transfer::callback "callback" member of the libusb_transfer
 * structure. libusb will call this function later, when the transfer has
 * completed or failed. See \ref libusb_asyncio for more information.
 * \param transfer The libusb_transfer struct the callback function is being
 * notified about.
 */
pub type libusb_transfer_cb_fn = Option<unsafe extern "C" fn(_: *mut libusb_transfer) -> ()>;
/* * \ingroup libusb_misc
 * Capabilities supported by an instance of libusb on the current running
 * platform. Test if the loaded library supports a given capability by calling
 * \ref libusb_has_capability().
 */
pub type libusb_capability = libc::c_uint;
/* * The library supports detaching of the default USB driver, using
 * \ref libusb_detach_kernel_driver(), if one is set by the OS kernel */
pub const LIBUSB_CAP_SUPPORTS_DETACH_KERNEL_DRIVER: libusb_capability = 257;
/* * The library can access HID devices without requiring user intervention.
 * Note that before being able to actually access an HID device, you may
 * still have to call additional libusb functions such as
 * \ref libusb_detach_kernel_driver(). */
pub const LIBUSB_CAP_HAS_HID_ACCESS: libusb_capability = 256;
/* * Hotplug support is available on this platform. */
pub const LIBUSB_CAP_HAS_HOTPLUG: libusb_capability = 1;
/* * The libusb_has_capability() API is available. */
pub const LIBUSB_CAP_HAS_CAPABILITY: libusb_capability = 0;
/* * \ingroup libusb_lib
 *  Log callback mode.
 * \see libusb_set_log_cb()
 */
pub type libusb_log_cb_mode = libc::c_uint;
/* * Callback function handling context related log mesages. */
pub const LIBUSB_LOG_CB_CONTEXT: libusb_log_cb_mode = 2;
/* * Callback function handling all log mesages. */
pub const LIBUSB_LOG_CB_GLOBAL: libusb_log_cb_mode = 1;
pub type usbi_mutex_static_t = pthread_mutex_t;
pub type libusb_hotplug_event = libc::c_uint;
pub const LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT: libusb_hotplug_event = 2;
pub const LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED: libusb_hotplug_event = 1;
/* OS abstraction */
/* This is the interface that OS backends need to implement.
 * All fields are mandatory, except ones explicitly noted as optional. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbi_os_backend {
    pub name: *const libc::c_char,
    pub caps: uint32_t,
    pub init: Option<unsafe extern "C" fn(_: *mut libusb_context) -> libc::c_int>,
    pub exit: Option<unsafe extern "C" fn(_: *mut libusb_context) -> ()>,
    pub set_option: Option<
        unsafe extern "C" fn(
            _: *mut libusb_context,
            _: libusb_option,
            _: ::std::ffi::VaList,
        ) -> libc::c_int,
    >,
    pub get_device_list: Option<
        unsafe extern "C" fn(_: *mut libusb_context, _: *mut *mut discovered_devs) -> libc::c_int,
    >,
    pub hotplug_poll: Option<unsafe extern "C" fn() -> ()>,
    pub wrap_sys_device: Option<
        unsafe extern "C" fn(
            _: *mut libusb_context,
            _: *mut libusb_device_handle,
            _: intptr_t,
        ) -> libc::c_int,
    >,
    pub open: Option<unsafe extern "C" fn(_: *mut libusb_device_handle) -> libc::c_int>,
    pub close: Option<unsafe extern "C" fn(_: *mut libusb_device_handle) -> ()>,
    pub get_active_config_descriptor: Option<
        unsafe extern "C" fn(_: *mut libusb_device, _: *mut libc::c_void, _: size_t) -> libc::c_int,
    >,
    pub get_config_descriptor: Option<
        unsafe extern "C" fn(
            _: *mut libusb_device,
            _: uint8_t,
            _: *mut libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub get_config_descriptor_by_value: Option<
        unsafe extern "C" fn(
            _: *mut libusb_device,
            _: uint8_t,
            _: *mut *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub get_configuration:
        Option<unsafe extern "C" fn(_: *mut libusb_device_handle, _: *mut uint8_t) -> libc::c_int>,
    pub set_configuration:
        Option<unsafe extern "C" fn(_: *mut libusb_device_handle, _: libc::c_int) -> libc::c_int>,
    pub claim_interface:
        Option<unsafe extern "C" fn(_: *mut libusb_device_handle, _: uint8_t) -> libc::c_int>,
    pub release_interface:
        Option<unsafe extern "C" fn(_: *mut libusb_device_handle, _: uint8_t) -> libc::c_int>,
    pub set_interface_altsetting: Option<
        unsafe extern "C" fn(_: *mut libusb_device_handle, _: uint8_t, _: uint8_t) -> libc::c_int,
    >,
    pub clear_halt:
        Option<unsafe extern "C" fn(_: *mut libusb_device_handle, _: libc::c_uchar) -> libc::c_int>,
    pub reset_device: Option<unsafe extern "C" fn(_: *mut libusb_device_handle) -> libc::c_int>,
    pub alloc_streams: Option<
        unsafe extern "C" fn(
            _: *mut libusb_device_handle,
            _: uint32_t,
            _: *mut libc::c_uchar,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub free_streams: Option<
        unsafe extern "C" fn(
            _: *mut libusb_device_handle,
            _: *mut libc::c_uchar,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub dev_mem_alloc:
        Option<unsafe extern "C" fn(_: *mut libusb_device_handle, _: size_t) -> *mut libc::c_void>,
    pub dev_mem_free: Option<
        unsafe extern "C" fn(
            _: *mut libusb_device_handle,
            _: *mut libc::c_void,
            _: size_t,
        ) -> libc::c_int,
    >,
    pub kernel_driver_active:
        Option<unsafe extern "C" fn(_: *mut libusb_device_handle, _: uint8_t) -> libc::c_int>,
    pub detach_kernel_driver:
        Option<unsafe extern "C" fn(_: *mut libusb_device_handle, _: uint8_t) -> libc::c_int>,
    pub attach_kernel_driver:
        Option<unsafe extern "C" fn(_: *mut libusb_device_handle, _: uint8_t) -> libc::c_int>,
    pub destroy_device: Option<unsafe extern "C" fn(_: *mut libusb_device) -> ()>,
    pub submit_transfer: Option<unsafe extern "C" fn(_: *mut usbi_transfer) -> libc::c_int>,
    pub cancel_transfer: Option<unsafe extern "C" fn(_: *mut usbi_transfer) -> libc::c_int>,
    pub clear_transfer_priv: Option<unsafe extern "C" fn(_: *mut usbi_transfer) -> ()>,
    pub handle_events: Option<
        unsafe extern "C" fn(
            _: *mut libusb_context,
            _: *mut pollfd,
            _: usbi_nfds_t,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub handle_transfer_completion:
        Option<unsafe extern "C" fn(_: *mut usbi_transfer) -> libc::c_int>,
    pub context_priv_size: size_t,
    pub device_priv_size: size_t,
    pub device_handle_priv_size: size_t,
    pub transfer_priv_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbi_transfer {
    pub num_iso_packets: libc::c_int,
    pub list: list_head,
    pub completed_list: list_head,
    pub timeout: timespec,
    pub transferred: libc::c_int,
    pub stream_id: uint32_t,
    pub state_flags: uint32_t,
    pub timeout_flags: uint32_t,
    pub lock: usbi_mutex_t,
    pub priv_0: *mut libc::c_void,
}
/* must come first */
/* accessor functions for structure private data */
/* device discovery */
/* we traverse usbfs without knowing how many devices we are going to find.
 * so we create this discovered_devs model which is similar to a linked-list
 * which grows when required. it can be freed once discovery has completed,
 * eliminating the need for a list node in the libusb_device structure
 * itself. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct discovered_devs {
    pub len: size_t,
    pub capacity: size_t,
    pub devices: [*mut libusb_device; 0],
}
/* * \ingroup libusb_hotplug
 *
 * Since version 1.0.16, \ref LIBUSB_API_VERSION >= 0x01000102
 *
 * Hotplug flags */
/* * Arm the callback and fire it for all matching currently attached devices. */
/* * \ingroup libusb_hotplug
 * Convenience macro when not using any flags */
/* * \ingroup libusb_hotplug
 * Wildcard matching for hotplug events */
/* * \ingroup libusb_hotplug
 * Hotplug callback function type. When requesting hotplug event notifications,
 * you pass a pointer to a callback function of this type.
 *
 * This callback may be called by an internal event thread and as such it is
 * recommended the callback do minimal processing before returning.
 *
 * libusb will call this function later, when a matching event had happened on
 * a matching device. See \ref libusb_hotplug for more information.
 *
 * It is safe to call either libusb_hotplug_register_callback() or
 * libusb_hotplug_deregister_callback() from within a callback function.
 *
 * Since version 1.0.16, \ref LIBUSB_API_VERSION >= 0x01000102
 *
 * \param ctx            context of this notification
 * \param device         libusb_device this event occurred on
 * \param event          event that occurred
 * \param user_data      user data provided when this callback was registered
 * \returns bool whether this callback is finished processing events.
 *                       returning 1 will cause this callback to be deregistered
 */
/* * \ingroup libusb_hotplug
 * Register a hotplug callback function
 *
 * Register a callback with the libusb_context. The callback will fire
 * when a matching event occurs on a matching device. The callback is
 * armed until either it is deregistered with libusb_hotplug_deregister_callback()
 * or the supplied callback returns 1 to indicate it is finished processing events.
 *
 * If the \ref LIBUSB_HOTPLUG_ENUMERATE is passed the callback will be
 * called with a \ref LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED for all devices
 * already plugged into the machine. Note that libusb modifies its internal
 * device list from a separate thread, while calling hotplug callbacks from
 * libusb_handle_events(), so it is possible for a device to already be present
 * on, or removed from, its internal device list, while the hotplug callbacks
 * still need to be dispatched. This means that when using \ref
 * LIBUSB_HOTPLUG_ENUMERATE, your callback may be called twice for the arrival
 * of the same device, once from libusb_hotplug_register_callback() and once
 * from libusb_handle_events(); and/or your callback may be called for the
 * removal of a device for which an arrived call was never made.
 *
 * Since version 1.0.16, \ref LIBUSB_API_VERSION >= 0x01000102
 *
 * \param[in] ctx context to register this callback with
 * \param[in] events bitwise or of hotplug events that will trigger this callback.
 *            See \ref libusb_hotplug_event
 * \param[in] flags bitwise or of hotplug flags that affect registration.
 *            See \ref libusb_hotplug_flag
 * \param[in] vendor_id the vendor id to match or \ref LIBUSB_HOTPLUG_MATCH_ANY
 * \param[in] product_id the product id to match or \ref LIBUSB_HOTPLUG_MATCH_ANY
 * \param[in] dev_class the device class to match or \ref LIBUSB_HOTPLUG_MATCH_ANY
 * \param[in] cb_fn the function to be invoked on a matching event/device
 * \param[in] user_data user data to pass to the callback function
 * \param[out] callback_handle pointer to store the handle of the allocated callback (can be NULL)
 * \returns LIBUSB_SUCCESS on success LIBUSB_ERROR code on failure
 */
/* * \ingroup libusb_hotplug
 * Deregisters a hotplug callback.
 *
 * Deregister a callback from a libusb_context. This function is safe to call from within
 * a hotplug callback.
 *
 * Since version 1.0.16, \ref LIBUSB_API_VERSION >= 0x01000102
 *
 * \param[in] ctx context this callback is registered with
 * \param[in] callback_handle the handle of the callback to deregister
 */
/* * \ingroup libusb_hotplug
 * Gets the user_data associated with a hotplug callback.
 *
 * Since version v1.0.24 \ref LIBUSB_API_VERSION >= 0x01000108
 *
 * \param[in] ctx context this callback is registered with
 * \param[in] callback_handle the handle of the callback to get the user_data of
 */
/* * \ingroup libusb_lib
 * Available option values for libusb_set_option().
 */
pub type libusb_option = libc::c_uint;
/* * Use the UsbDk backend for a specific context, if available.
 *
 * This option should be set immediately after calling libusb_init(), otherwise
 * unspecified behavior may occur.
 *
 * Only valid on Windows.
 */
pub const LIBUSB_OPTION_USE_USBDK: libusb_option = 1;
/* * Set the log message verbosity.
 *
 * The default level is LIBUSB_LOG_LEVEL_NONE, which means no messages are ever
 * printed. If you choose to increase the message verbosity level, ensure
 * that your application does not close the stderr file descriptor.
 *
 * You are advised to use level LIBUSB_LOG_LEVEL_WARNING. libusb is conservative
 * with its message logging and most of the time, will only log messages that
 * explain error conditions and other oddities. This will help you debug
 * your software.
 *
 * If the LIBUSB_DEBUG environment variable was set when libusb was
 * initialized, this function does nothing: the message verbosity is fixed
 * to the value in the environment variable.
 *
 * If libusb was compiled without any message logging, this function does
 * nothing: you'll never get any messages.
 *
 * If libusb was compiled with verbose debug message logging, this function
 * does nothing: you'll always get messages from all levels.
 */
pub const LIBUSB_OPTION_LOG_LEVEL: libusb_option = 0;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub const USBI_TRANSFER_CANCELLING: usbi_transfer_state_flags = 2;
pub const USBI_TRANSFER_DEVICE_DISAPPEARED: usbi_transfer_state_flags = 4;
pub type usbi_transfer_state_flags = libc::c_uint;
pub const USBI_TRANSFER_IN_FLIGHT: usbi_transfer_state_flags = 1;
#[inline]
unsafe extern "C" fn usbi_mutex_static_unlock(mutex: *mut usbi_mutex_static_t) {
    pthread_mutex_unlock(mutex);
}
#[inline]
unsafe extern "C" fn usbi_mutex_destroy(mutex: *mut usbi_mutex_t) {
    pthread_mutex_destroy(mutex);
}
#[inline]
unsafe extern "C" fn usbi_mutex_unlock(mutex: *mut usbi_mutex_t) {
    pthread_mutex_unlock(mutex);
}
#[inline]
unsafe extern "C" fn usbi_mutex_lock(mutex: *mut usbi_mutex_t) {
    pthread_mutex_lock(mutex);
}
#[inline]
unsafe extern "C" fn usbi_mutex_static_lock(mutex: *mut usbi_mutex_static_t) {
    pthread_mutex_lock(mutex);
}
#[inline]
unsafe extern "C" fn usbi_mutex_init(mutex: *mut usbi_mutex_t) -> libc::c_int {
    return pthread_mutex_init(mutex, 0 as *const pthread_mutexattr_t);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn usbi_tls_key_get(key: usbi_tls_key_t) -> *mut libc::c_void {
    return pthread_getspecific(key);
}
#[inline]
unsafe extern "C" fn usbi_handling_events(ctx: *mut libusb_context) -> libc::c_int {
    return (usbi_tls_key_get((*ctx).event_handling_key) != 0 as *mut libc::c_void) as libc::c_int;
}
#[inline]
unsafe extern "C" fn usbi_pending_events(ctx: *mut libusb_context) -> libc::c_int {
    return ((*ctx).event_flags != 0
        || (*ctx).device_close != 0
        || !((*ctx).hotplug_msgs.next == &mut (*ctx).hotplug_msgs as *mut list_head)
        || !((*ctx).completed_transfers.next == &mut (*ctx).completed_transfers as *mut list_head))
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn list_del(mut entry: *mut list_head) {
    (*(*entry).next).prev = (*entry).prev;
    (*(*entry).prev).next = (*entry).next;
    (*entry).prev = 0 as *mut list_head;
    (*entry).next = (*entry).prev;
}
#[inline]
unsafe extern "C" fn list_init(mut entry: *mut list_head) {
    (*entry).next = entry;
    (*entry).prev = (*entry).next;
}
#[inline]
unsafe extern "C" fn list_add(mut entry: *mut list_head, mut head: *mut list_head) {
    (*entry).next = (*head).next;
    (*entry).prev = head;
    (*(*head).next).prev = entry;
    (*head).next = entry;
}
#[inline]
unsafe extern "C" fn usbi_get_context(ctx: *mut libusb_context) -> *mut libusb_context {
    return if !ctx.is_null() {
        ctx
    } else {
        usbi_default_context
    };
}
/* -*- Mode: C; indent-tabs-mode:t ; c-basic-offset:8 -*- */
/*
 * Core functions for libusb
 * Copyright © 2012-2013 Nathan Hjelm <hjelmn@cs.unm.edu>
 * Copyright © 2007-2008 Daniel Drake <dsd@gentoo.org>
 * Copyright © 2001 Johannes Erdfelt <johannes@erdfelt.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */
#[no_mangle]
pub static mut usbi_default_context: *mut libusb_context =
    0 as *const libusb_context as *mut libusb_context;
static mut libusb_version_internal: libusb_version = {
    let init = libusb_version {
        major: 1 as libc::c_int as uint16_t,
        minor: 0 as libc::c_int as uint16_t,
        micro: 23 as libc::c_int as uint16_t,
        nano: 11522 as libc::c_int as uint16_t,
        rc: b"\x00" as *const u8 as *const libc::c_char,
        describe: b"http://libusb.info\x00" as *const u8 as *const libc::c_char,
    };
    init
};
static mut default_context_refcnt: libc::c_int = 0;
static mut default_context_lock: usbi_mutex_static_t = pthread_mutex_t {
    __data: {
        let init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: 0 as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut timestamp_origin: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut log_handler: libusb_log_cb = None;
#[no_mangle]
pub static mut active_contexts_lock: usbi_mutex_static_t = pthread_mutex_t {
    __data: {
        let init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: 0 as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
#[no_mangle]
pub static mut active_contexts_list: list_head = list_head {
    prev: 0 as *const list_head as *mut list_head,
    next: 0 as *const list_head as *mut list_head,
};
unsafe extern "C" fn discovered_devs_alloc() -> *mut discovered_devs {
    let mut ret: *mut discovered_devs = malloc(
        (::std::mem::size_of::<discovered_devs>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ),
    ) as *mut discovered_devs;
    if !ret.is_null() {
        (*ret).len = 0 as libc::c_int as size_t;
        (*ret).capacity = 8 as libc::c_int as size_t
    }
    return ret;
}
unsafe extern "C" fn discovered_devs_free(discdevs: *mut discovered_devs) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*discdevs).len {
        libusb_unref_device(*(*discdevs).devices.as_mut_ptr().offset(i as isize));
        i = i.wrapping_add(1)
    }
    free(discdevs as *mut libc::c_void);
}
/* append a device to the discovered devices collection. may realloc itself,
 * returning new discdevs. returns NULL on realloc failure. */
#[no_mangle]
pub unsafe extern "C" fn discovered_devs_append(
    mut discdevs: *mut discovered_devs,
    dev: *mut libusb_device,
) -> *mut discovered_devs {
    let len: size_t = (*discdevs).len;
    let mut capacity: size_t = 0;
    let mut new_discdevs: *mut discovered_devs = 0 as *mut discovered_devs;
    /* if there is space, just append the device */
    if len < (*discdevs).capacity {
        let ref mut fresh0 = *(*discdevs).devices.as_mut_ptr().offset(len as isize);
        *fresh0 = libusb_ref_device(dev);
        (*discdevs).len = (*discdevs).len.wrapping_add(1);
        return discdevs;
    }
    /* exceeded capacity, need to grow */
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"discovered_devs_append\x00"))
            .as_ptr(),
        b"need to increase capacity\x00" as *const u8 as *const libc::c_char,
    );
    capacity = (*discdevs)
        .capacity
        .wrapping_add(8 as libc::c_int as libc::c_ulong);
    /* can't use usbi_reallocf here because in failure cases it would
     * free the existing discdevs without unreferencing its devices. */
    new_discdevs = realloc(
        discdevs as *mut libc::c_void,
        (::std::mem::size_of::<discovered_devs>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong).wrapping_mul(capacity),
        ),
    ) as *mut discovered_devs;
    if new_discdevs.is_null() {
        discovered_devs_free(discdevs);
        return 0 as *mut discovered_devs;
    }
    discdevs = new_discdevs;
    (*discdevs).capacity = capacity;
    let ref mut fresh1 = *(*discdevs).devices.as_mut_ptr().offset(len as isize);
    *fresh1 = libusb_ref_device(dev);
    (*discdevs).len = (*discdevs).len.wrapping_add(1);
    return discdevs;
}
/* Allocate a new device with a specific session ID. The returned device has
 * a reference count of 1. */
#[no_mangle]
pub unsafe extern "C" fn usbi_alloc_device(
    ctx: *mut libusb_context,
    session_id: libc::c_ulong,
) -> *mut libusb_device {
    let priv_size: size_t = usbi_backend.device_priv_size;
    let mut dev: *mut libusb_device = calloc(
        1 as libc::c_int as libc::c_ulong,
        ((::std::mem::size_of::<libusb_device>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        .wrapping_add(priv_size),
    ) as *mut libusb_device;
    let mut r: libc::c_int = 0;
    if dev.is_null() {
        return 0 as *mut libusb_device;
    }
    r = usbi_mutex_init(&mut (*dev).lock);
    if r != 0 {
        free(dev as *mut libc::c_void);
        return 0 as *mut libusb_device;
    }
    (*dev).ctx = ctx;
    (*dev).refcnt = 1 as libc::c_int;
    (*dev).session_data = session_id;
    (*dev).speed = LIBUSB_SPEED_UNKNOWN;
    if libusb_has_capability(LIBUSB_CAP_HAS_HOTPLUG as libc::c_int as uint32_t) == 0 {
        usbi_connect_device(dev);
    }
    return dev;
}
#[no_mangle]
pub unsafe extern "C" fn usbi_connect_device(mut dev: *mut libusb_device) {
    let ctx: *mut libusb_context = (*dev).ctx;
    (*dev).attached = 1 as libc::c_int;
    usbi_mutex_lock(&mut (*(*dev).ctx).usb_devs_lock);
    list_add(&mut (*dev).list, &mut (*(*dev).ctx).usb_devs);
    usbi_mutex_unlock(&mut (*(*dev).ctx).usb_devs_lock);
    /* Signal that an event has occurred for this device if we support hotplug AND
     * the hotplug message list is ready. This prevents an event from getting raised
     * during initial enumeration. */
    if libusb_has_capability(LIBUSB_CAP_HAS_HOTPLUG as libc::c_int as uint32_t) != 0
        && !(*(*dev).ctx).hotplug_msgs.next.is_null()
    {
        usbi_hotplug_notification(ctx, dev, LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED);
    };
}
#[no_mangle]
pub unsafe extern "C" fn usbi_disconnect_device(mut dev: *mut libusb_device) {
    let ctx: *mut libusb_context = (*dev).ctx;
    usbi_mutex_lock(&mut (*dev).lock);
    (*dev).attached = 0 as libc::c_int;
    usbi_mutex_unlock(&mut (*dev).lock);
    usbi_mutex_lock(&mut (*ctx).usb_devs_lock);
    list_del(&mut (*dev).list);
    usbi_mutex_unlock(&mut (*ctx).usb_devs_lock);
    /* Signal that an event has occurred for this device if we support hotplug AND
     * the hotplug message list is ready. This prevents an event from getting raised
     * during initial enumeration. libusb_handle_events will take care of dereferencing
     * the device. */
    if libusb_has_capability(LIBUSB_CAP_HAS_HOTPLUG as libc::c_int as uint32_t) != 0
        && !(*(*dev).ctx).hotplug_msgs.next.is_null()
    {
        usbi_hotplug_notification(ctx, dev, LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT);
    };
}
/* Perform some final sanity checks on a newly discovered device. If this
 * function fails (negative return code), the device should not be added
 * to the discovered device list. */
#[no_mangle]
pub unsafe extern "C" fn usbi_sanitize_device(dev: *mut libusb_device) -> libc::c_int {
    let mut num_configurations: uint8_t = 0;
    if (*dev).device_descriptor.bLength as libc::c_int != 18 as libc::c_int
        || (*dev).device_descriptor.bDescriptorType as libc::c_int
            != LIBUSB_DT_DEVICE as libc::c_int
    {
        usbi_log(
            (*dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"usbi_sanitize_device\x00"))
                .as_ptr(),
            b"invalid device descriptor\x00" as *const u8 as *const libc::c_char,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    num_configurations = (*dev).device_descriptor.bNumConfigurations;
    if num_configurations as libc::c_int > 8 as libc::c_int {
        usbi_log(
            (*dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"usbi_sanitize_device\x00"))
                .as_ptr(),
            b"too many configurations\x00" as *const u8 as *const libc::c_char,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    } else {
        if 0 as libc::c_int == num_configurations as libc::c_int {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                    b"usbi_sanitize_device\x00",
                ))
                .as_ptr(),
                b"zero configurations, maybe an unauthorized device\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return 0 as libc::c_int;
}
/* Examine libusb's internal list of known devices, looking for one with
 * a specific session ID. Returns the matching device if it was found, and
 * NULL otherwise. */
#[no_mangle]
pub unsafe extern "C" fn usbi_get_device_by_session_id(
    ctx: *mut libusb_context,
    session_id: libc::c_ulong,
) -> *mut libusb_device {
    let mut dev: *mut libusb_device = 0 as *mut libusb_device;
    let mut ret: *mut libusb_device = 0 as *mut libusb_device;
    usbi_mutex_lock(&mut (*ctx).usb_devs_lock);
    dev =
        ((*ctx).usb_devs.next as uintptr_t).wrapping_sub(72 as libc::c_ulong) as *mut libusb_device;
    while &mut (*dev).list as *mut list_head != &mut (*ctx).usb_devs as *mut list_head {
        if (*dev).session_data == session_id {
            ret = libusb_ref_device(dev);
            break;
        } else {
            dev = ((*dev).list.next as uintptr_t).wrapping_sub(72 as libc::c_ulong)
                as *mut libusb_device
        }
    }
    usbi_mutex_unlock(&mut (*ctx).usb_devs_lock);
    return ret;
}
/* * @ingroup libusb_dev
 * Returns a list of USB devices currently attached to the system. This is
 * your entry point into finding a USB device to operate.
 *
 * You are expected to unreference all the devices when you are done with
 * them, and then free the list with libusb_free_device_list(). Note that
 * libusb_free_device_list() can unref all the devices for you. Be careful
 * not to unreference a device you are about to open until after you have
 * opened it.
 *
 * This return value of this function indicates the number of devices in
 * the resultant list. The list is actually one element larger, as it is
 * NULL-terminated.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param list output location for a list of devices. Must be later freed with
 * libusb_free_device_list().
 * \returns the number of devices in the outputted list, or any
 * \ref libusb_error according to errors encountered by the backend.
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_device_list(
    mut ctx: *mut libusb_context,
    list: *mut *mut *mut libusb_device,
) -> ssize_t {
    let mut discdevs: *mut discovered_devs = discovered_devs_alloc();
    let mut ret: *mut *mut libusb_device = 0 as *mut *mut libusb_device;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut i: ssize_t = 0;
    let mut len: ssize_t = 0;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"libusb_get_device_list\x00"))
            .as_ptr(),
        b" \x00" as *const u8 as *const libc::c_char,
    );
    if discdevs.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int as ssize_t;
    }
    ctx = usbi_get_context(ctx);
    if libusb_has_capability(LIBUSB_CAP_HAS_HOTPLUG as libc::c_int as uint32_t) != 0 {
        /* backend provides hotplug support */
        let mut dev: *mut libusb_device = 0 as *mut libusb_device;
        if usbi_backend.hotplug_poll.is_some() {
            usbi_backend
                .hotplug_poll
                .expect("non-null function pointer")();
        }
        usbi_mutex_lock(&mut (*ctx).usb_devs_lock);
        dev = ((*ctx).usb_devs.next as uintptr_t).wrapping_sub(72 as libc::c_ulong)
            as *mut libusb_device;
        while &mut (*dev).list as *mut list_head != &mut (*ctx).usb_devs as *mut list_head {
            discdevs = discovered_devs_append(discdevs, dev);
            if discdevs.is_null() {
                r = LIBUSB_ERROR_NO_MEM as libc::c_int;
                break;
            } else {
                dev = ((*dev).list.next as uintptr_t).wrapping_sub(72 as libc::c_ulong)
                    as *mut libusb_device
            }
        }
        usbi_mutex_unlock(&mut (*ctx).usb_devs_lock);
    } else {
        /* backend does not provide hotplug support */
        r = usbi_backend
            .get_device_list
            .expect("non-null function pointer")(ctx, &mut discdevs)
    }
    if r < 0 as libc::c_int {
        len = r as ssize_t
    } else {
        /* convert discovered_devs into a list */
        len = (*discdevs).len as ssize_t;
        ret = calloc(
            (len as size_t).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ::std::mem::size_of::<*mut libusb_device>() as libc::c_ulong,
        ) as *mut *mut libusb_device;
        if ret.is_null() {
            len = LIBUSB_ERROR_NO_MEM as libc::c_int as ssize_t
        } else {
            let ref mut fresh2 = *ret.offset(len as isize);
            *fresh2 = 0 as *mut libusb_device;
            i = 0 as libc::c_int as ssize_t;
            while i < len {
                let dev_0: *mut libusb_device =
                    *(*discdevs).devices.as_mut_ptr().offset(i as isize);
                let ref mut fresh3 = *ret.offset(i as isize);
                *fresh3 = libusb_ref_device(dev_0);
                i += 1
            }
            *list = ret
        }
    }
    if !discdevs.is_null() {
        discovered_devs_free(discdevs);
    }
    return len;
}
/* * \ingroup libusb_dev
 * Frees a list of devices previously discovered using
 * libusb_get_device_list(). If the unref_devices parameter is set, the
 * reference count of each device in the list is decremented by 1.
 * \param list the list to free
 * \param unref_devices whether to unref the devices in the list
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_free_device_list(
    list: *mut *mut libusb_device,
    unref_devices: libc::c_int,
) {
    if list.is_null() {
        return;
    }
    if unref_devices != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut dev: *mut libusb_device = 0 as *mut libusb_device;
        loop {
            let fresh4 = i;
            i = i + 1;
            dev = *list.offset(fresh4 as isize);
            if dev.is_null() {
                break;
            }
            libusb_unref_device(dev);
        }
    }
    free(list as *mut libc::c_void);
}
/* * \ingroup libusb_dev
 * Get the number of the bus that a device is connected to.
 * \param dev a device
 * \returns the bus number
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_bus_number(dev: *mut libusb_device) -> uint8_t {
    return (*dev).bus_number;
}
/* * \ingroup libusb_dev
 * Get the number of the port that a device is connected to.
 * Unless the OS does something funky, or you are hot-plugging USB extension cards,
 * the port number returned by this call is usually guaranteed to be uniquely tied
 * to a physical port, meaning that different devices plugged on the same physical
 * port should return the same port number.
 *
 * But outside of this, there is no guarantee that the port number returned by this
 * call will remain the same, or even match the order in which ports have been
 * numbered by the HUB/HCD manufacturer.
 *
 * \param dev a device
 * \returns the port number (0 if not available)
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_port_number(dev: *mut libusb_device) -> uint8_t {
    return (*dev).port_number;
}
/* * \ingroup libusb_dev
 * Get the list of all port numbers from root for the specified device
 *
 * Since version 1.0.16, \ref LIBUSB_API_VERSION >= 0x01000102
 * \param dev a device
 * \param port_numbers the array that should contain the port numbers
 * \param port_numbers_len the maximum length of the array. As per the USB 3.0
 * specs, the current maximum limit for the depth is 7.
 * \returns the number of elements filled
 * \returns LIBUSB_ERROR_OVERFLOW if the array is too small
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_port_numbers(
    mut dev: *mut libusb_device,
    port_numbers: *mut uint8_t,
    port_numbers_len: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = port_numbers_len;
    let ctx: *mut libusb_context = (*dev).ctx;
    if port_numbers_len <= 0 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    // HCDs can be listed as devices with port #0
    while !dev.is_null() && (*dev).port_number as libc::c_int != 0 as libc::c_int {
        i -= 1;
        if i < 0 as libc::c_int {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"libusb_get_port_numbers\x00",
                ))
                .as_ptr(),
                b"port numbers array is too small\x00" as *const u8 as *const libc::c_char,
            );
            return LIBUSB_ERROR_OVERFLOW as libc::c_int;
        }
        *port_numbers.offset(i as isize) = (*dev).port_number;
        dev = (*dev).parent_dev
    }
    if i < port_numbers_len {
        memmove(
            port_numbers as *mut libc::c_void,
            &mut *port_numbers.offset(i as isize) as *mut uint8_t as *const libc::c_void,
            (port_numbers_len - i) as libc::c_ulong,
        );
    }
    return port_numbers_len - i;
}
/* * \ingroup libusb_dev
 * \deprecated Please use \ref libusb_get_port_numbers() instead.
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_port_path(
    _ctx: *mut libusb_context,
    dev: *mut libusb_device,
    port_numbers: *mut uint8_t,
    port_numbers_len: uint8_t,
) -> libc::c_int {
    return libusb_get_port_numbers(dev, port_numbers, port_numbers_len as libc::c_int);
}
/* * \ingroup libusb_dev
 * Get the the parent from the specified device.
 * \param dev a device
 * \returns the device parent or NULL if not available
 * You should issue a \ref libusb_get_device_list() before calling this
 * function and make sure that you only access the parent before issuing
 * \ref libusb_free_device_list(). The reason is that libusb currently does
 * not maintain a permanent list of device instances, and therefore can
 * only guarantee that parents are fully instantiated within a
 * libusb_get_device_list() - libusb_free_device_list() block.
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_parent(dev: *mut libusb_device) -> *mut libusb_device {
    return (*dev).parent_dev;
}
/* * \ingroup libusb_dev
 * Get the address of the device on the bus it is connected to.
 * \param dev a device
 * \returns the device address
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_device_address(dev: *mut libusb_device) -> uint8_t {
    return (*dev).device_address;
}
/* * \ingroup libusb_dev
 * Get the negotiated connection speed for a device.
 * \param dev a device
 * \returns a \ref libusb_speed code, where LIBUSB_SPEED_UNKNOWN means that
 * the OS doesn't know or doesn't support returning the negotiated speed.
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_device_speed(dev: *mut libusb_device) -> libc::c_int {
    return (*dev).speed as libc::c_int;
}
unsafe extern "C" fn find_endpoint(
    config: *mut libusb_config_descriptor,
    endpoint: libc::c_uchar,
) -> *const libusb_endpoint_descriptor {
    let mut iface_idx: libc::c_int = 0;
    iface_idx = 0 as libc::c_int;
    while iface_idx < (*config).bNumInterfaces as libc::c_int {
        let iface: *const libusb_interface =
            &*(*config).interface.offset(iface_idx as isize) as *const libusb_interface;
        let mut altsetting_idx: libc::c_int = 0;
        altsetting_idx = 0 as libc::c_int;
        while altsetting_idx < (*iface).num_altsetting {
            let altsetting: *const libusb_interface_descriptor =
                &*(*iface).altsetting.offset(altsetting_idx as isize)
                    as *const libusb_interface_descriptor;
            let mut ep_idx: libc::c_int = 0;
            ep_idx = 0 as libc::c_int;
            while ep_idx < (*altsetting).bNumEndpoints as libc::c_int {
                let ep: *const libusb_endpoint_descriptor =
                    &*(*altsetting).endpoint.offset(ep_idx as isize)
                        as *const libusb_endpoint_descriptor;
                if (*ep).bEndpointAddress as libc::c_int == endpoint as libc::c_int {
                    return ep;
                }
                ep_idx += 1
            }
            altsetting_idx += 1
        }
        iface_idx += 1
    }
    return 0 as *const libusb_endpoint_descriptor;
}
/* * \ingroup libusb_dev
 * Convenience function to retrieve the wMaxPacketSize value for a particular
 * endpoint in the active device configuration.
 *
 * This function was originally intended to be of assistance when setting up
 * isochronous transfers, but a design mistake resulted in this function
 * instead. It simply returns the wMaxPacketSize value without considering
 * its contents. If you're dealing with isochronous transfers, you probably
 * want libusb_get_max_iso_packet_size() instead.
 *
 * \param dev a device
 * \param endpoint address of the endpoint in question
 * \returns the wMaxPacketSize value
 * \returns LIBUSB_ERROR_NOT_FOUND if the endpoint does not exist
 * \returns LIBUSB_ERROR_OTHER on other failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_max_packet_size(
    dev: *mut libusb_device,
    endpoint: libc::c_uchar,
) -> libc::c_int {
    let mut config: *mut libusb_config_descriptor = 0 as *mut libusb_config_descriptor;
    let mut ep: *const libusb_endpoint_descriptor = 0 as *const libusb_endpoint_descriptor;
    let mut r: libc::c_int = 0;
    r = libusb_get_active_config_descriptor(dev, &mut config);
    if r < 0 as libc::c_int {
        usbi_log(
            (*dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                b"libusb_get_max_packet_size\x00",
            ))
            .as_ptr(),
            b"could not retrieve active config descriptor\x00" as *const u8 as *const libc::c_char,
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    ep = find_endpoint(config, endpoint);
    if ep.is_null() {
        r = LIBUSB_ERROR_NOT_FOUND as libc::c_int
    } else {
        r = (*ep).wMaxPacketSize as libc::c_int
    }
    libusb_free_config_descriptor(config);
    return r;
}
/* * \ingroup libusb_dev
 * Calculate the maximum packet size which a specific endpoint is capable is
 * sending or receiving in the duration of 1 microframe
 *
 * Only the active configuration is examined. The calculation is based on the
 * wMaxPacketSize field in the endpoint descriptor as described in section
 * 9.6.6 in the USB 2.0 specifications.
 *
 * If acting on an isochronous or interrupt endpoint, this function will
 * multiply the value found in bits 0:10 by the number of transactions per
 * microframe (determined by bits 11:12). Otherwise, this function just
 * returns the numeric value found in bits 0:10. For USB 3.0 device, it
 * will attempts to retrieve the Endpoint Companion Descriptor to return
 * wBytesPerInterval.
 *
 * This function is useful for setting up isochronous transfers, for example
 * you might pass the return value from this function to
 * libusb_set_iso_packet_lengths() in order to set the length field of every
 * isochronous packet in a transfer.
 *
 * Since v1.0.3.
 *
 * \param dev a device
 * \param endpoint address of the endpoint in question
 * \returns the maximum packet size which can be sent/received on this endpoint
 * \returns LIBUSB_ERROR_NOT_FOUND if the endpoint does not exist
 * \returns LIBUSB_ERROR_OTHER on other failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_max_iso_packet_size(
    dev: *mut libusb_device,
    endpoint: libc::c_uchar,
) -> libc::c_int {
    let mut config: *mut libusb_config_descriptor = 0 as *mut libusb_config_descriptor;
    let mut ep: *const libusb_endpoint_descriptor = 0 as *const libusb_endpoint_descriptor;
    let mut ss_ep_cmp: *mut libusb_ss_endpoint_companion_descriptor =
        0 as *mut libusb_ss_endpoint_companion_descriptor;
    let mut ep_type: libusb_endpoint_transfer_type = LIBUSB_ENDPOINT_TRANSFER_TYPE_CONTROL;
    let mut val: uint16_t = 0;
    let mut r: libc::c_int = 0;
    let mut speed: libc::c_int = 0;
    r = libusb_get_active_config_descriptor(dev, &mut config);
    if r < 0 as libc::c_int {
        usbi_log(
            (*dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"libusb_get_max_iso_packet_size\x00",
            ))
            .as_ptr(),
            b"could not retrieve active config descriptor\x00" as *const u8 as *const libc::c_char,
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    ep = find_endpoint(config, endpoint);
    if ep.is_null() {
        r = LIBUSB_ERROR_NOT_FOUND as libc::c_int
    } else {
        speed = libusb_get_device_speed(dev);
        if speed >= LIBUSB_SPEED_SUPER as libc::c_int {
            r = libusb_get_ss_endpoint_companion_descriptor((*dev).ctx, ep, &mut ss_ep_cmp);
            if r == LIBUSB_SUCCESS as libc::c_int {
                r = (*ss_ep_cmp).wBytesPerInterval as libc::c_int;
                libusb_free_ss_endpoint_companion_descriptor(ss_ep_cmp);
            }
        }
        /* If the device isn't a SuperSpeed device or retrieving the SS endpoint didn't worked. */
        if speed < LIBUSB_SPEED_SUPER as libc::c_int || r < 0 as libc::c_int {
            val = (*ep).wMaxPacketSize;
            ep_type = ((*ep).bmAttributes as libc::c_int & 0x3 as libc::c_int)
                as libusb_endpoint_transfer_type;
            r = val as libc::c_int & 0x7ff as libc::c_int;
            if ep_type as libc::c_uint
                == LIBUSB_ENDPOINT_TRANSFER_TYPE_ISOCHRONOUS as libc::c_int as libc::c_uint
                || ep_type as libc::c_uint
                    == LIBUSB_ENDPOINT_TRANSFER_TYPE_INTERRUPT as libc::c_int as libc::c_uint
            {
                r *= 1 as libc::c_int + (val as libc::c_int >> 11 as libc::c_int & 3 as libc::c_int)
            }
        }
    }
    libusb_free_config_descriptor(config);
    return r;
}
/* * \ingroup libusb_dev
 * Increment the reference count of a device.
 * \param dev the device to reference
 * \returns the same device
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_ref_device(mut dev: *mut libusb_device) -> *mut libusb_device {
    usbi_mutex_lock(&mut (*dev).lock);
    (*dev).refcnt += 1;
    usbi_mutex_unlock(&mut (*dev).lock);
    return dev;
}
/* * \ingroup libusb_dev
 * Decrement the reference count of a device. If the decrement operation
 * causes the reference count to reach zero, the device shall be destroyed.
 * \param dev the device to unreference
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_unref_device(mut dev: *mut libusb_device) {
    let mut refcnt: libc::c_int = 0;
    if dev.is_null() {
        return;
    }
    usbi_mutex_lock(&mut (*dev).lock);
    (*dev).refcnt -= 1;
    refcnt = (*dev).refcnt;
    usbi_mutex_unlock(&mut (*dev).lock);
    if refcnt == 0 as libc::c_int {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"libusb_unref_device\x00"))
                .as_ptr(),
            b"destroy device %d.%d\x00" as *const u8 as *const libc::c_char,
            (*dev).bus_number as libc::c_int,
            (*dev).device_address as libc::c_int,
        );
        libusb_unref_device((*dev).parent_dev);
        if usbi_backend.destroy_device.is_some() {
            usbi_backend
                .destroy_device
                .expect("non-null function pointer")(dev);
        }
        if libusb_has_capability(LIBUSB_CAP_HAS_HOTPLUG as libc::c_int as uint32_t) == 0 {
            /* backend does not support hotplug */
            usbi_disconnect_device(dev);
        }
        usbi_mutex_destroy(&mut (*dev).lock);
        free(dev as *mut libc::c_void);
    };
}
/*
 * Signal the event pipe so that the event handling thread will be
 * interrupted to process an internal event.
 */
#[no_mangle]
pub unsafe extern "C" fn usbi_signal_event(ctx: *mut libusb_context) -> libc::c_int {
    let mut dummy: libc::c_uchar = 1 as libc::c_int as libc::c_uchar;
    let mut r: ssize_t = 0;
    /* write some data on event pipe to interrupt event handlers */
    r = write(
        (*ctx).event_pipe[1 as libc::c_int as usize],
        &mut dummy as *mut libc::c_uchar as *const libc::c_void,
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
    );
    if r as libc::c_ulong != ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_WARNING,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"usbi_signal_event\x00"))
                .as_ptr(),
            b"internal signalling write failed\x00" as *const u8 as *const libc::c_char,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    return 0 as libc::c_int;
}
/*
 * Clear the event pipe so that the event handling will no longer be
 * interrupted.
 */
#[no_mangle]
pub unsafe extern "C" fn usbi_clear_event(ctx: *mut libusb_context) -> libc::c_int {
    let mut dummy: libc::c_uchar = 0;
    let mut r: ssize_t = 0;
    /* read some data on event pipe to clear it */
    r = read(
        (*ctx).event_pipe[0 as libc::c_int as usize],
        &mut dummy as *mut libc::c_uchar as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
    );
    if r as libc::c_ulong != ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_WARNING,
            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"usbi_clear_event\x00"))
                .as_ptr(),
            b"internal signalling read failed\x00" as *const u8 as *const libc::c_char,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    return 0 as libc::c_int;
}
/* * \ingroup libusb_dev
 * Wrap a platform-specific system device handle and obtain a libusb device
 * handle for the underlying device. The handle allows you to use libusb to
 * perform I/O on the device in question.
 *
 * On Linux, the system device handle must be a valid file descriptor opened
 * on the device node.
 *
 * The system device handle must remain open until libusb_close() is called.
 * The system device handle will not be closed by libusb_close().
 *
 * Internally, this function creates a temporary device and makes it
 * available to you through libusb_get_device(). This device is destroyed
 * during libusb_close(). The device shall not be opened through libusb_open().
 *
 * This is a non-blocking function; no requests are sent over the bus.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param sys_dev the platform-specific system device handle
 * \param dev_handle output location for the returned device handle pointer. Only
 * populated when the return code is 0.
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NO_MEM on memory allocation failure
 * \returns LIBUSB_ERROR_ACCESS if the user has insufficient permissions
 * \returns LIBUSB_ERROR_NOT_SUPPORTED if the operation is not supported on this
 * platform
 * \returns another LIBUSB_ERROR code on other failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_wrap_sys_device(
    mut ctx: *mut libusb_context,
    sys_dev: intptr_t,
    dev_handle: *mut *mut libusb_device_handle,
) -> libc::c_int {
    let mut _dev_handle: *mut libusb_device_handle = 0 as *mut libusb_device_handle;
    let priv_size: size_t = usbi_backend.device_handle_priv_size;
    let mut r: libc::c_int = 0;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"libusb_wrap_sys_device\x00"))
            .as_ptr(),
        b"wrap_sys_device %p\x00" as *const u8 as *const libc::c_char,
        sys_dev as *mut libc::c_void,
    );
    ctx = usbi_get_context(ctx);
    if usbi_backend.wrap_sys_device.is_none() {
        return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    }
    _dev_handle = calloc(
        1 as libc::c_int as libc::c_ulong,
        ((::std::mem::size_of::<libusb_device_handle>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        .wrapping_add(priv_size),
    ) as *mut libusb_device_handle;
    if _dev_handle.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    r = usbi_mutex_init(&mut (*_dev_handle).lock);
    if r != 0 {
        free(_dev_handle as *mut libc::c_void);
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    r = usbi_backend
        .wrap_sys_device
        .expect("non-null function pointer")(ctx, _dev_handle, sys_dev);
    if r < 0 as libc::c_int {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"libusb_wrap_sys_device\x00",
            ))
            .as_ptr(),
            b"wrap_sys_device %p returns %d\x00" as *const u8 as *const libc::c_char,
            sys_dev as *mut libc::c_void,
            r,
        );
        usbi_mutex_destroy(&mut (*_dev_handle).lock);
        free(_dev_handle as *mut libc::c_void);
        return r;
    }
    usbi_mutex_lock(&mut (*ctx).open_devs_lock);
    list_add(&mut (*_dev_handle).list, &mut (*ctx).open_devs);
    usbi_mutex_unlock(&mut (*ctx).open_devs_lock);
    *dev_handle = _dev_handle;
    return 0 as libc::c_int;
}
/* * \ingroup libusb_dev
 * Open a device and obtain a device handle. A handle allows you to perform
 * I/O on the device in question.
 *
 * Internally, this function adds a reference to the device and makes it
 * available to you through libusb_get_device(). This reference is removed
 * during libusb_close().
 *
 * This is a non-blocking function; no requests are sent over the bus.
 *
 * \param dev the device to open
 * \param dev_handle output location for the returned device handle pointer. Only
 * populated when the return code is 0.
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NO_MEM on memory allocation failure
 * \returns LIBUSB_ERROR_ACCESS if the user has insufficient permissions
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns another LIBUSB_ERROR code on other failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_open(
    dev: *mut libusb_device,
    dev_handle: *mut *mut libusb_device_handle,
) -> libc::c_int {
    let ctx: *mut libusb_context = (*dev).ctx;
    let mut _dev_handle: *mut libusb_device_handle = 0 as *mut libusb_device_handle;
    let priv_size: size_t = usbi_backend.device_handle_priv_size;
    let mut r: libc::c_int = 0;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"libusb_open\x00")).as_ptr(),
        b"open %d.%d\x00" as *const u8 as *const libc::c_char,
        (*dev).bus_number as libc::c_int,
        (*dev).device_address as libc::c_int,
    );
    if (*dev).attached == 0 {
        return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
    }
    _dev_handle = calloc(
        1 as libc::c_int as libc::c_ulong,
        ((::std::mem::size_of::<libusb_device_handle>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        .wrapping_add(priv_size),
    ) as *mut libusb_device_handle;
    if _dev_handle.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    r = usbi_mutex_init(&mut (*_dev_handle).lock);
    if r != 0 {
        free(_dev_handle as *mut libc::c_void);
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    (*_dev_handle).dev = libusb_ref_device(dev);
    r = usbi_backend.open.expect("non-null function pointer")(_dev_handle);
    if r < 0 as libc::c_int {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"libusb_open\x00")).as_ptr(),
            b"open %d.%d returns %d\x00" as *const u8 as *const libc::c_char,
            (*dev).bus_number as libc::c_int,
            (*dev).device_address as libc::c_int,
            r,
        );
        libusb_unref_device(dev);
        usbi_mutex_destroy(&mut (*_dev_handle).lock);
        free(_dev_handle as *mut libc::c_void);
        return r;
    }
    usbi_mutex_lock(&mut (*ctx).open_devs_lock);
    list_add(&mut (*_dev_handle).list, &mut (*ctx).open_devs);
    usbi_mutex_unlock(&mut (*ctx).open_devs_lock);
    *dev_handle = _dev_handle;
    return 0 as libc::c_int;
}
/* * \ingroup libusb_dev
 * Convenience function for finding a device with a particular
 * <tt>idVendor</tt>/<tt>idProduct</tt> combination. This function is intended
 * for those scenarios where you are using libusb to knock up a quick test
 * application - it allows you to avoid calling libusb_get_device_list() and
 * worrying about traversing/freeing the list.
 *
 * This function has limitations and is hence not intended for use in real
 * applications: if multiple devices have the same IDs it will only
 * give you the first one, etc.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param vendor_id the idVendor value to search for
 * \param product_id the idProduct value to search for
 * \returns a device handle for the first found device, or NULL on error
 * or if the device could not be found. */
#[no_mangle]
pub unsafe extern "C" fn libusb_open_device_with_vid_pid(
    ctx: *mut libusb_context,
    vendor_id: uint16_t,
    product_id: uint16_t,
) -> *mut libusb_device_handle {
    let current_block: u64;
    let mut devs: *mut *mut libusb_device = 0 as *mut *mut libusb_device;
    let mut found: *mut libusb_device = 0 as *mut libusb_device;
    let mut dev: *mut libusb_device = 0 as *mut libusb_device;
    let mut dev_handle: *mut libusb_device_handle = 0 as *mut libusb_device_handle;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut r: libc::c_int = 0;
    if libusb_get_device_list(ctx, &mut devs) < 0 as libc::c_int as libc::c_long {
        return 0 as *mut libusb_device_handle;
    }
    loop {
        let fresh5 = i;
        i = i.wrapping_add(1);
        dev = *devs.offset(fresh5 as isize);
        if dev.is_null() {
            current_block = 7651349459974463963;
            break;
        }
        let mut desc: libusb_device_descriptor = libusb_device_descriptor {
            bLength: 0,
            bDescriptorType: 0,
            bcdUSB: 0,
            bDeviceClass: 0,
            bDeviceSubClass: 0,
            bDeviceProtocol: 0,
            bMaxPacketSize0: 0,
            idVendor: 0,
            idProduct: 0,
            bcdDevice: 0,
            iManufacturer: 0,
            iProduct: 0,
            iSerialNumber: 0,
            bNumConfigurations: 0,
        };
        r = libusb_get_device_descriptor(dev, &mut desc);
        if r < 0 as libc::c_int {
            current_block = 13636304053406529094;
            break;
        }
        if !(desc.idVendor as libc::c_int == vendor_id as libc::c_int
            && desc.idProduct as libc::c_int == product_id as libc::c_int)
        {
            continue;
        }
        found = dev;
        current_block = 7651349459974463963;
        break;
    }
    match current_block {
        7651349459974463963 => {
            if !found.is_null() {
                r = libusb_open(found, &mut dev_handle);
                if r < 0 as libc::c_int {
                    dev_handle = 0 as *mut libusb_device_handle
                }
            }
        }
        _ => {}
    }
    libusb_free_device_list(devs, 1 as libc::c_int);
    return dev_handle;
}
unsafe extern "C" fn do_close(ctx: *mut libusb_context, dev_handle: *mut libusb_device_handle) {
    let mut itransfer: *mut usbi_transfer = 0 as *mut usbi_transfer;
    let mut tmp: *mut usbi_transfer = 0 as *mut usbi_transfer;
    /* remove any transfers in flight that are for this device */
    usbi_mutex_lock(&mut (*ctx).flying_transfers_lock);
    /* safe iteration because transfers may be being deleted */
    itransfer = ((*ctx).flying_transfers.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
        as *mut usbi_transfer;
    tmp = ((*itransfer).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
        as *mut usbi_transfer;
    while &mut (*itransfer).list as *mut list_head != &mut (*ctx).flying_transfers as *mut list_head
    {
        let mut transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
            ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
                (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
        ) as *mut libusb_transfer;
        if !((*transfer).dev_handle != dev_handle) {
            usbi_mutex_lock(&mut (*itransfer).lock);
            if (*itransfer).state_flags
                & USBI_TRANSFER_DEVICE_DISAPPEARED as libc::c_int as libc::c_uint
                == 0
            {
                usbi_log(ctx, LIBUSB_LOG_LEVEL_ERROR,
                         (*::std::mem::transmute::<&[u8; 9],
                                                   &[libc::c_char; 9]>(b"do_close\x00")).as_ptr(),
                         b"Device handle closed while transfer was still being processed, but the device is still connected as far as we know\x00"
                             as *const u8 as *const libc::c_char);
                if (*itransfer).state_flags
                    & USBI_TRANSFER_CANCELLING as libc::c_int as libc::c_uint
                    != 0
                {
                    usbi_log(ctx, LIBUSB_LOG_LEVEL_WARNING,
                             (*::std::mem::transmute::<&[u8; 9],
                                                       &[libc::c_char; 9]>(b"do_close\x00")).as_ptr(),
                             b"A cancellation for an in-flight transfer hasn\'t completed but closing the device handle\x00"
                                 as *const u8 as *const libc::c_char);
                } else {
                    usbi_log(ctx, LIBUSB_LOG_LEVEL_ERROR,
                             (*::std::mem::transmute::<&[u8; 9],
                                                       &[libc::c_char; 9]>(b"do_close\x00")).as_ptr(),
                             b"A cancellation hasn\'t even been scheduled on the transfer for which the device is closing\x00"
                                 as *const u8 as *const libc::c_char);
                }
            }
            usbi_mutex_unlock(&mut (*itransfer).lock);
            /* remove from the list of in-flight transfers and make sure
             * we don't accidentally use the device handle in the future
             * (or that such accesses will be easily caught and identified as a crash)
             */
            list_del(&mut (*itransfer).list);
            (*transfer).dev_handle = 0 as *mut libusb_device_handle;
            /* it is up to the user to free up the actual transfer struct.  this is
             * just making sure that we don't attempt to process the transfer after
             * the device handle is invalid
             */
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 9], &[libc::c_char; 9]>(b"do_close\x00")).as_ptr(),
                b"Removed transfer %p from the in-flight list because device handle %p closed\x00"
                    as *const u8 as *const libc::c_char,
                transfer,
                dev_handle,
            );
        }
        itransfer = tmp;
        tmp = ((*tmp).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong) as *mut usbi_transfer
    }
    usbi_mutex_unlock(&mut (*ctx).flying_transfers_lock);
    usbi_mutex_lock(&mut (*ctx).open_devs_lock);
    list_del(&mut (*dev_handle).list);
    usbi_mutex_unlock(&mut (*ctx).open_devs_lock);
    usbi_backend.close.expect("non-null function pointer")(dev_handle);
    libusb_unref_device((*dev_handle).dev);
    usbi_mutex_destroy(&mut (*dev_handle).lock);
    free(dev_handle as *mut libc::c_void);
}
/* * \ingroup libusb_dev
 * Close a device handle. Should be called on all open handles before your
 * application exits.
 *
 * Internally, this function destroys the reference that was added by
 * libusb_open() on the given device.
 *
 * This is a non-blocking function; no requests are sent over the bus.
 *
 * \param dev_handle the device handle to close
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_close(dev_handle: *mut libusb_device_handle) {
    let mut ctx: *mut libusb_context = 0 as *mut libusb_context;
    let mut handling_events: libc::c_int = 0;
    let mut pending_events: libc::c_int = 0;
    if dev_handle.is_null() {
        return;
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"libusb_close\x00")).as_ptr(),
        b" \x00" as *const u8 as *const libc::c_char,
    );
    ctx = (*(*dev_handle).dev).ctx;
    handling_events = usbi_handling_events(ctx);
    /* Similarly to libusb_open(), we want to interrupt all event handlers
     * at this point. More importantly, we want to perform the actual close of
     * the device while holding the event handling lock (preventing any other
     * thread from doing event handling) because we will be removing a file
     * descriptor from the polling loop. If this is being called by the current
     * event handler, we can bypass the interruption code because we already
     * hold the event handling lock. */
    if handling_events == 0 {
        /* Record that we are closing a device.
         * Only signal an event if there are no prior pending events. */
        usbi_mutex_lock(&mut (*ctx).event_data_lock);
        pending_events = usbi_pending_events(ctx);
        (*ctx).device_close = (*ctx).device_close.wrapping_add(1);
        if pending_events == 0 {
            usbi_signal_event(ctx);
        }
        usbi_mutex_unlock(&mut (*ctx).event_data_lock);
        /* take event handling lock */
        libusb_lock_events(ctx);
    }
    /* Close the device */
    do_close(ctx, dev_handle);
    if handling_events == 0 {
        /* We're done with closing this device.
         * Clear the event pipe if there are no further pending events. */
        usbi_mutex_lock(&mut (*ctx).event_data_lock);
        (*ctx).device_close = (*ctx).device_close.wrapping_sub(1);
        pending_events = usbi_pending_events(ctx);
        if pending_events == 0 {
            usbi_clear_event(ctx);
        }
        usbi_mutex_unlock(&mut (*ctx).event_data_lock);
        /* Release event handling lock and wake up event waiters */
        libusb_unlock_events(ctx);
    };
}
/* * \ingroup libusb_dev
 * Get the underlying device for a device handle. This function does not modify
 * the reference count of the returned device, so do not feel compelled to
 * unreference it when you are done.
 * \param dev_handle a device handle
 * \returns the underlying device
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_device(
    dev_handle: *mut libusb_device_handle,
) -> *mut libusb_device {
    return (*dev_handle).dev;
}
/* * \ingroup libusb_dev
 * Determine the bConfigurationValue of the currently active configuration.
 *
 * You could formulate your own control request to obtain this information,
 * but this function has the advantage that it may be able to retrieve the
 * information from operating system caches (no I/O involved).
 *
 * If the OS does not cache this information, then this function will block
 * while a control transfer is submitted to retrieve the information.
 *
 * This function will return a value of 0 in the <tt>config</tt> output
 * parameter if the device is in unconfigured state.
 *
 * \param dev_handle a device handle
 * \param config output location for the bConfigurationValue of the active
 * configuration (only valid for return code 0)
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns another LIBUSB_ERROR code on other failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_configuration(
    dev_handle: *mut libusb_device_handle,
    config: *mut libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    let mut tmp: uint8_t = 0 as libc::c_int as uint8_t;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"libusb_get_configuration\x00"))
            .as_ptr(),
        b" \x00" as *const u8 as *const libc::c_char,
    );
    if usbi_backend.get_configuration.is_some() {
        r = usbi_backend
            .get_configuration
            .expect("non-null function pointer")(dev_handle, &mut tmp)
    }
    if r == LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"libusb_get_configuration\x00",
            ))
            .as_ptr(),
            b"falling back to control message\x00" as *const u8 as *const libc::c_char,
        );
        r = libusb_control_transfer(
            dev_handle,
            LIBUSB_ENDPOINT_IN as libc::c_int as uint8_t,
            LIBUSB_REQUEST_GET_CONFIGURATION as libc::c_int as uint8_t,
            0 as libc::c_int as uint16_t,
            0 as libc::c_int as uint16_t,
            &mut tmp,
            1 as libc::c_int as uint16_t,
            1000 as libc::c_int as libc::c_uint,
        );
        if r == 1 as libc::c_int {
            r = 0 as libc::c_int
        } else if r == 0 as libc::c_int {
            usbi_log(
                (*(*dev_handle).dev).ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                    b"libusb_get_configuration\x00",
                ))
                .as_ptr(),
                b"zero bytes returned in ctrl transfer?\x00" as *const u8 as *const libc::c_char,
            );
            r = LIBUSB_ERROR_IO as libc::c_int
        } else {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                    b"libusb_get_configuration\x00",
                ))
                .as_ptr(),
                b"control failed, error %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
        }
    }
    if r == 0 as libc::c_int {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"libusb_get_configuration\x00",
            ))
            .as_ptr(),
            b"active config %u\x00" as *const u8 as *const libc::c_char,
            tmp as libc::c_int,
        );
        *config = tmp as libc::c_int
    }
    return r;
}
/* * \ingroup libusb_dev
 * Set the active configuration for a device.
 *
 * The operating system may or may not have already set an active
 * configuration on the device. It is up to your application to ensure the
 * correct configuration is selected before you attempt to claim interfaces
 * and perform other operations.
 *
 * If you call this function on a device already configured with the selected
 * configuration, then this function will act as a lightweight device reset:
 * it will issue a SET_CONFIGURATION request using the current configuration,
 * causing most USB-related device state to be reset (altsetting reset to zero,
 * endpoint halts cleared, toggles reset).
 *
 * You cannot change/reset configuration if your application has claimed
 * interfaces. It is advised to set the desired configuration before claiming
 * interfaces.
 *
 * Alternatively you can call libusb_release_interface() first. Note if you
 * do things this way you must ensure that auto_detach_kernel_driver for
 * <tt>dev</tt> is 0, otherwise the kernel driver will be re-attached when you
 * release the interface(s).
 *
 * You cannot change/reset configuration if other applications or drivers have
 * claimed interfaces.
 *
 * A configuration value of -1 will put the device in unconfigured state.
 * The USB specifications state that a configuration value of 0 does this,
 * however buggy devices exist which actually have a configuration 0.
 *
 * You should always use this function rather than formulating your own
 * SET_CONFIGURATION control request. This is because the underlying operating
 * system needs to know when such changes happen.
 *
 * This is a blocking function.
 *
 * \param dev_handle a device handle
 * \param configuration the bConfigurationValue of the configuration you
 * wish to activate, or -1 if you wish to put the device in an unconfigured
 * state
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if the requested configuration does not exist
 * \returns LIBUSB_ERROR_BUSY if interfaces are currently claimed
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns another LIBUSB_ERROR code on other failure
 * \see libusb_set_auto_detach_kernel_driver()
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_set_configuration(
    dev_handle: *mut libusb_device_handle,
    configuration: libc::c_int,
) -> libc::c_int {
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"libusb_set_configuration\x00"))
            .as_ptr(),
        b"configuration %d\x00" as *const u8 as *const libc::c_char,
        configuration,
    );
    if configuration < -(1 as libc::c_int) || configuration > 255 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    return usbi_backend
        .set_configuration
        .expect("non-null function pointer")(dev_handle, configuration);
}
/* * \ingroup libusb_dev
 * Claim an interface on a given device handle. You must claim the interface
 * you wish to use before you can perform I/O on any of its endpoints.
 *
 * It is legal to attempt to claim an already-claimed interface, in which
 * case libusb just returns 0 without doing anything.
 *
 * If auto_detach_kernel_driver is set to 1 for <tt>dev</tt>, the kernel driver
 * will be detached if necessary, on failure the detach error is returned.
 *
 * Claiming of interfaces is a purely logical operation; it does not cause
 * any requests to be sent over the bus. Interface claiming is used to
 * instruct the underlying operating system that your application wishes
 * to take ownership of the interface.
 *
 * This is a non-blocking function.
 *
 * \param dev_handle a device handle
 * \param interface_number the <tt>bInterfaceNumber</tt> of the interface you
 * wish to claim
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if the requested interface does not exist
 * \returns LIBUSB_ERROR_BUSY if another program or driver has claimed the
 * interface
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns a LIBUSB_ERROR code on other failure
 * \see libusb_set_auto_detach_kernel_driver()
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_claim_interface(
    mut dev_handle: *mut libusb_device_handle,
    interface_number: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"libusb_claim_interface\x00"))
            .as_ptr(),
        b"interface %d\x00" as *const u8 as *const libc::c_char,
        interface_number,
    );
    if interface_number < 0 as libc::c_int || interface_number >= 32 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    if (*(*dev_handle).dev).attached == 0 {
        return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
    }
    usbi_mutex_lock(&mut (*dev_handle).lock);
    if !((*dev_handle).claimed_interfaces
        & ((1 as libc::c_uint) << interface_number) as libc::c_ulong
        != 0)
    {
        r = usbi_backend
            .claim_interface
            .expect("non-null function pointer")(
            dev_handle, interface_number as uint8_t
        );
        if r == 0 as libc::c_int {
            (*dev_handle).claimed_interfaces |=
                ((1 as libc::c_uint) << interface_number) as libc::c_ulong
        }
    }
    usbi_mutex_unlock(&mut (*dev_handle).lock);
    return r;
}
/* * \ingroup libusb_dev
 * Release an interface previously claimed with libusb_claim_interface(). You
 * should release all claimed interfaces before closing a device handle.
 *
 * This is a blocking function. A SET_INTERFACE control request will be sent
 * to the device, resetting interface state to the first alternate setting.
 *
 * If auto_detach_kernel_driver is set to 1 for <tt>dev</tt>, the kernel
 * driver will be re-attached after releasing the interface.
 *
 * \param dev_handle a device handle
 * \param interface_number the <tt>bInterfaceNumber</tt> of the
 * previously-claimed interface
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if the interface was not claimed
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns another LIBUSB_ERROR code on other failure
 * \see libusb_set_auto_detach_kernel_driver()
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_release_interface(
    mut dev_handle: *mut libusb_device_handle,
    interface_number: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"libusb_release_interface\x00"))
            .as_ptr(),
        b"interface %d\x00" as *const u8 as *const libc::c_char,
        interface_number,
    );
    if interface_number < 0 as libc::c_int || interface_number >= 32 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    usbi_mutex_lock(&mut (*dev_handle).lock);
    if (*dev_handle).claimed_interfaces & ((1 as libc::c_uint) << interface_number) as libc::c_ulong
        == 0
    {
        r = LIBUSB_ERROR_NOT_FOUND as libc::c_int
    } else {
        r = usbi_backend
            .release_interface
            .expect("non-null function pointer")(
            dev_handle, interface_number as uint8_t
        );
        if r == 0 as libc::c_int {
            (*dev_handle).claimed_interfaces &=
                !((1 as libc::c_uint) << interface_number) as libc::c_ulong
        }
    }
    usbi_mutex_unlock(&mut (*dev_handle).lock);
    return r;
}
/* * \ingroup libusb_dev
 * Activate an alternate setting for an interface. The interface must have
 * been previously claimed with libusb_claim_interface().
 *
 * You should always use this function rather than formulating your own
 * SET_INTERFACE control request. This is because the underlying operating
 * system needs to know when such changes happen.
 *
 * This is a blocking function.
 *
 * \param dev_handle a device handle
 * \param interface_number the <tt>bInterfaceNumber</tt> of the
 * previously-claimed interface
 * \param alternate_setting the <tt>bAlternateSetting</tt> of the alternate
 * setting to activate
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if the interface was not claimed, or the
 * requested alternate setting does not exist
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns another LIBUSB_ERROR code on other failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_set_interface_alt_setting(
    dev_handle: *mut libusb_device_handle,
    interface_number: libc::c_int,
    alternate_setting: libc::c_int,
) -> libc::c_int {
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
            b"libusb_set_interface_alt_setting\x00",
        ))
        .as_ptr(),
        b"interface %d altsetting %d\x00" as *const u8 as *const libc::c_char,
        interface_number,
        alternate_setting,
    );
    if interface_number < 0 as libc::c_int || interface_number >= 32 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    if alternate_setting < 0 as libc::c_int || alternate_setting > 255 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    usbi_mutex_lock(&mut (*dev_handle).lock);
    if (*(*dev_handle).dev).attached == 0 {
        usbi_mutex_unlock(&mut (*dev_handle).lock);
        return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
    }
    if (*dev_handle).claimed_interfaces & ((1 as libc::c_uint) << interface_number) as libc::c_ulong
        == 0
    {
        usbi_mutex_unlock(&mut (*dev_handle).lock);
        return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
    }
    usbi_mutex_unlock(&mut (*dev_handle).lock);
    return usbi_backend
        .set_interface_altsetting
        .expect("non-null function pointer")(
        dev_handle,
        interface_number as uint8_t,
        alternate_setting as uint8_t,
    );
}
/* * \ingroup libusb_dev
 * Clear the halt/stall condition for an endpoint. Endpoints with halt status
 * are unable to receive or transmit data until the halt condition is stalled.
 *
 * You should cancel all pending transfers before attempting to clear the halt
 * condition.
 *
 * This is a blocking function.
 *
 * \param dev_handle a device handle
 * \param endpoint the endpoint to clear halt status
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if the endpoint does not exist
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns another LIBUSB_ERROR code on other failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_clear_halt(
    dev_handle: *mut libusb_device_handle,
    endpoint: libc::c_uchar,
) -> libc::c_int {
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"libusb_clear_halt\x00"))
            .as_ptr(),
        b"endpoint %x\x00" as *const u8 as *const libc::c_char,
        endpoint as libc::c_int,
    );
    if (*(*dev_handle).dev).attached == 0 {
        return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
    }
    return usbi_backend.clear_halt.expect("non-null function pointer")(dev_handle, endpoint);
}
/* * \ingroup libusb_dev
 * Perform a USB port reset to reinitialize a device. The system will attempt
 * to restore the previous configuration and alternate settings after the
 * reset has completed.
 *
 * If the reset fails, the descriptors change, or the previous state cannot be
 * restored, the device will appear to be disconnected and reconnected. This
 * means that the device handle is no longer valid (you should close it) and
 * rediscover the device. A return code of LIBUSB_ERROR_NOT_FOUND indicates
 * when this is the case.
 *
 * This is a blocking function which usually incurs a noticeable delay.
 *
 * \param dev_handle a handle of the device to reset
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if re-enumeration is required, or if the
 * device has been disconnected
 * \returns another LIBUSB_ERROR code on other failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_reset_device(dev_handle: *mut libusb_device_handle) -> libc::c_int {
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"libusb_reset_device\x00"))
            .as_ptr(),
        b" \x00" as *const u8 as *const libc::c_char,
    );
    if (*(*dev_handle).dev).attached == 0 {
        return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
    }
    if usbi_backend.reset_device.is_some() {
        return usbi_backend
            .reset_device
            .expect("non-null function pointer")(dev_handle);
    } else {
        return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    };
}
/* * \ingroup libusb_asyncio
 * Allocate up to num_streams usb bulk streams on the specified endpoints. This
 * function takes an array of endpoints rather then a single endpoint because
 * some protocols require that endpoints are setup with similar stream ids.
 * All endpoints passed in must belong to the same interface.
 *
 * Note this function may return less streams then requested. Also note that the
 * same number of streams are allocated for each endpoint in the endpoint array.
 *
 * Stream id 0 is reserved, and should not be used to communicate with devices.
 * If libusb_alloc_streams() returns with a value of N, you may use stream ids
 * 1 to N.
 *
 * Since version 1.0.19, \ref LIBUSB_API_VERSION >= 0x01000103
 *
 * \param dev_handle a device handle
 * \param num_streams number of streams to try to allocate
 * \param endpoints array of endpoints to allocate streams on
 * \param num_endpoints length of the endpoints array
 * \returns number of streams allocated, or a LIBUSB_ERROR code on failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_alloc_streams(
    dev_handle: *mut libusb_device_handle,
    num_streams: uint32_t,
    endpoints: *mut libc::c_uchar,
    num_endpoints: libc::c_int,
) -> libc::c_int {
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"libusb_alloc_streams\x00"))
            .as_ptr(),
        b"streams %u eps %d\x00" as *const u8 as *const libc::c_char,
        num_streams,
        num_endpoints,
    );
    if num_streams == 0 || endpoints.is_null() || num_endpoints <= 0 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    if (*(*dev_handle).dev).attached == 0 {
        return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
    }
    if usbi_backend.alloc_streams.is_some() {
        return usbi_backend
            .alloc_streams
            .expect("non-null function pointer")(
            dev_handle,
            num_streams,
            endpoints,
            num_endpoints,
        );
    } else {
        return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    };
}
/* * \ingroup libusb_asyncio
 * Free usb bulk streams allocated with libusb_alloc_streams().
 *
 * Note streams are automatically free-ed when releasing an interface.
 *
 * Since version 1.0.19, \ref LIBUSB_API_VERSION >= 0x01000103
 *
 * \param dev_handle a device handle
 * \param endpoints array of endpoints to free streams on
 * \param num_endpoints length of the endpoints array
 * \returns LIBUSB_SUCCESS, or a LIBUSB_ERROR code on failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_free_streams(
    dev_handle: *mut libusb_device_handle,
    endpoints: *mut libc::c_uchar,
    num_endpoints: libc::c_int,
) -> libc::c_int {
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"libusb_free_streams\x00"))
            .as_ptr(),
        b"eps %d\x00" as *const u8 as *const libc::c_char,
        num_endpoints,
    );
    if endpoints.is_null() || num_endpoints <= 0 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    if (*(*dev_handle).dev).attached == 0 {
        return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
    }
    if usbi_backend.free_streams.is_some() {
        return usbi_backend
            .free_streams
            .expect("non-null function pointer")(
            dev_handle, endpoints, num_endpoints
        );
    } else {
        return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    };
}
/* * \ingroup libusb_asyncio
 * Attempts to allocate a block of persistent DMA memory suitable for transfers
 * against the given device. If successful, will return a block of memory
 * that is suitable for use as "buffer" in \ref libusb_transfer against this
 * device. Using this memory instead of regular memory means that the host
 * controller can use DMA directly into the buffer to increase performance, and
 * also that transfers can no longer fail due to kernel memory fragmentation.
 *
 * Note that this means you should not modify this memory (or even data on
 * the same cache lines) when a transfer is in progress, although it is legal
 * to have several transfers going on within the same memory block.
 *
 * Will return NULL on failure. Many systems do not support such zerocopy
 * and will always return NULL. Memory allocated with this function must be
 * freed with \ref libusb_dev_mem_free. Specifically, this means that the
 * flag \ref LIBUSB_TRANSFER_FREE_BUFFER cannot be used to free memory allocated
 * with this function.
 *
 * Since version 1.0.21, \ref LIBUSB_API_VERSION >= 0x01000105
 *
 * \param dev_handle a device handle
 * \param length size of desired data buffer
 * \returns a pointer to the newly allocated memory, or NULL on failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_dev_mem_alloc(
    dev_handle: *mut libusb_device_handle,
    length: size_t,
) -> *mut libc::c_uchar {
    if (*(*dev_handle).dev).attached == 0 {
        return 0 as *mut libc::c_uchar;
    }
    if usbi_backend.dev_mem_alloc.is_some() {
        return usbi_backend
            .dev_mem_alloc
            .expect("non-null function pointer")(dev_handle, length)
            as *mut libc::c_uchar;
    } else {
        return 0 as *mut libc::c_uchar;
    };
}
/* * \ingroup libusb_asyncio
 * Free device memory allocated with libusb_dev_mem_alloc().
 *
 * \param dev_handle a device handle
 * \param buffer pointer to the previously allocated memory
 * \param length size of previously allocated memory
 * \returns LIBUSB_SUCCESS, or a LIBUSB_ERROR code on failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_dev_mem_free(
    dev_handle: *mut libusb_device_handle,
    buffer: *mut libc::c_uchar,
    length: size_t,
) -> libc::c_int {
    if usbi_backend.dev_mem_free.is_some() {
        return usbi_backend
            .dev_mem_free
            .expect("non-null function pointer")(
            dev_handle, buffer as *mut libc::c_void, length
        );
    } else {
        return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    };
}
/* * \ingroup libusb_dev
 * Determine if a kernel driver is active on an interface. If a kernel driver
 * is active, you cannot claim the interface, and libusb will be unable to
 * perform I/O.
 *
 * This functionality is not available on Windows.
 *
 * \param dev_handle a device handle
 * \param interface_number the interface to check
 * \returns 0 if no kernel driver is active
 * \returns 1 if a kernel driver is active
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns LIBUSB_ERROR_NOT_SUPPORTED on platforms where the functionality
 * is not available
 * \returns another LIBUSB_ERROR code on other failure
 * \see libusb_detach_kernel_driver()
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_kernel_driver_active(
    dev_handle: *mut libusb_device_handle,
    interface_number: libc::c_int,
) -> libc::c_int {
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
            b"libusb_kernel_driver_active\x00",
        ))
        .as_ptr(),
        b"interface %d\x00" as *const u8 as *const libc::c_char,
        interface_number,
    );
    if interface_number < 0 as libc::c_int || interface_number >= 32 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    if (*(*dev_handle).dev).attached == 0 {
        return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
    }
    if usbi_backend.kernel_driver_active.is_some() {
        return usbi_backend
            .kernel_driver_active
            .expect("non-null function pointer")(
            dev_handle, interface_number as uint8_t
        );
    } else {
        return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    };
}
/* * \ingroup libusb_dev
 * Detach a kernel driver from an interface. If successful, you will then be
 * able to claim the interface and perform I/O.
 *
 * This functionality is not available on Darwin or Windows.
 *
 * Note that libusb itself also talks to the device through a special kernel
 * driver, if this driver is already attached to the device, this call will
 * not detach it and return LIBUSB_ERROR_NOT_FOUND.
 *
 * \param dev_handle a device handle
 * \param interface_number the interface to detach the driver from
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if no kernel driver was active
 * \returns LIBUSB_ERROR_INVALID_PARAM if the interface does not exist
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns LIBUSB_ERROR_NOT_SUPPORTED on platforms where the functionality
 * is not available
 * \returns another LIBUSB_ERROR code on other failure
 * \see libusb_kernel_driver_active()
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_detach_kernel_driver(
    dev_handle: *mut libusb_device_handle,
    interface_number: libc::c_int,
) -> libc::c_int {
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
            b"libusb_detach_kernel_driver\x00",
        ))
        .as_ptr(),
        b"interface %d\x00" as *const u8 as *const libc::c_char,
        interface_number,
    );
    if interface_number < 0 as libc::c_int || interface_number >= 32 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    if (*(*dev_handle).dev).attached == 0 {
        return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
    }
    if usbi_backend.detach_kernel_driver.is_some() {
        return usbi_backend
            .detach_kernel_driver
            .expect("non-null function pointer")(
            dev_handle, interface_number as uint8_t
        );
    } else {
        return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    };
}
/* * \ingroup libusb_dev
 * Re-attach an interface's kernel driver, which was previously detached
 * using libusb_detach_kernel_driver(). This call is only effective on
 * Linux and returns LIBUSB_ERROR_NOT_SUPPORTED on all other platforms.
 *
 * This functionality is not available on Darwin or Windows.
 *
 * \param dev_handle a device handle
 * \param interface_number the interface to attach the driver from
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if no kernel driver was active
 * \returns LIBUSB_ERROR_INVALID_PARAM if the interface does not exist
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns LIBUSB_ERROR_NOT_SUPPORTED on platforms where the functionality
 * is not available
 * \returns LIBUSB_ERROR_BUSY if the driver cannot be attached because the
 * interface is claimed by a program or driver
 * \returns another LIBUSB_ERROR code on other failure
 * \see libusb_kernel_driver_active()
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_attach_kernel_driver(
    dev_handle: *mut libusb_device_handle,
    interface_number: libc::c_int,
) -> libc::c_int {
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
            b"libusb_attach_kernel_driver\x00",
        ))
        .as_ptr(),
        b"interface %d\x00" as *const u8 as *const libc::c_char,
        interface_number,
    );
    if interface_number < 0 as libc::c_int || interface_number >= 32 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    if (*(*dev_handle).dev).attached == 0 {
        return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
    }
    if usbi_backend.attach_kernel_driver.is_some() {
        return usbi_backend
            .attach_kernel_driver
            .expect("non-null function pointer")(
            dev_handle, interface_number as uint8_t
        );
    } else {
        return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    };
}
/* * \ingroup libusb_dev
 * Enable/disable libusb's automatic kernel driver detachment. When this is
 * enabled libusb will automatically detach the kernel driver on an interface
 * when claiming the interface, and attach it when releasing the interface.
 *
 * Automatic kernel driver detachment is disabled on newly opened device
 * handles by default.
 *
 * On platforms which do not have LIBUSB_CAP_SUPPORTS_DETACH_KERNEL_DRIVER
 * this function will return LIBUSB_ERROR_NOT_SUPPORTED, and libusb will
 * continue as if this function was never called.
 *
 * \param dev_handle a device handle
 * \param enable whether to enable or disable auto kernel driver detachment
 *
 * \returns LIBUSB_SUCCESS on success
 * \returns LIBUSB_ERROR_NOT_SUPPORTED on platforms where the functionality
 * is not available
 * \see libusb_claim_interface()
 * \see libusb_release_interface()
 * \see libusb_set_configuration()
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_set_auto_detach_kernel_driver(
    mut dev_handle: *mut libusb_device_handle,
    enable: libc::c_int,
) -> libc::c_int {
    if usbi_backend.caps & 0x20000 as libc::c_int as libc::c_uint == 0 {
        return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    }
    (*dev_handle).auto_detach_kernel_driver = enable;
    return LIBUSB_SUCCESS as libc::c_int;
}
/* * \ingroup libusb_lib
 * \deprecated Use libusb_set_option() instead using the
 * \ref LIBUSB_OPTION_LOG_LEVEL option.
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_set_debug(mut ctx: *mut libusb_context, mut level: libc::c_int) {
    ctx = usbi_get_context(ctx);
    if (*ctx).debug_fixed == 0 {
        level = if level < LIBUSB_LOG_LEVEL_NONE as libc::c_int {
            LIBUSB_LOG_LEVEL_NONE as libc::c_int
        } else if level > LIBUSB_LOG_LEVEL_DEBUG as libc::c_int {
            LIBUSB_LOG_LEVEL_DEBUG as libc::c_int
        } else {
            level
        };
        (*ctx).debug = level as libusb_log_level
    };
}
/* * \ingroup libusb_lib
 * Set log handler.
 *
 * libusb will redirect its log messages to the provided callback function.
 * libusb supports redirection of per context and global log messages.
 * Log messages sent to the context will be sent to the global log handler too.
 *
 * If libusb is compiled without message logging or USE_SYSTEM_LOGGING_FACILITY
 * is defined then global callback function will never be called.
 * If ENABLE_DEBUG_LOGGING is defined then per context callback function will
 * never be called.
 *
 * \param ctx context on which to assign log handler, or NULL for the default
 * context. Parameter ignored if only LIBUSB_LOG_CB_GLOBAL mode is requested.
 * \param cb pointer to the callback function, or NULL to stop log
 * messages redirection
 * \param mode mode of callback function operation. Several modes can be
 * selected for a single callback function, see \ref libusb_log_cb_mode for
 * a description.
 * \see libusb_log_cb, libusb_log_cb_mode
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_set_log_cb(
    mut ctx: *mut libusb_context,
    cb: libusb_log_cb,
    mode: libc::c_int,
) {
    if mode & LIBUSB_LOG_CB_GLOBAL as libc::c_int != 0 {
        log_handler = cb
    }
    if mode & LIBUSB_LOG_CB_CONTEXT as libc::c_int != 0 {
        ctx = usbi_get_context(ctx);
        (*ctx).log_handler = cb
    };
}
/* * \ingroup libusb_lib
 * Set an option in the library.
 *
 * Use this function to configure a specific option within the library.
 *
 * Some options require one or more arguments to be provided. Consult each
 * option's documentation for specific requirements.
 *
 * Since version 1.0.22, \ref LIBUSB_API_VERSION >= 0x01000106
 *
 * \param ctx context on which to operate
 * \param option which option to set
 * \param ... any required arguments for the specified option
 *
 * \returns LIBUSB_SUCCESS on success
 * \returns LIBUSB_ERROR_INVALID_PARAM if the option or arguments are invalid
 * \returns LIBUSB_ERROR_NOT_SUPPORTED if the option is valid but not supported
 * on this platform
 * \returns LIBUSB_ERROR_NOT_FOUND if LIBUSB_OPTION_USE_USBDK is valid on this platform but UsbDk is not available
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_set_option(
    mut ctx: *mut libusb_context,
    option: libusb_option,
    args: ...
) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut r: libc::c_int = LIBUSB_SUCCESS as libc::c_int;
    let mut ap: ::std::ffi::VaListImpl;
    ctx = usbi_get_context(ctx);
    ap = args.clone();
    match option as libc::c_uint {
        0 => {
            arg = ap.arg::<libc::c_int>();
            if arg < LIBUSB_LOG_LEVEL_NONE as libc::c_int
                || arg > LIBUSB_LOG_LEVEL_DEBUG as libc::c_int
            {
                r = LIBUSB_ERROR_INVALID_PARAM as libc::c_int
            } else if (*ctx).debug_fixed == 0 {
                (*ctx).debug = arg as libusb_log_level
            }
        }
        1 => {
            /* Handle all backend-specific options here */
            if usbi_backend.set_option.is_some() {
                r = usbi_backend.set_option.expect("non-null function pointer")(
                    ctx,
                    option,
                    ap.as_va_list(),
                )
            } else {
                r = LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int
            }
        }
        _ => r = LIBUSB_ERROR_INVALID_PARAM as libc::c_int,
    }
    return r;
}
/* returns the log level as defined in the LIBUSB_DEBUG environment variable.
 * if LIBUSB_DEBUG is not present or not a number, returns LIBUSB_LOG_LEVEL_NONE.
 * value is clamped to ensure it is within the valid range of possibilities.
 */
unsafe extern "C" fn get_env_debug_level() -> libusb_log_level {
    let dbg: *const libc::c_char = getenv(b"LIBUSB_DEBUG\x00" as *const u8 as *const libc::c_char);
    let mut level: libusb_log_level = LIBUSB_LOG_LEVEL_NONE;
    if !dbg.is_null() {
        let mut dbg_level: libc::c_int = atoi(dbg);
        dbg_level = if dbg_level < LIBUSB_LOG_LEVEL_NONE as libc::c_int {
            LIBUSB_LOG_LEVEL_NONE as libc::c_int
        } else if dbg_level > LIBUSB_LOG_LEVEL_DEBUG as libc::c_int {
            LIBUSB_LOG_LEVEL_DEBUG as libc::c_int
        } else {
            dbg_level
        };
        level = dbg_level as libusb_log_level
    } else {
        level = LIBUSB_LOG_LEVEL_NONE
    }
    return level;
}
/* * \ingroup libusb_lib
 * Initialize libusb. This function must be called before calling any other
 * libusb function.
 *
 * If you do not provide an output location for a context pointer, a default
 * context will be created. If there was already a default context, it will
 * be reused (and nothing will be initialized/reinitialized).
 *
 * \param context Optional output location for context pointer.
 * Only valid on return code 0.
 * \returns 0 on success, or a LIBUSB_ERROR code on failure
 * \see libusb_contexts
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_init(context: *mut *mut libusb_context) -> libc::c_int {
    let current_block: u64;
    let mut dev: *mut libusb_device = 0 as *mut libusb_device;
    let mut next: *mut libusb_device = 0 as *mut libusb_device;
    let priv_size: size_t = usbi_backend.context_priv_size;
    let mut ctx: *mut libusb_context = 0 as *mut libusb_context;
    static mut first_init: libc::c_int = 1 as libc::c_int;
    let mut r: libc::c_int = 0 as libc::c_int;
    usbi_mutex_static_lock(&mut default_context_lock);
    if timestamp_origin.tv_sec == 0 {
        clock_gettime(0 as libc::c_int, &mut timestamp_origin);
    }
    if context.is_null() && !usbi_default_context.is_null() {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"libusb_init\x00")).as_ptr(),
            b"reusing default context\x00" as *const u8 as *const libc::c_char,
        );
        default_context_refcnt += 1;
        usbi_mutex_static_unlock(&mut default_context_lock);
        return 0 as libc::c_int;
    }
    ctx = calloc(
        1 as libc::c_int as libc::c_ulong,
        ((::std::mem::size_of::<libusb_context>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        .wrapping_add(priv_size),
    ) as *mut libusb_context;
    if ctx.is_null() {
        r = LIBUSB_ERROR_NO_MEM as libc::c_int
    } else {
        (*ctx).debug = get_env_debug_level();
        if (*ctx).debug as libc::c_uint != LIBUSB_LOG_LEVEL_NONE as libc::c_int as libc::c_uint {
            (*ctx).debug_fixed = 1 as libc::c_int
        }
        /* default context should be initialized before calling usbi_dbg */
        if usbi_default_context.is_null() {
            usbi_default_context = ctx;
            default_context_refcnt += 1;
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"libusb_init\x00"))
                    .as_ptr(),
                b"created default context\x00" as *const u8 as *const libc::c_char,
            );
        }
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"libusb_init\x00")).as_ptr(),
            b"libusb v%u.%u.%u.%u%s\x00" as *const u8 as *const libc::c_char,
            libusb_version_internal.major as libc::c_int,
            libusb_version_internal.minor as libc::c_int,
            libusb_version_internal.micro as libc::c_int,
            libusb_version_internal.nano as libc::c_int,
            libusb_version_internal.rc,
        );
        usbi_mutex_init(&mut (*ctx).usb_devs_lock);
        usbi_mutex_init(&mut (*ctx).open_devs_lock);
        usbi_mutex_init(&mut (*ctx).hotplug_cbs_lock);
        list_init(&mut (*ctx).usb_devs);
        list_init(&mut (*ctx).open_devs);
        list_init(&mut (*ctx).hotplug_cbs);
        (*ctx).next_hotplug_cb_handle = 1 as libc::c_int;
        usbi_mutex_static_lock(&mut active_contexts_lock);
        if first_init != 0 {
            first_init = 0 as libc::c_int;
            list_init(&mut active_contexts_list);
        }
        list_add(&mut (*ctx).list, &mut active_contexts_list);
        usbi_mutex_static_unlock(&mut active_contexts_lock);
        if usbi_backend.init.is_some() {
            r = usbi_backend.init.expect("non-null function pointer")(ctx);
            if r != 0 {
                current_block = 15926143460950347726;
            } else {
                current_block = 15597372965620363352;
            }
        } else {
            current_block = 15597372965620363352;
        }
        match current_block {
            15597372965620363352 => {
                r = usbi_io_init(ctx);
                if r < 0 as libc::c_int {
                    if usbi_backend.exit.is_some() {
                        usbi_backend.exit.expect("non-null function pointer")(ctx);
                    }
                } else {
                    usbi_mutex_static_unlock(&mut default_context_lock);
                    if !context.is_null() {
                        *context = ctx
                    }
                    return 0 as libc::c_int;
                }
            }
            _ => {}
        }
        if ctx == usbi_default_context {
            usbi_default_context = 0 as *mut libusb_context;
            default_context_refcnt -= 1
        }
        usbi_mutex_static_lock(&mut active_contexts_lock);
        list_del(&mut (*ctx).list);
        usbi_mutex_static_unlock(&mut active_contexts_lock);
        usbi_mutex_lock(&mut (*ctx).usb_devs_lock);
        dev = ((*ctx).usb_devs.next as uintptr_t).wrapping_sub(72 as libc::c_ulong)
            as *mut libusb_device;
        next =
            ((*dev).list.next as uintptr_t).wrapping_sub(72 as libc::c_ulong) as *mut libusb_device;
        while &mut (*dev).list as *mut list_head != &mut (*ctx).usb_devs as *mut list_head {
            list_del(&mut (*dev).list);
            libusb_unref_device(dev);
            dev = next;
            next = ((*next).list.next as uintptr_t).wrapping_sub(72 as libc::c_ulong)
                as *mut libusb_device
        }
        usbi_mutex_unlock(&mut (*ctx).usb_devs_lock);
        usbi_mutex_destroy(&mut (*ctx).open_devs_lock);
        usbi_mutex_destroy(&mut (*ctx).usb_devs_lock);
        usbi_mutex_destroy(&mut (*ctx).hotplug_cbs_lock);
        free(ctx as *mut libc::c_void);
    }
    usbi_mutex_static_unlock(&mut default_context_lock);
    return r;
}
/* * \ingroup libusb_lib
 * Deinitialize libusb. Should be called after closing all open devices and
 * before your application terminates.
 * \param ctx the context to deinitialize, or NULL for the default context
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_exit(mut ctx: *mut libusb_context) {
    let mut dev: *mut libusb_device = 0 as *mut libusb_device;
    let mut next: *mut libusb_device = 0 as *mut libusb_device;
    let mut tv: timeval = {
        let init = timeval {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_usec: 0 as libc::c_int as __suseconds_t,
        };
        init
    };
    let mut destroying_default_context: libc::c_int = 0 as libc::c_int;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"libusb_exit\x00")).as_ptr(),
        b" \x00" as *const u8 as *const libc::c_char,
    );
    ctx = usbi_get_context(ctx);
    /* if working with default context, only actually do the deinitialization
     * if we're the last user */
    usbi_mutex_static_lock(&mut default_context_lock);
    if ctx == usbi_default_context {
        if usbi_default_context.is_null() {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"libusb_exit\x00"))
                    .as_ptr(),
                b"no default context, not initialized?\x00" as *const u8 as *const libc::c_char,
            );
            usbi_mutex_static_unlock(&mut default_context_lock);
            return;
        }
        default_context_refcnt -= 1;
        if default_context_refcnt > 0 as libc::c_int {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"libusb_exit\x00"))
                    .as_ptr(),
                b"not destroying default context\x00" as *const u8 as *const libc::c_char,
            );
            usbi_mutex_static_unlock(&mut default_context_lock);
            return;
        }
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"libusb_exit\x00")).as_ptr(),
            b"destroying default context\x00" as *const u8 as *const libc::c_char,
        );
        /*
         * Setting this flag without unlocking the default context, as
         * we are actually destroying the default context.
         * usbi_default_context is not set to NULL yet, as all activities
         * would only stop after usbi_backend->exit() returns.
         */
        destroying_default_context = 1 as libc::c_int
    } else {
        /* Unlock default context, as we're not modifying it. */
        usbi_mutex_static_unlock(&mut default_context_lock);
    }
    usbi_mutex_static_lock(&mut active_contexts_lock);
    list_del(&mut (*ctx).list);
    usbi_mutex_static_unlock(&mut active_contexts_lock);
    if libusb_has_capability(LIBUSB_CAP_HAS_HOTPLUG as libc::c_int as uint32_t) != 0 {
        usbi_hotplug_deregister(ctx, 1 as libc::c_int);
        /*
         * Ensure any pending unplug events are read from the hotplug
         * pipe. The usb_device-s hold in the events are no longer part
         * of usb_devs, but the events still hold a reference!
         *
         * Note we don't do this if the application has left devices
         * open (which implies a buggy app) to avoid packet completion
         * handlers running when the app does not expect them to run.
         */
        if (*ctx).open_devs.next == &mut (*ctx).open_devs as *mut list_head {
            libusb_handle_events_timeout(ctx, &mut tv);
        }
        usbi_mutex_lock(&mut (*ctx).usb_devs_lock);
        dev = ((*ctx).usb_devs.next as uintptr_t).wrapping_sub(72 as libc::c_ulong)
            as *mut libusb_device;
        next =
            ((*dev).list.next as uintptr_t).wrapping_sub(72 as libc::c_ulong) as *mut libusb_device;
        while &mut (*dev).list as *mut list_head != &mut (*ctx).usb_devs as *mut list_head {
            list_del(&mut (*dev).list);
            libusb_unref_device(dev);
            dev = next;
            next = ((*next).list.next as uintptr_t).wrapping_sub(72 as libc::c_ulong)
                as *mut libusb_device
        }
        usbi_mutex_unlock(&mut (*ctx).usb_devs_lock);
    }
    /* a few sanity checks. don't bother with locking because unless
     * there is an application bug, nobody will be accessing these. */
    if !((*ctx).usb_devs.next == &mut (*ctx).usb_devs as *mut list_head) {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_WARNING,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"libusb_exit\x00")).as_ptr(),
            b"some libusb_devices were leaked\x00" as *const u8 as *const libc::c_char,
        );
    }
    if !((*ctx).open_devs.next == &mut (*ctx).open_devs as *mut list_head) {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_WARNING,
            (*::std::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"libusb_exit\x00")).as_ptr(),
            b"application left some devices open\x00" as *const u8 as *const libc::c_char,
        );
    }
    usbi_io_exit(ctx);
    if usbi_backend.exit.is_some() {
        usbi_backend.exit.expect("non-null function pointer")(ctx);
    }
    usbi_mutex_destroy(&mut (*ctx).open_devs_lock);
    usbi_mutex_destroy(&mut (*ctx).usb_devs_lock);
    usbi_mutex_destroy(&mut (*ctx).hotplug_cbs_lock);
    free(ctx as *mut libc::c_void);
    if destroying_default_context != 0 {
        usbi_default_context = 0 as *mut libusb_context;
        usbi_mutex_static_unlock(&mut default_context_lock);
    };
}
/* * \ingroup libusb_misc
 * Check at runtime if the loaded library has a given capability.
 * This call should be performed after \ref libusb_init(), to ensure the
 * backend has updated its capability set.
 *
 * \param capability the \ref libusb_capability to check for
 * \returns nonzero if the running library has the capability, 0 otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_has_capability(capability: uint32_t) -> libc::c_int {
    match capability {
        0 => return 1 as libc::c_int,
        1 => return usbi_backend.get_device_list.is_none() as libc::c_int,
        256 => return (usbi_backend.caps & 0x10000 as libc::c_int as libc::c_uint) as libc::c_int,
        257 => return (usbi_backend.caps & 0x20000 as libc::c_int as libc::c_uint) as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
/* this is defined in libusbi.h if needed */
/* LIBUSB_PRINTF_WIN32 */
unsafe extern "C" fn usbi_log_str(level: libusb_log_level, str: *const libc::c_char) {
    /* Global log handler */
    if log_handler.is_some() {
        log_handler.expect("non-null function pointer")(0 as *mut libusb_context, level, str);
    } else {
        fputs(str, stderr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn usbi_log_v(
    mut ctx: *mut libusb_context,
    level: libusb_log_level,
    function: *const libc::c_char,
    format: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    let mut prefix: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut global_debug: libc::c_int = 0;
    let mut header_len: libc::c_int = 0;
    let mut text_len: libc::c_int = 0;
    static mut has_debug_header_been_displayed: libc::c_int = 0 as libc::c_int;
    let mut ctx_level: libusb_log_level = LIBUSB_LOG_LEVEL_NONE;
    ctx = usbi_get_context(ctx);
    if !ctx.is_null() {
        ctx_level = (*ctx).debug
    } else {
        ctx_level = get_env_debug_level()
    }
    if ctx_level as libc::c_uint == LIBUSB_LOG_LEVEL_NONE as libc::c_int as libc::c_uint {
        return;
    }
    if level as libc::c_uint == LIBUSB_LOG_LEVEL_WARNING as libc::c_int as libc::c_uint
        && (ctx_level as libc::c_uint) < LIBUSB_LOG_LEVEL_WARNING as libc::c_int as libc::c_uint
    {
        return;
    }
    if level as libc::c_uint == LIBUSB_LOG_LEVEL_INFO as libc::c_int as libc::c_uint
        && (ctx_level as libc::c_uint) < LIBUSB_LOG_LEVEL_INFO as libc::c_int as libc::c_uint
    {
        return;
    }
    if level as libc::c_uint == LIBUSB_LOG_LEVEL_DEBUG as libc::c_int as libc::c_uint
        && (ctx_level as libc::c_uint) < LIBUSB_LOG_LEVEL_DEBUG as libc::c_int as libc::c_uint
    {
        return;
    }
    global_debug = (ctx_level as libc::c_uint
        == LIBUSB_LOG_LEVEL_DEBUG as libc::c_int as libc::c_uint) as libc::c_int;
    clock_gettime(0 as libc::c_int, &mut now);
    if global_debug != 0 && has_debug_header_been_displayed == 0 {
        has_debug_header_been_displayed = 1 as libc::c_int;
        usbi_log_str(
            LIBUSB_LOG_LEVEL_DEBUG,
            b"[timestamp] [threadID] facility level [function call] <message>\n\x00" as *const u8
                as *const libc::c_char,
        );
        usbi_log_str(LIBUSB_LOG_LEVEL_DEBUG,
                     b"--------------------------------------------------------------------------------\n\x00"
                         as *const u8 as *const libc::c_char);
    }
    if now.tv_nsec < timestamp_origin.tv_nsec {
        now.tv_sec -= 1;
        now.tv_nsec += 1000000000 as libc::c_long
    }
    now.tv_sec -= timestamp_origin.tv_sec;
    now.tv_nsec -= timestamp_origin.tv_nsec;
    match level as libc::c_uint {
        0 => return,
        1 => prefix = b"error\x00" as *const u8 as *const libc::c_char,
        2 => prefix = b"warning\x00" as *const u8 as *const libc::c_char,
        3 => prefix = b"info\x00" as *const u8 as *const libc::c_char,
        4 => prefix = b"debug\x00" as *const u8 as *const libc::c_char,
        _ => prefix = b"unknown\x00" as *const u8 as *const libc::c_char,
    }
    if global_debug != 0 {
        header_len = snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"[%2ld.%06ld] [%08x] libusb: %s [%s] \x00" as *const u8 as *const libc::c_char,
            now.tv_sec,
            now.tv_nsec / 1000 as libc::c_long,
            usbi_get_tid(),
            prefix,
            function,
        )
    } else {
        header_len = snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"libusb: %s [%s] \x00" as *const u8 as *const libc::c_char,
            prefix,
            function,
        )
    }
    if header_len < 0 as libc::c_int
        || header_len
            >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int
    {
        /* Somehow snprintf failed to write to the buffer,
         * remove the header so something useful is output. */
        header_len = 0 as libc::c_int
    }
    /* Make sure buffer is NUL terminated */
    buf[header_len as usize] = '\u{0}' as i32 as libc::c_char;
    text_len = vsnprintf(
        buf.as_mut_ptr().offset(header_len as isize),
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(header_len as size_t),
        format,
        args.as_va_list(),
    );
    if text_len < 0 as libc::c_int
        || text_len + header_len
            >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int
    {
        /* Truncated log output. On some platforms a -1 return value means
         * that the output was truncated. */
        text_len = ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int
            - header_len
    }
    if header_len
        + text_len
        + ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as libc::c_int
        >= ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int
    {
        /* Need to truncate the text slightly to fit on the terminator. */
        text_len -= header_len
            + text_len
            + ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as libc::c_int
            - ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int
    }
    strcpy(
        buf.as_mut_ptr()
            .offset(header_len as isize)
            .offset(text_len as isize),
        b"\n\x00" as *const u8 as *const libc::c_char,
    );
    usbi_log_str(level, buf.as_mut_ptr());
    /* Per context log handler */
    if !ctx.is_null() && (*ctx).log_handler.is_some() {
        (*ctx).log_handler.expect("non-null function pointer")(ctx, level, buf.as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn usbi_log(
    ctx: *mut libusb_context,
    level: libusb_log_level,
    function: *const libc::c_char,
    format: *const libc::c_char,
    args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    usbi_log_v(ctx, level, function, format, args_0.as_va_list());
}
/* ENABLE_LOGGING */
/* * \ingroup libusb_misc
 * Returns a constant NULL-terminated string with the ASCII name of a libusb
 * error or transfer status code. The caller must not free() the returned
 * string.
 *
 * \param error_code The \ref libusb_error or libusb_transfer_status code to
 * return the name of.
 * \returns The error name, or the string **UNKNOWN** if the value of
 * error_code is not a known error / status code.
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_error_name(error_code: libc::c_int) -> *const libc::c_char {
    match error_code {
        -1 => return b"LIBUSB_ERROR_IO\x00" as *const u8 as *const libc::c_char,
        -2 => return b"LIBUSB_ERROR_INVALID_PARAM\x00" as *const u8 as *const libc::c_char,
        -3 => return b"LIBUSB_ERROR_ACCESS\x00" as *const u8 as *const libc::c_char,
        -4 => return b"LIBUSB_ERROR_NO_DEVICE\x00" as *const u8 as *const libc::c_char,
        -5 => return b"LIBUSB_ERROR_NOT_FOUND\x00" as *const u8 as *const libc::c_char,
        -6 => return b"LIBUSB_ERROR_BUSY\x00" as *const u8 as *const libc::c_char,
        -7 => return b"LIBUSB_ERROR_TIMEOUT\x00" as *const u8 as *const libc::c_char,
        -8 => return b"LIBUSB_ERROR_OVERFLOW\x00" as *const u8 as *const libc::c_char,
        -9 => return b"LIBUSB_ERROR_PIPE\x00" as *const u8 as *const libc::c_char,
        -10 => return b"LIBUSB_ERROR_INTERRUPTED\x00" as *const u8 as *const libc::c_char,
        -11 => return b"LIBUSB_ERROR_NO_MEM\x00" as *const u8 as *const libc::c_char,
        -12 => return b"LIBUSB_ERROR_NOT_SUPPORTED\x00" as *const u8 as *const libc::c_char,
        -99 => return b"LIBUSB_ERROR_OTHER\x00" as *const u8 as *const libc::c_char,
        1 => return b"LIBUSB_TRANSFER_ERROR\x00" as *const u8 as *const libc::c_char,
        2 => return b"LIBUSB_TRANSFER_TIMED_OUT\x00" as *const u8 as *const libc::c_char,
        3 => return b"LIBUSB_TRANSFER_CANCELLED\x00" as *const u8 as *const libc::c_char,
        4 => return b"LIBUSB_TRANSFER_STALL\x00" as *const u8 as *const libc::c_char,
        5 => return b"LIBUSB_TRANSFER_NO_DEVICE\x00" as *const u8 as *const libc::c_char,
        6 => return b"LIBUSB_TRANSFER_OVERFLOW\x00" as *const u8 as *const libc::c_char,
        0 => {
            return b"LIBUSB_SUCCESS / LIBUSB_TRANSFER_COMPLETED\x00" as *const u8
                as *const libc::c_char
        }
        _ => return b"**UNKNOWN**\x00" as *const u8 as *const libc::c_char,
    };
}
/* * \ingroup libusb_misc
 * Returns a pointer to const struct libusb_version with the version
 * (major, minor, micro, nano and rc) of the running library.
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_version() -> *const libusb_version {
    return &libusb_version_internal;
}
