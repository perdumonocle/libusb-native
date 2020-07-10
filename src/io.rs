#![allow(non_camel_case_types,unused_assignments,unused_unsafe,non_snake_case,non_upper_case_globals,unused_must_use)]

use ::libc;
extern "C" {
    #[no_mangle]
    fn libusb_ref_device(dev: *mut libusb_device) -> *mut libusb_device;
    #[no_mangle]
    fn libusb_unref_device(dev: *mut libusb_device);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn usbi_log(
        ctx: *mut libusb_context,
        level: libusb_log_level,
        function: *const libc::c_char,
        format: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    #[no_mangle]
    static usbi_backend: usbi_os_backend;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    static mut usbi_default_context: *mut libusb_context;
    #[no_mangle]
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn usbi_signal_event(ctx: *mut libusb_context) -> libc::c_int;
    #[no_mangle]
    fn usbi_cond_timedwait(
        cond: *mut usbi_cond_t,
        mutex: *mut usbi_mutex_t,
        tv: *const timeval,
    ) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t)
        -> libc::c_int;
    #[no_mangle]
    fn pthread_setspecific(__key: pthread_key_t, __pointer: *const libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn usbi_clear_event(ctx: *mut libusb_context) -> libc::c_int;
    #[no_mangle]
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn usbi_pipe(pipefd: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    #[no_mangle]
    fn pthread_key_delete(__key: pthread_key_t) -> libc::c_int;
    #[no_mangle]
    fn usbi_hotplug_deregister(ctx: *mut libusb_context, forced: libc::c_int);
    #[no_mangle]
    fn usbi_hotplug_match(
        ctx: *mut libusb_context,
        dev: *mut libusb_device,
        event: libusb_hotplug_event,
    );
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn timerfd_settime(
        __ufd: libc::c_int,
        __flags: libc::c_int,
        __utmr: *const itimerspec,
        __otmr: *mut itimerspec,
    ) -> libc::c_int;
    #[no_mangle]
    fn timerfd_create(__clock_id: __clockid_t, __flags: libc::c_int) -> libc::c_int;
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
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
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
/* * \ingroup libusb_asyncio
 * Setup packet for control transfers. */
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct libusb_control_setup {
    pub bmRequestType: uint8_t,
    pub bRequest: uint8_t,
    pub wValue: uint16_t,
    pub wIndex: uint16_t,
    pub wLength: uint16_t,
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
/* Total number of error codes in enum libusb_error */
/* * \ingroup libusb_asyncio
 * Transfer type */
pub type libusb_transfer_type = libc::c_uint;
/* * Bulk stream transfer */
pub const LIBUSB_TRANSFER_TYPE_BULK_STREAM: libusb_transfer_type = 4;
/* * Interrupt transfer */
pub const LIBUSB_TRANSFER_TYPE_INTERRUPT: libusb_transfer_type = 3;
/* * Bulk transfer */
pub const LIBUSB_TRANSFER_TYPE_BULK: libusb_transfer_type = 2;
/* * Isochronous transfer */
pub const LIBUSB_TRANSFER_TYPE_ISOCHRONOUS: libusb_transfer_type = 1;
/* * Control transfer */
pub const LIBUSB_TRANSFER_TYPE_CONTROL: libusb_transfer_type = 0;
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
 * libusb_transfer.flags values */
pub type libusb_transfer_flags = libc::c_uint;
/* * Terminate transfers that are a multiple of the endpoint's
 * wMaxPacketSize with an extra zero length packet. This is useful
 * when a device protocol mandates that each logical request is
 * terminated by an incomplete packet (i.e. the logical requests are
 * not separated by other means).
 *
 * This flag only affects host-to-device transfers to bulk and interrupt
 * endpoints. In other situations, it is ignored.
 *
 * This flag only affects transfers with a length that is a multiple of
 * the endpoint's wMaxPacketSize. On transfers of other lengths, this
 * flag has no effect. Therefore, if you are working with a device that
 * needs a ZLP whenever the end of the logical request falls on a packet
 * boundary, then it is sensible to set this flag on <em>every</em>
 * transfer (you do not have to worry about only setting it on transfers
 * that end on the boundary).
 *
 * This flag is currently only supported on Linux.
 * On other systems, libusb_submit_transfer() will return
 * LIBUSB_ERROR_NOT_SUPPORTED for every transfer where this flag is set.
 *
 * Available since libusb-1.0.9.
 */
pub const LIBUSB_TRANSFER_ADD_ZERO_PACKET: libusb_transfer_flags = 8;
/* * Automatically call libusb_free_transfer() after callback returns.
 * If this flag is set, it is illegal to call libusb_free_transfer()
 * from your transfer callback, as this will result in a double-free
 * when this flag is acted upon. */
pub const LIBUSB_TRANSFER_FREE_TRANSFER: libusb_transfer_flags = 4;
/* * Automatically free() transfer buffer during libusb_free_transfer().
 * Note that buffers allocated with libusb_dev_mem_alloc() should not
 * be attempted freed in this way, since free() is not an appropriate
 * way to release such memory. */
pub const LIBUSB_TRANSFER_FREE_BUFFER: libusb_transfer_flags = 2;
/* * Report short frames as errors */
pub const LIBUSB_TRANSFER_SHORT_NOT_OK: libusb_transfer_flags = 1;
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
pub const TFD_TIMER_ABSTIME: C2RustUnnamed_4 = 1;
pub const USBI_TRANSFER_OS_HANDLES_TIMEOUT: usbi_transfer_timeout_flags = 1;
pub const USBI_TRANSFER_TIMEOUT_HANDLED: usbi_transfer_timeout_flags = 2;
pub const USBI_TRANSFER_IN_FLIGHT: usbi_transfer_state_flags = 1;
pub const USBI_TRANSFER_CANCELLING: usbi_transfer_state_flags = 2;
pub const USBI_TRANSFER_DEVICE_DISAPPEARED: usbi_transfer_state_flags = 4;
pub const USBI_EVENT_USER_INTERRUPT: usbi_event_flags = 2;
pub const USBI_TRANSFER_TIMED_OUT: usbi_transfer_timeout_flags = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbi_pollfd {
    pub pollfd: libusb_pollfd,
    pub list: list_head,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_hotplug_message {
    pub event: libusb_hotplug_event,
    pub device: *mut libusb_device,
    pub list: list_head,
}
pub type libusb_hotplug_event = libc::c_uint;
pub const LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT: libusb_hotplug_event = 2;
pub const LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED: libusb_hotplug_event = 1;
pub const USBI_EVENT_HOTPLUG_CB_DEREGISTERED: usbi_event_flags = 4;
pub const USBI_EVENT_POLLFDS_MODIFIED: usbi_event_flags = 1;
pub type usbi_event_flags = libc::c_uint;
pub type usbi_transfer_state_flags = libc::c_uint;
pub type usbi_transfer_timeout_flags = libc::c_uint;
pub const TFD_CLOEXEC: C2RustUnnamed_3 = 524288;
pub const TFD_NONBLOCK: C2RustUnnamed_3 = 2048;
pub type C2RustUnnamed_3 = libc::c_uint;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const TFD_TIMER_CANCEL_ON_SET: C2RustUnnamed_4 = 2;
#[inline]
unsafe extern "C" fn usbi_mutex_init(mutex: *mut usbi_mutex_t) -> libc::c_int {
    return pthread_mutex_init(mutex, 0 as *const pthread_mutexattr_t);
}
#[inline]
unsafe extern "C" fn usbi_mutex_unlock(mutex: *mut usbi_mutex_t) {
    pthread_mutex_unlock(mutex);
}
#[inline]
unsafe extern "C" fn usbi_using_timerfd(ctx: *mut libusb_context) -> libc::c_int {
    return ((*ctx).timerfd >= 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn list_del(mut entry: *mut list_head) {
    (*(*entry).next).prev = (*entry).prev;
    (*(*entry).prev).next = (*entry).next;
    (*entry).prev = 0 as *mut list_head;
    (*entry).next = (*entry).prev;
}
#[inline]
unsafe extern "C" fn usbi_mutex_lock(mutex: *mut usbi_mutex_t) {
    pthread_mutex_lock(mutex);
}
#[inline]
unsafe extern "C" fn list_add(mut entry: *mut list_head, mut head: *mut list_head) {
    (*entry).next = (*head).next;
    (*entry).prev = head;
    (*(*head).next).prev = entry;
    (*head).next = entry;
}
#[inline]
unsafe extern "C" fn list_add_tail(mut entry: *mut list_head, mut head: *mut list_head) {
    (*entry).next = head;
    (*entry).prev = (*head).prev;
    (*(*head).prev).next = entry;
    (*head).prev = entry;
}
#[inline]
unsafe extern "C" fn usbi_mutex_destroy(mutex: *mut usbi_mutex_t) {
    pthread_mutex_destroy(mutex);
}
#[inline]
unsafe extern "C" fn usbi_mutex_trylock(mutex: *mut usbi_mutex_t) -> libc::c_int {
    return pthread_mutex_trylock(mutex);
}
#[inline]
unsafe extern "C" fn usbi_get_context(ctx: *mut libusb_context) -> *mut libusb_context {
    return if !ctx.is_null() {
        ctx
    } else {
        usbi_default_context
    };
}
#[inline]
unsafe extern "C" fn usbi_cond_broadcast(cond: *mut usbi_cond_t) {
    pthread_cond_broadcast(cond);
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
unsafe extern "C" fn usbi_cond_wait(cond: *mut usbi_cond_t, mutex: *mut usbi_mutex_t) {
    pthread_cond_wait(cond, mutex);
}
#[inline]
unsafe extern "C" fn usbi_end_event_handling(ctx: *mut libusb_context) {
    usbi_tls_key_set((*ctx).event_handling_key, 0 as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn usbi_tls_key_set(key: usbi_tls_key_t, ptr: *mut libc::c_void) {
    pthread_setspecific(key, ptr);
}
#[inline]
unsafe extern "C" fn list_cut(mut list: *mut list_head, head: *mut list_head) {
    if (*head).next == head {
        return;
    }
    (*list).next = (*head).next;
    (*(*list).next).prev = list;
    (*list).prev = (*head).prev;
    (*(*list).prev).next = list;
    list_init(head);
}
#[inline]
unsafe extern "C" fn list_init(mut entry: *mut list_head) {
    (*entry).next = entry;
    (*entry).prev = (*entry).next;
}
#[inline]
unsafe extern "C" fn usbi_start_event_handling(ctx: *mut libusb_context) {
    usbi_tls_key_set((*ctx).event_handling_key, ctx as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn usbi_handling_events(ctx: *mut libusb_context) -> libc::c_int {
    return (usbi_tls_key_get((*ctx).event_handling_key) != 0 as *mut libc::c_void) as libc::c_int;
}
#[inline]
unsafe extern "C" fn usbi_tls_key_get(key: usbi_tls_key_t) -> *mut libc::c_void {
    return pthread_getspecific(key);
}
#[inline]
unsafe extern "C" fn usbi_cond_init(cond: *mut pthread_cond_t) {
    pthread_cond_init(cond, 0 as *const pthread_condattr_t);
}
#[inline]
unsafe extern "C" fn usbi_cond_destroy(cond: *mut usbi_cond_t) {
    pthread_cond_destroy(cond);
}
#[inline]
unsafe extern "C" fn usbi_tls_key_create(key: *mut usbi_tls_key_t) {
    pthread_key_create(key, None);
}
#[inline]
unsafe extern "C" fn usbi_tls_key_delete(key: usbi_tls_key_t) {
    pthread_key_delete(key);
}
/* -*- Mode: C; indent-tabs-mode:t ; c-basic-offset:8 -*- */
/*
 * I/O functions for libusb
 * Copyright © 2007-2009 Daniel Drake <dsd@gentoo.org>
 * Copyright © 2001 Johannes Erdfelt <johannes@erdfelt.com>
 * Copyright © 2019 Nathan Hjelm <hjelmn@cs.umm.edu>
 * Copyright © 2019 Google LLC. All rights reserved.
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
/* *
 * \page libusb_io Synchronous and asynchronous device I/O
 *
 * \section io_intro Introduction
 *
 * If you're using libusb in your application, you're probably wanting to
 * perform I/O with devices - you want to perform USB data transfers.
 *
 * libusb offers two separate interfaces for device I/O. This page aims to
 * introduce the two in order to help you decide which one is more suitable
 * for your application. You can also choose to use both interfaces in your
 * application by considering each transfer on a case-by-case basis.
 *
 * Once you have read through the following discussion, you should consult the
 * detailed API documentation pages for the details:
 * - \ref libusb_syncio
 * - \ref libusb_asyncio
 *
 * \section theory Transfers at a logical level
 *
 * At a logical level, USB transfers typically happen in two parts. For
 * example, when reading data from a endpoint:
 * -# A request for data is sent to the device
 * -# Some time later, the incoming data is received by the host
 *
 * or when writing data to an endpoint:
 *
 * -# The data is sent to the device
 * -# Some time later, the host receives acknowledgement from the device that
 *    the data has been transferred.
 *
 * There may be an indefinite delay between the two steps. Consider a
 * fictional USB input device with a button that the user can press. In order
 * to determine when the button is pressed, you would likely submit a request
 * to read data on a bulk or interrupt endpoint and wait for data to arrive.
 * Data will arrive when the button is pressed by the user, which is
 * potentially hours later.
 *
 * libusb offers both a synchronous and an asynchronous interface to performing
 * USB transfers. The main difference is that the synchronous interface
 * combines both steps indicated above into a single function call, whereas
 * the asynchronous interface separates them.
 *
 * \section sync The synchronous interface
 *
 * The synchronous I/O interface allows you to perform a USB transfer with
 * a single function call. When the function call returns, the transfer has
 * completed and you can parse the results.
 *
 * If you have used the libusb-0.1 before, this I/O style will seem familar to
 * you. libusb-0.1 only offered a synchronous interface.
 *
 * In our input device example, to read button presses you might write code
 * in the following style:
\code
unsigned char data[4];
int actual_length;
int r = libusb_bulk_transfer(dev_handle, LIBUSB_ENDPOINT_IN, data, sizeof(data), &actual_length, 0);
if (r == 0 && actual_length == sizeof(data)) {
    // results of the transaction can now be found in the data buffer
    // parse them here and report button press
} else {
    error();
}
\endcode
 *
 * The main advantage of this model is simplicity: you did everything with
 * a single simple function call.
 *
 * However, this interface has its limitations. Your application will sleep
 * inside libusb_bulk_transfer() until the transaction has completed. If it
 * takes the user 3 hours to press the button, your application will be
 * sleeping for that long. Execution will be tied up inside the library -
 * the entire thread will be useless for that duration.
 *
 * Another issue is that by tieing up the thread with that single transaction
 * there is no possibility of performing I/O with multiple endpoints and/or
 * multiple devices simultaneously, unless you resort to creating one thread
 * per transaction.
 *
 * Additionally, there is no opportunity to cancel the transfer after the
 * request has been submitted.
 *
 * For details on how to use the synchronous API, see the
 * \ref libusb_syncio "synchronous I/O API documentation" pages.
 *
 * \section async The asynchronous interface
 *
 * Asynchronous I/O is the most significant new feature in libusb-1.0.
 * Although it is a more complex interface, it solves all the issues detailed
 * above.
 *
 * Instead of providing which functions that block until the I/O has complete,
 * libusb's asynchronous interface presents non-blocking functions which
 * begin a transfer and then return immediately. Your application passes a
 * callback function pointer to this non-blocking function, which libusb will
 * call with the results of the transaction when it has completed.
 *
 * Transfers which have been submitted through the non-blocking functions
 * can be cancelled with a separate function call.
 *
 * The non-blocking nature of this interface allows you to be simultaneously
 * performing I/O to multiple endpoints on multiple devices, without having
 * to use threads.
 *
 * This added flexibility does come with some complications though:
 * - In the interest of being a lightweight library, libusb does not create
 * threads and can only operate when your application is calling into it. Your
 * application must call into libusb from it's main loop when events are ready
 * to be handled, or you must use some other scheme to allow libusb to
 * undertake whatever work needs to be done.
 * - libusb also needs to be called into at certain fixed points in time in
 * order to accurately handle transfer timeouts.
 * - Memory handling becomes more complex. You cannot use stack memory unless
 * the function with that stack is guaranteed not to return until the transfer
 * callback has finished executing.
 * - You generally lose some linearity from your code flow because submitting
 * the transfer request is done in a separate function from where the transfer
 * results are handled. This becomes particularly obvious when you want to
 * submit a second transfer based on the results of an earlier transfer.
 *
 * Internally, libusb's synchronous interface is expressed in terms of function
 * calls to the asynchronous interface.
 *
 * For details on how to use the asynchronous API, see the
 * \ref libusb_asyncio "asynchronous I/O API" documentation pages.
 */
/* *
 * \page libusb_packetoverflow Packets and overflows
 *
 * \section packets Packet abstraction
 *
 * The USB specifications describe how data is transmitted in packets, with
 * constraints on packet size defined by endpoint descriptors. The host must
 * not send data payloads larger than the endpoint's maximum packet size.
 *
 * libusb and the underlying OS abstract out the packet concept, allowing you
 * to request transfers of any size. Internally, the request will be divided
 * up into correctly-sized packets. You do not have to be concerned with
 * packet sizes, but there is one exception when considering overflows.
 *
 * \section overflow Bulk/interrupt transfer overflows
 *
 * When requesting data on a bulk endpoint, libusb requires you to supply a
 * buffer and the maximum number of bytes of data that libusb can put in that
 * buffer. However, the size of the buffer is not communicated to the device -
 * the device is just asked to send any amount of data.
 *
 * There is no problem if the device sends an amount of data that is less than
 * or equal to the buffer size. libusb reports this condition to you through
 * the \ref libusb_transfer::actual_length "libusb_transfer.actual_length"
 * field.
 *
 * Problems may occur if the device attempts to send more data than can fit in
 * the buffer. libusb reports LIBUSB_TRANSFER_OVERFLOW for this condition but
 * other behaviour is largely undefined: actual_length may or may not be
 * accurate, the chunk of data that can fit in the buffer (before overflow)
 * may or may not have been transferred.
 *
 * Overflows are nasty, but can be avoided. Even though you were told to
 * ignore packets above, think about the lower level details: each transfer is
 * split into packets (typically small, with a maximum size of 512 bytes).
 * Overflows can only happen if the final packet in an incoming data transfer
 * is smaller than the actual packet that the device wants to transfer.
 * Therefore, you will never see an overflow if your transfer buffer size is a
 * multiple of the endpoint's packet size: the final packet will either
 * fill up completely or will be only partially filled.
 */
/* *
 * @defgroup libusb_asyncio Asynchronous device I/O
 *
 * This page details libusb's asynchronous (non-blocking) API for USB device
 * I/O. This interface is very powerful but is also quite complex - you will
 * need to read this page carefully to understand the necessary considerations
 * and issues surrounding use of this interface. Simplistic applications
 * may wish to consider the \ref libusb_syncio "synchronous I/O API" instead.
 *
 * The asynchronous interface is built around the idea of separating transfer
 * submission and handling of transfer completion (the synchronous model
 * combines both of these into one). There may be a long delay between
 * submission and completion, however the asynchronous submission function
 * is non-blocking so will return control to your application during that
 * potentially long delay.
 *
 * \section asyncabstraction Transfer abstraction
 *
 * For the asynchronous I/O, libusb implements the concept of a generic
 * transfer entity for all types of I/O (control, bulk, interrupt,
 * isochronous). The generic transfer object must be treated slightly
 * differently depending on which type of I/O you are performing with it.
 *
 * This is represented by the public libusb_transfer structure type.
 *
 * \section asynctrf Asynchronous transfers
 *
 * We can view asynchronous I/O as a 5 step process:
 * -# <b>Allocation</b>: allocate a libusb_transfer
 * -# <b>Filling</b>: populate the libusb_transfer instance with information
 *    about the transfer you wish to perform
 * -# <b>Submission</b>: ask libusb to submit the transfer
 * -# <b>Completion handling</b>: examine transfer results in the
 *    libusb_transfer structure
 * -# <b>Deallocation</b>: clean up resources
 *
 *
 * \subsection asyncalloc Allocation
 *
 * This step involves allocating memory for a USB transfer. This is the
 * generic transfer object mentioned above. At this stage, the transfer
 * is "blank" with no details about what type of I/O it will be used for.
 *
 * Allocation is done with the libusb_alloc_transfer() function. You must use
 * this function rather than allocating your own transfers.
 *
 * \subsection asyncfill Filling
 *
 * This step is where you take a previously allocated transfer and fill it
 * with information to determine the message type and direction, data buffer,
 * callback function, etc.
 *
 * You can either fill the required fields yourself or you can use the
 * helper functions: libusb_fill_control_transfer(), libusb_fill_bulk_transfer()
 * and libusb_fill_interrupt_transfer().
 *
 * \subsection asyncsubmit Submission
 *
 * When you have allocated a transfer and filled it, you can submit it using
 * libusb_submit_transfer(). This function returns immediately but can be
 * regarded as firing off the I/O request in the background.
 *
 * \subsection asynccomplete Completion handling
 *
 * After a transfer has been submitted, one of four things can happen to it:
 *
 * - The transfer completes (i.e. some data was transferred)
 * - The transfer has a timeout and the timeout expires before all data is
 * transferred
 * - The transfer fails due to an error
 * - The transfer is cancelled
 *
 * Each of these will cause the user-specified transfer callback function to
 * be invoked. It is up to the callback function to determine which of the
 * above actually happened and to act accordingly.
 *
 * The user-specified callback is passed a pointer to the libusb_transfer
 * structure which was used to setup and submit the transfer. At completion
 * time, libusb has populated this structure with results of the transfer:
 * success or failure reason, number of bytes of data transferred, etc. See
 * the libusb_transfer structure documentation for more information.
 *
 * <b>Important Note</b>: The user-specified callback is called from an event
 * handling context. It is therefore important that no calls are made into
 * libusb that will attempt to perform any event handling. Examples of such
 * functions are any listed in the \ref libusb_syncio "synchronous API" and any of
 * the blocking functions that retrieve \ref libusb_desc "USB descriptors".
 *
 * \subsection Deallocation
 *
 * When a transfer has completed (i.e. the callback function has been invoked),
 * you are advised to free the transfer (unless you wish to resubmit it, see
 * below). Transfers are deallocated with libusb_free_transfer().
 *
 * It is undefined behaviour to free a transfer which has not completed.
 *
 * \section asyncresubmit Resubmission
 *
 * You may be wondering why allocation, filling, and submission are all
 * separated above where they could reasonably be combined into a single
 * operation.
 *
 * The reason for separation is to allow you to resubmit transfers without
 * having to allocate new ones every time. This is especially useful for
 * common situations dealing with interrupt endpoints - you allocate one
 * transfer, fill and submit it, and when it returns with results you just
 * resubmit it for the next interrupt.
 *
 * \section asynccancel Cancellation
 *
 * Another advantage of using the asynchronous interface is that you have
 * the ability to cancel transfers which have not yet completed. This is
 * done by calling the libusb_cancel_transfer() function.
 *
 * libusb_cancel_transfer() is asynchronous/non-blocking in itself. When the
 * cancellation actually completes, the transfer's callback function will
 * be invoked, and the callback function should check the transfer status to
 * determine that it was cancelled.
 *
 * Freeing the transfer after it has been cancelled but before cancellation
 * has completed will result in undefined behaviour.
 *
 * When a transfer is cancelled, some of the data may have been transferred.
 * libusb will communicate this to you in the transfer callback. Do not assume
 * that no data was transferred.
 *
 * \section bulk_overflows Overflows on device-to-host bulk/interrupt endpoints
 *
 * If your device does not have predictable transfer sizes (or it misbehaves),
 * your application may submit a request for data on an IN endpoint which is
 * smaller than the data that the device wishes to send. In some circumstances
 * this will cause an overflow, which is a nasty condition to deal with. See
 * the \ref libusb_packetoverflow page for discussion.
 *
 * \section asyncctrl Considerations for control transfers
 *
 * The <tt>libusb_transfer</tt> structure is generic and hence does not
 * include specific fields for the control-specific setup packet structure.
 *
 * In order to perform a control transfer, you must place the 8-byte setup
 * packet at the start of the data buffer. To simplify this, you could
 * cast the buffer pointer to type struct libusb_control_setup, or you can
 * use the helper function libusb_fill_control_setup().
 *
 * The wLength field placed in the setup packet must be the length you would
 * expect to be sent in the setup packet: the length of the payload that
 * follows (or the expected maximum number of bytes to receive). However,
 * the length field of the libusb_transfer object must be the length of
 * the data buffer - i.e. it should be wLength <em>plus</em> the size of
 * the setup packet (LIBUSB_CONTROL_SETUP_SIZE).
 *
 * If you use the helper functions, this is simplified for you:
 * -# Allocate a buffer of size LIBUSB_CONTROL_SETUP_SIZE plus the size of the
 * data you are sending/requesting.
 * -# Call libusb_fill_control_setup() on the data buffer, using the transfer
 * request size as the wLength value (i.e. do not include the extra space you
 * allocated for the control setup).
 * -# If this is a host-to-device transfer, place the data to be transferred
 * in the data buffer, starting at offset LIBUSB_CONTROL_SETUP_SIZE.
 * -# Call libusb_fill_control_transfer() to associate the data buffer with
 * the transfer (and to set the remaining details such as callback and timeout).
 *   - Note that there is no parameter to set the length field of the transfer.
 *     The length is automatically inferred from the wLength field of the setup
 *     packet.
 * -# Submit the transfer.
 *
 * The multi-byte control setup fields (wValue, wIndex and wLength) must
 * be given in little-endian byte order (the endianness of the USB bus).
 * Endianness conversion is transparently handled by
 * libusb_fill_control_setup() which is documented to accept host-endian
 * values.
 *
 * Further considerations are needed when handling transfer completion in
 * your callback function:
 * - As you might expect, the setup packet will still be sitting at the start
 * of the data buffer.
 * - If this was a device-to-host transfer, the received data will be sitting
 * at offset LIBUSB_CONTROL_SETUP_SIZE into the buffer.
 * - The actual_length field of the transfer structure is relative to the
 * wLength of the setup packet, rather than the size of the data buffer. So,
 * if your wLength was 4, your transfer's <tt>length</tt> was 12, then you
 * should expect an <tt>actual_length</tt> of 4 to indicate that the data was
 * transferred in entirity.
 *
 * To simplify parsing of setup packets and obtaining the data from the
 * correct offset, you may wish to use the libusb_control_transfer_get_data()
 * and libusb_control_transfer_get_setup() functions within your transfer
 * callback.
 *
 * Even though control endpoints do not halt, a completed control transfer
 * may have a LIBUSB_TRANSFER_STALL status code. This indicates the control
 * request was not supported.
 *
 * \section asyncintr Considerations for interrupt transfers
 *
 * All interrupt transfers are performed using the polling interval presented
 * by the bInterval value of the endpoint descriptor.
 *
 * \section asynciso Considerations for isochronous transfers
 *
 * Isochronous transfers are more complicated than transfers to
 * non-isochronous endpoints.
 *
 * To perform I/O to an isochronous endpoint, allocate the transfer by calling
 * libusb_alloc_transfer() with an appropriate number of isochronous packets.
 *
 * During filling, set \ref libusb_transfer::type "type" to
 * \ref libusb_transfer_type::LIBUSB_TRANSFER_TYPE_ISOCHRONOUS
 * "LIBUSB_TRANSFER_TYPE_ISOCHRONOUS", and set
 * \ref libusb_transfer::num_iso_packets "num_iso_packets" to a value less than
 * or equal to the number of packets you requested during allocation.
 * libusb_alloc_transfer() does not set either of these fields for you, given
 * that you might not even use the transfer on an isochronous endpoint.
 *
 * Next, populate the length field for the first num_iso_packets entries in
 * the \ref libusb_transfer::iso_packet_desc "iso_packet_desc" array. Section
 * 5.6.3 of the USB2 specifications describe how the maximum isochronous
 * packet length is determined by the wMaxPacketSize field in the endpoint
 * descriptor.
 * Two functions can help you here:
 *
 * - libusb_get_max_iso_packet_size() is an easy way to determine the max
 *   packet size for an isochronous endpoint. Note that the maximum packet
 *   size is actually the maximum number of bytes that can be transmitted in
 *   a single microframe, therefore this function multiplies the maximum number
 *   of bytes per transaction by the number of transaction opportunities per
 *   microframe.
 * - libusb_set_iso_packet_lengths() assigns the same length to all packets
 *   within a transfer, which is usually what you want.
 *
 * For outgoing transfers, you'll obviously fill the buffer and populate the
 * packet descriptors in hope that all the data gets transferred. For incoming
 * transfers, you must ensure the buffer has sufficient capacity for
 * the situation where all packets transfer the full amount of requested data.
 *
 * Completion handling requires some extra consideration. The
 * \ref libusb_transfer::actual_length "actual_length" field of the transfer
 * is meaningless and should not be examined; instead you must refer to the
 * \ref libusb_iso_packet_descriptor::actual_length "actual_length" field of
 * each individual packet.
 *
 * The \ref libusb_transfer::status "status" field of the transfer is also a
 * little misleading:
 *  - If the packets were submitted and the isochronous data microframes
 *    completed normally, status will have value
 *    \ref libusb_transfer_status::LIBUSB_TRANSFER_COMPLETED
 *    "LIBUSB_TRANSFER_COMPLETED". Note that bus errors and software-incurred
 *    delays are not counted as transfer errors; the transfer.status field may
 *    indicate COMPLETED even if some or all of the packets failed. Refer to
 *    the \ref libusb_iso_packet_descriptor::status "status" field of each
 *    individual packet to determine packet failures.
 *  - The status field will have value
 *    \ref libusb_transfer_status::LIBUSB_TRANSFER_ERROR
 *    "LIBUSB_TRANSFER_ERROR" only when serious errors were encountered.
 *  - Other transfer status codes occur with normal behaviour.
 *
 * The data for each packet will be found at an offset into the buffer that
 * can be calculated as if each prior packet completed in full. The
 * libusb_get_iso_packet_buffer() and libusb_get_iso_packet_buffer_simple()
 * functions may help you here.
 *
 * <b>Note</b>: Some operating systems (e.g. Linux) may impose limits on the
 * length of individual isochronous packets and/or the total length of the
 * isochronous transfer. Such limits can be difficult for libusb to detect,
 * so the library will simply try and submit the transfer as set up by you.
 * If the transfer fails to submit because it is too large,
 * libusb_submit_transfer() will return
 * \ref libusb_error::LIBUSB_ERROR_INVALID_PARAM "LIBUSB_ERROR_INVALID_PARAM".
 *
 * \section asyncmem Memory caveats
 *
 * In most circumstances, it is not safe to use stack memory for transfer
 * buffers. This is because the function that fired off the asynchronous
 * transfer may return before libusb has finished using the buffer, and when
 * the function returns it's stack gets destroyed. This is true for both
 * host-to-device and device-to-host transfers.
 *
 * The only case in which it is safe to use stack memory is where you can
 * guarantee that the function owning the stack space for the buffer does not
 * return until after the transfer's callback function has completed. In every
 * other case, you need to use heap memory instead.
 *
 * \section asyncflags Fine control
 *
 * Through using this asynchronous interface, you may find yourself repeating
 * a few simple operations many times. You can apply a bitwise OR of certain
 * flags to a transfer to simplify certain things:
 * - \ref libusb_transfer_flags::LIBUSB_TRANSFER_SHORT_NOT_OK
 *   "LIBUSB_TRANSFER_SHORT_NOT_OK" results in transfers which transferred
 *   less than the requested amount of data being marked with status
 *   \ref libusb_transfer_status::LIBUSB_TRANSFER_ERROR "LIBUSB_TRANSFER_ERROR"
 *   (they would normally be regarded as COMPLETED)
 * - \ref libusb_transfer_flags::LIBUSB_TRANSFER_FREE_BUFFER
 *   "LIBUSB_TRANSFER_FREE_BUFFER" allows you to ask libusb to free the transfer
 *   buffer when freeing the transfer.
 * - \ref libusb_transfer_flags::LIBUSB_TRANSFER_FREE_TRANSFER
 *   "LIBUSB_TRANSFER_FREE_TRANSFER" causes libusb to automatically free the
 *   transfer after the transfer callback returns.
 *
 * \section asyncevent Event handling
 *
 * An asynchronous model requires that libusb perform work at various
 * points in time - namely processing the results of previously-submitted
 * transfers and invoking the user-supplied callback function.
 *
 * This gives rise to the libusb_handle_events() function which your
 * application must call into when libusb has work do to. This gives libusb
 * the opportunity to reap pending transfers, invoke callbacks, etc.
 *
 * There are 2 different approaches to dealing with libusb_handle_events:
 *
 * -# Repeatedly call libusb_handle_events() in blocking mode from a dedicated
 *    thread.
 * -# Integrate libusb with your application's main event loop. libusb
 *    exposes a set of file descriptors which allow you to do this.
 *
 * The first approach has the big advantage that it will also work on Windows
 * were libusb' poll API for select / poll integration is not available. So
 * if you want to support Windows and use the async API, you must use this
 * approach, see the \ref eventthread "Using an event handling thread" section
 * below for details.
 *
 * If you prefer a single threaded approach with a single central event loop,
 * see the \ref libusb_poll "polling and timing" section for how to integrate libusb
 * into your application's main event loop.
 *
 * \section eventthread Using an event handling thread
 *
 * Lets begin with stating the obvious: If you're going to use a separate
 * thread for libusb event handling, your callback functions MUST be
 * threadsafe.
 *
 * Other then that doing event handling from a separate thread, is mostly
 * simple. You can use an event thread function as follows:
\code
void *event_thread_func(void *ctx)
{
    while (event_thread_run)
        libusb_handle_events(ctx);

    return NULL;
}
\endcode
 *
 * There is one caveat though, stopping this thread requires setting the
 * event_thread_run variable to 0, and after that libusb_handle_events() needs
 * to return control to event_thread_func. But unless some event happens,
 * libusb_handle_events() will not return.
 *
 * There are 2 different ways of dealing with this, depending on if your
 * application uses libusb' \ref libusb_hotplug "hotplug" support or not.
 *
 * Applications which do not use hotplug support, should not start the event
 * thread until after their first call to libusb_open(), and should stop the
 * thread when closing the last open device as follows:
\code
void my_close_handle(libusb_device_handle *dev_handle)
{
    if (open_devs == 1)
        event_thread_run = 0;

    libusb_close(dev_handle); // This wakes up libusb_handle_events()

    if (open_devs == 1)
        pthread_join(event_thread);

    open_devs--;
}
\endcode
 *
 * Applications using hotplug support should start the thread at program init,
 * after having successfully called libusb_hotplug_register_callback(), and
 * should stop the thread at program exit as follows:
\code
void my_libusb_exit(void)
{
    event_thread_run = 0;
    libusb_hotplug_deregister_callback(ctx, hotplug_cb_handle); // This wakes up libusb_handle_events()
    pthread_join(event_thread);
    libusb_exit(ctx);
}
\endcode
 */
/* *
 * @defgroup libusb_poll Polling and timing
 *
 * This page documents libusb's functions for polling events and timing.
 * These functions are only necessary for users of the
 * \ref libusb_asyncio "asynchronous API". If you are only using the simpler
 * \ref libusb_syncio "synchronous API" then you do not need to ever call these
 * functions.
 *
 * The justification for the functionality described here has already been
 * discussed in the \ref asyncevent "event handling" section of the
 * asynchronous API documentation. In summary, libusb does not create internal
 * threads for event processing and hence relies on your application calling
 * into libusb at certain points in time so that pending events can be handled.
 *
 * Your main loop is probably already calling poll() or select() or a
 * variant on a set of file descriptors for other event sources (e.g. keyboard
 * button presses, mouse movements, network sockets, etc). You then add
 * libusb's file descriptors to your poll()/select() calls, and when activity
 * is detected on such descriptors you know it is time to call
 * libusb_handle_events().
 *
 * There is one final event handling complication. libusb supports
 * asynchronous transfers which time out after a specified time period.
 *
 * On some platforms a timerfd is used, so the timeout handling is just another
 * fd, on other platforms this requires that libusb is called into at or after
 * the timeout to handle it. So, in addition to considering libusb's file
 * descriptors in your main event loop, you must also consider that libusb
 * sometimes needs to be called into at fixed points in time even when there
 * is no file descriptor activity, see \ref polltime details.
 *
 * In order to know precisely when libusb needs to be called into, libusb
 * offers you a set of pollable file descriptors and information about when
 * the next timeout expires.
 *
 * If you are using the asynchronous I/O API, you must take one of the two
 * following options, otherwise your I/O will not complete.
 *
 * \section pollsimple The simple option
 *
 * If your application revolves solely around libusb and does not need to
 * handle other event sources, you can have a program structure as follows:
\code
// initialize libusb
// find and open device
// maybe fire off some initial async I/O

while (user_has_not_requested_exit)
    libusb_handle_events(ctx);

// clean up and exit
\endcode
 *
 * With such a simple main loop, you do not have to worry about managing
 * sets of file descriptors or handling timeouts. libusb_handle_events() will
 * handle those details internally.
 *
 * \section libusb_pollmain The more advanced option
 *
 * \note This functionality is currently only available on Unix-like platforms.
 * On Windows, libusb_get_pollfds() simply returns NULL. Applications which
 * want to support Windows are advised to use an \ref eventthread
 * "event handling thread" instead.
 *
 * In more advanced applications, you will already have a main loop which
 * is monitoring other event sources: network sockets, X11 events, mouse
 * movements, etc. Through exposing a set of file descriptors, libusb is
 * designed to cleanly integrate into such main loops.
 *
 * In addition to polling file descriptors for the other event sources, you
 * take a set of file descriptors from libusb and monitor those too. When you
 * detect activity on libusb's file descriptors, you call
 * libusb_handle_events_timeout() in non-blocking mode.
 *
 * What's more, libusb may also need to handle events at specific moments in
 * time. No file descriptor activity is generated at these times, so your
 * own application needs to be continually aware of when the next one of these
 * moments occurs (through calling libusb_get_next_timeout()), and then it
 * needs to call libusb_handle_events_timeout() in non-blocking mode when
 * these moments occur. This means that you need to adjust your
 * poll()/select() timeout accordingly.
 *
 * libusb provides you with a set of file descriptors to poll and expects you
 * to poll all of them, treating them as a single entity. The meaning of each
 * file descriptor in the set is an internal implementation detail,
 * platform-dependent and may vary from release to release. Don't try and
 * interpret the meaning of the file descriptors, just do as libusb indicates,
 * polling all of them at once.
 *
 * In pseudo-code, you want something that looks like:
\code
// initialise libusb

libusb_get_pollfds(ctx)
while (user has not requested application exit) {
    libusb_get_next_timeout(ctx);
    poll(on libusb file descriptors plus any other event sources of interest,
        using a timeout no larger than the value libusb just suggested)
    if (poll() indicated activity on libusb file descriptors)
        libusb_handle_events_timeout(ctx, &zero_tv);
    if (time has elapsed to or beyond the libusb timeout)
        libusb_handle_events_timeout(ctx, &zero_tv);
    // handle events from other sources here
}

// clean up and exit
\endcode
 *
 * \subsection polltime Notes on time-based events
 *
 * The above complication with having to track time and call into libusb at
 * specific moments is a bit of a headache. For maximum compatibility, you do
 * need to write your main loop as above, but you may decide that you can
 * restrict the supported platforms of your application and get away with
 * a more simplistic scheme.
 *
 * These time-based event complications are \b not required on the following
 * platforms:
 *  - Darwin
 *  - Linux, provided that the following version requirements are satisfied:
 *   - Linux v2.6.27 or newer, compiled with timerfd support
 *   - glibc v2.9 or newer
 *   - libusb v1.0.5 or newer
 *
 * Under these configurations, libusb_get_next_timeout() will \em always return
 * 0, so your main loop can be simplified to:
\code
// initialise libusb

libusb_get_pollfds(ctx)
while (user has not requested application exit) {
    poll(on libusb file descriptors plus any other event sources of interest,
        using any timeout that you like)
    if (poll() indicated activity on libusb file descriptors)
        libusb_handle_events_timeout(ctx, &zero_tv);
    // handle events from other sources here
}

// clean up and exit
\endcode
 *
 * Do remember that if you simplify your main loop to the above, you will
 * lose compatibility with some platforms (including legacy Linux platforms,
 * and <em>any future platforms supported by libusb which may have time-based
 * event requirements</em>). The resultant problems will likely appear as
 * strange bugs in your application.
 *
 * You can use the libusb_pollfds_handle_timeouts() function to do a runtime
 * check to see if it is safe to ignore the time-based event complications.
 * If your application has taken the shortcut of ignoring libusb's next timeout
 * in your main loop, then you are advised to check the return value of
 * libusb_pollfds_handle_timeouts() during application startup, and to abort
 * if the platform does suffer from these timing complications.
 *
 * \subsection fdsetchange Changes in the file descriptor set
 *
 * The set of file descriptors that libusb uses as event sources may change
 * during the life of your application. Rather than having to repeatedly
 * call libusb_get_pollfds(), you can set up notification functions for when
 * the file descriptor set changes using libusb_set_pollfd_notifiers().
 *
 * \subsection mtissues Multi-threaded considerations
 *
 * Unfortunately, the situation is complicated further when multiple threads
 * come into play. If two threads are monitoring the same file descriptors,
 * the fact that only one thread will be woken up when an event occurs causes
 * some headaches.
 *
 * The events lock, event waiters lock, and libusb_handle_events_locked()
 * entities are added to solve these problems. You do not need to be concerned
 * with these entities otherwise.
 *
 * See the extra documentation: \ref libusb_mtasync
 */
/* * \page libusb_mtasync Multi-threaded applications and asynchronous I/O
 *
 * libusb is a thread-safe library, but extra considerations must be applied
 * to applications which interact with libusb from multiple threads.
 *
 * The underlying issue that must be addressed is that all libusb I/O
 * revolves around monitoring file descriptors through the poll()/select()
 * system calls. This is directly exposed at the
 * \ref libusb_asyncio "asynchronous interface" but it is important to note that the
 * \ref libusb_syncio "synchronous interface" is implemented on top of the
 * asynchonrous interface, therefore the same considerations apply.
 *
 * The issue is that if two or more threads are concurrently calling poll()
 * or select() on libusb's file descriptors then only one of those threads
 * will be woken up when an event arrives. The others will be completely
 * oblivious that anything has happened.
 *
 * Consider the following pseudo-code, which submits an asynchronous transfer
 * then waits for its completion. This style is one way you could implement a
 * synchronous interface on top of the asynchronous interface (and libusb
 * does something similar, albeit more advanced due to the complications
 * explained on this page).
 *
\code
void cb(struct libusb_transfer *transfer)
{
    int *completed = transfer->user_data;
    *completed = 1;
}

void myfunc() {
    struct libusb_transfer *transfer;
    unsigned char buffer[LIBUSB_CONTROL_SETUP_SIZE] __attribute__ ((aligned (2)));
    int completed = 0;

    transfer = libusb_alloc_transfer(0);
    libusb_fill_control_setup(buffer,
        LIBUSB_REQUEST_TYPE_VENDOR | LIBUSB_ENDPOINT_OUT, 0x04, 0x01, 0, 0);
    libusb_fill_control_transfer(transfer, dev, buffer, cb, &completed, 1000);
    libusb_submit_transfer(transfer);

    while (!completed) {
        poll(libusb file descriptors, 120*1000);
        if (poll indicates activity)
            libusb_handle_events_timeout(ctx, &zero_tv);
    }
    printf("completed!");
    // other code here
}
\endcode
 *
 * Here we are <em>serializing</em> completion of an asynchronous event
 * against a condition - the condition being completion of a specific transfer.
 * The poll() loop has a long timeout to minimize CPU usage during situations
 * when nothing is happening (it could reasonably be unlimited).
 *
 * If this is the only thread that is polling libusb's file descriptors, there
 * is no problem: there is no danger that another thread will swallow up the
 * event that we are interested in. On the other hand, if there is another
 * thread polling the same descriptors, there is a chance that it will receive
 * the event that we were interested in. In this situation, <tt>myfunc()</tt>
 * will only realise that the transfer has completed on the next iteration of
 * the loop, <em>up to 120 seconds later.</em> Clearly a two-minute delay is
 * undesirable, and don't even think about using short timeouts to circumvent
 * this issue!
 *
 * The solution here is to ensure that no two threads are ever polling the
 * file descriptors at the same time. A naive implementation of this would
 * impact the capabilities of the library, so libusb offers the scheme
 * documented below to ensure no loss of functionality.
 *
 * Before we go any further, it is worth mentioning that all libusb-wrapped
 * event handling procedures fully adhere to the scheme documented below.
 * This includes libusb_handle_events() and its variants, and all the
 * synchronous I/O functions - libusb hides this headache from you.
 *
 * \section Using libusb_handle_events() from multiple threads
 *
 * Even when only using libusb_handle_events() and synchronous I/O functions,
 * you can still have a race condition. You might be tempted to solve the
 * above with libusb_handle_events() like so:
 *
\code
    libusb_submit_transfer(transfer);

    while (!completed) {
        libusb_handle_events(ctx);
    }
    printf("completed!");
\endcode
 *
 * This however has a race between the checking of completed and
 * libusb_handle_events() acquiring the events lock, so another thread
 * could have completed the transfer, resulting in this thread hanging
 * until either a timeout or another event occurs. See also commit
 * 6696512aade99bb15d6792af90ae329af270eba6 which fixes this in the
 * synchronous API implementation of libusb.
 *
 * Fixing this race requires checking the variable completed only after
 * taking the event lock, which defeats the concept of just calling
 * libusb_handle_events() without worrying about locking. This is why
 * libusb-1.0.9 introduces the new libusb_handle_events_timeout_completed()
 * and libusb_handle_events_completed() functions, which handles doing the
 * completion check for you after they have acquired the lock:
 *
\code
    libusb_submit_transfer(transfer);

    while (!completed) {
        libusb_handle_events_completed(ctx, &completed);
    }
    printf("completed!");
\endcode
 *
 * This nicely fixes the race in our example. Note that if all you want to
 * do is submit a single transfer and wait for its completion, then using
 * one of the synchronous I/O functions is much easier.
 *
 * \section eventlock The events lock
 *
 * The problem is when we consider the fact that libusb exposes file
 * descriptors to allow for you to integrate asynchronous USB I/O into
 * existing main loops, effectively allowing you to do some work behind
 * libusb's back. If you do take libusb's file descriptors and pass them to
 * poll()/select() yourself, you need to be aware of the associated issues.
 *
 * The first concept to be introduced is the events lock. The events lock
 * is used to serialize threads that want to handle events, such that only
 * one thread is handling events at any one time.
 *
 * You must take the events lock before polling libusb file descriptors,
 * using libusb_lock_events(). You must release the lock as soon as you have
 * aborted your poll()/select() loop, using libusb_unlock_events().
 *
 * \section threadwait Letting other threads do the work for you
 *
 * Although the events lock is a critical part of the solution, it is not
 * enough on it's own. You might wonder if the following is sufficient...
\code
    libusb_lock_events(ctx);
    while (!completed) {
        poll(libusb file descriptors, 120*1000);
        if (poll indicates activity)
            libusb_handle_events_timeout(ctx, &zero_tv);
    }
    libusb_unlock_events(ctx);
\endcode
 * ...and the answer is that it is not. This is because the transfer in the
 * code shown above may take a long time (say 30 seconds) to complete, and
 * the lock is not released until the transfer is completed.
 *
 * Another thread with similar code that wants to do event handling may be
 * working with a transfer that completes after a few milliseconds. Despite
 * having such a quick completion time, the other thread cannot check that
 * status of its transfer until the code above has finished (30 seconds later)
 * due to contention on the lock.
 *
 * To solve this, libusb offers you a mechanism to determine when another
 * thread is handling events. It also offers a mechanism to block your thread
 * until the event handling thread has completed an event (and this mechanism
 * does not involve polling of file descriptors).
 *
 * After determining that another thread is currently handling events, you
 * obtain the <em>event waiters</em> lock using libusb_lock_event_waiters().
 * You then re-check that some other thread is still handling events, and if
 * so, you call libusb_wait_for_event().
 *
 * libusb_wait_for_event() puts your application to sleep until an event
 * occurs, or until a thread releases the events lock. When either of these
 * things happen, your thread is woken up, and should re-check the condition
 * it was waiting on. It should also re-check that another thread is handling
 * events, and if not, it should start handling events itself.
 *
 * This looks like the following, as pseudo-code:
\code
retry:
if (libusb_try_lock_events(ctx) == 0) {
    // we obtained the event lock: do our own event handling
    while (!completed) {
        if (!libusb_event_handling_ok(ctx)) {
            libusb_unlock_events(ctx);
            goto retry;
        }
        poll(libusb file descriptors, 120*1000);
        if (poll indicates activity)
            libusb_handle_events_locked(ctx, 0);
    }
    libusb_unlock_events(ctx);
} else {
    // another thread is doing event handling. wait for it to signal us that
    // an event has completed
    libusb_lock_event_waiters(ctx);

    while (!completed) {
        // now that we have the event waiters lock, double check that another
        // thread is still handling events for us. (it may have ceased handling
        // events in the time it took us to reach this point)
        if (!libusb_event_handler_active(ctx)) {
            // whoever was handling events is no longer doing so, try again
            libusb_unlock_event_waiters(ctx);
            goto retry;
        }

        libusb_wait_for_event(ctx, NULL);
    }
    libusb_unlock_event_waiters(ctx);
}
printf("completed!\n");
\endcode
 *
 * A naive look at the above code may suggest that this can only support
 * one event waiter (hence a total of 2 competing threads, the other doing
 * event handling), because the event waiter seems to have taken the event
 * waiters lock while waiting for an event. However, the system does support
 * multiple event waiters, because libusb_wait_for_event() actually drops
 * the lock while waiting, and reaquires it before continuing.
 *
 * We have now implemented code which can dynamically handle situations where
 * nobody is handling events (so we should do it ourselves), and it can also
 * handle situations where another thread is doing event handling (so we can
 * piggyback onto them). It is also equipped to handle a combination of
 * the two, for example, another thread is doing event handling, but for
 * whatever reason it stops doing so before our condition is met, so we take
 * over the event handling.
 *
 * Four functions were introduced in the above pseudo-code. Their importance
 * should be apparent from the code shown above.
 * -# libusb_try_lock_events() is a non-blocking function which attempts
 *    to acquire the events lock but returns a failure code if it is contended.
 * -# libusb_event_handling_ok() checks that libusb is still happy for your
 *    thread to be performing event handling. Sometimes, libusb needs to
 *    interrupt the event handler, and this is how you can check if you have
 *    been interrupted. If this function returns 0, the correct behaviour is
 *    for you to give up the event handling lock, and then to repeat the cycle.
 *    The following libusb_try_lock_events() will fail, so you will become an
 *    events waiter. For more information on this, read \ref fullstory below.
 * -# libusb_handle_events_locked() is a variant of
 *    libusb_handle_events_timeout() that you can call while holding the
 *    events lock. libusb_handle_events_timeout() itself implements similar
 *    logic to the above, so be sure not to call it when you are
 *    "working behind libusb's back", as is the case here.
 * -# libusb_event_handler_active() determines if someone is currently
 *    holding the events lock
 *
 * You might be wondering why there is no function to wake up all threads
 * blocked on libusb_wait_for_event(). This is because libusb can do this
 * internally: it will wake up all such threads when someone calls
 * libusb_unlock_events() or when a transfer completes (at the point after its
 * callback has returned).
 *
 * \subsection fullstory The full story
 *
 * The above explanation should be enough to get you going, but if you're
 * really thinking through the issues then you may be left with some more
 * questions regarding libusb's internals. If you're curious, read on, and if
 * not, skip to the next section to avoid confusing yourself!
 *
 * The immediate question that may spring to mind is: what if one thread
 * modifies the set of file descriptors that need to be polled while another
 * thread is doing event handling?
 *
 * There are 2 situations in which this may happen.
 * -# libusb_open() will add another file descriptor to the poll set,
 *    therefore it is desirable to interrupt the event handler so that it
 *    restarts, picking up the new descriptor.
 * -# libusb_close() will remove a file descriptor from the poll set. There
 *    are all kinds of race conditions that could arise here, so it is
 *    important that nobody is doing event handling at this time.
 *
 * libusb handles these issues internally, so application developers do not
 * have to stop their event handlers while opening/closing devices. Here's how
 * it works, focusing on the libusb_close() situation first:
 *
 * -# During initialization, libusb opens an internal pipe, and it adds the read
 *    end of this pipe to the set of file descriptors to be polled.
 * -# During libusb_close(), libusb writes some dummy data on this event pipe.
 *    This immediately interrupts the event handler. libusb also records
 *    internally that it is trying to interrupt event handlers for this
 *    high-priority event.
 * -# At this point, some of the functions described above start behaving
 *    differently:
 *   - libusb_event_handling_ok() starts returning 1, indicating that it is NOT
 *     OK for event handling to continue.
 *   - libusb_try_lock_events() starts returning 1, indicating that another
 *     thread holds the event handling lock, even if the lock is uncontended.
 *   - libusb_event_handler_active() starts returning 1, indicating that
 *     another thread is doing event handling, even if that is not true.
 * -# The above changes in behaviour result in the event handler stopping and
 *    giving up the events lock very quickly, giving the high-priority
 *    libusb_close() operation a "free ride" to acquire the events lock. All
 *    threads that are competing to do event handling become event waiters.
 * -# With the events lock held inside libusb_close(), libusb can safely remove
 *    a file descriptor from the poll set, in the safety of knowledge that
 *    nobody is polling those descriptors or trying to access the poll set.
 * -# After obtaining the events lock, the close operation completes very
 *    quickly (usually a matter of milliseconds) and then immediately releases
 *    the events lock.
 * -# At the same time, the behaviour of libusb_event_handling_ok() and friends
 *    reverts to the original, documented behaviour.
 * -# The release of the events lock causes the threads that are waiting for
 *    events to be woken up and to start competing to become event handlers
 *    again. One of them will succeed; it will then re-obtain the list of poll
 *    descriptors, and USB I/O will then continue as normal.
 *
 * libusb_open() is similar, and is actually a more simplistic case. Upon a
 * call to libusb_open():
 *
 * -# The device is opened and a file descriptor is added to the poll set.
 * -# libusb sends some dummy data on the event pipe, and records that it
 *    is trying to modify the poll descriptor set.
 * -# The event handler is interrupted, and the same behaviour change as for
 *    libusb_close() takes effect, causing all event handling threads to become
 *    event waiters.
 * -# The libusb_open() implementation takes its free ride to the events lock.
 * -# Happy that it has successfully paused the events handler, libusb_open()
 *    releases the events lock.
 * -# The event waiter threads are all woken up and compete to become event
 *    handlers again. The one that succeeds will obtain the list of poll
 *    descriptors again, which will include the addition of the new device.
 *
 * \subsection concl Closing remarks
 *
 * The above may seem a little complicated, but hopefully I have made it clear
 * why such complications are necessary. Also, do not forget that this only
 * applies to applications that take libusb's file descriptors and integrate
 * them into their own polling loops.
 *
 * You may decide that it is OK for your multi-threaded application to ignore
 * some of the rules and locks detailed above, because you don't think that
 * two threads can ever be polling the descriptors at the same time. If that
 * is the case, then that's good news for you because you don't have to worry.
 * But be careful here; remember that the synchronous I/O functions do event
 * handling internally. If you have one thread doing event handling in a loop
 * (without implementing the rules and locking semantics documented above)
 * and another trying to send a synchronous USB transfer, you will end up with
 * two threads monitoring the same descriptors, and the above-described
 * undesirable behaviour occurring. The solution is for your polling thread to
 * play by the rules; the synchronous I/O functions do so, and this will result
 * in them getting along in perfect harmony.
 *
 * If you do have a dedicated thread doing event handling, it is perfectly
 * legal for it to take the event handling lock for long periods of time. Any
 * synchronous I/O functions you call from other threads will transparently
 * fall back to the "event waiters" mechanism detailed above. The only
 * consideration that your event handling thread must apply is the one related
 * to libusb_event_handling_ok(): you must call this before every poll(), and
 * give up the events lock if instructed.
 */
#[no_mangle]
pub unsafe extern "C" fn usbi_io_init(mut ctx: *mut libusb_context) -> libc::c_int {
    let current_block: u64;
    let mut r: libc::c_int = 0;
    usbi_mutex_init(&mut (*ctx).flying_transfers_lock);
    usbi_mutex_init(&mut (*ctx).events_lock);
    usbi_mutex_init(&mut (*ctx).event_waiters_lock);
    usbi_cond_init(&mut (*ctx).event_waiters_cond);
    usbi_mutex_init(&mut (*ctx).event_data_lock);
    usbi_tls_key_create(&mut (*ctx).event_handling_key);
    list_init(&mut (*ctx).flying_transfers);
    list_init(&mut (*ctx).ipollfds);
    list_init(&mut (*ctx).removed_ipollfds);
    list_init(&mut (*ctx).hotplug_msgs);
    list_init(&mut (*ctx).completed_transfers);
    /* FIXME should use an eventfd on kernels that support it */
    r = usbi_pipe((*ctx).event_pipe.as_mut_ptr());
    if r < 0 as libc::c_int {
        r = LIBUSB_ERROR_OTHER as libc::c_int
    } else {
        r = usbi_add_pollfd(
            ctx,
            (*ctx).event_pipe[0 as libc::c_int as usize],
            0x1 as libc::c_int as libc::c_short,
        );
        if !(r < 0 as libc::c_int) {
            (*ctx).timerfd = timerfd_create(
                1 as libc::c_int,
                TFD_NONBLOCK as libc::c_int | TFD_CLOEXEC as libc::c_int,
            );
            if (*ctx).timerfd >= 0 as libc::c_int {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"usbi_io_init\x00"))
                        .as_ptr(),
                    b"using timerfd for timeouts\x00" as *const u8 as *const libc::c_char,
                );
                r = usbi_add_pollfd(ctx, (*ctx).timerfd, 0x1 as libc::c_int as libc::c_short);
                if r < 0 as libc::c_int {
                    close((*ctx).timerfd);
                    usbi_remove_pollfd(ctx, (*ctx).event_pipe[0 as libc::c_int as usize]);
                    current_block = 6613235314810327775;
                } else {
                    current_block = 15089075282327824602;
                }
            } else {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"usbi_io_init\x00"))
                        .as_ptr(),
                    b"timerfd not available, errno=%d\x00" as *const u8 as *const libc::c_char,
                    *__errno_location(),
                );
                current_block = 15089075282327824602;
            }
            match current_block {
                6613235314810327775 => {}
                _ => return 0 as libc::c_int,
            }
        }
        close((*ctx).event_pipe[0 as libc::c_int as usize]);
        close((*ctx).event_pipe[1 as libc::c_int as usize]);
    }
    usbi_mutex_destroy(&mut (*ctx).flying_transfers_lock);
    usbi_mutex_destroy(&mut (*ctx).events_lock);
    usbi_mutex_destroy(&mut (*ctx).event_waiters_lock);
    usbi_cond_destroy(&mut (*ctx).event_waiters_cond);
    usbi_mutex_destroy(&mut (*ctx).event_data_lock);
    usbi_tls_key_delete((*ctx).event_handling_key);
    return r;
}
unsafe extern "C" fn cleanup_removed_pollfds(ctx: *mut libusb_context) {
    let mut ipollfd: *mut usbi_pollfd = 0 as *mut usbi_pollfd;
    let mut tmp: *mut usbi_pollfd = 0 as *mut usbi_pollfd;
    ipollfd = ((*ctx).removed_ipollfds.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
        as *mut usbi_pollfd;
    tmp = ((*ipollfd).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong) as *mut usbi_pollfd;
    while &mut (*ipollfd).list as *mut list_head != &mut (*ctx).removed_ipollfds as *mut list_head {
        list_del(&mut (*ipollfd).list);
        free(ipollfd as *mut libc::c_void);
        ipollfd = tmp;
        tmp = ((*tmp).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong) as *mut usbi_pollfd
    }
}
#[no_mangle]
pub unsafe extern "C" fn usbi_io_exit(ctx: *mut libusb_context) {
    usbi_remove_pollfd(ctx, (*ctx).event_pipe[0 as libc::c_int as usize]);
    close((*ctx).event_pipe[0 as libc::c_int as usize]);
    close((*ctx).event_pipe[1 as libc::c_int as usize]);
    if usbi_using_timerfd(ctx) != 0 {
        usbi_remove_pollfd(ctx, (*ctx).timerfd);
        close((*ctx).timerfd);
    }
    usbi_mutex_destroy(&mut (*ctx).flying_transfers_lock);
    usbi_mutex_destroy(&mut (*ctx).events_lock);
    usbi_mutex_destroy(&mut (*ctx).event_waiters_lock);
    usbi_cond_destroy(&mut (*ctx).event_waiters_cond);
    usbi_mutex_destroy(&mut (*ctx).event_data_lock);
    usbi_tls_key_delete((*ctx).event_handling_key);
    free((*ctx).pollfds as *mut libc::c_void);
    cleanup_removed_pollfds(ctx);
}
unsafe extern "C" fn calculate_timeout(mut itransfer: *mut usbi_transfer) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let timeout: libc::c_uint = (*((itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer))
        .timeout;
    if timeout == 0 {
        (*itransfer).timeout.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
        (*itransfer).timeout.tv_sec = (*itransfer).timeout.tv_nsec;
        return 0 as libc::c_int;
    }
    r = clock_gettime(1 as libc::c_int, &mut (*itransfer).timeout);
    if r < 0 as libc::c_int {
        usbi_log(
            (*(*(*((itransfer as *mut libc::c_uchar).offset(
                ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
                    (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
            ) as *mut libusb_transfer))
                .dev_handle)
                .dev)
                .ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"calculate_timeout\x00"))
                .as_ptr(),
            b"failed to read monotonic clock, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    (*itransfer).timeout.tv_sec += timeout.wrapping_div(1000 as libc::c_uint) as libc::c_long;
    (*itransfer).timeout.tv_nsec +=
        timeout.wrapping_rem(1000 as libc::c_uint) as libc::c_long * 1000000 as libc::c_long;
    if (*itransfer).timeout.tv_nsec >= 1000000000 as libc::c_long {
        (*itransfer).timeout.tv_sec += 1;
        (*itransfer).timeout.tv_nsec -= 1000000000 as libc::c_long
    }
    return 0 as libc::c_int;
}
/* * \ingroup libusb_asyncio
 * Allocate a libusb transfer with a specified number of isochronous packet
 * descriptors. The returned transfer is pre-initialized for you. When the new
 * transfer is no longer needed, it should be freed with
 * libusb_free_transfer().
 *
 * Transfers intended for non-isochronous endpoints (e.g. control, bulk,
 * interrupt) should specify an iso_packets count of zero.
 *
 * For transfers intended for isochronous endpoints, specify an appropriate
 * number of packet descriptors to be allocated as part of the transfer.
 * The returned transfer is not specially initialized for isochronous I/O;
 * you are still required to set the
 * \ref libusb_transfer::num_iso_packets "num_iso_packets" and
 * \ref libusb_transfer::type "type" fields accordingly.
 *
 * It is safe to allocate a transfer with some isochronous packets and then
 * use it on a non-isochronous endpoint. If you do this, ensure that at time
 * of submission, num_iso_packets is 0 and that type is set appropriately.
 *
 * \param iso_packets number of isochronous packet descriptors to allocate. Must be non-negative.
 * \returns a newly allocated transfer, or NULL on error
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_alloc_transfer(iso_packets: libc::c_int) -> *mut libusb_transfer {
    let mut priv_size: size_t = 0;
    let mut alloc_size: size_t = 0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut itransfer: *mut usbi_transfer = 0 as *mut usbi_transfer;
    let mut transfer: *mut libusb_transfer = 0 as *mut libusb_transfer;
    if iso_packets >= 0 as libc::c_int {
    } else {
        __assert_fail(
            b"iso_packets >= 0\x00" as *const u8 as *const libc::c_char,
            b"/home/konstantin/Projects/tests/rust/libusb2rs/libusb/libusb/io.c\x00" as *const u8
                as *const libc::c_char,
            1263 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"struct libusb_transfer *libusb_alloc_transfer(int)\x00",
            ))
            .as_ptr(),
        );
    }
    if iso_packets < 0 as libc::c_int {
        return 0 as *mut libusb_transfer;
    }
    priv_size = usbi_backend.transfer_priv_size.wrapping_add(
        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    alloc_size = priv_size
        .wrapping_add(::std::mem::size_of::<usbi_transfer>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<libusb_transfer>() as libc::c_ulong)
        .wrapping_add(
            (::std::mem::size_of::<libusb_iso_packet_descriptor>() as libc::c_ulong)
                .wrapping_mul(iso_packets as size_t),
        );
    ptr = calloc(1 as libc::c_int as libc::c_ulong, alloc_size) as *mut libc::c_uchar;
    if ptr.is_null() {
        return 0 as *mut libusb_transfer;
    }
    itransfer = ptr.offset(priv_size as isize) as *mut usbi_transfer;
    (*itransfer).num_iso_packets = iso_packets;
    (*itransfer).priv_0 = ptr as *mut libc::c_void;
    usbi_mutex_init(&mut (*itransfer).lock);
    transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"libusb_alloc_transfer\x00"))
            .as_ptr(),
        b"transfer %p\x00" as *const u8 as *const libc::c_char,
        transfer,
    );
    return transfer;
}
/* * \ingroup libusb_asyncio
 * Free a transfer structure. This should be called for all transfers
 * allocated with libusb_alloc_transfer().
 *
 * If the \ref libusb_transfer_flags::LIBUSB_TRANSFER_FREE_BUFFER
 * "LIBUSB_TRANSFER_FREE_BUFFER" flag is set and the transfer buffer is
 * non-NULL, this function will also free the transfer buffer using the
 * standard system memory allocator (e.g. free()).
 *
 * It is legal to call this function with a NULL transfer. In this case,
 * the function will simply return safely.
 *
 * It is not legal to free an active transfer (one which has been submitted
 * and has not yet completed).
 *
 * \param transfer the transfer to free
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_free_transfer(transfer: *mut libusb_transfer) {
    let mut itransfer: *mut usbi_transfer = 0 as *mut usbi_transfer;
    let mut priv_size: size_t = 0;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if transfer.is_null() {
        return;
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"libusb_free_transfer\x00"))
            .as_ptr(),
        b"transfer %p\x00" as *const u8 as *const libc::c_char,
        transfer,
    );
    if (*transfer).flags as libc::c_int & LIBUSB_TRANSFER_FREE_BUFFER as libc::c_int != 0 {
        free((*transfer).buffer as *mut libc::c_void);
    }
    itransfer = (transfer as *mut libc::c_uchar).offset(
        -(((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize),
    ) as *mut usbi_transfer;
    usbi_mutex_destroy(&mut (*itransfer).lock);
    priv_size = usbi_backend.transfer_priv_size.wrapping_add(
        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    ptr = (itransfer as *mut libc::c_uchar).offset(-(priv_size as isize));
    if ptr == (*itransfer).priv_0 as *mut libc::c_uchar {
    } else {
        __assert_fail(
            b"ptr == itransfer->priv\x00" as *const u8 as *const libc::c_char,
            b"/home/konstantin/Projects/tests/rust/libusb2rs/libusb/libusb/io.c\x00" as *const u8
                as *const libc::c_char,
            1320 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 52], &[libc::c_char; 52]>(
                b"void libusb_free_transfer(struct libusb_transfer *)\x00",
            ))
            .as_ptr(),
        );
    }
    free(ptr as *mut libc::c_void);
}
unsafe extern "C" fn disarm_timerfd(ctx: *mut libusb_context) -> libc::c_int {
    let disarm_timer: itimerspec = {
        let init = itimerspec {
            it_interval: {
                let init = timespec {
                    tv_sec: 0 as libc::c_int as __time_t,
                    tv_nsec: 0 as libc::c_int as __syscall_slong_t,
                };
                init
            },
            it_value: {
                let init = timespec {
                    tv_sec: 0 as libc::c_int as __time_t,
                    tv_nsec: 0 as libc::c_int as __syscall_slong_t,
                };
                init
            },
        };
        init
    };
    let mut r: libc::c_int = 0;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"disarm_timerfd\x00")).as_ptr(),
        b" \x00" as *const u8 as *const libc::c_char,
    );
    r = timerfd_settime(
        (*ctx).timerfd,
        0 as libc::c_int,
        &disarm_timer,
        0 as *mut itimerspec,
    );
    if r < 0 as libc::c_int {
        return LIBUSB_ERROR_OTHER as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
/* iterates through the flying transfers, and rearms the timerfd based on the
 * next upcoming timeout.
 * must be called with flying_list locked.
 * returns 0 on success or a LIBUSB_ERROR code on failure.
 */
unsafe extern "C" fn arm_timerfd_for_next_timeout(ctx: *mut libusb_context) -> libc::c_int {
    let mut itransfer: *mut usbi_transfer = 0 as *mut usbi_transfer;
    itransfer = ((*ctx).flying_transfers.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
        as *mut usbi_transfer;
    while &mut (*itransfer).list as *mut list_head != &mut (*ctx).flying_transfers as *mut list_head
    {
        let cur_ts: *mut timespec = &mut (*itransfer).timeout;
        /* if we've reached transfers of infinite timeout, then we have no
         * arming to do */
        if !((*cur_ts).tv_sec != 0 || (*cur_ts).tv_nsec != 0) {
            break;
        }
        /* act on first transfer that has not already been handled */
        if (*itransfer).timeout_flags
            & (USBI_TRANSFER_TIMEOUT_HANDLED as libc::c_int
                | USBI_TRANSFER_OS_HANDLES_TIMEOUT as libc::c_int) as libc::c_uint
            == 0
        {
            let mut r: libc::c_int = 0;
            let it: itimerspec = {
                let init = itimerspec {
                    it_interval: {
                        let init = timespec {
                            tv_sec: 0 as libc::c_int as __time_t,
                            tv_nsec: 0 as libc::c_int as __syscall_slong_t,
                        };
                        init
                    },
                    it_value: {
                        let init = timespec {
                            tv_sec: (*cur_ts).tv_sec,
                            tv_nsec: (*cur_ts).tv_nsec,
                        };
                        init
                    },
                };
                init
            };
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                    b"arm_timerfd_for_next_timeout\x00",
                ))
                .as_ptr(),
                b"next timeout originally %ums\x00" as *const u8 as *const libc::c_char,
                (*((itransfer as *mut libc::c_uchar).offset(
                    ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
                        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as isize,
                ) as *mut libusb_transfer))
                    .timeout,
            );
            r = timerfd_settime(
                (*ctx).timerfd,
                TFD_TIMER_ABSTIME as libc::c_int,
                &it,
                0 as *mut itimerspec,
            );
            if r < 0 as libc::c_int {
                return LIBUSB_ERROR_OTHER as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        itransfer = ((*itransfer).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
            as *mut usbi_transfer
    }
    return disarm_timerfd(ctx);
}
/* add a transfer to the (timeout-sorted) active transfers list.
 * This function will return non 0 if fails to update the timer,
 * in which case the transfer is *not* on the flying_transfers list. */
unsafe extern "C" fn add_to_flying_list(itransfer: *mut usbi_transfer) -> libc::c_int {
    let current_block: u64;
    let mut cur: *mut usbi_transfer = 0 as *mut usbi_transfer;
    let timeout: *mut timespec = &mut (*itransfer).timeout;
    let ctx: *mut libusb_context = (*(*(*((itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer))
        .dev_handle)
        .dev)
        .ctx;
    let mut r: libc::c_int = 0;
    let mut first: libc::c_int = 1 as libc::c_int;
    r = calculate_timeout(itransfer);
    if r != 0 {
        return r;
    }
    /* if we have no other flying transfers, start the list with this one */
    if (*ctx).flying_transfers.next == &mut (*ctx).flying_transfers as *mut list_head {
        list_add(&mut (*itransfer).list, &mut (*ctx).flying_transfers);
    } else if !((*timeout).tv_sec != 0 || (*timeout).tv_nsec != 0) {
        list_add_tail(&mut (*itransfer).list, &mut (*ctx).flying_transfers);
    } else {
        /* if we have infinite timeout, append to end of list */
        /* otherwise, find appropriate place in list */
        cur = ((*ctx).flying_transfers.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
            as *mut usbi_transfer;
        loop {
            if !(&mut (*cur).list as *mut list_head
                != &mut (*ctx).flying_transfers as *mut list_head)
            {
                current_block = 4956146061682418353;
                break;
            }
            /* find first timeout that occurs after the transfer in question */
            let cur_ts: *mut timespec = &mut (*cur).timeout;
            if !((*cur_ts).tv_sec != 0 || (*cur_ts).tv_nsec != 0)
                || (if (*cur_ts).tv_sec == (*timeout).tv_sec {
                    ((*cur_ts).tv_nsec > (*timeout).tv_nsec) as libc::c_int
                } else {
                    ((*cur_ts).tv_sec > (*timeout).tv_sec) as libc::c_int
                }) != 0
            {
                list_add_tail(&mut (*itransfer).list, &mut (*cur).list);
                current_block = 7593490379297353364;
                break;
            } else {
                first = 0 as libc::c_int;
                cur = ((*cur).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
                    as *mut usbi_transfer
            }
        }
        match current_block {
            7593490379297353364 => {}
            _ => {
                /* first is 0 at this stage (list not empty) */
                /* otherwise we need to be inserted at the end */
                list_add_tail(&mut (*itransfer).list, &mut (*ctx).flying_transfers);
            }
        }
    }
    /* first is irrelevant in this case */
    if first != 0
        && usbi_using_timerfd(ctx) != 0
        && ((*timeout).tv_sec != 0 || (*timeout).tv_nsec != 0)
    {
        /* if this transfer has the lowest timeout of all active transfers,
         * rearm the timerfd with this transfer's timeout */
        let it: itimerspec = {
            let init = itimerspec {
                it_interval: {
                    let init = timespec {
                        tv_sec: 0 as libc::c_int as __time_t,
                        tv_nsec: 0 as libc::c_int as __syscall_slong_t,
                    };
                    init
                },
                it_value: {
                    let init = timespec {
                        tv_sec: (*timeout).tv_sec,
                        tv_nsec: (*timeout).tv_nsec,
                    };
                    init
                },
            };
            init
        };
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"add_to_flying_list\x00"))
                .as_ptr(),
            b"arm timerfd for timeout in %ums (first in line)\x00" as *const u8
                as *const libc::c_char,
            (*((itransfer as *mut libc::c_uchar).offset(
                ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
                    (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
            ) as *mut libusb_transfer))
                .timeout,
        );
        r = timerfd_settime(
            (*ctx).timerfd,
            TFD_TIMER_ABSTIME as libc::c_int,
            &it,
            0 as *mut itimerspec,
        );
        if r < 0 as libc::c_int {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"add_to_flying_list\x00",
                ))
                .as_ptr(),
                b"failed to arm first timerfd, errno=%d\x00" as *const u8 as *const libc::c_char,
                *__errno_location(),
            );
            r = LIBUSB_ERROR_OTHER as libc::c_int
        }
    }
    if r != 0 {
        list_del(&mut (*itransfer).list);
    }
    return r;
}
/* remove a transfer from the active transfers list.
 * This function will *always* remove the transfer from the
 * flying_transfers list. It will return a LIBUSB_ERROR code
 * if it fails to update the timer for the next timeout. */
unsafe extern "C" fn remove_from_flying_list(itransfer: *mut usbi_transfer) -> libc::c_int {
    let ctx: *mut libusb_context = (*(*(*((itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer))
        .dev_handle)
        .dev)
        .ctx;
    let mut rearm_timerfd: libc::c_int = 0;
    let mut r: libc::c_int = 0 as libc::c_int;
    usbi_mutex_lock(&mut (*ctx).flying_transfers_lock);
    rearm_timerfd = (((*itransfer).timeout.tv_sec != 0 || (*itransfer).timeout.tv_nsec != 0)
        && ((*ctx).flying_transfers.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
            as *mut usbi_transfer
            == itransfer) as libc::c_int;
    list_del(&mut (*itransfer).list);
    if usbi_using_timerfd(ctx) != 0 && rearm_timerfd != 0 {
        r = arm_timerfd_for_next_timeout(ctx)
    }
    usbi_mutex_unlock(&mut (*ctx).flying_transfers_lock);
    return r;
}
/* * \ingroup libusb_asyncio
 * Submit a transfer. This function will fire off the USB transfer and then
 * return immediately.
 *
 * \param transfer the transfer to submit
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns LIBUSB_ERROR_BUSY if the transfer has already been submitted.
 * \returns LIBUSB_ERROR_NOT_SUPPORTED if the transfer flags are not supported
 * by the operating system.
 * \returns LIBUSB_ERROR_INVALID_PARAM if the transfer size is larger than
 * the operating system and/or hardware can support
 * \returns another LIBUSB_ERROR code on other failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_submit_transfer(transfer: *mut libusb_transfer) -> libc::c_int {
    let mut itransfer: *mut usbi_transfer = (transfer as *mut libc::c_uchar).offset(
        -(((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize),
    ) as *mut usbi_transfer;
    let ctx: *mut libusb_context = (*(*(*transfer).dev_handle).dev).ctx;
    let mut r: libc::c_int = 0;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"libusb_submit_transfer\x00"))
            .as_ptr(),
        b"transfer %p\x00" as *const u8 as *const libc::c_char,
        transfer,
    );
    /*
     * Important note on locking, this function takes / releases locks
     * in the following order:
     *  take flying_transfers_lock
     *  take itransfer->lock
     *  clear transfer
     *  add to flying_transfers list
     *  release flying_transfers_lock
     *  submit transfer
     *  release itransfer->lock
     *  if submit failed:
     *   take flying_transfers_lock
     *   remove from flying_transfers list
     *   release flying_transfers_lock
     *
     * Note that it takes locks in the order a-b and then releases them
     * in the same order a-b. This is somewhat unusual but not wrong,
     * release order is not important as long as *all* locks are released
     * before re-acquiring any locks.
     *
     * This means that the ordering of first releasing itransfer->lock
     * and then re-acquiring the flying_transfers_list on error is
     * important and must not be changed!
     *
     * This is done this way because when we take both locks we must always
     * take flying_transfers_lock first to avoid ab-ba style deadlocks with
     * the timeout handling and usbi_handle_disconnect paths.
     *
     * And we cannot release itransfer->lock before the submission is
     * complete otherwise timeout handling for transfers with short
     * timeouts may run before submission.
     */
    usbi_mutex_lock(&mut (*ctx).flying_transfers_lock);
    usbi_mutex_lock(&mut (*itransfer).lock);
    if (*itransfer).state_flags & USBI_TRANSFER_IN_FLIGHT as libc::c_int as libc::c_uint != 0 {
        usbi_mutex_unlock(&mut (*ctx).flying_transfers_lock);
        usbi_mutex_unlock(&mut (*itransfer).lock);
        return LIBUSB_ERROR_BUSY as libc::c_int;
    }
    (*itransfer).transferred = 0 as libc::c_int;
    (*itransfer).state_flags = 0 as libc::c_int as uint32_t;
    (*itransfer).timeout_flags = 0 as libc::c_int as uint32_t;
    r = add_to_flying_list(itransfer);
    if r != 0 {
        usbi_mutex_unlock(&mut (*ctx).flying_transfers_lock);
        usbi_mutex_unlock(&mut (*itransfer).lock);
        return r;
    }
    /*
     * We must release the flying transfers lock here, because with
     * some backends the submit_transfer method is synchroneous.
     */
    usbi_mutex_unlock(&mut (*ctx).flying_transfers_lock);
    r = usbi_backend
        .submit_transfer
        .expect("non-null function pointer")(itransfer);
    if r == LIBUSB_SUCCESS as libc::c_int {
        (*itransfer).state_flags |= USBI_TRANSFER_IN_FLIGHT as libc::c_int as libc::c_uint;
        /* keep a reference to this device */
        libusb_ref_device((*(*transfer).dev_handle).dev);
    }
    usbi_mutex_unlock(&mut (*itransfer).lock);
    if r != LIBUSB_SUCCESS as libc::c_int {
        remove_from_flying_list(itransfer);
    }
    return r;
}
/* * \ingroup libusb_asyncio
 * Asynchronously cancel a previously submitted transfer.
 * This function returns immediately, but this does not indicate cancellation
 * is complete. Your callback function will be invoked at some later time
 * with a transfer status of
 * \ref libusb_transfer_status::LIBUSB_TRANSFER_CANCELLED
 * "LIBUSB_TRANSFER_CANCELLED."
 *
 * \param transfer the transfer to cancel
 * \returns 0 on success
 * \returns LIBUSB_ERROR_NOT_FOUND if the transfer is not in progress,
 * already complete, or already cancelled.
 * \returns a LIBUSB_ERROR code on failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_cancel_transfer(transfer: *mut libusb_transfer) -> libc::c_int {
    let mut itransfer: *mut usbi_transfer = (transfer as *mut libc::c_uchar).offset(
        -(((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize),
    ) as *mut usbi_transfer;
    let mut r: libc::c_int = 0;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"libusb_cancel_transfer\x00"))
            .as_ptr(),
        b"transfer %p\x00" as *const u8 as *const libc::c_char,
        transfer,
    );
    usbi_mutex_lock(&mut (*itransfer).lock);
    if (*itransfer).state_flags & USBI_TRANSFER_IN_FLIGHT as libc::c_int as libc::c_uint == 0
        || (*itransfer).state_flags & USBI_TRANSFER_CANCELLING as libc::c_int as libc::c_uint != 0
    {
        r = LIBUSB_ERROR_NOT_FOUND as libc::c_int
    } else {
        r = usbi_backend
            .cancel_transfer
            .expect("non-null function pointer")(itransfer);
        if r < 0 as libc::c_int {
            if r != LIBUSB_ERROR_NOT_FOUND as libc::c_int
                && r != LIBUSB_ERROR_NO_DEVICE as libc::c_int
            {
                usbi_log(
                    (*(*(*transfer).dev_handle).dev).ctx,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"libusb_cancel_transfer\x00",
                    ))
                    .as_ptr(),
                    b"cancel transfer failed error %d\x00" as *const u8 as *const libc::c_char,
                    r,
                );
            } else {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"libusb_cancel_transfer\x00",
                    ))
                    .as_ptr(),
                    b"cancel transfer failed error %d\x00" as *const u8 as *const libc::c_char,
                    r,
                );
            }
            if r == LIBUSB_ERROR_NO_DEVICE as libc::c_int {
                (*itransfer).state_flags |=
                    USBI_TRANSFER_DEVICE_DISAPPEARED as libc::c_int as libc::c_uint
            }
        }
        (*itransfer).state_flags |= USBI_TRANSFER_CANCELLING as libc::c_int as libc::c_uint
    }
    usbi_mutex_unlock(&mut (*itransfer).lock);
    return r;
}
/* * \ingroup libusb_asyncio
 * Set a transfers bulk stream id. Note users are advised to use
 * libusb_fill_bulk_stream_transfer() instead of calling this function
 * directly.
 *
 * Since version 1.0.19, \ref LIBUSB_API_VERSION >= 0x01000103
 *
 * \param transfer the transfer to set the stream id for
 * \param stream_id the stream id to set
 * \see libusb_alloc_streams()
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_transfer_set_stream_id(
    transfer: *mut libusb_transfer,
    stream_id: uint32_t,
) {
    let mut itransfer: *mut usbi_transfer = (transfer as *mut libc::c_uchar).offset(
        -(((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize),
    ) as *mut usbi_transfer;
    (*itransfer).stream_id = stream_id;
}
/* * \ingroup libusb_asyncio
 * Get a transfers bulk stream id.
 *
 * Since version 1.0.19, \ref LIBUSB_API_VERSION >= 0x01000103
 *
 * \param transfer the transfer to get the stream id for
 * \returns the stream id for the transfer
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_transfer_get_stream_id(transfer: *mut libusb_transfer) -> uint32_t {
    let itransfer: *mut usbi_transfer = (transfer as *mut libc::c_uchar).offset(
        -(((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize),
    ) as *mut usbi_transfer;
    return (*itransfer).stream_id;
}
/* Handle completion of a transfer (completion might be an error condition).
 * This will invoke the user-supplied callback function, which may end up
 * freeing the transfer. Therefore you cannot use the transfer structure
 * after calling this function, and you should free all backend-specific
 * data before calling it.
 * Do not call this function with the usbi_transfer lock held. User-specified
 * callback functions may attempt to directly resubmit the transfer, which
 * will attempt to take the lock. */
#[no_mangle]
pub unsafe extern "C" fn usbi_handle_transfer_completion(
    mut itransfer: *mut usbi_transfer,
    mut status: libusb_transfer_status,
) -> libc::c_int {
    let mut transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    let dev_handle: *mut libusb_device_handle = (*transfer).dev_handle;
    let mut flags: uint8_t = 0;
    let mut r: libc::c_int = 0;
    r = remove_from_flying_list(itransfer);
    if r < 0 as libc::c_int {
        usbi_log(
            (*(*(*((itransfer as *mut libc::c_uchar).offset(
                ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
                    (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
            ) as *mut libusb_transfer))
                .dev_handle)
                .dev)
                .ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"usbi_handle_transfer_completion\x00",
            ))
            .as_ptr(),
            b"failed to set timer for next timeout, errno=%d\x00" as *const u8
                as *const libc::c_char,
            *__errno_location(),
        );
    }
    usbi_mutex_lock(&mut (*itransfer).lock);
    (*itransfer).state_flags &= !(USBI_TRANSFER_IN_FLIGHT as libc::c_int) as libc::c_uint;
    usbi_mutex_unlock(&mut (*itransfer).lock);
    if status as libc::c_uint == LIBUSB_TRANSFER_COMPLETED as libc::c_int as libc::c_uint
        && (*transfer).flags as libc::c_int & LIBUSB_TRANSFER_SHORT_NOT_OK as libc::c_int != 0
    {
        let mut rqlen: libc::c_int = (*transfer).length;
        if (*transfer).type_0 as libc::c_int == LIBUSB_TRANSFER_TYPE_CONTROL as libc::c_int {
            rqlen = (rqlen as libc::c_ulong)
                .wrapping_sub(::std::mem::size_of::<libusb_control_setup>() as libc::c_ulong)
                as libc::c_int as libc::c_int
        }
        if rqlen != (*itransfer).transferred {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    b"usbi_handle_transfer_completion\x00",
                ))
                .as_ptr(),
                b"interpreting short transfer as error\x00" as *const u8 as *const libc::c_char,
            );
            status = LIBUSB_TRANSFER_ERROR
        }
    }
    flags = (*transfer).flags;
    (*transfer).status = status;
    (*transfer).actual_length = (*itransfer).transferred;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
            b"usbi_handle_transfer_completion\x00",
        ))
        .as_ptr(),
        b"transfer %p has callback %p\x00" as *const u8 as *const libc::c_char,
        transfer,
        (*transfer).callback,
    );
    if (*transfer).callback.is_some() {
        (*transfer).callback.expect("non-null function pointer")(transfer);
    }
    /* transfer might have been freed by the above call, do not use from
     * this point. */
    if flags as libc::c_int & LIBUSB_TRANSFER_FREE_TRANSFER as libc::c_int != 0 {
        libusb_free_transfer(transfer);
    }
    libusb_unref_device((*dev_handle).dev);
    return r;
}
/* Similar to usbi_handle_transfer_completion() but exclusively for transfers
 * that were asynchronously cancelled. The same concerns w.r.t. freeing of
 * transfers exist here.
 * Do not call this function with the usbi_transfer lock held. User-specified
 * callback functions may attempt to directly resubmit the transfer, which
 * will attempt to take the lock. */
#[no_mangle]
pub unsafe extern "C" fn usbi_handle_transfer_cancellation(
    itransfer: *mut usbi_transfer,
) -> libc::c_int {
    let ctx: *mut libusb_context = (*(*(*((itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer))
        .dev_handle)
        .dev)
        .ctx;
    let mut timed_out: uint8_t = 0;
    usbi_mutex_lock(&mut (*ctx).flying_transfers_lock);
    timed_out = ((*itransfer).timeout_flags
        & USBI_TRANSFER_TIMED_OUT as libc::c_int as libc::c_uint) as uint8_t;
    usbi_mutex_unlock(&mut (*ctx).flying_transfers_lock);
    /* if the URB was cancelled due to timeout, report timeout to the user */
    if timed_out != 0 {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                b"usbi_handle_transfer_cancellation\x00",
            ))
            .as_ptr(),
            b"detected timeout cancellation\x00" as *const u8 as *const libc::c_char,
        );
        return usbi_handle_transfer_completion(itransfer, LIBUSB_TRANSFER_TIMED_OUT);
    }
    /* otherwise its a normal async cancel */
    return usbi_handle_transfer_completion(itransfer, LIBUSB_TRANSFER_CANCELLED);
}
/* Add a completed transfer to the completed_transfers list of the
 * context and signal the event. The backend's handle_transfer_completion()
 * function will be called the next time an event handler runs. */
