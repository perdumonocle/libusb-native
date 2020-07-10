#![allow(non_camel_case_types,unused_assignments,unused_unsafe,non_snake_case,non_upper_case_globals,unused_must_use)]

use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn usbi_log(
        ctx: *mut libusb_context,
        level: libusb_log_level,
        function: *const libc::c_char,
        format: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    static usbi_backend: usbi_os_backend;
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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
 * USB capability types
 */
pub type libusb_bos_type = libc::c_uint;
/* * Container ID type */
pub const LIBUSB_BT_CONTAINER_ID: libusb_bos_type = 4;
/* * SuperSpeed USB device capability */
pub const LIBUSB_BT_SS_USB_DEVICE_CAPABILITY: libusb_bos_type = 3;
/* * USB 2.0 extensions */
pub const LIBUSB_BT_USB_2_0_EXTENSION: libusb_bos_type = 2;
/* * Wireless USB device capability */
pub const LIBUSB_BT_WIRELESS_USB_DEVICE_CAPABILITY: libusb_bos_type = 1;
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
/* * \ingroup libusb_desc
 * A generic representation of a BOS Device Capability descriptor. It is
 * advised to check bDevCapabilityType and call the matching
 * libusb_get_*_descriptor function to get a structure fully matching the type.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_bos_dev_capability_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub bDevCapabilityType: uint8_t,
    pub dev_capability_data: [uint8_t; 0],
}
/* * \ingroup libusb_desc
 * A structure representing the Binary Device Object Store (BOS) descriptor.
 * This descriptor is documented in section 9.6.2 of the USB 3.0 specification.
 * All multiple-byte fields are represented in host-endian format.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_bos_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub wTotalLength: uint16_t,
    pub bNumDeviceCaps: uint8_t,
    pub dev_capability: [*mut libusb_bos_dev_capability_descriptor; 0],
}
/* * \ingroup libusb_desc
 * A structure representing the USB 2.0 Extension descriptor
 * This descriptor is documented in section 9.6.2.1 of the USB 3.0 specification.
 * All multiple-byte fields are represented in host-endian format.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_usb_2_0_extension_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub bDevCapabilityType: uint8_t,
    pub bmAttributes: uint32_t,
}
/* * \ingroup libusb_desc
 * A structure representing the SuperSpeed USB Device Capability descriptor
 * This descriptor is documented in section 9.6.2.2 of the USB 3.0 specification.
 * All multiple-byte fields are represented in host-endian format.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_ss_usb_device_capability_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub bDevCapabilityType: uint8_t,
    pub bmAttributes: uint8_t,
    pub wSpeedSupported: uint16_t,
    pub bFunctionalitySupport: uint8_t,
    pub bU1DevExitLat: uint8_t,
    pub bU2DevExitLat: uint16_t,
}
/* * \ingroup libusb_desc
 * A structure representing the Container ID descriptor.
 * This descriptor is documented in section 9.6.2.3 of the USB 3.0 specification.
 * All multiple-byte fields, except UUIDs, are represented in host-endian format.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_container_id_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub bDevCapabilityType: uint8_t,
    pub bReserved: uint8_t,
    pub ContainerID: [uint8_t; 16],
}
/*
 * Internal header for libusb
 * Copyright © 2007-2009 Daniel Drake <dsd@gentoo.org>
 * Copyright © 2001 Johannes Erdfelt <johannes@erdfelt.com>
 * Copyright © 2019 Nathan Hjelm <hjelmn@cs.umm.edu>
 * Copyright © 2019 Google LLC. All rights reserved.
 * Copyright © 2020 Chris Dickens <christopher.a.dickens@gmail.com>
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
/* Not all C standard library headers define static_assert in assert.h
 * Additionally, Visual Studio treats static_assert as a keyword.
 */
/* The following is used to silence warnings for unused variables */
/* Macro to align a value up to the next multiple of the size of a pointer */
/* Internal abstraction for poll */
/* Internal abstraction for thread synchronization */
/* Inside the libusb code, mark all public functions as follows:
 *   return_type API_EXPORTED function_name(params) { ... }
 * But if the function returns a pointer, mark it as follows:
 *   DEFAULT_VISIBILITY return_type * LIBUSB_CALL function_name(params) { ... }
 * In the libusb public header, mark all declarations as:
 *   return_type LIBUSB_CALL function_name(params);
 */
/* Macro to decorate printf-like functions, in order to get
 * compiler warnings about format string mistakes.
 */
/* Backend specific capabilities */
/* Maximum number of bytes in a log line */
/* Terminator for log lines */
/* Get an entry from the list
 *  ptr - the address of this list_head element in "type"
 *  type - the data type that contains "member"
 *  member - the list_head element in "type"
 */
/* Get each entry from a list
 *  pos - A structure pointer has a "member" element
 *  head - list head
 *  member - the list_head element in "pos"
 *  type - the type of the first parameter
 */
/* Some platforms don't have this define */
/* defined(_MSC_VER) && (_MSC_VER < 1900) */
/* ENABLE_LOGGING */
/* ENABLE_LOGGING */
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
/* polling and timeouts */
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
/* The list of pollfds has been modified */
/* The user has interrupted the event handler */
/* A hotplug callback deregistration is pending */
/* Macros for managing event handling state */
/* Update the following function if new event sources are added */
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
/* Function called by backend during device initialization to convert
 * multi-byte fields in the device descriptor to host-endian format.
 */
/* in-memory transfer layout:
 *
 * 1. os private data
 * 2. struct usbi_transfer
 * 3. struct libusb_transfer (which includes iso packets) [variable size]
 *
 * from a libusb_transfer, you can get the usbi_transfer by rewinding the
 * appropriate number of bytes.
 */
/* Protected by usbi_transfer->lock */
/* Protected by the flying_stransfers_lock */
/* this lock is held during libusb_submit_transfer() and
 * libusb_cancel_transfer() (allowing the OS backend to prevent duplicate
 * cancellation, submission-during-cancellation, etc). the OS backend
 * should also take this lock in the handle_events path, to prevent the user
 * cancelling the transfer from another thread while you are processing
 * its completion (presumably there would be races within your OS backend
 * if this were possible).
 * Note paths taking both this and the flying_transfers_lock must
 * always take the flying_transfers_lock first */
/* Transfer successfully submitted by backend */
/* Cancellation was requested via libusb_cancel_transfer() */
/* Operation on the transfer failed because the device disappeared */
/* Set by backend submit_transfer() if the OS handles timeout */
/* The transfer timeout has been handled */
/* The transfer timeout was successfully processed */
/* All standard descriptors have these 2 fields in common */
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct usbi_descriptor_header {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
}
/* shared data and functions */
/* must come first */
/* accessor functions for structure private data */
/* device discovery */
/* we traverse usbfs without knowing how many devices we are going to find.
 * so we create this discovered_devs model which is similar to a linked-list
 * which grows when required. it can be freed once discovery has completed,
 * eliminating the need for a list node in the libusb_device structure
 * itself. */
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
 * Hotplug events */
/* * A device has been plugged in and is ready to use */
/* * A device has left and is no longer available.
 * It is the user's responsibility to call libusb_close on any handle associated with a disconnected device.
 * It is safe to call libusb_get_device_descriptor on a device that has left */
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
#[inline]
unsafe extern "C" fn libusb_get_string_descriptor(
    dev_handle: *mut libusb_device_handle,
    desc_index: uint8_t,
    langid: uint16_t,
    data: *mut libc::c_uchar,
    length: libc::c_int,
) -> libc::c_int {
    return libusb_control_transfer(
        dev_handle,
        LIBUSB_ENDPOINT_IN as libc::c_int as uint8_t,
        LIBUSB_REQUEST_GET_DESCRIPTOR as libc::c_int as uint8_t,
        ((LIBUSB_DT_STRING as libc::c_int) << 8 as libc::c_int | desc_index as libc::c_int)
            as uint16_t,
        langid,
        data,
        length as uint16_t,
        1000 as libc::c_int as libc::c_uint,
    );
}
#[inline]
unsafe extern "C" fn usbi_reallocf(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void {
    let ret: *mut libc::c_void = realloc(ptr, size);
    if ret.is_null() {
        free(ptr);
    }
    return ret;
}
#[inline]
unsafe extern "C" fn libusb_get_descriptor(
    dev_handle: *mut libusb_device_handle,
    desc_type: uint8_t,
    desc_index: uint8_t,
    data: *mut libc::c_uchar,
    length: libc::c_int,
) -> libc::c_int {
    return libusb_control_transfer(
        dev_handle,
        LIBUSB_ENDPOINT_IN as libc::c_int as uint8_t,
        LIBUSB_REQUEST_GET_DESCRIPTOR as libc::c_int as uint8_t,
        ((desc_type as libc::c_int) << 8 as libc::c_int | desc_index as libc::c_int) as uint16_t,
        0 as libc::c_int as uint16_t,
        data,
        length as uint16_t,
        1000 as libc::c_int as libc::c_uint,
    );
}
unsafe extern "C" fn parse_descriptor(
    source: *const libc::c_void,
    mut descriptor: *const libc::c_char,
    dest: *mut libc::c_void,
) {
    let mut sp: *const uint8_t = source as *const uint8_t;
    let mut dp: *mut uint8_t = dest as *mut uint8_t;
    let mut field_type: libc::c_char = 0;
    while *descriptor != 0 {
        let fresh0 = descriptor;
        descriptor = descriptor.offset(1);
        field_type = *fresh0;
        match field_type as libc::c_int {
            98 => {
                /* 8-bit byte */
                let fresh1 = sp; /* Align to word boundary */
                sp = sp.offset(1);
                let fresh2 = dp;
                dp = dp.offset(1);
                *fresh2 = *fresh1
            }
            119 => {
                /* 16-bit word, convert from little endian to CPU */
                dp = dp.offset((dp as uintptr_t & 1 as libc::c_int as libc::c_ulong) as isize); /* Align to word boundary */
                *(dp as *mut uint16_t) = ((*sp.offset(1 as libc::c_int as isize) as uint16_t
                    as libc::c_int)
                    << 8 as libc::c_int
                    | *sp.offset(0 as libc::c_int as isize) as uint16_t as libc::c_int)
                    as uint16_t;
                sp = sp.offset(2 as libc::c_int as isize);
                dp = dp.offset(2 as libc::c_int as isize)
            }
            100 => {
                /* 32-bit word, convert from little endian to CPU */
                dp = dp.offset((dp as uintptr_t & 1 as libc::c_int as libc::c_ulong) as isize);
                *(dp as *mut uint32_t) = (*sp.offset(3 as libc::c_int as isize) as uint32_t)
                    << 24 as libc::c_int
                    | (*sp.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
                    | (*sp.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
                    | *sp.offset(0 as libc::c_int as isize) as uint32_t;
                sp = sp.offset(4 as libc::c_int as isize);
                dp = dp.offset(4 as libc::c_int as isize)
            }
            117 => {
                /* 16 byte UUID */
                memcpy(
                    dp as *mut libc::c_void,
                    sp as *const libc::c_void,
                    16 as libc::c_int as libc::c_ulong,
                );
                sp = sp.offset(16 as libc::c_int as isize);
                dp = dp.offset(16 as libc::c_int as isize)
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn clear_endpoint(endpoint: *mut libusb_endpoint_descriptor) {
    free((*endpoint).extra as *mut libc::c_void);
}
unsafe extern "C" fn parse_endpoint(
    ctx: *mut libusb_context,
    mut endpoint: *mut libusb_endpoint_descriptor,
    mut buffer: *mut libc::c_uchar,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut header: *mut usbi_descriptor_header = 0 as *mut usbi_descriptor_header;
    let mut extra: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut begin: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut parsed: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0;
    if size < 2 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_endpoint\x00"))
                .as_ptr(),
            b"short endpoint descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
            size,
            2 as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    header = buffer as *mut usbi_descriptor_header;
    if (*header).bDescriptorType as libc::c_int != LIBUSB_DT_ENDPOINT as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_endpoint\x00"))
                .as_ptr(),
            b"unexpected descriptor %x (expected %x)\x00" as *const u8 as *const libc::c_char,
            (*header).bDescriptorType as libc::c_int,
            LIBUSB_DT_ENDPOINT as libc::c_int,
        );
        return parsed;
    } else {
        if (*header).bLength as libc::c_int > size {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_endpoint\x00"))
                    .as_ptr(),
                b"short endpoint descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
                size,
                (*header).bLength as libc::c_int,
            );
            return parsed;
        }
    }
    if (*header).bLength as libc::c_int >= 9 as libc::c_int {
        parse_descriptor(
            buffer as *const libc::c_void,
            b"bbbbwbbb\x00" as *const u8 as *const libc::c_char,
            endpoint as *mut libc::c_void,
        );
    } else if (*header).bLength as libc::c_int >= 7 as libc::c_int {
        parse_descriptor(
            buffer as *const libc::c_void,
            b"bbbbwb\x00" as *const u8 as *const libc::c_char,
            endpoint as *mut libc::c_void,
        );
    } else {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_endpoint\x00"))
                .as_ptr(),
            b"invalid endpoint bLength (%d)\x00" as *const u8 as *const libc::c_char,
            (*header).bLength as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    buffer = buffer.offset((*header).bLength as libc::c_int as isize);
    size -= (*header).bLength as libc::c_int;
    parsed += (*header).bLength as libc::c_int;
    /* Skip over the rest of the Class Specific or Vendor Specific */
    /*  descriptors */
    begin = buffer;
    while size >= 2 as libc::c_int {
        header = buffer as *mut usbi_descriptor_header;
        if ((*header).bLength as libc::c_int) < 2 as libc::c_int {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_endpoint\x00"))
                    .as_ptr(),
                b"invalid extra ep desc len (%d)\x00" as *const u8 as *const libc::c_char,
                (*header).bLength as libc::c_int,
            );
            return LIBUSB_ERROR_IO as libc::c_int;
        } else {
            if (*header).bLength as libc::c_int > size {
                usbi_log(
                    ctx,
                    LIBUSB_LOG_LEVEL_WARNING,
                    (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"parse_endpoint\x00",
                    ))
                    .as_ptr(),
                    b"short extra ep desc read %d/%d\x00" as *const u8 as *const libc::c_char,
                    size,
                    (*header).bLength as libc::c_int,
                );
                return parsed;
            }
        }
        /* If we find another "proper" descriptor then we're done  */
        if (*header).bDescriptorType as libc::c_int == LIBUSB_DT_ENDPOINT as libc::c_int
            || (*header).bDescriptorType as libc::c_int == LIBUSB_DT_INTERFACE as libc::c_int
            || (*header).bDescriptorType as libc::c_int == LIBUSB_DT_CONFIG as libc::c_int
            || (*header).bDescriptorType as libc::c_int == LIBUSB_DT_DEVICE as libc::c_int
        {
            break;
        }
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"parse_endpoint\x00"))
                .as_ptr(),
            b"skipping descriptor %x\x00" as *const u8 as *const libc::c_char,
            (*header).bDescriptorType as libc::c_int,
        );
        buffer = buffer.offset((*header).bLength as libc::c_int as isize);
        size -= (*header).bLength as libc::c_int;
        parsed += (*header).bLength as libc::c_int
    }
    /* Copy any unknown descriptors into a storage area for drivers */
    /*  to later parse */
    len = buffer.wrapping_offset_from(begin) as libc::c_long as libc::c_int;
    if len <= 0 as libc::c_int {
        return parsed;
    }
    extra = malloc(len as size_t) as *mut libc::c_uchar;
    if extra.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    memcpy(
        extra as *mut libc::c_void,
        begin as *const libc::c_void,
        len as libc::c_ulong,
    );
    (*endpoint).extra = extra;
    (*endpoint).extra_length = len;
    return parsed;
}
unsafe extern "C" fn clear_interface(mut usb_interface: *mut libusb_interface) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if !(*usb_interface).altsetting.is_null() {
        i = 0 as libc::c_int;
        while i < (*usb_interface).num_altsetting {
            let ifp: *mut libusb_interface_descriptor = ((*usb_interface).altsetting
                as *mut libusb_interface_descriptor)
                .offset(i as isize);
            free((*ifp).extra as *mut libc::c_void);
            if !(*ifp).endpoint.is_null() {
                j = 0 as libc::c_int;
                while j < (*ifp).bNumEndpoints as libc::c_int {
                    clear_endpoint(
                        ((*ifp).endpoint as *mut libusb_endpoint_descriptor).offset(j as isize),
                    );
                    j += 1
                }
            }
            free((*ifp).endpoint as *mut libc::c_void);
            i += 1
        }
    }
    free((*usb_interface).altsetting as *mut libc::c_void);
    (*usb_interface).altsetting = 0 as *const libusb_interface_descriptor;
}
unsafe extern "C" fn parse_interface(
    ctx: *mut libusb_context,
    mut usb_interface: *mut libusb_interface,
    mut buffer: *mut libc::c_uchar,
    mut size: libc::c_int,
) -> libc::c_int {
    let current_block: u64;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut parsed: libc::c_int = 0 as libc::c_int;
    let mut interface_number: libc::c_int = -(1 as libc::c_int);
    let mut header: *mut usbi_descriptor_header = 0 as *mut usbi_descriptor_header;
    let mut ifp: *mut libusb_interface_descriptor = 0 as *mut libusb_interface_descriptor;
    let mut extra: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut begin: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    's_29: loop {
        if !(size >= 9 as libc::c_int) {
            current_block = 17239133558811367971;
            break;
        }
        let mut altsetting: *mut libusb_interface_descriptor =
            0 as *mut libusb_interface_descriptor;
        altsetting = usbi_reallocf(
            (*usb_interface).altsetting as *mut libc::c_void,
            (::std::mem::size_of::<libusb_interface_descriptor>() as libc::c_ulong)
                .wrapping_mul(((*usb_interface).num_altsetting + 1 as libc::c_int) as size_t),
        ) as *mut libusb_interface_descriptor;
        if altsetting.is_null() {
            r = LIBUSB_ERROR_NO_MEM as libc::c_int;
            current_block = 12646643519710607562;
            break;
        } else {
            (*usb_interface).altsetting = altsetting;
            ifp = altsetting.offset((*usb_interface).num_altsetting as isize);
            parse_descriptor(
                buffer as *const libc::c_void,
                b"bbbbbbbbb\x00" as *const u8 as *const libc::c_char,
                ifp as *mut libc::c_void,
            );
            if (*ifp).bDescriptorType as libc::c_int != LIBUSB_DT_INTERFACE as libc::c_int {
                usbi_log(
                    ctx,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"parse_interface\x00",
                    ))
                    .as_ptr(),
                    b"unexpected descriptor %x (expected %x)\x00" as *const u8
                        as *const libc::c_char,
                    (*ifp).bDescriptorType as libc::c_int,
                    LIBUSB_DT_INTERFACE as libc::c_int,
                );
                return parsed;
            }
            if ((*ifp).bLength as libc::c_int) < 9 as libc::c_int {
                usbi_log(
                    ctx,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"parse_interface\x00",
                    ))
                    .as_ptr(),
                    b"invalid interface bLength (%d)\x00" as *const u8 as *const libc::c_char,
                    (*ifp).bLength as libc::c_int,
                );
                r = LIBUSB_ERROR_IO as libc::c_int;
                current_block = 12646643519710607562;
                break;
            } else {
                if (*ifp).bLength as libc::c_int > size {
                    usbi_log(
                        ctx,
                        LIBUSB_LOG_LEVEL_WARNING,
                        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                            b"parse_interface\x00",
                        ))
                        .as_ptr(),
                        b"short intf descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
                        size,
                        (*ifp).bLength as libc::c_int,
                    );
                    return parsed;
                }
                if (*ifp).bNumEndpoints as libc::c_int > 32 as libc::c_int {
                    usbi_log(
                        ctx,
                        LIBUSB_LOG_LEVEL_ERROR,
                        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                            b"parse_interface\x00",
                        ))
                        .as_ptr(),
                        b"too many endpoints (%d)\x00" as *const u8 as *const libc::c_char,
                        (*ifp).bNumEndpoints as libc::c_int,
                    );
                    r = LIBUSB_ERROR_IO as libc::c_int;
                    current_block = 12646643519710607562;
                    break;
                } else {
                    (*usb_interface).num_altsetting += 1;
                    (*ifp).extra = 0 as *const libc::c_uchar;
                    (*ifp).extra_length = 0 as libc::c_int;
                    (*ifp).endpoint = 0 as *const libusb_endpoint_descriptor;
                    if interface_number == -(1 as libc::c_int) {
                        interface_number = (*ifp).bInterfaceNumber as libc::c_int
                    }
                    /* Skip over the interface */
                    buffer = buffer.offset((*ifp).bLength as libc::c_int as isize);
                    parsed += (*ifp).bLength as libc::c_int;
                    size -= (*ifp).bLength as libc::c_int;
                    begin = buffer;
                    /* Skip over any interface, class or vendor descriptors */
                    while size >= 2 as libc::c_int {
                        header = buffer as *mut usbi_descriptor_header;
                        if ((*header).bLength as libc::c_int) < 2 as libc::c_int {
                            usbi_log(
                                ctx,
                                LIBUSB_LOG_LEVEL_ERROR,
                                (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                                    b"parse_interface\x00",
                                ))
                                .as_ptr(),
                                b"invalid extra intf desc len (%d)\x00" as *const u8
                                    as *const libc::c_char,
                                (*header).bLength as libc::c_int,
                            );
                            r = LIBUSB_ERROR_IO as libc::c_int;
                            current_block = 12646643519710607562;
                            break 's_29;
                        } else {
                            if (*header).bLength as libc::c_int > size {
                                usbi_log(
                                    ctx,
                                    LIBUSB_LOG_LEVEL_WARNING,
                                    (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                                        b"parse_interface\x00",
                                    ))
                                    .as_ptr(),
                                    b"short extra intf desc read %d/%d\x00" as *const u8
                                        as *const libc::c_char,
                                    size,
                                    (*header).bLength as libc::c_int,
                                );
                                return parsed;
                            }
                            /* If we find another "proper" descriptor then we're done */
                            if (*header).bDescriptorType as libc::c_int
                                == LIBUSB_DT_INTERFACE as libc::c_int
                                || (*header).bDescriptorType as libc::c_int
                                    == LIBUSB_DT_ENDPOINT as libc::c_int
                                || (*header).bDescriptorType as libc::c_int
                                    == LIBUSB_DT_CONFIG as libc::c_int
                                || (*header).bDescriptorType as libc::c_int
                                    == LIBUSB_DT_DEVICE as libc::c_int
                            {
                                break;
                            }
                            buffer = buffer.offset((*header).bLength as libc::c_int as isize);
                            parsed += (*header).bLength as libc::c_int;
                            size -= (*header).bLength as libc::c_int
                        }
                    }
                    /* Copy any unknown descriptors into a storage area for */
                    /*  drivers to later parse */
                    len = buffer.wrapping_offset_from(begin) as libc::c_long as libc::c_int;
                    if len > 0 as libc::c_int {
                        extra = malloc(len as size_t) as *mut libc::c_uchar;
                        if extra.is_null() {
                            r = LIBUSB_ERROR_NO_MEM as libc::c_int;
                            current_block = 12646643519710607562;
                            break;
                        } else {
                            memcpy(
                                extra as *mut libc::c_void,
                                begin as *const libc::c_void,
                                len as libc::c_ulong,
                            );
                            (*ifp).extra = extra;
                            (*ifp).extra_length = len
                        }
                    }
                    if (*ifp).bNumEndpoints as libc::c_int > 0 as libc::c_int {
                        let mut endpoint: *mut libusb_endpoint_descriptor =
                            0 as *mut libusb_endpoint_descriptor;
                        endpoint = calloc(
                            (*ifp).bNumEndpoints as libc::c_ulong,
                            ::std::mem::size_of::<libusb_endpoint_descriptor>() as libc::c_ulong,
                        ) as *mut libusb_endpoint_descriptor;
                        if endpoint.is_null() {
                            r = LIBUSB_ERROR_NO_MEM as libc::c_int;
                            current_block = 12646643519710607562;
                            break;
                        } else {
                            (*ifp).endpoint = endpoint;
                            i = 0 as libc::c_int;
                            while i < (*ifp).bNumEndpoints as libc::c_int {
                                r = parse_endpoint(ctx, endpoint.offset(i as isize), buffer, size);
                                if r < 0 as libc::c_int {
                                    current_block = 12646643519710607562;
                                    break 's_29;
                                }
                                if r == 0 as libc::c_int {
                                    (*ifp).bNumEndpoints = i as uint8_t;
                                    break;
                                } else {
                                    buffer = buffer.offset(r as isize);
                                    parsed += r;
                                    size -= r;
                                    i += 1
                                }
                            }
                        }
                    }
                    /* We check to see if it's an alternate to this one */
                    ifp = buffer as *mut libusb_interface_descriptor;
                    if size < 9 as libc::c_int
                        || (*ifp).bDescriptorType as libc::c_int
                            != LIBUSB_DT_INTERFACE as libc::c_int
                        || (*ifp).bInterfaceNumber as libc::c_int != interface_number
                    {
                        return parsed;
                    }
                }
            }
        }
    }
    match current_block {
        17239133558811367971 => return parsed,
        _ => {
            clear_interface(usb_interface);
            return r;
        }
    };
}
unsafe extern "C" fn clear_configuration(config: *mut libusb_config_descriptor) {
    let mut i: libc::c_int = 0;
    if !(*config).interface.is_null() {
        i = 0 as libc::c_int;
        while i < (*config).bNumInterfaces as libc::c_int {
            clear_interface(((*config).interface as *mut libusb_interface).offset(i as isize));
            i += 1
        }
    }
    free((*config).interface as *mut libc::c_void);
    free((*config).extra as *mut libc::c_void);
}
unsafe extern "C" fn parse_configuration(
    ctx: *mut libusb_context,
    mut config: *mut libusb_config_descriptor,
    mut buffer: *mut libc::c_uchar,
    mut size: libc::c_int,
) -> libc::c_int {
    let current_block: u64;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut header: *mut usbi_descriptor_header = 0 as *mut usbi_descriptor_header;
    let mut usb_interface: *mut libusb_interface = 0 as *mut libusb_interface;
    let mut extra: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if size < 9 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"parse_configuration\x00"))
                .as_ptr(),
            b"short config descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
            size,
            9 as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    parse_descriptor(
        buffer as *const libc::c_void,
        b"bbwbbbbb\x00" as *const u8 as *const libc::c_char,
        config as *mut libc::c_void,
    );
    if (*config).bDescriptorType as libc::c_int != LIBUSB_DT_CONFIG as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"parse_configuration\x00"))
                .as_ptr(),
            b"unexpected descriptor %x (expected %x)\x00" as *const u8 as *const libc::c_char,
            (*config).bDescriptorType as libc::c_int,
            LIBUSB_DT_CONFIG as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    if ((*config).bLength as libc::c_int) < 9 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"parse_configuration\x00"))
                .as_ptr(),
            b"invalid config bLength (%d)\x00" as *const u8 as *const libc::c_char,
            (*config).bLength as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    if (*config).bLength as libc::c_int > size {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"parse_configuration\x00"))
                .as_ptr(),
            b"short config descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
            size,
            (*config).bLength as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    if (*config).bNumInterfaces as libc::c_int > 32 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"parse_configuration\x00"))
                .as_ptr(),
            b"too many interfaces (%d)\x00" as *const u8 as *const libc::c_char,
            (*config).bNumInterfaces as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    usb_interface = calloc(
        (*config).bNumInterfaces as libc::c_ulong,
        ::std::mem::size_of::<libusb_interface>() as libc::c_ulong,
    ) as *mut libusb_interface;
    if usb_interface.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    (*config).interface = usb_interface;
    buffer = buffer.offset((*config).bLength as libc::c_int as isize);
    size -= (*config).bLength as libc::c_int;
    i = 0 as libc::c_int;
    's_126: loop {
        if !(i < (*config).bNumInterfaces as libc::c_int) {
            current_block = 6721012065216013753;
            break;
        }
        let mut len: libc::c_int = 0;
        let mut begin: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        /* Skip over the rest of the Class Specific or Vendor */
        /*  Specific descriptors */
        begin = buffer;
        while size >= 2 as libc::c_int {
            header = buffer as *mut usbi_descriptor_header;
            if ((*header).bLength as libc::c_int) < 2 as libc::c_int {
                usbi_log(
                    ctx,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                        b"parse_configuration\x00",
                    ))
                    .as_ptr(),
                    b"invalid extra config desc len (%d)\x00" as *const u8 as *const libc::c_char,
                    (*header).bLength as libc::c_int,
                );
                r = LIBUSB_ERROR_IO as libc::c_int;
                current_block = 16948900988800720545;
                break 's_126;
            } else {
                if (*header).bLength as libc::c_int > size {
                    usbi_log(
                        ctx,
                        LIBUSB_LOG_LEVEL_WARNING,
                        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                            b"parse_configuration\x00",
                        ))
                        .as_ptr(),
                        b"short extra config desc read %d/%d\x00" as *const u8
                            as *const libc::c_char,
                        size,
                        (*header).bLength as libc::c_int,
                    );
                    (*config).bNumInterfaces = i as uint8_t;
                    return size;
                }
                /* If we find another "proper" descriptor then we're done */
                if (*header).bDescriptorType as libc::c_int == LIBUSB_DT_ENDPOINT as libc::c_int
                    || (*header).bDescriptorType as libc::c_int
                        == LIBUSB_DT_INTERFACE as libc::c_int
                    || (*header).bDescriptorType as libc::c_int == LIBUSB_DT_CONFIG as libc::c_int
                    || (*header).bDescriptorType as libc::c_int == LIBUSB_DT_DEVICE as libc::c_int
                {
                    break;
                }
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                        b"parse_configuration\x00",
                    ))
                    .as_ptr(),
                    b"skipping descriptor 0x%x\x00" as *const u8 as *const libc::c_char,
                    (*header).bDescriptorType as libc::c_int,
                );
                buffer = buffer.offset((*header).bLength as libc::c_int as isize);
                size -= (*header).bLength as libc::c_int
            }
        }
        /* Copy any unknown descriptors into a storage area for */
        /*  drivers to later parse */
        len = buffer.wrapping_offset_from(begin) as libc::c_long as libc::c_int;
        if len > 0 as libc::c_int {
            /* FIXME: We should realloc and append here */
            if (*config).extra_length == 0 {
                extra = malloc(len as size_t) as *mut libc::c_uchar;
                if extra.is_null() {
                    r = LIBUSB_ERROR_NO_MEM as libc::c_int;
                    current_block = 16948900988800720545;
                    break;
                } else {
                    memcpy(
                        extra as *mut libc::c_void,
                        begin as *const libc::c_void,
                        len as libc::c_ulong,
                    );
                    (*config).extra = extra;
                    (*config).extra_length = len
                }
            }
        }
        r = parse_interface(ctx, usb_interface.offset(i as isize), buffer, size);
        if r < 0 as libc::c_int {
            current_block = 16948900988800720545;
            break;
        }
        if r == 0 as libc::c_int {
            (*config).bNumInterfaces = i as uint8_t;
            current_block = 6721012065216013753;
            break;
        } else {
            buffer = buffer.offset(r as isize);
            size -= r;
            i += 1
        }
    }
    match current_block {
        6721012065216013753 => return size,
        _ => {
            clear_configuration(config);
            return r;
        }
    };
}
unsafe extern "C" fn raw_desc_to_config(
    ctx: *mut libusb_context,
    buf: *mut libc::c_uchar,
    size: libc::c_int,
    config: *mut *mut libusb_config_descriptor,
) -> libc::c_int {
    let mut _config: *mut libusb_config_descriptor = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<libusb_config_descriptor>() as libc::c_ulong,
    ) as *mut libusb_config_descriptor;
    let mut r: libc::c_int = 0;
    if _config.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    r = parse_configuration(ctx, _config, buf, size);
    if r < 0 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"raw_desc_to_config\x00"))
                .as_ptr(),
            b"parse_configuration failed with error %d\x00" as *const u8 as *const libc::c_char,
            r,
        );
        free(_config as *mut libc::c_void);
        return r;
    } else {
        if r > 0 as libc::c_int {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"raw_desc_to_config\x00",
                ))
                .as_ptr(),
                b"still %d bytes of descriptor data left\x00" as *const u8 as *const libc::c_char,
                r,
            );
        }
    }
    *config = _config;
    return LIBUSB_SUCCESS as libc::c_int;
}
/* * \ingroup libusb_desc
 * Get the USB device descriptor for a given device.
 *
 * This is a non-blocking function; the device descriptor is cached in memory.
 *
 * Note since libusb-1.0.16, \ref LIBUSB_API_VERSION >= 0x01000102, this
 * function always succeeds.
 *
 * \param dev the device
 * \param desc output location for the descriptor data
 * \returns 0 on success or a LIBUSB_ERROR code on failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_device_descriptor(
    dev: *mut libusb_device,
    desc: *mut libusb_device_descriptor,
) -> libc::c_int {
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
            b"libusb_get_device_descriptor\x00",
        ))
        .as_ptr(),
        b" \x00" as *const u8 as *const libc::c_char,
    );
    *desc = (*dev).device_descriptor;
    return 0 as libc::c_int;
}
/* * \ingroup libusb_desc
 * Get the USB configuration descriptor for the currently active configuration.
 * This is a non-blocking function which does not involve any requests being
 * sent to the device.
 *
 * \param dev a device
 * \param config output location for the USB configuration descriptor. Only
 * valid if 0 was returned. Must be freed with libusb_free_config_descriptor()
 * after use.
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if the device is in unconfigured state
 * \returns another LIBUSB_ERROR code on error
 * \see libusb_get_config_descriptor
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_active_config_descriptor(
    dev: *mut libusb_device,
    config: *mut *mut libusb_config_descriptor,
) -> libc::c_int {
    let mut _config: libusb_config_descriptor = libusb_config_descriptor {
        bLength: 0,
        bDescriptorType: 0,
        wTotalLength: 0,
        bNumInterfaces: 0,
        bConfigurationValue: 0,
        iConfiguration: 0,
        bmAttributes: 0,
        MaxPower: 0,
        interface: 0 as *const libusb_interface,
        extra: 0 as *const libc::c_uchar,
        extra_length: 0,
    };
    let mut tmp: [libc::c_uchar; 9] = [0; 9];
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut r: libc::c_int = 0;
    r = usbi_backend
        .get_active_config_descriptor
        .expect("non-null function pointer")(
        dev,
        tmp.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 9]>() as libc::c_ulong,
    );
    if r < 0 as libc::c_int {
        return r;
    }
    if r < 9 as libc::c_int {
        usbi_log(
            (*dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 36], &[libc::c_char; 36]>(
                b"libusb_get_active_config_descriptor\x00",
            ))
            .as_ptr(),
            b"short config descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
            r,
            9 as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    parse_descriptor(
        tmp.as_mut_ptr() as *const libc::c_void,
        b"bbw\x00" as *const u8 as *const libc::c_char,
        &mut _config as *mut libusb_config_descriptor as *mut libc::c_void,
    );
    buf = malloc(_config.wTotalLength as libc::c_ulong) as *mut libc::c_uchar;
    if buf.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    r = usbi_backend
        .get_active_config_descriptor
        .expect("non-null function pointer")(
        dev,
        buf as *mut libc::c_void,
        _config.wTotalLength as size_t,
    );
    if r >= 0 as libc::c_int {
        r = raw_desc_to_config((*dev).ctx, buf, r, config)
    }
    free(buf as *mut libc::c_void);
    return r;
}
/* * \ingroup libusb_desc
 * Get a USB configuration descriptor based on its index.
 * This is a non-blocking function which does not involve any requests being
 * sent to the device.
 *
 * \param dev a device
 * \param config_index the index of the configuration you wish to retrieve
 * \param config output location for the USB configuration descriptor. Only
 * valid if 0 was returned. Must be freed with libusb_free_config_descriptor()
 * after use.
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if the configuration does not exist
 * \returns another LIBUSB_ERROR code on error
 * \see libusb_get_active_config_descriptor()
 * \see libusb_get_config_descriptor_by_value()
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_config_descriptor(
    dev: *mut libusb_device,
    config_index: uint8_t,
    config: *mut *mut libusb_config_descriptor,
) -> libc::c_int {
    let mut _config: libusb_config_descriptor = libusb_config_descriptor {
        bLength: 0,
        bDescriptorType: 0,
        wTotalLength: 0,
        bNumInterfaces: 0,
        bConfigurationValue: 0,
        iConfiguration: 0,
        bmAttributes: 0,
        MaxPower: 0,
        interface: 0 as *const libusb_interface,
        extra: 0 as *const libc::c_uchar,
        extra_length: 0,
    };
    let mut tmp: [libc::c_uchar; 9] = [0; 9];
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut r: libc::c_int = 0;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
            b"libusb_get_config_descriptor\x00",
        ))
        .as_ptr(),
        b"index %d\x00" as *const u8 as *const libc::c_char,
        config_index as libc::c_int,
    );
    if config_index as libc::c_int >= (*dev).device_descriptor.bNumConfigurations as libc::c_int {
        return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
    }
    r = usbi_backend
        .get_config_descriptor
        .expect("non-null function pointer")(
        dev,
        config_index,
        tmp.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 9]>() as libc::c_ulong,
    );
    if r < 0 as libc::c_int {
        return r;
    }
    if r < 9 as libc::c_int {
        usbi_log(
            (*dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"libusb_get_config_descriptor\x00",
            ))
            .as_ptr(),
            b"short config descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
            r,
            9 as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    parse_descriptor(
        tmp.as_mut_ptr() as *const libc::c_void,
        b"bbw\x00" as *const u8 as *const libc::c_char,
        &mut _config as *mut libusb_config_descriptor as *mut libc::c_void,
    );
    buf = malloc(_config.wTotalLength as libc::c_ulong) as *mut libc::c_uchar;
    if buf.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    r = usbi_backend
        .get_config_descriptor
        .expect("non-null function pointer")(
        dev,
        config_index,
        buf as *mut libc::c_void,
        _config.wTotalLength as size_t,
    );
    if r >= 0 as libc::c_int {
        r = raw_desc_to_config((*dev).ctx, buf, r, config)
    }
    free(buf as *mut libc::c_void);
    return r;
}
/* * \ingroup libusb_desc
 * Get a USB configuration descriptor with a specific bConfigurationValue.
 * This is a non-blocking function which does not involve any requests being
 * sent to the device.
 *
 * \param dev a device
 * \param bConfigurationValue the bConfigurationValue of the configuration you
 * wish to retrieve
 * \param config output location for the USB configuration descriptor. Only
 * valid if 0 was returned. Must be freed with libusb_free_config_descriptor()
 * after use.
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if the configuration does not exist
 * \returns another LIBUSB_ERROR code on error
 * \see libusb_get_active_config_descriptor()
 * \see libusb_get_config_descriptor()
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_config_descriptor_by_value(
    dev: *mut libusb_device,
    bConfigurationValue: uint8_t,
    config: *mut *mut libusb_config_descriptor,
) -> libc::c_int {
    let mut idx: uint8_t = 0;
    let mut r: libc::c_int = 0;
    if usbi_backend.get_config_descriptor_by_value.is_some() {
        let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
        r = usbi_backend
            .get_config_descriptor_by_value
            .expect("non-null function pointer")(dev, bConfigurationValue, &mut buf);
        if r < 0 as libc::c_int {
            return r;
        }
        return raw_desc_to_config((*dev).ctx, buf as *mut libc::c_uchar, r, config);
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
            b"libusb_get_config_descriptor_by_value\x00",
        ))
        .as_ptr(),
        b"value %u\x00" as *const u8 as *const libc::c_char,
        bConfigurationValue as libc::c_int,
    );
    idx = 0 as libc::c_int as uint8_t;
    while (idx as libc::c_int) < (*dev).device_descriptor.bNumConfigurations as libc::c_int {
        let mut tmp: [libc::c_uchar; 6] = [0; 6];
        r = usbi_backend
            .get_config_descriptor
            .expect("non-null function pointer")(
            dev,
            idx,
            tmp.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 6]>() as libc::c_ulong,
        );
        if r < 0 as libc::c_int {
            return r;
        }
        if tmp[5 as libc::c_int as usize] as libc::c_int == bConfigurationValue as libc::c_int {
            return libusb_get_config_descriptor(dev, idx, config);
        }
        idx = idx.wrapping_add(1)
    }
    return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
}
/* * \ingroup libusb_desc
 * Free a configuration descriptor obtained from
 * libusb_get_active_config_descriptor() or libusb_get_config_descriptor().
 * It is safe to call this function with a NULL config parameter, in which
 * case the function simply returns.
 *
 * \param config the configuration descriptor to free
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_free_config_descriptor(config: *mut libusb_config_descriptor) {
    if config.is_null() {
        return;
    }
    clear_configuration(config);
    free(config as *mut libc::c_void);
}
/* * \ingroup libusb_desc
 * Get an endpoints superspeed endpoint companion descriptor (if any)
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param endpoint endpoint descriptor from which to get the superspeed
 * endpoint companion descriptor
 * \param ep_comp output location for the superspeed endpoint companion
 * descriptor. Only valid if 0 was returned. Must be freed with
 * libusb_free_ss_endpoint_companion_descriptor() after use.
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if the configuration does not exist
 * \returns another LIBUSB_ERROR code on error
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_ss_endpoint_companion_descriptor(
    ctx: *mut libusb_context,
    endpoint: *const libusb_endpoint_descriptor,
    ep_comp: *mut *mut libusb_ss_endpoint_companion_descriptor,
) -> libc::c_int {
    let mut header: *mut usbi_descriptor_header = 0 as *mut usbi_descriptor_header;
    let mut size: libc::c_int = (*endpoint).extra_length;
    let mut buffer: *const libc::c_uchar = (*endpoint).extra;
    *ep_comp = 0 as *mut libusb_ss_endpoint_companion_descriptor;
    while size >= 2 as libc::c_int {
        header = buffer as *mut usbi_descriptor_header;
        if ((*header).bLength as libc::c_int) < 2 as libc::c_int
            || (*header).bLength as libc::c_int > size
        {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                    b"libusb_get_ss_endpoint_companion_descriptor\x00",
                ))
                .as_ptr(),
                b"invalid descriptor length %d\x00" as *const u8 as *const libc::c_char,
                (*header).bLength as libc::c_int,
            );
            return LIBUSB_ERROR_IO as libc::c_int;
        }
        if (*header).bDescriptorType as libc::c_int
            != LIBUSB_DT_SS_ENDPOINT_COMPANION as libc::c_int
        {
            buffer = buffer.offset((*header).bLength as libc::c_int as isize);
            size -= (*header).bLength as libc::c_int
        } else {
            if ((*header).bLength as libc::c_int) < 6 as libc::c_int {
                usbi_log(
                    ctx,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                        b"libusb_get_ss_endpoint_companion_descriptor\x00",
                    ))
                    .as_ptr(),
                    b"invalid ss-ep-comp-desc length %d\x00" as *const u8 as *const libc::c_char,
                    (*header).bLength as libc::c_int,
                );
                return LIBUSB_ERROR_IO as libc::c_int;
            }
            *ep_comp = malloc(
                ::std::mem::size_of::<libusb_ss_endpoint_companion_descriptor>() as libc::c_ulong,
            ) as *mut libusb_ss_endpoint_companion_descriptor;
            if (*ep_comp).is_null() {
                return LIBUSB_ERROR_NO_MEM as libc::c_int;
            }
            parse_descriptor(
                buffer as *const libc::c_void,
                b"bbbbw\x00" as *const u8 as *const libc::c_char,
                *ep_comp as *mut libc::c_void,
            );
            return LIBUSB_SUCCESS as libc::c_int;
        }
    }
    return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
}
/* * \ingroup libusb_desc
 * Free a superspeed endpoint companion descriptor obtained from
 * libusb_get_ss_endpoint_companion_descriptor().
 * It is safe to call this function with a NULL ep_comp parameter, in which
 * case the function simply returns.
 *
 * \param ep_comp the superspeed endpoint companion descriptor to free
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_free_ss_endpoint_companion_descriptor(
    ep_comp: *mut libusb_ss_endpoint_companion_descriptor,
) {
    free(ep_comp as *mut libc::c_void);
}
unsafe extern "C" fn parse_bos(
    ctx: *mut libusb_context,
    bos: *mut *mut libusb_bos_descriptor,
    mut buffer: *mut libc::c_uchar,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut bos_header: libusb_bos_descriptor = libusb_bos_descriptor {
        bLength: 0,
        bDescriptorType: 0,
        wTotalLength: 0,
        bNumDeviceCaps: 0,
        dev_capability: [],
    };
    let mut _bos: *mut libusb_bos_descriptor = 0 as *mut libusb_bos_descriptor;
    let mut dev_cap: libusb_bos_dev_capability_descriptor = libusb_bos_dev_capability_descriptor {
        bLength: 0,
        bDescriptorType: 0,
        bDevCapabilityType: 0,
        dev_capability_data: [],
    };
    let mut i: libc::c_int = 0;
    if size < 5 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"parse_bos\x00")).as_ptr(),
            b"short bos descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
            size,
            5 as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    parse_descriptor(
        buffer as *const libc::c_void,
        b"bbwb\x00" as *const u8 as *const libc::c_char,
        &mut bos_header as *mut libusb_bos_descriptor as *mut libc::c_void,
    );
    if bos_header.bDescriptorType as libc::c_int != LIBUSB_DT_BOS as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"parse_bos\x00")).as_ptr(),
            b"unexpected descriptor %x (expected %x)\x00" as *const u8 as *const libc::c_char,
            bos_header.bDescriptorType as libc::c_int,
            LIBUSB_DT_BOS as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    if (bos_header.bLength as libc::c_int) < 5 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"parse_bos\x00")).as_ptr(),
            b"invalid bos bLength (%d)\x00" as *const u8 as *const libc::c_char,
            bos_header.bLength as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    if bos_header.bLength as libc::c_int > size {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"parse_bos\x00")).as_ptr(),
            b"short bos descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
            size,
            bos_header.bLength as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    _bos = calloc(
        1 as libc::c_int as libc::c_ulong,
        (::std::mem::size_of::<libusb_bos_descriptor>() as libc::c_ulong).wrapping_add(
            (bos_header.bNumDeviceCaps as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
        ),
    ) as *mut libusb_bos_descriptor;
    if _bos.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    parse_descriptor(
        buffer as *const libc::c_void,
        b"bbwb\x00" as *const u8 as *const libc::c_char,
        _bos as *mut libc::c_void,
    );
    buffer = buffer.offset(bos_header.bLength as libc::c_int as isize);
    size -= bos_header.bLength as libc::c_int;
    /* Get the device capability descriptors */
    i = 0 as libc::c_int;
    while i < bos_header.bNumDeviceCaps as libc::c_int {
        if size < 3 as libc::c_int {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"parse_bos\x00"))
                    .as_ptr(),
                b"short dev-cap descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
                size,
                3 as libc::c_int,
            );
            break;
        } else {
            parse_descriptor(
                buffer as *const libc::c_void,
                b"bbb\x00" as *const u8 as *const libc::c_char,
                &mut dev_cap as *mut libusb_bos_dev_capability_descriptor as *mut libc::c_void,
            );
            if dev_cap.bDescriptorType as libc::c_int != LIBUSB_DT_DEVICE_CAPABILITY as libc::c_int
            {
                usbi_log(
                    ctx,
                    LIBUSB_LOG_LEVEL_WARNING,
                    (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(b"parse_bos\x00"))
                        .as_ptr(),
                    b"unexpected descriptor %x (expected %x)\x00" as *const u8
                        as *const libc::c_char,
                    dev_cap.bDescriptorType as libc::c_int,
                    LIBUSB_DT_DEVICE_CAPABILITY as libc::c_int,
                );
                break;
            } else {
                if (dev_cap.bLength as libc::c_int) < 3 as libc::c_int {
                    usbi_log(
                        ctx,
                        LIBUSB_LOG_LEVEL_ERROR,
                        (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(
                            b"parse_bos\x00",
                        ))
                        .as_ptr(),
                        b"invalid dev-cap bLength (%d)\x00" as *const u8 as *const libc::c_char,
                        dev_cap.bLength as libc::c_int,
                    );
                    libusb_free_bos_descriptor(_bos);
                    return LIBUSB_ERROR_IO as libc::c_int;
                }
                if dev_cap.bLength as libc::c_int > size {
                    usbi_log(
                        ctx,
                        LIBUSB_LOG_LEVEL_WARNING,
                        (*::std::mem::transmute::<&[u8; 10], &[libc::c_char; 10]>(
                            b"parse_bos\x00",
                        ))
                        .as_ptr(),
                        b"short dev-cap descriptor read %d/%d\x00" as *const u8
                            as *const libc::c_char,
                        size,
                        dev_cap.bLength as libc::c_int,
                    );
                    break;
                } else {
                    let ref mut fresh3 = *(*_bos).dev_capability.as_mut_ptr().offset(i as isize);
                    *fresh3 = malloc(dev_cap.bLength as libc::c_ulong)
                        as *mut libusb_bos_dev_capability_descriptor;
                    if (*(*_bos).dev_capability.as_mut_ptr().offset(i as isize)).is_null() {
                        libusb_free_bos_descriptor(_bos);
                        return LIBUSB_ERROR_NO_MEM as libc::c_int;
                    }
                    memcpy(
                        *(*_bos).dev_capability.as_mut_ptr().offset(i as isize)
                            as *mut libc::c_void,
                        buffer as *const libc::c_void,
                        dev_cap.bLength as libc::c_ulong,
                    );
                    buffer = buffer.offset(dev_cap.bLength as libc::c_int as isize);
                    size -= dev_cap.bLength as libc::c_int;
                    i += 1
                }
            }
        }
    }
    (*_bos).bNumDeviceCaps = i as uint8_t;
    *bos = _bos;
    return LIBUSB_SUCCESS as libc::c_int;
}
/* * \ingroup libusb_desc
 * Get a Binary Object Store (BOS) descriptor
 * This is a BLOCKING function, which will send requests to the device.
 *
 * \param dev_handle the handle of an open libusb device
 * \param bos output location for the BOS descriptor. Only valid if 0 was returned.
 * Must be freed with \ref libusb_free_bos_descriptor() after use.
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if the device doesn't have a BOS descriptor
 * \returns another LIBUSB_ERROR code on error
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_bos_descriptor(
    dev_handle: *mut libusb_device_handle,
    bos: *mut *mut libusb_bos_descriptor,
) -> libc::c_int {
    let mut _bos: libusb_bos_descriptor = libusb_bos_descriptor {
        bLength: 0,
        bDescriptorType: 0,
        wTotalLength: 0,
        bNumDeviceCaps: 0,
        dev_capability: [],
    };
    let mut bos_header: [uint8_t; 5] = [0 as libc::c_int as uint8_t, 0, 0, 0, 0];
    let mut bos_data: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut r: libc::c_int = 0;
    /* Read the BOS. This generates 2 requests on the bus,
     * one for the header, and one for the full BOS */
    r = libusb_get_descriptor(
        dev_handle,
        LIBUSB_DT_BOS as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        bos_header.as_mut_ptr(),
        5 as libc::c_int,
    );
    if r < 0 as libc::c_int {
        if r != LIBUSB_ERROR_PIPE as libc::c_int {
            usbi_log(
                (*(*dev_handle).dev).ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"libusb_get_bos_descriptor\x00",
                ))
                .as_ptr(),
                b"failed to read BOS (%d)\x00" as *const u8 as *const libc::c_char,
                r,
            );
        }
        return r;
    }
    if r < 5 as libc::c_int {
        usbi_log(
            (*(*dev_handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"libusb_get_bos_descriptor\x00",
            ))
            .as_ptr(),
            b"short BOS read %d/%d\x00" as *const u8 as *const libc::c_char,
            r,
            5 as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    parse_descriptor(
        bos_header.as_mut_ptr() as *const libc::c_void,
        b"bbwb\x00" as *const u8 as *const libc::c_char,
        &mut _bos as *mut libusb_bos_descriptor as *mut libc::c_void,
    );
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
            b"libusb_get_bos_descriptor\x00",
        ))
        .as_ptr(),
        b"found BOS descriptor: size %d bytes, %d capabilities\x00" as *const u8
            as *const libc::c_char,
        _bos.wTotalLength as libc::c_int,
        _bos.bNumDeviceCaps as libc::c_int,
    );
    bos_data = calloc(
        1 as libc::c_int as libc::c_ulong,
        _bos.wTotalLength as libc::c_ulong,
    ) as *mut libc::c_uchar;
    if bos_data.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    r = libusb_get_descriptor(
        dev_handle,
        LIBUSB_DT_BOS as libc::c_int as uint8_t,
        0 as libc::c_int as uint8_t,
        bos_data,
        _bos.wTotalLength as libc::c_int,
    );
    if r >= 0 as libc::c_int {
        r = parse_bos((*(*dev_handle).dev).ctx, bos, bos_data, r)
    } else {
        usbi_log(
            (*(*dev_handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"libusb_get_bos_descriptor\x00",
            ))
            .as_ptr(),
            b"failed to read BOS (%d)\x00" as *const u8 as *const libc::c_char,
            r,
        );
    }
    free(bos_data as *mut libc::c_void);
    return r;
}
/* * \ingroup libusb_desc
 * Free a BOS descriptor obtained from libusb_get_bos_descriptor().
 * It is safe to call this function with a NULL bos parameter, in which
 * case the function simply returns.
 *
 * \param bos the BOS descriptor to free
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_free_bos_descriptor(bos: *mut libusb_bos_descriptor) {
    let mut i: libc::c_int = 0;
    if bos.is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < (*bos).bNumDeviceCaps as libc::c_int {
        free(*(*bos).dev_capability.as_mut_ptr().offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    free(bos as *mut libc::c_void);
}
/* * \ingroup libusb_desc
 * Get an USB 2.0 Extension descriptor
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param dev_cap Device Capability descriptor with a bDevCapabilityType of
 * \ref libusb_capability_type::LIBUSB_BT_USB_2_0_EXTENSION
 * LIBUSB_BT_USB_2_0_EXTENSION
 * \param usb_2_0_extension output location for the USB 2.0 Extension
 * descriptor. Only valid if 0 was returned. Must be freed with
 * libusb_free_usb_2_0_extension_descriptor() after use.
 * \returns 0 on success
 * \returns a LIBUSB_ERROR code on error
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_usb_2_0_extension_descriptor(
    ctx: *mut libusb_context,
    dev_cap: *mut libusb_bos_dev_capability_descriptor,
    usb_2_0_extension: *mut *mut libusb_usb_2_0_extension_descriptor,
) -> libc::c_int {
    let mut _usb_2_0_extension: *mut libusb_usb_2_0_extension_descriptor =
        0 as *mut libusb_usb_2_0_extension_descriptor;
    if (*dev_cap).bDevCapabilityType as libc::c_int != LIBUSB_BT_USB_2_0_EXTENSION as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"libusb_get_usb_2_0_extension_descriptor\x00",
            ))
            .as_ptr(),
            b"unexpected bDevCapabilityType %x (expected %x)\x00" as *const u8
                as *const libc::c_char,
            (*dev_cap).bDevCapabilityType as libc::c_int,
            LIBUSB_BT_USB_2_0_EXTENSION as libc::c_int,
        );
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    if ((*dev_cap).bLength as libc::c_int) < 7 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"libusb_get_usb_2_0_extension_descriptor\x00",
            ))
            .as_ptr(),
            b"short dev-cap descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
            (*dev_cap).bLength as libc::c_int,
            7 as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    _usb_2_0_extension =
        malloc(::std::mem::size_of::<libusb_usb_2_0_extension_descriptor>() as libc::c_ulong)
            as *mut libusb_usb_2_0_extension_descriptor;
    if _usb_2_0_extension.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    parse_descriptor(
        dev_cap as *const libc::c_void,
        b"bbbd\x00" as *const u8 as *const libc::c_char,
        _usb_2_0_extension as *mut libc::c_void,
    );
    *usb_2_0_extension = _usb_2_0_extension;
    return LIBUSB_SUCCESS as libc::c_int;
}
/* * \ingroup libusb_desc
 * Free a USB 2.0 Extension descriptor obtained from
 * libusb_get_usb_2_0_extension_descriptor().
 * It is safe to call this function with a NULL usb_2_0_extension parameter,
 * in which case the function simply returns.
 *
 * \param usb_2_0_extension the USB 2.0 Extension descriptor to free
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_free_usb_2_0_extension_descriptor(
    usb_2_0_extension: *mut libusb_usb_2_0_extension_descriptor,
) {
    free(usb_2_0_extension as *mut libc::c_void);
}
/* * \ingroup libusb_desc
 * Get a SuperSpeed USB Device Capability descriptor
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param dev_cap Device Capability descriptor with a bDevCapabilityType of
 * \ref libusb_capability_type::LIBUSB_BT_SS_USB_DEVICE_CAPABILITY
 * LIBUSB_BT_SS_USB_DEVICE_CAPABILITY
 * \param ss_usb_device_cap output location for the SuperSpeed USB Device
 * Capability descriptor. Only valid if 0 was returned. Must be freed with
 * libusb_free_ss_usb_device_capability_descriptor() after use.
 * \returns 0 on success
 * \returns a LIBUSB_ERROR code on error
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_ss_usb_device_capability_descriptor(
    ctx: *mut libusb_context,
    dev_cap: *mut libusb_bos_dev_capability_descriptor,
    ss_usb_device_cap: *mut *mut libusb_ss_usb_device_capability_descriptor,
) -> libc::c_int {
    let mut _ss_usb_device_cap: *mut libusb_ss_usb_device_capability_descriptor =
        0 as *mut libusb_ss_usb_device_capability_descriptor;
    if (*dev_cap).bDevCapabilityType as libc::c_int
        != LIBUSB_BT_SS_USB_DEVICE_CAPABILITY as libc::c_int
    {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"libusb_get_ss_usb_device_capability_descriptor\x00",
            ))
            .as_ptr(),
            b"unexpected bDevCapabilityType %x (expected %x)\x00" as *const u8
                as *const libc::c_char,
            (*dev_cap).bDevCapabilityType as libc::c_int,
            LIBUSB_BT_SS_USB_DEVICE_CAPABILITY as libc::c_int,
        );
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    if ((*dev_cap).bLength as libc::c_int) < 10 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 47], &[libc::c_char; 47]>(
                b"libusb_get_ss_usb_device_capability_descriptor\x00",
            ))
            .as_ptr(),
            b"short dev-cap descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
            (*dev_cap).bLength as libc::c_int,
            10 as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    _ss_usb_device_cap = malloc(
        ::std::mem::size_of::<libusb_ss_usb_device_capability_descriptor>() as libc::c_ulong,
    ) as *mut libusb_ss_usb_device_capability_descriptor;
    if _ss_usb_device_cap.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    parse_descriptor(
        dev_cap as *const libc::c_void,
        b"bbbbwbbw\x00" as *const u8 as *const libc::c_char,
        _ss_usb_device_cap as *mut libc::c_void,
    );
    *ss_usb_device_cap = _ss_usb_device_cap;
    return LIBUSB_SUCCESS as libc::c_int;
}
/* * \ingroup libusb_desc
 * Free a SuperSpeed USB Device Capability descriptor obtained from
 * libusb_get_ss_usb_device_capability_descriptor().
 * It is safe to call this function with a NULL ss_usb_device_cap
 * parameter, in which case the function simply returns.
 *
 * \param ss_usb_device_cap the SuperSpeed USB Device Capability descriptor
 * to free
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_free_ss_usb_device_capability_descriptor(
    ss_usb_device_cap: *mut libusb_ss_usb_device_capability_descriptor,
) {
    free(ss_usb_device_cap as *mut libc::c_void);
}
/* * \ingroup libusb_desc
 * Get a Container ID descriptor
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param dev_cap Device Capability descriptor with a bDevCapabilityType of
 * \ref libusb_capability_type::LIBUSB_BT_CONTAINER_ID
 * LIBUSB_BT_CONTAINER_ID
 * \param container_id output location for the Container ID descriptor.
 * Only valid if 0 was returned. Must be freed with
 * libusb_free_container_id_descriptor() after use.
 * \returns 0 on success
 * \returns a LIBUSB_ERROR code on error
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_container_id_descriptor(
    ctx: *mut libusb_context,
    dev_cap: *mut libusb_bos_dev_capability_descriptor,
    container_id: *mut *mut libusb_container_id_descriptor,
) -> libc::c_int {
    let mut _container_id: *mut libusb_container_id_descriptor =
        0 as *mut libusb_container_id_descriptor;
    if (*dev_cap).bDevCapabilityType as libc::c_int != LIBUSB_BT_CONTAINER_ID as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"libusb_get_container_id_descriptor\x00",
            ))
            .as_ptr(),
            b"unexpected bDevCapabilityType %x (expected %x)\x00" as *const u8
                as *const libc::c_char,
            (*dev_cap).bDevCapabilityType as libc::c_int,
            LIBUSB_BT_CONTAINER_ID as libc::c_int,
        );
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    if ((*dev_cap).bLength as libc::c_int) < 20 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"libusb_get_container_id_descriptor\x00",
            ))
            .as_ptr(),
            b"short dev-cap descriptor read %d/%d\x00" as *const u8 as *const libc::c_char,
            (*dev_cap).bLength as libc::c_int,
            20 as libc::c_int,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    _container_id = malloc(::std::mem::size_of::<libusb_container_id_descriptor>() as libc::c_ulong)
        as *mut libusb_container_id_descriptor;
    if _container_id.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    parse_descriptor(
        dev_cap as *const libc::c_void,
        b"bbbbu\x00" as *const u8 as *const libc::c_char,
        _container_id as *mut libc::c_void,
    );
    *container_id = _container_id;
    return LIBUSB_SUCCESS as libc::c_int;
}
/* * \ingroup libusb_desc
 * Free a Container ID descriptor obtained from
 * libusb_get_container_id_descriptor().
 * It is safe to call this function with a NULL container_id parameter,
 * in which case the function simply returns.
 *
 * \param container_id the Container ID descriptor to free
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_free_container_id_descriptor(
    container_id: *mut libusb_container_id_descriptor,
) {
    free(container_id as *mut libc::c_void);
}
/* async I/O */
/* * \ingroup libusb_asyncio
 * Get the data section of a control transfer. This convenience function is here
 * to remind you that the data does not start until 8 bytes into the actual
 * buffer, as the setup packet comes first.
 *
 * Calling this function only makes sense from a transfer callback function,
 * or situations where you have already allocated a suitably sized buffer at
 * transfer->buffer.
 *
 * \param transfer a transfer
 * \returns pointer to the first byte of the data section
 */
