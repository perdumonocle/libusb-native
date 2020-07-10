#![allow(non_camel_case_types,unused_assignments,unused_unsafe,non_snake_case,non_upper_case_globals,unused_must_use)]

use ::libc;
extern "C" {
    #[no_mangle]
    fn libusb_has_capability(capability: uint32_t) -> libc::c_int;
    #[no_mangle]
    fn libusb_get_device_list(
        ctx: *mut libusb_context,
        list: *mut *mut *mut libusb_device,
    ) -> ssize_t;
    #[no_mangle]
    fn libusb_free_device_list(list: *mut *mut libusb_device, unref_devices: libc::c_int);
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn usbi_signal_event(ctx: *mut libusb_context) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    static mut usbi_default_context: *mut libusb_context;
    #[no_mangle]
    fn usbi_log(
        ctx: *mut libusb_context,
        level: libusb_log_level,
        function: *const libc::c_char,
        format: *const libc::c_char,
        _: ...
    );
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
pub type uintptr_t = libc::c_ulong;
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
pub type libusb_hotplug_event = libc::c_uint;
pub const LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT: libusb_hotplug_event = 2;
pub const LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED: libusb_hotplug_event = 1;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const LIBUSB_HOTPLUG_ENUMERATE: C2RustUnnamed_3 = 1;
pub type libusb_hotplug_callback_fn = Option<
    unsafe extern "C" fn(
        _: *mut libusb_context,
        _: *mut libusb_device,
        _: libusb_hotplug_event,
        _: *mut libc::c_void,
    ) -> libc::c_int,
>;
/* * \ingroup hotplug
 * The hotplug callback structure. The user populates this structure with
 * libusb_hotplug_prepare_callback() and then calls libusb_hotplug_register_callback()
 * to receive notification of hotplug events.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_hotplug_callback {
    pub flags: uint8_t,
    pub vendor_id: uint16_t,
    pub product_id: uint16_t,
    pub dev_class: uint8_t,
    pub cb: libusb_hotplug_callback_fn,
    pub handle: libusb_hotplug_callback_handle,
    pub user_data: *mut libc::c_void,
    pub list: list_head,
}
/* The dev_class field is valid for matching */
pub const USBI_HOTPLUG_DEV_CLASS_VALID: usbi_hotplug_flags = 32;
/* The product_id field is valid for matching */
pub const USBI_HOTPLUG_PRODUCT_ID_VALID: usbi_hotplug_flags = 16;
/* IMPORTANT: The values for the below entries must start *after*
 * the highest value of the above entries!!!
 */
/* The vendor_id field is valid for matching */
pub const USBI_HOTPLUG_VENDOR_ID_VALID: usbi_hotplug_flags = 8;
pub const USBI_EVENT_HOTPLUG_CB_DEREGISTERED: usbi_event_flags = 4;
/* This callback has been unregistered and needs to be freed */
pub const USBI_HOTPLUG_NEEDS_FREE: usbi_hotplug_flags = 64;
pub type usbi_event_flags = libc::c_uint;
pub const USBI_EVENT_USER_INTERRUPT: usbi_event_flags = 2;
pub const USBI_EVENT_POLLFDS_MODIFIED: usbi_event_flags = 1;
/* -*- Mode: C; indent-tabs-mode:t ; c-basic-offset:8 -*- */
/*
 * Hotplug support for libusb
 * Copyright © 2012-2013 Nathan Hjelm <hjelmn@mac.com>
 * Copyright © 2012-2013 Peter Stuge <peter@stuge.se>
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
pub type usbi_hotplug_flags = libc::c_uint;
/* This callback is interested in device removals */
pub const USBI_HOTPLUG_DEVICE_LEFT: usbi_hotplug_flags = 2;
/* This callback is interested in device arrivals */
pub const USBI_HOTPLUG_DEVICE_ARRIVED: usbi_hotplug_flags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_hotplug_message {
    pub event: libusb_hotplug_event,
    pub device: *mut libusb_device,
    pub list: list_head,
}
#[inline]
unsafe extern "C" fn usbi_mutex_unlock(mutex: *mut usbi_mutex_t) {
    pthread_mutex_unlock(mutex);
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
unsafe extern "C" fn usbi_mutex_lock(mutex: *mut usbi_mutex_t) {
    pthread_mutex_lock(mutex);
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
unsafe extern "C" fn list_del(mut entry: *mut list_head) {
    (*(*entry).next).prev = (*entry).prev;
    (*(*entry).prev).next = (*entry).next;
    (*entry).prev = 0 as *mut list_head;
    (*entry).next = (*entry).prev;
}
unsafe extern "C" fn usbi_hotplug_match_cb(
    ctx: *mut libusb_context,
    dev: *mut libusb_device,
    event: libusb_hotplug_event,
    hotplug_cb: *mut libusb_hotplug_callback,
) -> libc::c_int {
    if (*hotplug_cb).flags as libc::c_uint & event as libc::c_uint == 0 {
        return 0 as libc::c_int;
    }
    if (*hotplug_cb).flags as libc::c_int & USBI_HOTPLUG_VENDOR_ID_VALID as libc::c_int != 0
        && (*hotplug_cb).vendor_id as libc::c_int
            != (*dev).device_descriptor.idVendor as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*hotplug_cb).flags as libc::c_int & USBI_HOTPLUG_PRODUCT_ID_VALID as libc::c_int != 0
        && (*hotplug_cb).product_id as libc::c_int
            != (*dev).device_descriptor.idProduct as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*hotplug_cb).flags as libc::c_int & USBI_HOTPLUG_DEV_CLASS_VALID as libc::c_int != 0
        && (*hotplug_cb).dev_class as libc::c_int
            != (*dev).device_descriptor.bDeviceClass as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return (*hotplug_cb).cb.expect("non-null function pointer")(
        ctx,
        dev,
        event,
        (*hotplug_cb).user_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn usbi_hotplug_match(
    ctx: *mut libusb_context,
    dev: *mut libusb_device,
    event: libusb_hotplug_event,
) {
    let mut hotplug_cb: *mut libusb_hotplug_callback = 0 as *mut libusb_hotplug_callback;
    let mut next: *mut libusb_hotplug_callback = 0 as *mut libusb_hotplug_callback;
    let mut ret: libc::c_int = 0;
    usbi_mutex_lock(&mut (*ctx).hotplug_cbs_lock);
    hotplug_cb = ((*ctx).hotplug_cbs.next as uintptr_t).wrapping_sub(32 as libc::c_ulong)
        as *mut libusb_hotplug_callback;
    next = ((*hotplug_cb).list.next as uintptr_t).wrapping_sub(32 as libc::c_ulong)
        as *mut libusb_hotplug_callback;
    while &mut (*hotplug_cb).list as *mut list_head != &mut (*ctx).hotplug_cbs as *mut list_head {
        if !((*hotplug_cb).flags as libc::c_int & USBI_HOTPLUG_NEEDS_FREE as libc::c_int != 0) {
            usbi_mutex_unlock(&mut (*ctx).hotplug_cbs_lock);
            ret = usbi_hotplug_match_cb(ctx, dev, event, hotplug_cb);
            usbi_mutex_lock(&mut (*ctx).hotplug_cbs_lock);
            if ret != 0 {
                list_del(&mut (*hotplug_cb).list);
                free(hotplug_cb as *mut libc::c_void);
            }
        }
        /* process deregistration in usbi_hotplug_deregister() */
        hotplug_cb = next;
        next = ((*next).list.next as uintptr_t).wrapping_sub(32 as libc::c_ulong)
            as *mut libusb_hotplug_callback
    }
    usbi_mutex_unlock(&mut (*ctx).hotplug_cbs_lock);
}
#[no_mangle]
pub unsafe extern "C" fn usbi_hotplug_notification(
    ctx: *mut libusb_context,
    dev: *mut libusb_device,
    event: libusb_hotplug_event,
) {
    let mut pending_events: libc::c_int = 0;
    let mut message: *mut libusb_hotplug_message = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<libusb_hotplug_message>() as libc::c_ulong,
    ) as *mut libusb_hotplug_message;
    if message.is_null() {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                b"usbi_hotplug_notification\x00",
            ))
            .as_ptr(),
            b"error allocating hotplug message\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    (*message).event = event;
    (*message).device = dev;
    /* Take the event data lock and add this message to the list.
     * Only signal an event if there are no prior pending events. */
    usbi_mutex_lock(&mut (*ctx).event_data_lock);
    pending_events = usbi_pending_events(ctx);
    list_add_tail(&mut (*message).list, &mut (*ctx).hotplug_msgs);
    if pending_events == 0 {
        usbi_signal_event(ctx);
    }
    usbi_mutex_unlock(&mut (*ctx).event_data_lock);
}
#[no_mangle]
pub unsafe extern "C" fn libusb_hotplug_register_callback(
    mut ctx: *mut libusb_context,
    events: libc::c_int,
    flags: libc::c_int,
    vendor_id: libc::c_int,
    product_id: libc::c_int,
    dev_class: libc::c_int,
    cb_fn: libusb_hotplug_callback_fn,
    user_data: *mut libc::c_void,
    callback_handle: *mut libusb_hotplug_callback_handle,
) -> libc::c_int {
    let mut new_callback: *mut libusb_hotplug_callback = 0 as *mut libusb_hotplug_callback;
    /* check for sane values */
    if events == 0
        || !(LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED as libc::c_int
            | LIBUSB_HOTPLUG_EVENT_DEVICE_LEFT as libc::c_int)
            & events
            != 0
        || !(LIBUSB_HOTPLUG_ENUMERATE as libc::c_int) & flags != 0
        || -(1 as libc::c_int) != vendor_id && !(0xffff as libc::c_int) & vendor_id != 0
        || -(1 as libc::c_int) != product_id && !(0xffff as libc::c_int) & product_id != 0
        || -(1 as libc::c_int) != dev_class && !(0xff as libc::c_int) & dev_class != 0
        || cb_fn.is_none()
    {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    /* check for hotplug support */
    if libusb_has_capability(LIBUSB_CAP_HAS_HOTPLUG as libc::c_int as uint32_t) == 0 {
        return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    }
    ctx = usbi_get_context(ctx);
    new_callback = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<libusb_hotplug_callback>() as libc::c_ulong,
    ) as *mut libusb_hotplug_callback;
    if new_callback.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    (*new_callback).flags = events as uint8_t;
    if -(1 as libc::c_int) != vendor_id {
        (*new_callback).flags = ((*new_callback).flags as libc::c_int
            | USBI_HOTPLUG_VENDOR_ID_VALID as libc::c_int)
            as uint8_t;
        (*new_callback).vendor_id = vendor_id as uint16_t
    }
    if -(1 as libc::c_int) != product_id {
        (*new_callback).flags = ((*new_callback).flags as libc::c_int
            | USBI_HOTPLUG_PRODUCT_ID_VALID as libc::c_int)
            as uint8_t;
        (*new_callback).product_id = product_id as uint16_t
    }
    if -(1 as libc::c_int) != dev_class {
        (*new_callback).flags = ((*new_callback).flags as libc::c_int
            | USBI_HOTPLUG_DEV_CLASS_VALID as libc::c_int)
            as uint8_t;
        (*new_callback).dev_class = dev_class as uint8_t
    }
    (*new_callback).cb = cb_fn;
    (*new_callback).user_data = user_data;
    usbi_mutex_lock(&mut (*ctx).hotplug_cbs_lock);
    /* protect the handle by the context hotplug lock */
    let fresh0 = (*ctx).next_hotplug_cb_handle;
    (*ctx).next_hotplug_cb_handle = (*ctx).next_hotplug_cb_handle + 1;
    (*new_callback).handle = fresh0;
    /* handle the unlikely case of overflow */
    if (*ctx).next_hotplug_cb_handle < 0 as libc::c_int {
        (*ctx).next_hotplug_cb_handle = 1 as libc::c_int
    }
    list_add(&mut (*new_callback).list, &mut (*ctx).hotplug_cbs);
    usbi_mutex_unlock(&mut (*ctx).hotplug_cbs_lock);
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
            b"libusb_hotplug_register_callback\x00",
        ))
        .as_ptr(),
        b"new hotplug cb %p with handle %d\x00" as *const u8 as *const libc::c_char,
        new_callback,
        (*new_callback).handle,
    );
    if flags & LIBUSB_HOTPLUG_ENUMERATE as libc::c_int != 0
        && events & LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED as libc::c_int != 0
    {
        let mut i: ssize_t = 0;
        let mut len: ssize_t = 0;
        let mut devs: *mut *mut libusb_device = 0 as *mut *mut libusb_device;
        len = libusb_get_device_list(ctx, &mut devs);
        if len < 0 as libc::c_int as libc::c_long {
            libusb_hotplug_deregister_callback(ctx, (*new_callback).handle);
            return len as libc::c_int;
        }
        i = 0 as libc::c_int as ssize_t;
        while i < len {
            usbi_hotplug_match_cb(
                ctx,
                *devs.offset(i as isize),
                LIBUSB_HOTPLUG_EVENT_DEVICE_ARRIVED,
                new_callback,
            );
            i += 1
        }
        libusb_free_device_list(devs, 1 as libc::c_int);
    }
    if !callback_handle.is_null() {
        *callback_handle = (*new_callback).handle
    }
    return LIBUSB_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn libusb_hotplug_deregister_callback(
    mut ctx: *mut libusb_context,
    callback_handle: libusb_hotplug_callback_handle,
) {
    let mut hotplug_cb: *mut libusb_hotplug_callback = 0 as *mut libusb_hotplug_callback;
    let mut deregistered: libc::c_int = 0 as libc::c_int;
    /* check for hotplug support */
    if libusb_has_capability(LIBUSB_CAP_HAS_HOTPLUG as libc::c_int as uint32_t) == 0 {
        return;
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
            b"libusb_hotplug_deregister_callback\x00",
        ))
        .as_ptr(),
        b"deregister hotplug cb %d\x00" as *const u8 as *const libc::c_char,
        callback_handle,
    );
    ctx = usbi_get_context(ctx);
    usbi_mutex_lock(&mut (*ctx).hotplug_cbs_lock);
    hotplug_cb = ((*ctx).hotplug_cbs.next as uintptr_t).wrapping_sub(32 as libc::c_ulong)
        as *mut libusb_hotplug_callback;
    while &mut (*hotplug_cb).list as *mut list_head != &mut (*ctx).hotplug_cbs as *mut list_head {
        if callback_handle == (*hotplug_cb).handle {
            /* Mark this callback for deregistration */
            (*hotplug_cb).flags = ((*hotplug_cb).flags as libc::c_int
                | USBI_HOTPLUG_NEEDS_FREE as libc::c_int)
                as uint8_t;
            deregistered = 1 as libc::c_int
        }
        hotplug_cb = ((*hotplug_cb).list.next as uintptr_t).wrapping_sub(32 as libc::c_ulong)
            as *mut libusb_hotplug_callback
    }
    usbi_mutex_unlock(&mut (*ctx).hotplug_cbs_lock);
    if deregistered != 0 {
        let mut pending_events: libc::c_int = 0;
        usbi_mutex_lock(&mut (*ctx).event_data_lock);
        pending_events = usbi_pending_events(ctx);
        (*ctx).event_flags |= USBI_EVENT_HOTPLUG_CB_DEREGISTERED as libc::c_int as libc::c_uint;
        if pending_events == 0 {
            usbi_signal_event(ctx);
        }
        usbi_mutex_unlock(&mut (*ctx).event_data_lock);
    };
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
#[no_mangle]
pub unsafe extern "C" fn libusb_hotplug_get_user_data(
    mut ctx: *mut libusb_context,
    callback_handle: libusb_hotplug_callback_handle,
) -> *mut libc::c_void {
    let mut hotplug_cb: *mut libusb_hotplug_callback = 0 as *mut libusb_hotplug_callback;
    let mut user_data: *mut libc::c_void = 0 as *mut libc::c_void;
    /* check for hotplug support */
    if libusb_has_capability(LIBUSB_CAP_HAS_HOTPLUG as libc::c_int as uint32_t) == 0 {
        return 0 as *mut libc::c_void;
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
            b"libusb_hotplug_get_user_data\x00",
        ))
        .as_ptr(),
        b"get hotplug user data %d\x00" as *const u8 as *const libc::c_char,
        callback_handle,
    );
    ctx = usbi_get_context(ctx);
    usbi_mutex_lock(&mut (*ctx).hotplug_cbs_lock);
    hotplug_cb = ((*ctx).hotplug_cbs.next as uintptr_t).wrapping_sub(32 as libc::c_ulong)
        as *mut libusb_hotplug_callback;
    while &mut (*hotplug_cb).list as *mut list_head != &mut (*ctx).hotplug_cbs as *mut list_head {
        if callback_handle == (*hotplug_cb).handle {
            user_data = (*hotplug_cb).user_data
        }
        hotplug_cb = ((*hotplug_cb).list.next as uintptr_t).wrapping_sub(32 as libc::c_ulong)
            as *mut libusb_hotplug_callback
    }
    usbi_mutex_unlock(&mut (*ctx).hotplug_cbs_lock);
    return user_data;
}
#[no_mangle]
pub unsafe extern "C" fn usbi_hotplug_deregister(ctx: *mut libusb_context, forced: libc::c_int) {
    let mut hotplug_cb: *mut libusb_hotplug_callback = 0 as *mut libusb_hotplug_callback;
    let mut next: *mut libusb_hotplug_callback = 0 as *mut libusb_hotplug_callback;
    usbi_mutex_lock(&mut (*ctx).hotplug_cbs_lock);
    hotplug_cb = ((*ctx).hotplug_cbs.next as uintptr_t).wrapping_sub(32 as libc::c_ulong)
        as *mut libusb_hotplug_callback;
    next = ((*hotplug_cb).list.next as uintptr_t).wrapping_sub(32 as libc::c_ulong)
        as *mut libusb_hotplug_callback;
    while &mut (*hotplug_cb).list as *mut list_head != &mut (*ctx).hotplug_cbs as *mut list_head {
        if forced != 0
            || (*hotplug_cb).flags as libc::c_int & USBI_HOTPLUG_NEEDS_FREE as libc::c_int != 0
        {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"usbi_hotplug_deregister\x00",
                ))
                .as_ptr(),
                b"freeing hotplug cb %p with handle %d\x00" as *const u8 as *const libc::c_char,
                hotplug_cb,
                (*hotplug_cb).handle,
            );
            list_del(&mut (*hotplug_cb).list);
            free(hotplug_cb as *mut libc::c_void);
        }
        hotplug_cb = next;
        next = ((*next).list.next as uintptr_t).wrapping_sub(32 as libc::c_ulong)
            as *mut libusb_hotplug_callback
    }
    usbi_mutex_unlock(&mut (*ctx).hotplug_cbs_lock);
}
