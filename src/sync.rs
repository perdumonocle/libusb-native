#![allow(non_camel_case_types,unused_assignments,unused_unsafe,non_snake_case,non_upper_case_globals,unused_must_use)]

use ::libc;
extern "C" {
    #[no_mangle]
    fn libusb_error_name(errcode: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn libusb_alloc_transfer(iso_packets: libc::c_int) -> *mut libusb_transfer;
    #[no_mangle]
    fn libusb_submit_transfer(transfer: *mut libusb_transfer) -> libc::c_int;
    #[no_mangle]
    fn libusb_cancel_transfer(transfer: *mut libusb_transfer) -> libc::c_int;
    #[no_mangle]
    fn libusb_free_transfer(transfer: *mut libusb_transfer);
    #[no_mangle]
    fn usbi_log(
        ctx: *mut libusb_context,
        level: libusb_log_level,
        function: *const libc::c_char,
        format: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn libusb_handle_events_completed(
        ctx: *mut libusb_context,
        completed: *mut libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub b8: [uint8_t; 2],
    pub b16: uint16_t,
}
/* * \ingroup libusb_desc
 * Endpoint direction. Values for bit 7 of the
 * \ref libusb_endpoint_descriptor::bEndpointAddress "endpoint address" scheme.
 */
pub type libusb_endpoint_direction = libc::c_uint;
/* * In: device-to-host */
pub const LIBUSB_ENDPOINT_IN: libusb_endpoint_direction = 128;
/* * Out: host-to-device */
pub const LIBUSB_ENDPOINT_OUT: libusb_endpoint_direction = 0;
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
/*
 * Public libusb header file
 * Copyright © 2001 Johannes Erdfelt <johannes@erdfelt.com>
 * Copyright © 2007-2008 Daniel Drake <dsd@gentoo.org>
 * Copyright © 2012 Pete Batard <pete@akeo.ie>
 * Copyright © 2012-2018 Nathan Hjelm <hjelmn@cs.unm.edu>
 * Copyright © 2014-2020 Chris Dickens <christopher.a.dickens@gmail.com>
 * For more information, please visit: http://libusb.info
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
/* _MSC_VER */
/* [] - valid C99 code */
/* __STDC_VERSION__ */
/* 'interface' might be defined as a macro on Windows, so we need to
 * undefine it so as not to break the current libusb API, because
 * libusb_config_descriptor has an 'interface' member
 * As this can be problematic if you include windows.h after libusb.h
 * in your sources, we force windows.h to be included first. */
/* _WIN32 || __CYGWIN__ */
/* __GNUC__ */
/* __GNUC__ */
/* * \def LIBUSB_CALL
 * \ingroup libusb_misc
 * libusb's Windows calling convention.
 *
 * Under Windows, the selection of available compilers and configurations
 * means that, unlike other platforms, there is not <em>one true calling
 * convention</em> (calling convention: the manner in which parameters are
 * passed to functions in the generated assembly code).
 *
 * Matching the Windows API itself, libusb uses the WINAPI convention (which
 * translates to the <tt>stdcall</tt> convention) and guarantees that the
 * library is compiled in this way. The public header file also includes
 * appropriate annotations so that your own software will use the right
 * convention, even if another convention is being used by default within
 * your codebase.
 *
 * The one consideration that you must apply in your software is to mark
 * all functions which you use as libusb callbacks with this LIBUSB_CALL
 * annotation, so that they too get compiled for the correct calling
 * convention.
 *
 * On non-Windows operating systems, this macro is defined as nothing. This
 * means that you can apply it to your code without worrying about
 * cross-platform compatibility.
 */
/* LIBUSB_CALL must be defined on both definition and declaration of libusb
 * functions. You'd think that declaration would be enough, but cygwin will
 * complain about conflicting types unless both are marked this way.
 * The placement of this macro is important too; it must appear after the
 * return type, before the function name. See internal documentation for
 * API_EXPORTED.
 */
/* _WIN32 || __CYGWIN__ */
/* * \def LIBUSB_API_VERSION
 * \ingroup libusb_misc
 * libusb's API version.
 *
 * Since version 1.0.13, to help with feature detection, libusb defines
 * a LIBUSB_API_VERSION macro that gets increased every time there is a
 * significant change to the API, such as the introduction of a new call,
 * the definition of a new macro/enum member, or any other element that
 * libusb applications may want to detect at compilation time.
 *
 * The macro is typically used in an application as follows:
 * \code
 * #if defined(LIBUSB_API_VERSION) && (LIBUSB_API_VERSION >= 0x01001234)
 * // Use one of the newer features from the libusb API
 * #endif
 * \endcode
 *
 * Internally, LIBUSB_API_VERSION is defined as follows:
 * (libusb major << 24) | (libusb minor << 16) | (16 bit incremental)
 */
/* The following is kept for compatibility, but will be deprecated in the future */
/* *
 * \ingroup libusb_misc
 * Convert a 16-bit value from host-endian to little-endian format. On
 * little endian systems, this function does nothing. On big endian systems,
 * the bytes are swapped.
 * \param x the host-endian value to convert
 * \returns the value in little-endian byte order
 */
#[inline]
unsafe extern "C" fn libusb_cpu_to_le16(x: uint16_t) -> uint16_t {
    let mut _tmp: C2RustUnnamed_3 = C2RustUnnamed_3 { b8: [0; 2] };
    _tmp.b8[1 as libc::c_int as usize] = (x as libc::c_int >> 8 as libc::c_int) as uint8_t;
    _tmp.b8[0 as libc::c_int as usize] = (x as libc::c_int & 0xff as libc::c_int) as uint8_t;
    return _tmp.b16;
}
#[inline]
unsafe extern "C" fn libusb_control_transfer_get_data(
    transfer: *mut libusb_transfer,
) -> *mut libc::c_uchar {
    return (*transfer)
        .buffer
        .offset(::std::mem::size_of::<libusb_control_setup>() as libc::c_ulong as isize);
}
#[inline]
unsafe extern "C" fn libusb_fill_control_setup(
    buffer: *mut libc::c_uchar,
    bmRequestType: uint8_t,
    bRequest: uint8_t,
    wValue: uint16_t,
    wIndex: uint16_t,
    wLength: uint16_t,
) {
    let mut setup: *mut libusb_control_setup =
        buffer as *mut libc::c_void as *mut libusb_control_setup;
    (*setup).bmRequestType = bmRequestType;
    (*setup).bRequest = bRequest;
    (*setup).wValue = libusb_cpu_to_le16(wValue);
    (*setup).wIndex = libusb_cpu_to_le16(wIndex);
    (*setup).wLength = libusb_cpu_to_le16(wLength);
}
#[inline]
unsafe extern "C" fn libusb_fill_control_transfer(
    mut transfer: *mut libusb_transfer,
    dev_handle: *mut libusb_device_handle,
    buffer: *mut libc::c_uchar,
    callback: libusb_transfer_cb_fn,
    user_data: *mut libc::c_void,
    timeout: libc::c_uint,
) {
    let setup: *mut libusb_control_setup = buffer as *mut libc::c_void as *mut libusb_control_setup;
    (*transfer).dev_handle = dev_handle;
    (*transfer).endpoint = 0 as libc::c_int as libc::c_uchar;
    (*transfer).type_0 = LIBUSB_TRANSFER_TYPE_CONTROL as libc::c_int as libc::c_uchar;
    (*transfer).timeout = timeout;
    (*transfer).buffer = buffer;
    if !setup.is_null() {
        (*transfer).length = (::std::mem::size_of::<libusb_control_setup>() as libc::c_ulong)
            .wrapping_add(libusb_cpu_to_le16((*setup).wLength) as libc::c_ulong)
            as libc::c_int
    }
    (*transfer).user_data = user_data;
    (*transfer).callback = callback;
}
#[inline]
unsafe extern "C" fn libusb_fill_bulk_transfer(
    mut transfer: *mut libusb_transfer,
    dev_handle: *mut libusb_device_handle,
    endpoint: libc::c_uchar,
    buffer: *mut libc::c_uchar,
    length: libc::c_int,
    callback: libusb_transfer_cb_fn,
    user_data: *mut libc::c_void,
    timeout: libc::c_uint,
) {
    (*transfer).dev_handle = dev_handle;
    (*transfer).endpoint = endpoint;
    (*transfer).type_0 = LIBUSB_TRANSFER_TYPE_BULK as libc::c_int as libc::c_uchar;
    (*transfer).timeout = timeout;
    (*transfer).buffer = buffer;
    (*transfer).length = length;
    (*transfer).user_data = user_data;
    (*transfer).callback = callback;
}
#[inline]
unsafe extern "C" fn usbi_handling_events(ctx: *mut libusb_context) -> libc::c_int {
    return (usbi_tls_key_get((*ctx).event_handling_key) != 0 as *mut libc::c_void) as libc::c_int;
}
#[inline]
unsafe extern "C" fn usbi_tls_key_get(key: usbi_tls_key_t) -> *mut libc::c_void {
    return pthread_getspecific(key);
}
/* -*- Mode: C; indent-tabs-mode:t ; c-basic-offset:8 -*- */
/*
 * Synchronous I/O functions for libusb
 * Copyright © 2007-2008 Daniel Drake <dsd@gentoo.org>
 * Copyright © 2019 Nathan Hjelm <hjelmn@cs.unm.edu>
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
 * @defgroup libusb_syncio Synchronous device I/O
 *
 * This page documents libusb's synchronous (blocking) API for USB device I/O.
 * This interface is easy to use but has some limitations. More advanced users
 * may wish to consider using the \ref libusb_asyncio "asynchronous I/O API" instead.
 */
unsafe extern "C" fn sync_transfer_cb(transfer: *mut libusb_transfer) {
    let completed: *mut libc::c_int = (*transfer).user_data as *mut libc::c_int;
    *completed = 1 as libc::c_int;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"sync_transfer_cb\x00"))
            .as_ptr(),
        b"actual_length=%d\x00" as *const u8 as *const libc::c_char,
        (*transfer).actual_length,
    );
    /* caller interprets result and frees transfer */
}
unsafe extern "C" fn sync_transfer_wait_for_completion(mut transfer: *mut libusb_transfer) {
    let mut r: libc::c_int = 0;
    let completed: *mut libc::c_int = (*transfer).user_data as *mut libc::c_int;
    let ctx: *mut libusb_context = (*(*(*transfer).dev_handle).dev).ctx;
    while *completed == 0 {
        r = libusb_handle_events_completed(ctx, completed);
        if r < 0 as libc::c_int {
            if r == LIBUSB_ERROR_INTERRUPTED as libc::c_int {
                continue;
            }
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 34], &[libc::c_char; 34]>(
                    b"sync_transfer_wait_for_completion\x00",
                ))
                .as_ptr(),
                b"libusb_handle_events failed: %s, cancelling transfer and retrying\x00"
                    as *const u8 as *const libc::c_char,
                libusb_error_name(r),
            );
            libusb_cancel_transfer(transfer);
        } else if (*transfer).dev_handle.is_null() {
            /* transfer completion after libusb_close() */
            (*transfer).status = LIBUSB_TRANSFER_NO_DEVICE;
            *completed = 1 as libc::c_int
        }
    }
}
/* * \ingroup libusb_syncio
 * Perform a USB control transfer.
 *
 * The direction of the transfer is inferred from the bmRequestType field of
 * the setup packet.
 *
 * The wValue, wIndex and wLength fields values should be given in host-endian
 * byte order.
 *
 * \param dev_handle a handle for the device to communicate with
 * \param bmRequestType the request type field for the setup packet
 * \param bRequest the request field for the setup packet
 * \param wValue the value field for the setup packet
 * \param wIndex the index field for the setup packet
 * \param data a suitably-sized data buffer for either input or output
 * (depending on direction bits within bmRequestType)
 * \param wLength the length field for the setup packet. The data buffer should
 * be at least this size.
 * \param timeout timeout (in millseconds) that this function should wait
 * before giving up due to no response being received. For an unlimited
 * timeout, use value 0.
 * \returns on success, the number of bytes actually transferred
 * \returns LIBUSB_ERROR_TIMEOUT if the transfer timed out
 * \returns LIBUSB_ERROR_PIPE if the control request was not supported by the
 * device
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns LIBUSB_ERROR_BUSY if called from event handling context
 * \returns LIBUSB_ERROR_INVALID_PARAM if the transfer size is larger than
 * the operating system and/or hardware can support
 * \returns another LIBUSB_ERROR code on other failures
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_control_transfer(
    dev_handle: *mut libusb_device_handle,
    bmRequestType: uint8_t,
    bRequest: uint8_t,
    wValue: uint16_t,
    wIndex: uint16_t,
    data: *mut libc::c_uchar,
    wLength: uint16_t,
    timeout: libc::c_uint,
) -> libc::c_int {
    let mut transfer: *mut libusb_transfer = 0 as *mut libusb_transfer;
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut completed: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0;
    if usbi_handling_events((*(*dev_handle).dev).ctx) != 0 {
        return LIBUSB_ERROR_BUSY as libc::c_int;
    }
    transfer = libusb_alloc_transfer(0 as libc::c_int);
    if transfer.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    buffer = malloc(
        (::std::mem::size_of::<libusb_control_setup>() as libc::c_ulong)
            .wrapping_add(wLength as libc::c_ulong),
    ) as *mut libc::c_uchar;
    if buffer.is_null() {
        libusb_free_transfer(transfer);
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    libusb_fill_control_setup(buffer, bmRequestType, bRequest, wValue, wIndex, wLength);
    if bmRequestType as libc::c_int & 0x80 as libc::c_int == LIBUSB_ENDPOINT_OUT as libc::c_int {
        memcpy(
            buffer.offset(::std::mem::size_of::<libusb_control_setup>() as libc::c_ulong as isize)
                as *mut libc::c_void,
            data as *const libc::c_void,
            wLength as libc::c_ulong,
        );
    }
    libusb_fill_control_transfer(
        transfer,
        dev_handle,
        buffer,
        Some(sync_transfer_cb as unsafe extern "C" fn(_: *mut libusb_transfer) -> ()),
        &mut completed as *mut libc::c_int as *mut libc::c_void,
        timeout,
    );
    (*transfer).flags = LIBUSB_TRANSFER_FREE_BUFFER as libc::c_int as uint8_t;
    r = libusb_submit_transfer(transfer);
    if r < 0 as libc::c_int {
        libusb_free_transfer(transfer);
        return r;
    }
    sync_transfer_wait_for_completion(transfer);
    if bmRequestType as libc::c_int & 0x80 as libc::c_int == LIBUSB_ENDPOINT_IN as libc::c_int {
        memcpy(
            data as *mut libc::c_void,
            libusb_control_transfer_get_data(transfer) as *const libc::c_void,
            (*transfer).actual_length as libc::c_ulong,
        );
    }
    match (*transfer).status as libc::c_uint {
        0 => r = (*transfer).actual_length,
        2 => r = LIBUSB_ERROR_TIMEOUT as libc::c_int,
        4 => r = LIBUSB_ERROR_PIPE as libc::c_int,
        5 => r = LIBUSB_ERROR_NO_DEVICE as libc::c_int,
        6 => r = LIBUSB_ERROR_OVERFLOW as libc::c_int,
        1 | 3 => r = LIBUSB_ERROR_IO as libc::c_int,
        _ => {
            usbi_log(
                (*(*dev_handle).dev).ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"libusb_control_transfer\x00",
                ))
                .as_ptr(),
                b"unrecognised status code %d\x00" as *const u8 as *const libc::c_char,
                (*transfer).status as libc::c_uint,
            );
            r = LIBUSB_ERROR_OTHER as libc::c_int
        }
    }
    libusb_free_transfer(transfer);
    return r;
}
unsafe extern "C" fn do_sync_bulk_transfer(
    dev_handle: *mut libusb_device_handle,
    endpoint: libc::c_uchar,
    buffer: *mut libc::c_uchar,
    length: libc::c_int,
    transferred: *mut libc::c_int,
    timeout: libc::c_uint,
    type_0: libc::c_uchar,
) -> libc::c_int {
    let mut transfer: *mut libusb_transfer = 0 as *mut libusb_transfer;
    let mut completed: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0;
    if usbi_handling_events((*(*dev_handle).dev).ctx) != 0 {
        return LIBUSB_ERROR_BUSY as libc::c_int;
    }
    transfer = libusb_alloc_transfer(0 as libc::c_int);
    if transfer.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    libusb_fill_bulk_transfer(
        transfer,
        dev_handle,
        endpoint,
        buffer,
        length,
        Some(sync_transfer_cb as unsafe extern "C" fn(_: *mut libusb_transfer) -> ()),
        &mut completed as *mut libc::c_int as *mut libc::c_void,
        timeout,
    );
    (*transfer).type_0 = type_0;
    r = libusb_submit_transfer(transfer);
    if r < 0 as libc::c_int {
        libusb_free_transfer(transfer);
        return r;
    }
    sync_transfer_wait_for_completion(transfer);
    if !transferred.is_null() {
        *transferred = (*transfer).actual_length
    }
    match (*transfer).status as libc::c_uint {
        0 => r = 0 as libc::c_int,
        2 => r = LIBUSB_ERROR_TIMEOUT as libc::c_int,
        4 => r = LIBUSB_ERROR_PIPE as libc::c_int,
        6 => r = LIBUSB_ERROR_OVERFLOW as libc::c_int,
        5 => r = LIBUSB_ERROR_NO_DEVICE as libc::c_int,
        1 | 3 => r = LIBUSB_ERROR_IO as libc::c_int,
        _ => {
            usbi_log(
                (*(*dev_handle).dev).ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"do_sync_bulk_transfer\x00",
                ))
                .as_ptr(),
                b"unrecognised status code %d\x00" as *const u8 as *const libc::c_char,
                (*transfer).status as libc::c_uint,
            );
            r = LIBUSB_ERROR_OTHER as libc::c_int
        }
    }
    libusb_free_transfer(transfer);
    return r;
}
/* * \ingroup libusb_syncio
 * Perform a USB bulk transfer. The direction of the transfer is inferred from
 * the direction bits of the endpoint address.
 *
 * For bulk reads, the <tt>length</tt> field indicates the maximum length of
 * data you are expecting to receive. If less data arrives than expected,
 * this function will return that data, so be sure to check the
 * <tt>transferred</tt> output parameter.
 *
 * You should also check the <tt>transferred</tt> parameter for bulk writes.
 * Not all of the data may have been written.
 *
 * Also check <tt>transferred</tt> when dealing with a timeout error code.
 * libusb may have to split your transfer into a number of chunks to satisfy
 * underlying O/S requirements, meaning that the timeout may expire after
 * the first few chunks have completed. libusb is careful not to lose any data
 * that may have been transferred; do not assume that timeout conditions
 * indicate a complete lack of I/O.
 *
 * \param dev_handle a handle for the device to communicate with
 * \param endpoint the address of a valid endpoint to communicate with
 * \param data a suitably-sized data buffer for either input or output
 * (depending on endpoint)
 * \param length for bulk writes, the number of bytes from data to be sent. for
 * bulk reads, the maximum number of bytes to receive into the data buffer.
 * \param transferred output location for the number of bytes actually
 * transferred. Since version 1.0.21 (\ref LIBUSB_API_VERSION >= 0x01000105),
 * it is legal to pass a NULL pointer if you do not wish to receive this
 * information.
 * \param timeout timeout (in millseconds) that this function should wait
 * before giving up due to no response being received. For an unlimited
 * timeout, use value 0.
 *
 * \returns 0 on success (and populates <tt>transferred</tt>)
 * \returns LIBUSB_ERROR_TIMEOUT if the transfer timed out (and populates
 * <tt>transferred</tt>)
 * \returns LIBUSB_ERROR_PIPE if the endpoint halted
 * \returns LIBUSB_ERROR_OVERFLOW if the device offered more data, see
 * \ref libusb_packetoverflow
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns LIBUSB_ERROR_BUSY if called from event handling context
 * \returns another LIBUSB_ERROR code on other failures
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_bulk_transfer(
    dev_handle: *mut libusb_device_handle,
    endpoint: libc::c_uchar,
    data: *mut libc::c_uchar,
    length: libc::c_int,
    transferred: *mut libc::c_int,
    timeout: libc::c_uint,
) -> libc::c_int {
    return do_sync_bulk_transfer(
        dev_handle,
        endpoint,
        data,
        length,
        transferred,
        timeout,
        LIBUSB_TRANSFER_TYPE_BULK as libc::c_int as libc::c_uchar,
    );
}
/* * \ingroup libusb_syncio
 * Perform a USB interrupt transfer. The direction of the transfer is inferred
 * from the direction bits of the endpoint address.
 *
 * For interrupt reads, the <tt>length</tt> field indicates the maximum length
 * of data you are expecting to receive. If less data arrives than expected,
 * this function will return that data, so be sure to check the
 * <tt>transferred</tt> output parameter.
 *
 * You should also check the <tt>transferred</tt> parameter for interrupt
 * writes. Not all of the data may have been written.
 *
 * Also check <tt>transferred</tt> when dealing with a timeout error code.
 * libusb may have to split your transfer into a number of chunks to satisfy
 * underlying O/S requirements, meaning that the timeout may expire after
 * the first few chunks have completed. libusb is careful not to lose any data
 * that may have been transferred; do not assume that timeout conditions
 * indicate a complete lack of I/O.
 *
 * The default endpoint bInterval value is used as the polling interval.
 *
 * \param dev_handle a handle for the device to communicate with
 * \param endpoint the address of a valid endpoint to communicate with
 * \param data a suitably-sized data buffer for either input or output
 * (depending on endpoint)
 * \param length for bulk writes, the number of bytes from data to be sent. for
 * bulk reads, the maximum number of bytes to receive into the data buffer.
 * \param transferred output location for the number of bytes actually
 * transferred. Since version 1.0.21 (\ref LIBUSB_API_VERSION >= 0x01000105),
 * it is legal to pass a NULL pointer if you do not wish to receive this
 * information.
 * \param timeout timeout (in millseconds) that this function should wait
 * before giving up due to no response being received. For an unlimited
 * timeout, use value 0.
 *
 * \returns 0 on success (and populates <tt>transferred</tt>)
 * \returns LIBUSB_ERROR_TIMEOUT if the transfer timed out
 * \returns LIBUSB_ERROR_PIPE if the endpoint halted
 * \returns LIBUSB_ERROR_OVERFLOW if the device offered more data, see
 * \ref libusb_packetoverflow
 * \returns LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
 * \returns LIBUSB_ERROR_BUSY if called from event handling context
 * \returns another LIBUSB_ERROR code on other error
 */
#[no_mangle]
pub unsafe extern "C" fn libusb_interrupt_transfer(
    dev_handle: *mut libusb_device_handle,
    endpoint: libc::c_uchar,
    data: *mut libc::c_uchar,
    length: libc::c_int,
    transferred: *mut libc::c_int,
    timeout: libc::c_uint,
) -> libc::c_int {
    return do_sync_bulk_transfer(
        dev_handle,
        endpoint,
        data,
        length,
        transferred,
        timeout,
        LIBUSB_TRANSFER_TYPE_INTERRUPT as libc::c_int as libc::c_uchar,
    );
}