/* * \ingroup libusb_asyncio
 * Get the control setup packet of a control transfer. This convenience
 * function is here to remind you that the control setup occupies the first
 * 8 bytes of the transfer data buffer.
 *
 * Calling this function only makes sense from a transfer callback function,
 * or situations where you have already allocated a suitably sized buffer at
 * transfer->buffer.
 *
 * \param transfer a transfer
 * \returns a casted pointer to the start of the transfer data buffer
 */
/* * \ingroup libusb_asyncio
 * Helper function to populate the setup packet (first 8 bytes of the data
 * buffer) for a control transfer. The wIndex, wValue and wLength values should
 * be given in host-endian byte order.
 *
 * \param buffer buffer to output the setup packet into
 * This pointer must be aligned to at least 2 bytes boundary.
 * \param bmRequestType see the
 * \ref libusb_control_setup::bmRequestType "bmRequestType" field of
 * \ref libusb_control_setup
 * \param bRequest see the
 * \ref libusb_control_setup::bRequest "bRequest" field of
 * \ref libusb_control_setup
 * \param wValue see the
 * \ref libusb_control_setup::wValue "wValue" field of
 * \ref libusb_control_setup
 * \param wIndex see the
 * \ref libusb_control_setup::wIndex "wIndex" field of
 * \ref libusb_control_setup
 * \param wLength see the
 * \ref libusb_control_setup::wLength "wLength" field of
 * \ref libusb_control_setup
 */