#[no_mangle]
pub unsafe extern "C" fn usbi_signal_transfer_completion(itransfer: *mut usbi_transfer) {
    let dev_handle: *mut libusb_device_handle = (*((itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer))
        .dev_handle;
    if !dev_handle.is_null() {
        let ctx: *mut libusb_context = (*(*dev_handle).dev).ctx;
        let mut pending_events: libc::c_int = 0;
        usbi_mutex_lock(&mut (*ctx).event_data_lock);
        pending_events = usbi_pending_events(ctx);
        list_add_tail(
            &mut (*itransfer).completed_list,
            &mut (*ctx).completed_transfers,
        );
        if pending_events == 0 {
            usbi_signal_event(ctx);
        }
        usbi_mutex_unlock(&mut (*ctx).event_data_lock);
    };
}
/* * \ingroup libusb_poll
 * Attempt to acquire the event handling lock. This lock is used to ensure that
 * only one thread is monitoring libusb event sources at any one time.
 *
 * You only need to use this lock if you are developing an application
 * which calls poll() or select() on libusb's file descriptors directly.
 * If you stick to libusb's event handling loop functions (e.g.
 * libusb_handle_events()) then you do not need to be concerned with this
 * locking.
 *
 * While holding this lock, you are trusted to actually be handling events.
 * If you are no longer handling events, you must call libusb_unlock_events()
 * as soon as possible.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \returns 0 if the lock was obtained successfully
 * \returns 1 if the lock was not obtained (i.e. another thread holds the lock)
 * \ref libusb_mtasync
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_try_lock_events(mut ctx: *mut libusb_context) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut ru: libc::c_uint = 0;
    ctx = usbi_get_context(ctx);
    /* is someone else waiting to close a device? if so, don't let this thread
     * start event handling */
    usbi_mutex_lock(&mut (*ctx).event_data_lock);
    ru = (*ctx).device_close;
    usbi_mutex_unlock(&mut (*ctx).event_data_lock);
    if ru != 0 {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"libusb_try_lock_events\x00",
            ))
            .as_ptr(),
            b"someone else is closing a device\x00" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    r = usbi_mutex_trylock(&mut (*ctx).events_lock);
    if r != 0 {
        return 1 as libc::c_int;
    }
    (*ctx).event_handler_active = 1 as libc::c_int;
    return 0 as libc::c_int;
}
/* * \ingroup libusb_poll
 * Acquire the event handling lock, blocking until successful acquisition if
 * it is contended. This lock is used to ensure that only one thread is
 * monitoring libusb event sources at any one time.
 *
 * You only need to use this lock if you are developing an application
 * which calls poll() or select() on libusb's file descriptors directly.
 * If you stick to libusb's event handling loop functions (e.g.
 * libusb_handle_events()) then you do not need to be concerned with this
 * locking.
 *
 * While holding this lock, you are trusted to actually be handling events.
 * If you are no longer handling events, you must call libusb_unlock_events()
 * as soon as possible.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \ref libusb_mtasync
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_lock_events(mut ctx: *mut libusb_context) {
    ctx = usbi_get_context(ctx);
    usbi_mutex_lock(&mut (*ctx).events_lock);
    (*ctx).event_handler_active = 1 as libc::c_int;
}
/* * \ingroup libusb_poll
 * Release the lock previously acquired with libusb_try_lock_events() or
 * libusb_lock_events(). Releasing this lock will wake up any threads blocked
 * on libusb_wait_for_event().
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \ref libusb_mtasync
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_unlock_events(mut ctx: *mut libusb_context) {
    ctx = usbi_get_context(ctx);
    (*ctx).event_handler_active = 0 as libc::c_int;
    usbi_mutex_unlock(&mut (*ctx).events_lock);
    /* FIXME: perhaps we should be a bit more efficient by not broadcasting
     * the availability of the events lock when we are modifying pollfds
     * (check ctx->device_close)? */
    usbi_mutex_lock(&mut (*ctx).event_waiters_lock);
    usbi_cond_broadcast(&mut (*ctx).event_waiters_cond);
    usbi_mutex_unlock(&mut (*ctx).event_waiters_lock);
}
/* * \ingroup libusb_poll
 * Determine if it is still OK for this thread to be doing event handling.
 *
 * Sometimes, libusb needs to temporarily pause all event handlers, and this
 * is the function you should use before polling file descriptors to see if
 * this is the case.
 *
 * If this function instructs your thread to give up the events lock, you
 * should just continue the usual logic that is documented in \ref libusb_mtasync.
 * On the next iteration, your thread will fail to obtain the events lock,
 * and will hence become an event waiter.
 *
 * This function should be called while the events lock is held: you don't
 * need to worry about the results of this function if your thread is not
 * the current event handler.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \returns 1 if event handling can start or continue
 * \returns 0 if this thread must give up the events lock
 * \ref fullstory "Multi-threaded I/O: the full story"
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_event_handling_ok(mut ctx: *mut libusb_context) -> libc::c_int {
    let mut r: libc::c_uint = 0;
    ctx = usbi_get_context(ctx);
    /* is someone else waiting to close a device? if so, don't let this thread
     * continue event handling */
    usbi_mutex_lock(&mut (*ctx).event_data_lock);
    r = (*ctx).device_close;
    usbi_mutex_unlock(&mut (*ctx).event_data_lock);
    if r != 0 {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                b"libusb_event_handling_ok\x00",
            ))
            .as_ptr(),
            b"someone else is closing a device\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