/* * \ingroup libusb_asyncio
 * Helper function to populate the required \ref libusb_transfer fields
 * for a control transfer.
 *
 * If you pass a transfer buffer to this function, the first 8 bytes will
 * be interpreted as a control setup packet, and the wLength field will be
 * used to automatically populate the \ref libusb_transfer::length "length"
 * field of the transfer. Therefore the recommended approach is:
 * -# Allocate a suitably sized data buffer (including space for control setup)
 * -# Call libusb_fill_control_setup()
 * -# If this is a host-to-device transfer with a data stage, put the data
 *    in place after the setup packet
 * -# Call this function
 * -# Call libusb_submit_transfer()
 *
 * It is also legal to pass a NULL buffer to this function, in which case this
 * function will not attempt to populate the length field. Remember that you
 * must then populate the buffer and length fields later.
 *
 * \param transfer the transfer to populate
 * \param dev_handle handle of the device that will handle the transfer
 * \param buffer data buffer. If provided, this function will interpret the
 * first 8 bytes as a setup packet and infer the transfer length from that.
 * This pointer must be aligned to at least 2 bytes boundary.
 * \param callback callback function to be invoked on transfer completion
 * \param user_data user data to pass to callback function
 * \param timeout timeout for the transfer in milliseconds
 */
/* * \ingroup libusb_asyncio
 * Helper function to populate the required \ref libusb_transfer fields
 * for a bulk transfer.
 *
 * \param transfer the transfer to populate
 * \param dev_handle handle of the device that will handle the transfer
 * \param endpoint address of the endpoint where this transfer will be sent
 * \param buffer data buffer
 * \param length length of data buffer
 * \param callback callback function to be invoked on transfer completion
 * \param user_data user data to pass to callback function
 * \param timeout timeout for the transfer in milliseconds
 */
/* * \ingroup libusb_asyncio
 * Helper function to populate the required \ref libusb_transfer fields
 * for a bulk transfer using bulk streams.
 *
 * Since version 1.0.19, \ref LIBUSB_API_VERSION >= 0x01000103
 *
 * \param transfer the transfer to populate
 * \param dev_handle handle of the device that will handle the transfer
 * \param endpoint address of the endpoint where this transfer will be sent
 * \param stream_id bulk stream id for this transfer
 * \param buffer data buffer
 * \param length length of data buffer
 * \param callback callback function to be invoked on transfer completion
 * \param user_data user data to pass to callback function
 * \param timeout timeout for the transfer in milliseconds
 */
/* * \ingroup libusb_asyncio
 * Helper function to populate the required \ref libusb_transfer fields
 * for an interrupt transfer.
 *
 * \param transfer the transfer to populate
 * \param dev_handle handle of the device that will handle the transfer
 * \param endpoint address of the endpoint where this transfer will be sent
 * \param buffer data buffer
 * \param length length of data buffer
 * \param callback callback function to be invoked on transfer completion
 * \param user_data user data to pass to callback function
 * \param timeout timeout for the transfer in milliseconds
 */
/* * \ingroup libusb_asyncio
 * Helper function to populate the required \ref libusb_transfer fields
 * for an isochronous transfer.
 *
 * \param transfer the transfer to populate
 * \param dev_handle handle of the device that will handle the transfer
 * \param endpoint address of the endpoint where this transfer will be sent
 * \param buffer data buffer
 * \param length length of data buffer
 * \param num_iso_packets the number of isochronous packets
 * \param callback callback function to be invoked on transfer completion
 * \param user_data user data to pass to callback function
 * \param timeout timeout for the transfer in milliseconds
 */