/* * \ingroup libusb_poll
 * Determine if an active thread is handling events (i.e. if anyone is holding
 * the event handling lock).
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \returns 1 if a thread is handling events
 * \returns 0 if there are no threads currently handling events
 * \ref libusb_mtasync
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_event_handler_active(mut ctx: *mut libusb_context) -> libc::c_int {
    let mut r: libc::c_uint = 0;
    ctx = usbi_get_context(ctx);
    /* is someone else waiting to close a device? if so, don't let this thread
     * start event handling -- indicate that event handling is happening */
    usbi_mutex_lock(&mut (*ctx).event_data_lock);
    r = (*ctx).device_close;
    usbi_mutex_unlock(&mut (*ctx).event_data_lock);
    if r != 0 {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                b"libusb_event_handler_active\x00",
            ))
            .as_ptr(),
            b"someone else is closing a device\x00" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    return (*ctx).event_handler_active;
}
/* * \ingroup libusb_poll
 * Interrupt any active thread that is handling events. This is mainly useful
 * for interrupting a dedicated event handling thread when an application
 * wishes to call libusb_exit().
 *
 * Since version 1.0.21, \ref LIBUSB_API_VERSION >= 0x01000105
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \ref libusb_mtasync
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_interrupt_event_handler(mut ctx: *mut libusb_context) {
    let mut pending_events: libc::c_int = 0;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
            b"libusb_interrupt_event_handler\x00",
        ))
        .as_ptr(),
        b" \x00" as *const u8 as *const libc::c_char,
    );
    ctx = usbi_get_context(ctx);
    usbi_mutex_lock(&mut (*ctx).event_data_lock);
    pending_events = usbi_pending_events(ctx);
    (*ctx).event_flags |= USBI_EVENT_USER_INTERRUPT as libc::c_int as libc::c_uint;
    if pending_events == 0 {
        usbi_signal_event(ctx);
    }
    usbi_mutex_unlock(&mut (*ctx).event_data_lock);
}
/* * \ingroup libusb_poll
 * Acquire the event waiters lock. This lock is designed to be obtained under
 * the situation where you want to be aware when events are completed, but
 * some other thread is event handling so calling libusb_handle_events() is not
 * allowed.
 *
 * You then obtain this lock, re-check that another thread is still handling
 * events, then call libusb_wait_for_event().
 *
 * You only need to use this lock if you are developing an application
 * which calls poll() or select() on libusb's file descriptors directly,
 * <b>and</b> may potentially be handling events from 2 threads simultaenously.
 * If you stick to libusb's event handling loop functions (e.g.
 * libusb_handle_events()) then you do not need to be concerned with this
 * locking.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \ref libusb_mtasync
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_lock_event_waiters(mut ctx: *mut libusb_context) {
    ctx = usbi_get_context(ctx);
    usbi_mutex_lock(&mut (*ctx).event_waiters_lock);
}
/* * \ingroup libusb_poll
 * Release the event waiters lock.
 * \param ctx the context to operate on, or NULL for the default context
 * \ref libusb_mtasync
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_unlock_event_waiters(mut ctx: *mut libusb_context) {
    ctx = usbi_get_context(ctx);
    usbi_mutex_unlock(&mut (*ctx).event_waiters_lock);
}
/* * \ingroup libusb_poll
 * Wait for another thread to signal completion of an event. Must be called
 * with the event waiters lock held, see libusb_lock_event_waiters().
 *
 * This function will block until any of the following conditions are met:
 * -# The timeout expires
 * -# A transfer completes
 * -# A thread releases the event handling lock through libusb_unlock_events()
 *
 * Condition 1 is obvious. Condition 2 unblocks your thread <em>after</em>
 * the callback for the transfer has completed. Condition 3 is important
 * because it means that the thread that was previously handling events is no
 * longer doing so, so if any events are to complete, another thread needs to
 * step up and start event handling.
 *
 * This function releases the event waiters lock before putting your thread
 * to sleep, and reacquires the lock as it is being woken up.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param tv maximum timeout for this blocking function. A NULL value
 * indicates unlimited timeout.
 * \returns 0 after a transfer completes or another thread stops event handling
 * \returns 1 if the timeout expired
 * \ref libusb_mtasync
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_wait_for_event(
    mut ctx: *mut libusb_context,
    tv: *mut timeval,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    ctx = usbi_get_context(ctx);
    if tv.is_null() {
        usbi_cond_wait(
            &mut (*ctx).event_waiters_cond,
            &mut (*ctx).event_waiters_lock,
        );
        return 0 as libc::c_int;
    }
    r = usbi_cond_timedwait(
        &mut (*ctx).event_waiters_cond,
        &mut (*ctx).event_waiters_lock,
        tv,
    );
    if r < 0 as libc::c_int {
        return r;
    } else {
        return (r == 110 as libc::c_int) as libc::c_int;
    };
}
unsafe extern "C" fn handle_timeout(mut itransfer: *mut usbi_transfer) {
    let transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    let mut r: libc::c_int = 0;
    (*itransfer).timeout_flags |= USBI_TRANSFER_TIMEOUT_HANDLED as libc::c_int as libc::c_uint;
    r = libusb_cancel_transfer(transfer);
    if r == LIBUSB_SUCCESS as libc::c_int {
        (*itransfer).timeout_flags |= USBI_TRANSFER_TIMED_OUT as libc::c_int as libc::c_uint
    } else {
        usbi_log(
            (*(*(*transfer).dev_handle).dev).ctx,
            LIBUSB_LOG_LEVEL_WARNING,
            (*::std::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"handle_timeout\x00"))
                .as_ptr(),
            b"async cancel failed %d errno=%d\x00" as *const u8 as *const libc::c_char,
            r,
            *__errno_location(),
        );
    };
}
unsafe extern "C" fn handle_timeouts_locked(ctx: *mut libusb_context) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut systime: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut itransfer: *mut usbi_transfer = 0 as *mut usbi_transfer;
    if (*ctx).flying_transfers.next == &mut (*ctx).flying_transfers as *mut list_head {
        return 0 as libc::c_int;
    }
    /* get current time */
    r = clock_gettime(1 as libc::c_int, &mut systime);
    if r < 0 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"handle_timeouts_locked\x00",
            ))
            .as_ptr(),
            b"failed to read monotonic clock, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    /* iterate through flying transfers list, finding all transfers that
     * have expired timeouts */
    itransfer = ((*ctx).flying_transfers.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
        as *mut usbi_transfer;
    while &mut (*itransfer).list as *mut list_head != &mut (*ctx).flying_transfers as *mut list_head
    {
        let cur_ts: *mut timespec = &mut (*itransfer).timeout;
        /* if we've reached transfers of infinite timeout, we're all done */
        if !((*cur_ts).tv_sec != 0 || (*cur_ts).tv_nsec != 0) {
            return 0 as libc::c_int;
        }
        /* ignore timeouts we've already handled */
        if !((*itransfer).timeout_flags
            & (USBI_TRANSFER_TIMEOUT_HANDLED as libc::c_int
                | USBI_TRANSFER_OS_HANDLES_TIMEOUT as libc::c_int) as libc::c_uint
            != 0)
        {
            /* if transfer has non-expired timeout, nothing more to do */
            if if (*cur_ts).tv_sec == systime.tv_sec {
                ((*cur_ts).tv_nsec > systime.tv_nsec) as libc::c_int
            } else {
                ((*cur_ts).tv_sec > systime.tv_sec) as libc::c_int
            } != 0
            {
                return 0 as libc::c_int;
            }
            /* otherwise, we've got an expired timeout to handle */
            handle_timeout(itransfer);
        }
        itransfer = ((*itransfer).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
            as *mut usbi_transfer
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_timeouts(mut ctx: *mut libusb_context) -> libc::c_int {
    let mut r: libc::c_int = 0;
    ctx = usbi_get_context(ctx);
    usbi_mutex_lock(&mut (*ctx).flying_transfers_lock);
    r = handle_timeouts_locked(ctx);
    usbi_mutex_unlock(&mut (*ctx).flying_transfers_lock);
    return r;
}
unsafe extern "C" fn handle_timerfd_trigger(ctx: *mut libusb_context) -> libc::c_int {
    let mut r: libc::c_int = 0;
    usbi_mutex_lock(&mut (*ctx).flying_transfers_lock);
    /* process the timeout that just happened */
    r = handle_timeouts_locked(ctx);
    if !(r < 0 as libc::c_int) {
        /* arm for next timeout */
        r = arm_timerfd_for_next_timeout(ctx)
    }
    usbi_mutex_unlock(&mut (*ctx).flying_transfers_lock);
    return r;
}
/* do the actual event handling. assumes that no other thread is concurrently
 * doing the same thing. */
unsafe extern "C" fn handle_events(mut ctx: *mut libusb_context, tv: *mut timeval) -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    let mut ipollfd: *mut usbi_pollfd = 0 as *mut usbi_pollfd;
    let mut nfds: usbi_nfds_t = 0 as libc::c_int as usbi_nfds_t;
    let mut internal_nfds: usbi_nfds_t = 0;
    let mut fds: *mut pollfd = 0 as *mut pollfd;
    let mut timeout_ms: libc::c_int = 0;
    /* prevent attempts to recursively handle events (e.g. calling into
     * libusb_handle_events() from within a hotplug or transfer callback) */
    usbi_mutex_lock(&mut (*ctx).event_data_lock);
    r = 0 as libc::c_int;
    if usbi_handling_events(ctx) != 0 {
        r = LIBUSB_ERROR_BUSY as libc::c_int
    } else {
        usbi_start_event_handling(ctx);
    }
    usbi_mutex_unlock(&mut (*ctx).event_data_lock);
    if r != 0 {
        return r;
    }
    /* there are certain fds that libusb uses internally, currently:
     *
     *   1) event pipe
     *   2) timerfd
     *
     * the backend will never need to attempt to handle events on these fds, so
     * we determine how many fds are in use internally for this context and when
     * handle_events() is called in the backend, the pollfd list and count will
     * be adjusted to skip over these internal fds */
    if usbi_using_timerfd(ctx) != 0 {
        internal_nfds = 2 as libc::c_int as usbi_nfds_t
    } else {
        internal_nfds = 1 as libc::c_int as usbi_nfds_t
    }
    /* only reallocate the poll fds when the list of poll fds has been modified
     * since the last poll, otherwise reuse them to save the additional overhead */
    usbi_mutex_lock(&mut (*ctx).event_data_lock);
    /* clean up removed poll fds */
    cleanup_removed_pollfds(ctx);
    if (*ctx).event_flags & USBI_EVENT_POLLFDS_MODIFIED as libc::c_int as libc::c_uint != 0 {
        let mut i: libc::c_int = 0 as libc::c_int;
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"handle_events\x00"))
                .as_ptr(),
            b"poll fds modified, reallocating\x00" as *const u8 as *const libc::c_char,
        );
        free((*ctx).pollfds as *mut libc::c_void);
        (*ctx).pollfds = 0 as *mut pollfd;
        /* sanity check - it is invalid for a context to have fewer than the
         * required internal fds (memory corruption?) */
        if (*ctx).pollfds_cnt >= internal_nfds {
        } else {
            __assert_fail(
                b"ctx->pollfds_cnt >= internal_nfds\x00" as *const u8 as *const libc::c_char,
                b"/home/konstantin/Projects/tests/rust/libusb2rs/libusb/libusb/io.c\x00"
                    as *const u8 as *const libc::c_char,
                2144 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 61], &[libc::c_char; 61]>(
                    b"int handle_events(struct libusb_context *, struct timeval *)\x00",
                ))
                .as_ptr(),
            );
        }
        (*ctx).pollfds = calloc(
            (*ctx).pollfds_cnt,
            ::std::mem::size_of::<pollfd>() as libc::c_ulong,
        ) as *mut pollfd;
        if (*ctx).pollfds.is_null() {
            usbi_mutex_unlock(&mut (*ctx).event_data_lock);
            r = LIBUSB_ERROR_NO_MEM as libc::c_int;
            current_block = 17317664605740480400;
        } else {
            ipollfd = ((*ctx).ipollfds.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
                as *mut usbi_pollfd;
            while &mut (*ipollfd).list as *mut list_head != &mut (*ctx).ipollfds as *mut list_head {
                let pollfd: *mut libusb_pollfd = &mut (*ipollfd).pollfd;
                (*(*ctx).pollfds.offset(i as isize)).fd = (*pollfd).fd;
                (*(*ctx).pollfds.offset(i as isize)).events = (*pollfd).events;
                i += 1;
                ipollfd = ((*ipollfd).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
                    as *mut usbi_pollfd
            }
            /* reset the flag now that we have the updated list */
            (*ctx).event_flags &= !(USBI_EVENT_POLLFDS_MODIFIED as libc::c_int) as libc::c_uint;
            /* if no further pending events, clear the event pipe so that we do
             * not immediately return from poll */
            if usbi_pending_events(ctx) == 0 {
                usbi_clear_event(ctx);
            }
            current_block = 18377268871191777778;
        }
    } else {
        current_block = 18377268871191777778;
    }
    match current_block {
        18377268871191777778 => {
            fds = (*ctx).pollfds;
            nfds = (*ctx).pollfds_cnt;
            usbi_mutex_unlock(&mut (*ctx).event_data_lock);
            timeout_ms = (((*tv).tv_sec * 1000 as libc::c_int as libc::c_long) as libc::c_int
                as libc::c_long
                + (*tv).tv_usec / 1000 as libc::c_int as libc::c_long)
                as libc::c_int;
            /* round up to next millisecond */
            if (*tv).tv_usec % 1000 as libc::c_int as libc::c_long != 0 {
                timeout_ms += 1
            }
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"handle_events\x00"))
                    .as_ptr(),
                b"poll() %d fds with timeout in %dms\x00" as *const u8 as *const libc::c_char,
                nfds as libc::c_int,
                timeout_ms,
            );
            r = poll(fds, nfds, timeout_ms);
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"handle_events\x00"))
                    .as_ptr(),
                b"poll() returned %d\x00" as *const u8 as *const libc::c_char,
                r,
            );
            if r == 0 as libc::c_int {
                r = handle_timeouts(ctx)
            } else if r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int {
                r = LIBUSB_ERROR_INTERRUPTED as libc::c_int
            } else if r < 0 as libc::c_int {
                usbi_log(
                    ctx,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                        b"handle_events\x00",
                    ))
                    .as_ptr(),
                    b"poll failed, errno=%d\x00" as *const u8 as *const libc::c_char,
                    *__errno_location(),
                );
                r = LIBUSB_ERROR_IO as libc::c_int
            } else {
                /* fds[0] is always the event pipe */
                if (*fds.offset(0 as libc::c_int as isize)).revents != 0 {
                    let mut hotplug_msgs: list_head = list_head {
                        prev: 0 as *mut list_head,
                        next: 0 as *mut list_head,
                    };
                    let mut itransfer: *mut usbi_transfer = 0 as *mut usbi_transfer;
                    let mut hotplug_cb_deregistered: libc::c_int = 0 as libc::c_int;
                    let mut ret: libc::c_int = 0 as libc::c_int;
                    list_init(&mut hotplug_msgs);
                    usbi_log(
                        0 as *mut libusb_context,
                        LIBUSB_LOG_LEVEL_DEBUG,
                        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                            b"handle_events\x00",
                        ))
                        .as_ptr(),
                        b"caught a fish on the event pipe\x00" as *const u8 as *const libc::c_char,
                    );
                    /* take the the event data lock while processing events */
                    usbi_mutex_lock(&mut (*ctx).event_data_lock);
                    /* check if someone added a new poll fd */
                    if (*ctx).event_flags
                        & USBI_EVENT_POLLFDS_MODIFIED as libc::c_int as libc::c_uint
                        != 0
                    {
                        usbi_log(
                            0 as *mut libusb_context,
                            LIBUSB_LOG_LEVEL_DEBUG,
                            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                b"handle_events\x00",
                            ))
                            .as_ptr(),
                            b"someone updated the poll fds\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    if (*ctx).event_flags & USBI_EVENT_USER_INTERRUPT as libc::c_int as libc::c_uint
                        != 0
                    {
                        usbi_log(
                            0 as *mut libusb_context,
                            LIBUSB_LOG_LEVEL_DEBUG,
                            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                b"handle_events\x00",
                            ))
                            .as_ptr(),
                            b"someone purposely interrupted\x00" as *const u8
                                as *const libc::c_char,
                        );
                        (*ctx).event_flags &=
                            !(USBI_EVENT_USER_INTERRUPT as libc::c_int) as libc::c_uint
                    }
                    if (*ctx).event_flags
                        & USBI_EVENT_HOTPLUG_CB_DEREGISTERED as libc::c_int as libc::c_uint
                        != 0
                    {
                        usbi_log(
                            0 as *mut libusb_context,
                            LIBUSB_LOG_LEVEL_DEBUG,
                            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                b"handle_events\x00",
                            ))
                            .as_ptr(),
                            b"someone unregistered a hotplug cb\x00" as *const u8
                                as *const libc::c_char,
                        );
                        (*ctx).event_flags &=
                            !(USBI_EVENT_HOTPLUG_CB_DEREGISTERED as libc::c_int) as libc::c_uint;
                        hotplug_cb_deregistered = 1 as libc::c_int
                    }
                    /* check if someone is closing a device */
                    if (*ctx).device_close != 0 {
                        usbi_log(
                            0 as *mut libusb_context,
                            LIBUSB_LOG_LEVEL_DEBUG,
                            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                b"handle_events\x00",
                            ))
                            .as_ptr(),
                            b"someone is closing a device\x00" as *const u8 as *const libc::c_char,
                        );
                    }
                    /* check for any pending hotplug messages */
                    if !((*ctx).hotplug_msgs.next == &mut (*ctx).hotplug_msgs as *mut list_head) {
                        usbi_log(
                            0 as *mut libusb_context,
                            LIBUSB_LOG_LEVEL_DEBUG,
                            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                b"handle_events\x00",
                            ))
                            .as_ptr(),
                            b"hotplug message received\x00" as *const u8 as *const libc::c_char,
                        );
                        list_cut(&mut hotplug_msgs, &mut (*ctx).hotplug_msgs);
                    }
                    /* complete any pending transfers */
                    while ret == 0 as libc::c_int
                        && !((*ctx).completed_transfers.next
                            == &mut (*ctx).completed_transfers as *mut list_head)
                    {
                        itransfer = ((*ctx).completed_transfers.next as uintptr_t)
                            .wrapping_sub(24 as libc::c_ulong)
                            as *mut usbi_transfer;
                        list_del(&mut (*itransfer).completed_list);
                        usbi_mutex_unlock(&mut (*ctx).event_data_lock);
                        ret = usbi_backend
                            .handle_transfer_completion
                            .expect("non-null function pointer")(
                            itransfer
                        );
                        if ret != 0 {
                            usbi_log(
                                ctx,
                                LIBUSB_LOG_LEVEL_ERROR,
                                (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                    b"handle_events\x00",
                                ))
                                .as_ptr(),
                                b"backend handle_transfer_completion failed with error %d\x00"
                                    as *const u8
                                    as *const libc::c_char,
                                ret,
                            );
                        }
                        usbi_mutex_lock(&mut (*ctx).event_data_lock);
                    }
                    /* if no further pending events, clear the event pipe */
                    if usbi_pending_events(ctx) == 0 {
                        usbi_clear_event(ctx);
                    }
                    usbi_mutex_unlock(&mut (*ctx).event_data_lock);
                    if hotplug_cb_deregistered != 0 {
                        usbi_hotplug_deregister(ctx, 0 as libc::c_int);
                    }
                    /* process the hotplug messages, if any */
                    while !(hotplug_msgs.next == &mut hotplug_msgs as *mut list_head) {
                        let message: *mut libusb_hotplug_message = (hotplug_msgs.next as uintptr_t)
                            .wrapping_sub(16 as libc::c_ulong)
                            as *mut libusb_hotplug_message;
                        usbi_hotplug_match(ctx, (*message).device, (*message).event);
                        /* the device left, dereference the device */
                        if LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT as libc::c_int as libc::c_uint
                            == (*message).event as libc::c_uint
                        {
                            libusb_unref_device((*message).device);
                        }
                        list_del(&mut (*message).list);
                        free(message as *mut libc::c_void);
                    }
                    if ret != 0 {
                        /* return error code */
                        r = ret;
                        current_block = 17317664605740480400;
                    } else {
                        r -= 1;
                        if 0 as libc::c_int == r {
                            current_block = 17317664605740480400;
                        } else {
                            current_block = 8723848109087415604;
                        }
                    }
                } else {
                    current_block = 8723848109087415604;
                }
                match current_block {
                    17317664605740480400 => {}
                    _ =>
                    /* on timerfd configurations, fds[1] is the timerfd */
                    {
                        if usbi_using_timerfd(ctx) != 0
                            && (*fds.offset(1 as libc::c_int as isize)).revents as libc::c_int != 0
                        {
                            /* timerfd indicates that a timeout has expired */
                            let mut ret_0: libc::c_int = 0;
                            usbi_log(
                                0 as *mut libusb_context,
                                LIBUSB_LOG_LEVEL_DEBUG,
                                (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                    b"handle_events\x00",
                                ))
                                .as_ptr(),
                                b"timerfd triggered\x00" as *const u8 as *const libc::c_char,
                            );
                            ret_0 = handle_timerfd_trigger(ctx);
                            if ret_0 < 0 as libc::c_int {
                                /* return error code */
                                r = ret_0;
                                current_block = 17317664605740480400;
                            } else {
                                r -= 1;
                                if 0 as libc::c_int == r {
                                    current_block = 17317664605740480400;
                                } else {
                                    current_block = 10301740260014665685;
                                }
                            }
                        } else {
                            current_block = 10301740260014665685;
                        }
                        match current_block {
                            17317664605740480400 => {}
                            _ => {
                                ipollfd = ((*ctx).removed_ipollfds.next as uintptr_t)
                                    .wrapping_sub(8 as libc::c_ulong)
                                    as *mut usbi_pollfd;
                                while &mut (*ipollfd).list as *mut list_head
                                    != &mut (*ctx).removed_ipollfds as *mut list_head
                                {
                                    let mut n: usbi_nfds_t = 0;
                                    n = internal_nfds;
                                    while n < nfds {
                                        if (*ipollfd).pollfd.fd == (*fds.offset(n as isize)).fd {
                                            /* pollfd was removed between the creation of the fd
                                             * array and here. remove any triggered revent as
                                             * it is no longer relevant */
                                            usbi_log(
                                                0 as *mut libusb_context,
                                                LIBUSB_LOG_LEVEL_DEBUG,
                                                (*::std::mem::transmute::<
                                                    &[u8; 14],
                                                    &[libc::c_char; 14],
                                                >(
                                                    b"handle_events\x00"
                                                ))
                                                .as_ptr(),
                                                b"pollfd %d was removed. ignoring raised events\x00"
                                                    as *const u8
                                                    as *const libc::c_char,
                                                (*fds.offset(n as isize)).fd,
                                            );
                                            (*fds.offset(n as isize)).revents =
                                                0 as libc::c_int as libc::c_short;
                                            break;
                                        } else {
                                            n = n.wrapping_add(1)
                                        }
                                    }
                                    ipollfd = ((*ipollfd).list.next as uintptr_t)
                                        .wrapping_sub(8 as libc::c_ulong)
                                        as *mut usbi_pollfd
                                }
                                r = usbi_backend
                                    .handle_events
                                    .expect("non-null function pointer")(
                                    ctx,
                                    fds.offset(internal_nfds as isize),
                                    nfds.wrapping_sub(internal_nfds),
                                    r,
                                );
                                if r != 0 {
                                    usbi_log(
                                        ctx,
                                        LIBUSB_LOG_LEVEL_ERROR,
                                        (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                            b"handle_events\x00",
                                        ))
                                        .as_ptr(),
                                        b"backend handle_events failed with error %d\x00"
                                            as *const u8
                                            as *const libc::c_char,
                                        r,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    usbi_end_event_handling(ctx);
    return r;
}
/* returns the smallest of:
 *  1. timeout of next URB
 *  2. user-supplied timeout
 * returns 1 if there is an already-expired timeout, otherwise returns 0
 * and populates out
 */
unsafe extern "C" fn get_next_timeout(
    ctx: *mut libusb_context,
    tv: *mut timeval,
    out: *mut timeval,
) -> libc::c_int {
    let mut timeout: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let r: libc::c_int = libusb_get_next_timeout(ctx, &mut timeout);
    if r != 0 {
        /* timeout already expired? */
        if !(timeout.tv_sec != 0 || timeout.tv_usec != 0) {
            return 1 as libc::c_int;
        }
        /* choose the smallest of next URB timeout or user specified timeout */
        if if timeout.tv_sec == (*tv).tv_sec {
            (timeout.tv_usec < (*tv).tv_usec) as libc::c_int
        } else {
            (timeout.tv_sec < (*tv).tv_sec) as libc::c_int
        } != 0
        {
            *out = timeout
        } else {
            *out = *tv
        }
    } else {
        *out = *tv
    }
    return 0 as libc::c_int;
}
/* * \ingroup libusb_poll
 * Handle any pending events.
 *
 * libusb determines "pending events" by checking if any timeouts have expired
 * and by checking the set of file descriptors for activity.
 *
 * If a zero timeval is passed, this function will handle any already-pending
 * events and then immediately return in non-blocking style.
 *
 * If a non-zero timeval is passed and no events are currently pending, this
 * function will block waiting for events to handle up until the specified
 * timeout. If an event arrives or a signal is raised, this function will
 * return early.
 *
 * If the parameter completed is not NULL then <em>after obtaining the event
 * handling lock</em> this function will return immediately if the integer
 * pointed to is not 0. This allows for race free waiting for the completion
 * of a specific transfer.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param tv the maximum time to block waiting for events, or an all zero
 * timeval struct for non-blocking mode
 * \param completed pointer to completion integer to check, or NULL
 * \returns 0 on success, or a LIBUSB_ERROR code on failure
 * \ref libusb_mtasync
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_handle_events_timeout_completed(
    mut ctx: *mut libusb_context,
    tv: *mut timeval,
    completed: *mut libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut poll_timeout: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    ctx = usbi_get_context(ctx);
    r = get_next_timeout(ctx, tv, &mut poll_timeout);
    if r != 0 {
        /* timeout already expired */
        return handle_timeouts(ctx);
    }
    loop {
        if libusb_try_lock_events(ctx) == 0 as libc::c_int {
            if completed.is_null() || *completed == 0 {
                /* we obtained the event lock: do our own event handling */
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                        b"libusb_handle_events_timeout_completed\x00",
                    ))
                    .as_ptr(),
                    b"doing our own event handling\x00" as *const u8 as *const libc::c_char,
                );
                r = handle_events(ctx, &mut poll_timeout)
            }
            libusb_unlock_events(ctx);
            return r;
        }
        /* another thread is doing event handling. wait for thread events that
         * notify event completion. */
        libusb_lock_event_waiters(ctx);
        if !completed.is_null() && *completed != 0 {
            break;
        }
        if libusb_event_handler_active(ctx) == 0 {
            /* we hit a race: whoever was event handling earlier finished in the
             * time it took us to reach this point. try the cycle again. */
            libusb_unlock_event_waiters(ctx);
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"libusb_handle_events_timeout_completed\x00",
                ))
                .as_ptr(),
                b"event handler was active but went away, retrying\x00" as *const u8
                    as *const libc::c_char,
            );
        } else {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    b"libusb_handle_events_timeout_completed\x00",
                ))
                .as_ptr(),
                b"another thread is doing event handling\x00" as *const u8 as *const libc::c_char,
            );
            r = libusb_wait_for_event(ctx, &mut poll_timeout);
            break;
        }
    }
    libusb_unlock_event_waiters(ctx);
    if r < 0 as libc::c_int {
        return r;
    } else if r == 1 as libc::c_int {
        return handle_timeouts(ctx);
    } else {
        return 0 as libc::c_int;
    };
}
/* * \ingroup libusb_poll
 * Handle any pending events
 *
 * Like libusb_handle_events_timeout_completed(), but without the completed
 * parameter, calling this function is equivalent to calling
 * libusb_handle_events_timeout_completed() with a NULL completed parameter.
 *
 * This function is kept primarily for backwards compatibility.
 * All new code should call libusb_handle_events_completed() or
 * libusb_handle_events_timeout_completed() to avoid race conditions.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param tv the maximum time to block waiting for events, or an all zero
 * timeval struct for non-blocking mode
 * \returns 0 on success, or a LIBUSB_ERROR code on failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_handle_events_timeout(
    ctx: *mut libusb_context,
    tv: *mut timeval,
) -> libc::c_int {
    return libusb_handle_events_timeout_completed(ctx, tv, 0 as *mut libc::c_int);
}
/* * \ingroup libusb_poll
 * Handle any pending events in blocking mode. There is currently a timeout
 * hardcoded at 60 seconds but we plan to make it unlimited in future. For
 * finer control over whether this function is blocking or non-blocking, or
 * for control over the timeout, use libusb_handle_events_timeout_completed()
 * instead.
 *
 * This function is kept primarily for backwards compatibility.
 * All new code should call libusb_handle_events_completed() or
 * libusb_handle_events_timeout_completed() to avoid race conditions.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \returns 0 on success, or a LIBUSB_ERROR code on failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_handle_events(ctx: *mut libusb_context) -> libc::c_int {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    tv.tv_sec = 60 as libc::c_int as __time_t;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    return libusb_handle_events_timeout_completed(ctx, &mut tv, 0 as *mut libc::c_int);
}
/* * \ingroup libusb_poll
 * Handle any pending events in blocking mode.
 *
 * Like libusb_handle_events(), with the addition of a completed parameter
 * to allow for race free waiting for the completion of a specific transfer.
 *
 * See libusb_handle_events_timeout_completed() for details on the completed
 * parameter.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param completed pointer to completion integer to check, or NULL
 * \returns 0 on success, or a LIBUSB_ERROR code on failure
 * \ref libusb_mtasync
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_handle_events_completed(
    ctx: *mut libusb_context,
    completed: *mut libc::c_int,
) -> libc::c_int {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    tv.tv_sec = 60 as libc::c_int as __time_t;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    return libusb_handle_events_timeout_completed(ctx, &mut tv, completed);
}
/* * \ingroup libusb_poll
 * Handle any pending events by polling file descriptors, without checking if
 * any other threads are already doing so. Must be called with the event lock
 * held, see libusb_lock_events().
 *
 * This function is designed to be called under the situation where you have
 * taken the event lock and are calling poll()/select() directly on libusb's
 * file descriptors (as opposed to using libusb_handle_events() or similar).
 * You detect events on libusb's descriptors, so you then call this function
 * with a zero timeout value (while still holding the event lock).
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param tv the maximum time to block waiting for events, or zero for
 * non-blocking mode
 * \returns 0 on success, or a LIBUSB_ERROR code on failure
 * \ref libusb_mtasync
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_handle_events_locked(
    mut ctx: *mut libusb_context,
    tv: *mut timeval,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut poll_timeout: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    ctx = usbi_get_context(ctx);
    r = get_next_timeout(ctx, tv, &mut poll_timeout);
    if r != 0 {
        /* timeout already expired */
        return handle_timeouts(ctx);
    }
    return handle_events(ctx, &mut poll_timeout);
}
/* * \ingroup libusb_poll
 * Determines whether your application must apply special timing considerations
 * when monitoring libusb's file descriptors.
 *
 * This function is only useful for applications which retrieve and poll
 * libusb's file descriptors in their own main loop (\ref libusb_pollmain).
 *
 * Ordinarily, libusb's event handler needs to be called into at specific
 * moments in time (in addition to times when there is activity on the file
 * descriptor set). The usual approach is to use libusb_get_next_timeout()
 * to learn about when the next timeout occurs, and to adjust your
 * poll()/select() timeout accordingly so that you can make a call into the
 * library at that time.
 *
 * Some platforms supported by libusb do not come with this baggage - any
 * events relevant to timing will be represented by activity on the file
 * descriptor set, and libusb_get_next_timeout() will always return 0.
 * This function allows you to detect whether you are running on such a
 * platform.
 *
 * Since v1.0.5.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \returns 0 if you must call into libusb at times determined by
 * libusb_get_next_timeout(), or 1 if all timeout events are handled internally
 * or through regular activity on the file descriptors.
 * \ref libusb_pollmain "Polling libusb file descriptors for event handling"
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_pollfds_handle_timeouts(
    mut ctx: *mut libusb_context,
) -> libc::c_int {
    ctx = usbi_get_context(ctx);
    return usbi_using_timerfd(ctx);
}
/* * \ingroup libusb_poll
 * Determine the next internal timeout that libusb needs to handle. You only
 * need to use this function if you are calling poll() or select() or similar
 * on libusb's file descriptors yourself - you do not need to use it if you
 * are calling libusb_handle_events() or a variant directly.
 *
 * You should call this function in your main loop in order to determine how
 * long to wait for select() or poll() to return results. libusb needs to be
 * called into at this timeout, so you should use it as an upper bound on
 * your select() or poll() call.
 *
 * When the timeout has expired, call into libusb_handle_events_timeout()
 * (perhaps in non-blocking mode) so that libusb can handle the timeout.
 *
 * This function may return 1 (success) and an all-zero timeval. If this is
 * the case, it indicates that libusb has a timeout that has already expired
 * so you should call libusb_handle_events_timeout() or similar immediately.
 * A return code of 0 indicates that there are no pending timeouts.
 *
 * On some platforms, this function will always returns 0 (no pending
 * timeouts). See \ref polltime.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param tv output location for a relative time against the current
 * clock in which libusb must be called into in order to process timeout events
 * \returns 0 if there are no pending timeouts, 1 if a timeout was returned,
 * or LIBUSB_ERROR_OTHER on failure
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_next_timeout(
    mut ctx: *mut libusb_context,
    mut tv: *mut timeval,
) -> libc::c_int {
    let mut itransfer: *mut usbi_transfer = 0 as *mut usbi_transfer;
    let mut systime: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut next_timeout: timespec = {
        let init = timespec {
            tv_sec: 0 as libc::c_int as __time_t,
            tv_nsec: 0 as libc::c_int as __syscall_slong_t,
        };
        init
    };
    let mut r: libc::c_int = 0;
    ctx = usbi_get_context(ctx);
    if usbi_using_timerfd(ctx) != 0 {
        return 0 as libc::c_int;
    }
    usbi_mutex_lock(&mut (*ctx).flying_transfers_lock);
    if (*ctx).flying_transfers.next == &mut (*ctx).flying_transfers as *mut list_head {
        usbi_mutex_unlock(&mut (*ctx).flying_transfers_lock);
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"libusb_get_next_timeout\x00",
            ))
            .as_ptr(),
            b"no URBs, no timeout!\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    /* find next transfer which hasn't already been processed as timed out */
    itransfer = ((*ctx).flying_transfers.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
        as *mut usbi_transfer;
    while &mut (*itransfer).list as *mut list_head != &mut (*ctx).flying_transfers as *mut list_head
    {
        if (*itransfer).timeout_flags
            & (USBI_TRANSFER_TIMEOUT_HANDLED as libc::c_int
                | USBI_TRANSFER_OS_HANDLES_TIMEOUT as libc::c_int) as libc::c_uint
            != 0
        {
            itransfer = ((*itransfer).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
                as *mut usbi_transfer
        } else {
            /* if we've reached transfers of infinte timeout, we're done looking */
            if !((*itransfer).timeout.tv_sec != 0 || (*itransfer).timeout.tv_nsec != 0) {
                break;
            }
            next_timeout = (*itransfer).timeout;
            break;
        }
    }
    usbi_mutex_unlock(&mut (*ctx).flying_transfers_lock);
    if !(next_timeout.tv_sec != 0 || next_timeout.tv_nsec != 0) {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"libusb_get_next_timeout\x00",
            ))
            .as_ptr(),
            b"no URB with timeout or all handled by OS; no timeout!\x00" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    r = clock_gettime(1 as libc::c_int, &mut systime);
    if r < 0 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"libusb_get_next_timeout\x00",
            ))
            .as_ptr(),
            b"failed to read monotonic clock, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return 0 as libc::c_int;
    }
    if if systime.tv_sec == next_timeout.tv_sec {
        (systime.tv_nsec < next_timeout.tv_nsec) as libc::c_int
    } else {
        (systime.tv_sec < next_timeout.tv_sec) as libc::c_int
    } == 0
    {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"libusb_get_next_timeout\x00",
            ))
            .as_ptr(),
            b"first timeout already expired\x00" as *const u8 as *const libc::c_char,
        );
        (*tv).tv_usec = 0 as libc::c_int as __suseconds_t;
        (*tv).tv_sec = (*tv).tv_usec
    } else {
        next_timeout.tv_sec = next_timeout.tv_sec - systime.tv_sec;
        next_timeout.tv_nsec = next_timeout.tv_nsec - systime.tv_nsec;
        if next_timeout.tv_nsec < 0 as libc::c_long {
            next_timeout.tv_sec -= 1;
            next_timeout.tv_nsec += 1000000000 as libc::c_long
        }
        (*tv).tv_sec = next_timeout.tv_sec;
        (*tv).tv_usec = next_timeout.tv_nsec / 1000 as libc::c_int as libc::c_long;
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"libusb_get_next_timeout\x00",
            ))
            .as_ptr(),
            b"next timeout in %ld.%06lds\x00" as *const u8 as *const libc::c_char,
            (*tv).tv_sec,
            (*tv).tv_usec,
        );
    }
    return 1 as libc::c_int;
}
/* * \ingroup libusb_poll
 * Register notification functions for file descriptor additions/removals.
 * These functions will be invoked for every new or removed file descriptor
 * that libusb uses as an event source.
 *
 * To remove notifiers, pass NULL values for the function pointers.
 *
 * Note that file descriptors may have been added even before you register
 * these notifiers (e.g. at libusb_init() time).
 *
 * Additionally, note that the removal notifier may be called during
 * libusb_exit() (e.g. when it is closing file descriptors that were opened
 * and added to the poll set at libusb_init() time). If you don't want this,
 * remove the notifiers immediately before calling libusb_exit().
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \param added_cb pointer to function for addition notifications
 * \param removed_cb pointer to function for removal notifications
 * \param user_data User data to be passed back to callbacks (useful for
 * passing context information)
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_set_pollfd_notifiers(
    mut ctx: *mut libusb_context,
    added_cb: libusb_pollfd_added_cb,
    removed_cb: libusb_pollfd_removed_cb,
    user_data: *mut libc::c_void,
) {
    ctx = usbi_get_context(ctx);
    (*ctx).fd_added_cb = added_cb;
    (*ctx).fd_removed_cb = removed_cb;
    (*ctx).fd_cb_user_data = user_data;
}
/*
 * Interrupt the iteration of the event handling thread, so that it picks
 * up the fd change. Callers of this function must hold the event_data_lock.
 */