/* * \ingroup libusb_asyncio
 * Convenience function to set the length of all packets in an isochronous
 * transfer, based on the num_iso_packets field in the transfer structure.
 *
 * \param transfer a transfer
 * \param length the length to set in each isochronous packet descriptor
 * \see libusb_get_max_packet_size()
 */
/* * \ingroup libusb_asyncio
 * Convenience function to locate the position of an isochronous packet
 * within the buffer of an isochronous transfer.
 *
 * This is a thorough function which loops through all preceding packets,
 * accumulating their lengths to find the position of the specified packet.
 * Typically you will assign equal lengths to each packet in the transfer,
 * and hence the above method is sub-optimal. You may wish to use
 * libusb_get_iso_packet_buffer_simple() instead.
 *
 * \param transfer a transfer
 * \param packet the packet to return the address of
 * \returns the base address of the packet buffer inside the transfer buffer,
 * or NULL if the packet does not exist.
 * \see libusb_get_iso_packet_buffer_simple()
 */
/* oops..slight bug in the API. packet is an unsigned int, but we use
 * signed integers almost everywhere else. range-check and convert to
 * signed to avoid compiler warnings. FIXME for libusb-2. */
/* * \ingroup libusb_asyncio
 * Convenience function to locate the position of an isochronous packet
 * within the buffer of an isochronous transfer, for transfers where each
 * packet is of identical size.
 *
 * This function relies on the assumption that every packet within the transfer
 * is of identical size to the first packet. Calculating the location of
 * the packet buffer is then just a simple calculation:
 * <tt>buffer + (packet_size * packet)</tt>
 *
 * Do not use this function on transfers other than those that have identical
 * packet lengths for each packet.
 *
 * \param transfer a transfer
 * \param packet the packet to return the address of
 * \returns the base address of the packet buffer inside the transfer buffer,
 * or NULL if the packet does not exist.
 * \see libusb_get_iso_packet_buffer()
 */