unsafe extern "C" fn usbi_fd_notification(mut ctx: *mut libusb_context) {
    let mut pending_events: libc::c_int = 0;
    /* Record that there is a new poll fd.
     * Only signal an event if there are no prior pending events. */
    pending_events = usbi_pending_events(ctx);
    (*ctx).event_flags |= USBI_EVENT_POLLFDS_MODIFIED as libc::c_int as libc::c_uint;
    if pending_events == 0 {
        usbi_signal_event(ctx);
    };
}
/* Add a file descriptor to the list of file descriptors to be monitored.
 * events should be specified as a bitmask of events passed to poll(), e.g.
 * POLLIN and/or POLLOUT. */
#[no_mangle]
pub unsafe extern "C" fn usbi_add_pollfd(
    mut ctx: *mut libusb_context,
    fd: libc::c_int,
    events: libc::c_short,
) -> libc::c_int {
    let mut ipollfd: *mut usbi_pollfd =
        malloc(::std::mem::size_of::<usbi_pollfd>() as libc::c_ulong) as *mut usbi_pollfd;
    if ipollfd.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"usbi_add_pollfd\x00")).as_ptr(),
        b"add fd %d events %d\x00" as *const u8 as *const libc::c_char,
        fd,
        events as libc::c_int,
    );
    (*ipollfd).pollfd.fd = fd;
    (*ipollfd).pollfd.events = events;
    usbi_mutex_lock(&mut (*ctx).event_data_lock);
    list_add_tail(&mut (*ipollfd).list, &mut (*ctx).ipollfds);
    (*ctx).pollfds_cnt = (*ctx).pollfds_cnt.wrapping_add(1);
    usbi_fd_notification(ctx);
    usbi_mutex_unlock(&mut (*ctx).event_data_lock);
    if (*ctx).fd_added_cb.is_some() {
        (*ctx).fd_added_cb.expect("non-null function pointer")(fd, events, (*ctx).fd_cb_user_data);
    }
    return 0 as libc::c_int;
}
/* Remove a file descriptor from the list of file descriptors to be polled. */
#[no_mangle]
pub unsafe extern "C" fn usbi_remove_pollfd(mut ctx: *mut libusb_context, fd: libc::c_int) {
    let mut ipollfd: *mut usbi_pollfd = 0 as *mut usbi_pollfd;
    let mut found: libc::c_int = 0 as libc::c_int;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"usbi_remove_pollfd\x00"))
            .as_ptr(),
        b"remove fd %d\x00" as *const u8 as *const libc::c_char,
        fd,
    );
    usbi_mutex_lock(&mut (*ctx).event_data_lock);
    ipollfd =
        ((*ctx).ipollfds.next as uintptr_t).wrapping_sub(8 as libc::c_ulong) as *mut usbi_pollfd;
    while &mut (*ipollfd).list as *mut list_head != &mut (*ctx).ipollfds as *mut list_head {
        if (*ipollfd).pollfd.fd == fd {
            found = 1 as libc::c_int;
            break;
        } else {
            ipollfd = ((*ipollfd).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
                as *mut usbi_pollfd
        }
    }
    if found == 0 {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"usbi_remove_pollfd\x00"))
                .as_ptr(),
            b"couldn\'t find fd %d to remove\x00" as *const u8 as *const libc::c_char,
            fd,
        );
        usbi_mutex_unlock(&mut (*ctx).event_data_lock);
        return;
    }
    list_del(&mut (*ipollfd).list);
    list_add_tail(&mut (*ipollfd).list, &mut (*ctx).removed_ipollfds);
    (*ctx).pollfds_cnt = (*ctx).pollfds_cnt.wrapping_sub(1);
    usbi_fd_notification(ctx);
    usbi_mutex_unlock(&mut (*ctx).event_data_lock);
    if (*ctx).fd_removed_cb.is_some() {
        (*ctx).fd_removed_cb.expect("non-null function pointer")(fd, (*ctx).fd_cb_user_data);
    };
}
/* * \ingroup libusb_poll
 * Retrieve a list of file descriptors that should be polled by your main loop
 * as libusb event sources.
 *
 * The returned list is NULL-terminated and should be freed with libusb_free_pollfds()
 * when done. The actual list contents must not be touched.
 *
 * As file descriptors are a Unix-specific concept, this function is not
 * available on Windows and will always return NULL.
 *
 * \param ctx the context to operate on, or NULL for the default context
 * \returns a NULL-terminated list of libusb_pollfd structures
 * \returns NULL on error
 * \returns NULL on platforms where the functionality is not available
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_get_pollfds(
    mut ctx: *mut libusb_context,
) -> *mut *const libusb_pollfd {
    let mut ret: *mut *mut libusb_pollfd = 0 as *mut *mut libusb_pollfd;
    let mut ipollfd: *mut usbi_pollfd = 0 as *mut usbi_pollfd;
    let mut i: size_t = 0 as libc::c_int as size_t;
    ctx = usbi_get_context(ctx);
    usbi_mutex_lock(&mut (*ctx).event_data_lock);
    ret = calloc(
        (*ctx)
            .pollfds_cnt
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<*mut libusb_pollfd>() as libc::c_ulong,
    ) as *mut *mut libusb_pollfd;
    if !ret.is_null() {
        ipollfd = ((*ctx).ipollfds.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
            as *mut usbi_pollfd;
        while &mut (*ipollfd).list as *mut list_head != &mut (*ctx).ipollfds as *mut list_head {
            let fresh0 = i;
            i = i.wrapping_add(1);
            let ref mut fresh1 = *ret.offset(fresh0 as isize);
            *fresh1 = ipollfd as *mut libusb_pollfd;
            ipollfd = ((*ipollfd).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
                as *mut usbi_pollfd
        }
        let ref mut fresh2 = *ret.offset((*ctx).pollfds_cnt as isize);
        *fresh2 = 0 as *mut libusb_pollfd
    }
    usbi_mutex_unlock(&mut (*ctx).event_data_lock);
    return ret as *mut *const libusb_pollfd;
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
/* * \ingroup libusb_poll
 * Free a list of libusb_pollfd structures. This should be called for all
 * pollfd lists allocated with libusb_get_pollfds().
 *
 * Since version 1.0.20, \ref LIBUSB_API_VERSION >= 0x01000104
 *
 * It is legal to call this function with a NULL pollfd list. In this case,
 * the function will simply do nothing.
 *
 * \param pollfds the list of libusb_pollfd structures to free
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_free_pollfds(pollfds: *mut *const libusb_pollfd) {
    free(pollfds as *mut libc::c_void);
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
/* internal event pipe, used for signalling occurrence of an internal event. */
/* A list of open handles. Backends are free to traverse this if required.
 */