/* oops..slight bug in the API. packet is an unsigned int, but we use
 * signed integers almost everywhere else. range-check and convert to
 * signed to avoid compiler warnings. FIXME for libusb-2. */
/* sync I/O */
/* * \ingroup libusb_desc
 * Retrieve a descriptor from the default control pipe.
 * This is a convenience function which formulates the appropriate control
 * message to retrieve the descriptor.
 *
 * \param dev_handle a device handle
 * \param desc_type the descriptor type, see \ref libusb_descriptor_type
 * \param desc_index the index of the descriptor to retrieve
 * \param data output buffer for descriptor
 * \param length size of data buffer
 * \returns number of bytes returned in data, or LIBUSB_ERROR code on failure
 */
/* * \ingroup libusb_desc
 * Retrieve a descriptor from a device.
 * This is a convenience function which formulates the appropriate control
 * message to retrieve the descriptor. The string returned is Unicode, as
 * detailed in the USB specifications.
 *
 * \param dev_handle a device handle
 * \param desc_index the index of the descriptor to retrieve
 * \param langid the language ID for the string descriptor
 * \param data output buffer for descriptor
 * \param length size of data buffer
 * \returns number of bytes returned in data, or LIBUSB_ERROR code on failure
 * \see libusb_get_string_descriptor_ascii()
 */