/* A list of registered hotplug callbacks */
/* this is a list of in-flight transfer handles, sorted by timeout
 * expiration. URBs to timeout the soonest are placed at the beginning of
 * the list, URBs that will time out later are placed after, and urbs with
 * infinite timeout are always placed at the very end. */
/* Note paths taking both this and usbi_transfer->lock must always
 * take this lock first */
/* user callbacks for pollfd changes */
/* ensures that only one thread is handling events at any one time */
/* used to see if there is an active thread doing event handling */
/* A thread-local storage key to track which thread is performing event
 * handling */
/* used to wait for event completion in threads other than the one that is
 * event handling */
/* A lock to protect internal context event data. */
/* A bitmask of flags that are set to indicate specific events that need to
 * be handled. Protected by event_data_lock. */
/* A counter that is set when we want to interrupt and prevent event handling,
 * in order to safely close a device. Protected by event_data_lock. */
/* list and count of poll fds and an array of poll fd structures that is
 * (re)allocated as necessary prior to polling. Protected by event_data_lock. */
/* list of pollfds that have been removed. keeps track of pollfd changes
 * between the poll call and */
/* A list of pending hotplug messages. Protected by event_data_lock. */
/* A list of pending completed transfers. Protected by event_data_lock. */
/* used for timeout handling, if supported by OS.
 * this timerfd is maintained to trigger on the next pending timeout */
/* The list of pollfds has been modified */
/* The user has interrupted the event handler */
/* A hotplug callback deregistration is pending */
/* Macros for managing event handling state */
/* Update the following function if new event sources are added */
/* lock protects refcnt, everything else is finalized at initialization
 * time */
/* lock protects claimed_interfaces */
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
/* shared data and functions */
/* Backends may call this from handle_events to report disconnection of a
 * device. This function ensures transfers get cancelled appropriately.
 * Callers of this function must hold the events_lock.
 */
#[no_mangle]
pub unsafe extern "C" fn usbi_handle_disconnect(dev_handle: *mut libusb_device_handle) {
    let mut cur: *mut usbi_transfer = 0 as *mut usbi_transfer;
    let mut to_cancel: *mut usbi_transfer = 0 as *mut usbi_transfer;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"usbi_handle_disconnect\x00"))
            .as_ptr(),
        b"device %d.%d\x00" as *const u8 as *const libc::c_char,
        (*(*dev_handle).dev).bus_number as libc::c_int,
        (*(*dev_handle).dev).device_address as libc::c_int,
    );
    loop
    /* terminate all pending transfers with the LIBUSB_TRANSFER_NO_DEVICE
     * status code.
     *
     * when we find a transfer for this device on the list, there are two
     * possible scenarios:
     * 1. the transfer is currently in-flight, in which case we terminate the
     *    transfer here
     * 2. the transfer has been added to the flying transfer list by
     *    libusb_submit_transfer, has failed to submit and
     *    libusb_submit_transfer is waiting for us to release the
     *    flying_transfers_lock to remove it, so we ignore it
     */
    {
        to_cancel = 0 as *mut usbi_transfer;
        usbi_mutex_lock(&mut (*(*(*dev_handle).dev).ctx).flying_transfers_lock);
        cur = ((*(*(*dev_handle).dev).ctx).flying_transfers.next as uintptr_t)
            .wrapping_sub(8 as libc::c_ulong) as *mut usbi_transfer;
        while &mut (*cur).list as *mut list_head
            != &mut (*(*(*dev_handle).dev).ctx).flying_transfers as *mut list_head
        {
            if (*((cur as *mut libc::c_uchar).offset(
                ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
                    (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
            ) as *mut libusb_transfer))
                .dev_handle
                == dev_handle
            {
                usbi_mutex_lock(&mut (*cur).lock);
                if (*cur).state_flags & USBI_TRANSFER_IN_FLIGHT as libc::c_int as libc::c_uint != 0
                {
                    to_cancel = cur
                }
                usbi_mutex_unlock(&mut (*cur).lock);
                if !to_cancel.is_null() {
                    break;
                }
            }
            cur = ((*cur).list.next as uintptr_t).wrapping_sub(8 as libc::c_ulong)
                as *mut usbi_transfer
        }
        usbi_mutex_unlock(&mut (*(*(*dev_handle).dev).ctx).flying_transfers_lock);
        if to_cancel.is_null() {
            break;
        }
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"usbi_handle_disconnect\x00",
            ))
            .as_ptr(),
            b"cancelling transfer %p from disconnect\x00" as *const u8 as *const libc::c_char,
            (to_cancel as *mut libc::c_uchar).offset(
                ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
                    (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
            ) as *mut libusb_transfer,
        );
        usbi_mutex_lock(&mut (*to_cancel).lock);
        usbi_backend
            .clear_transfer_priv
            .expect("non-null function pointer")(to_cancel);
        usbi_mutex_unlock(&mut (*to_cancel).lock);
        usbi_handle_transfer_completion(to_cancel, LIBUSB_TRANSFER_NO_DEVICE);
    }
}