/* * \ingroup libusb_desc
 * Retrieve a string descriptor in C style ASCII.
 *
 * Wrapper around libusb_get_string_descriptor(). Uses the first language
 * supported by the device.
 *
 * \param dev_handle a device handle
 * \param desc_index the index of the descriptor to retrieve
 * \param data output buffer for ASCII string descriptor
 * \param length size of data buffer
 * \returns number of bytes returned in data, or LIBUSB_ERROR code on failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_string_descriptor_ascii(
    dev_handle: *mut libusb_device_handle,
    desc_index: uint8_t,
    data: *mut libc::c_uchar,
    length: libc::c_int,
) -> libc::c_int {
    let mut tbuf: [libc::c_uchar; 255] = [0; 255]; /* Some devices choke on size > 255 */
    let mut r: libc::c_int = 0;
    let mut si: libc::c_int = 0;
    let mut di: libc::c_int = 0;
    let mut langid: uint16_t = 0;
    /* Asking for the zero'th index is special - it returns a string
     * descriptor that contains all the language IDs supported by the
     * device. Typically there aren't many - often only one. Language
     * IDs are 16 bit numbers, and they start at the third byte in the
     * descriptor. There's also no point in trying to read descriptor 0
     * with this function. See USB 2.0 specification section 9.6.7 for
     * more information.
     */
    if desc_index as libc::c_int == 0 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    r = libusb_get_string_descriptor(
        dev_handle,
        0 as libc::c_int as uint8_t,
        0 as libc::c_int as uint16_t,
        tbuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 255]>() as libc::c_ulong as libc::c_int,
    );
    if r < 0 as libc::c_int {
        return r;
    }
    if r < 4 as libc::c_int {
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    langid = (tbuf[2 as libc::c_int as usize] as libc::c_int
        | (tbuf[3 as libc::c_int as usize] as libc::c_int) << 8 as libc::c_int)
        as uint16_t;
    r = libusb_get_string_descriptor(
        dev_handle,
        desc_index,
        langid,
        tbuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 255]>() as libc::c_ulong as libc::c_int,
    );
    if r < 0 as libc::c_int {
        return r;
    }
    if tbuf[1 as libc::c_int as usize] as libc::c_int != LIBUSB_DT_STRING as libc::c_int {
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    if tbuf[0 as libc::c_int as usize] as libc::c_int > r {
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    di = 0 as libc::c_int;
    si = 2 as libc::c_int;
    while si < tbuf[0 as libc::c_int as usize] as libc::c_int {
        if di >= length - 1 as libc::c_int {
            break;
        }
        if tbuf[si as usize] as libc::c_int & 0x80 as libc::c_int != 0
            || tbuf[(si + 1 as libc::c_int) as usize] as libc::c_int != 0
        {
            /* non-ASCII */
            let fresh4 = di;
            di = di + 1;
            *data.offset(fresh4 as isize) = '?' as i32 as libc::c_uchar
        } else {
            let fresh5 = di;
            di = di + 1;
            *data.offset(fresh5 as isize) = tbuf[si as usize]
        }
        si += 2 as libc::c_int
    }
    *data.offset(di as isize) = 0 as libc::c_int as libc::c_uchar;
    return di;
}
