#![allow(non_camel_case_types,unused_assignments,unused_unsafe,non_snake_case,non_upper_case_globals,unused_must_use)]

use ::libc;
extern "C" {
    pub type udev;
    pub type udev_monitor;
    pub type udev_device;
    pub type udev_enumerate;
    pub type udev_list_entry;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn usbi_pipe(pipefd: *mut libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_self() -> pthread_t;
    #[no_mangle]
    fn pthread_setname_np(__target_thread: pthread_t, __name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn usbi_log(
        ctx: *mut libusb_context,
        level: libusb_log_level,
        function: *const libc::c_char,
        format: *const libc::c_char,
        _: ...
    );
    /*
     * usbfs header structures
     * Copyright © 2007 Daniel Drake <dsd@gentoo.org>
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
    /* keep in sync with usbdevice_fs.h:usbdevfs_ctrltransfer */
    /* in milliseconds */
    /* pointer to data */
    /* keep in sync with usbdevice_fs.h:usbdevfs_setinterface */
    /* Only used for isoc urbs */
    /* Only used with bulk streams */
    /* interface 0..N ; negative numbers reserved */
    /* MUST encode size + direction of data so the
     * macros in <asm/ioctl.h> give correct values */
    /* param buffer (in, or out) */
    /* Not used by USBDEVFS_FREE_STREAMS */
    #[no_mangle]
    fn linux_enumerate_device(
        ctx: *mut libusb_context,
        busnum: uint8_t,
        devaddr: uint8_t,
        sysfs_dir: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn linux_get_device_address(
        ctx: *mut libusb_context,
        detached: libc::c_int,
        busnum: *mut uint8_t,
        devaddr: *mut uint8_t,
        dev_node: *const libc::c_char,
        sys_name: *const libc::c_char,
        fd: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    static mut linux_hotplug_lock: usbi_mutex_static_t;
    #[no_mangle]
    fn linux_hotplug_enumerate(busnum: uint8_t, devaddr: uint8_t, sys_name: *const libc::c_char);
    #[no_mangle]
    fn linux_device_disconnected(busnum: uint8_t, devaddr: uint8_t);
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn udev_unref(udev: *mut udev) -> *mut udev;
    #[no_mangle]
    fn udev_new() -> *mut udev;
    #[no_mangle]
    fn udev_list_entry_get_next(list_entry: *mut udev_list_entry) -> *mut udev_list_entry;
    #[no_mangle]
    fn udev_list_entry_get_name(list_entry: *mut udev_list_entry) -> *const libc::c_char;
    #[no_mangle]
    fn udev_device_unref(udev_device: *mut udev_device) -> *mut udev_device;
    #[no_mangle]
    fn udev_device_new_from_syspath(
        udev: *mut udev,
        syspath: *const libc::c_char,
    ) -> *mut udev_device;
    #[no_mangle]
    fn udev_device_get_sysname(udev_device: *mut udev_device) -> *const libc::c_char;
    #[no_mangle]
    fn udev_device_get_devnode(udev_device: *mut udev_device) -> *const libc::c_char;
    #[no_mangle]
    fn udev_device_get_action(udev_device: *mut udev_device) -> *const libc::c_char;
    #[no_mangle]
    fn udev_monitor_unref(udev_monitor_0: *mut udev_monitor) -> *mut udev_monitor;
    #[no_mangle]
    fn udev_monitor_new_from_netlink(
        udev: *mut udev,
        name: *const libc::c_char,
    ) -> *mut udev_monitor;
    #[no_mangle]
    fn udev_monitor_enable_receiving(udev_monitor_0: *mut udev_monitor) -> libc::c_int;
    #[no_mangle]
    fn udev_monitor_get_fd(udev_monitor_0: *mut udev_monitor) -> libc::c_int;
    #[no_mangle]
    fn udev_monitor_receive_device(udev_monitor_0: *mut udev_monitor) -> *mut udev_device;
    #[no_mangle]
    fn udev_monitor_filter_add_match_subsystem_devtype(
        udev_monitor_0: *mut udev_monitor,
        subsystem: *const libc::c_char,
        devtype: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn udev_enumerate_unref(udev_enumerate: *mut udev_enumerate) -> *mut udev_enumerate;
    #[no_mangle]
    fn udev_enumerate_new(udev: *mut udev) -> *mut udev_enumerate;
    #[no_mangle]
    fn udev_enumerate_add_match_subsystem(
        udev_enumerate: *mut udev_enumerate,
        subsystem: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn udev_enumerate_add_match_property(
        udev_enumerate: *mut udev_enumerate,
        property: *const libc::c_char,
        value: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn udev_enumerate_scan_devices(udev_enumerate: *mut udev_enumerate) -> libc::c_int;
    #[no_mangle]
    fn udev_enumerate_get_list_entry(udev_enumerate: *mut udev_enumerate) -> *mut udev_list_entry;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
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
pub type pthread_t = libc::c_ulong;
pub type pthread_key_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
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
/* libusb */
/* * \ingroup libusb_lib
 * Structure providing the version of the libusb runtime
 */
/* * Library major version. */
/* * Library minor version. */
/* * Library micro version. */
/* * Library nano version. */
/* * Library release candidate suffix string, e.g. "-rc4". */
/* * For ABI compatibility only. */
/* * \ingroup libusb_lib
 * Structure representing a libusb session. The concept of individual libusb
 * sessions allows for your program to use two libraries (or dynamically
 * load two modules) which both independently use libusb. This will prevent
 * interference between the individual libusb users - for example
 * libusb_set_option() will not affect the other user of the library, and
 * libusb_exit() will not destroy resources that the other user is still
 * using.
 *
 * Sessions are created by libusb_init() and destroyed through libusb_exit().
 * If your application is guaranteed to only ever include a single libusb
 * user (i.e. you), you do not have to worry about contexts: pass NULL in
 * every function call where a context is required. The default context
 * will be used.
 *
 * For more information, see \ref libusb_contexts.
 */
/* * \ingroup libusb_dev
 * Structure representing a USB device detected on the system. This is an
 * opaque type for which you are only ever provided with a pointer, usually
 * originating from libusb_get_device_list().
 *
 * Certain operations can be performed on a device, but in order to do any
 * I/O you will have to first obtain a device handle using libusb_open().
 *
 * Devices are reference counted with libusb_ref_device() and
 * libusb_unref_device(), and are freed when the reference count reaches 0.
 * New devices presented by libusb_get_device_list() have a reference count of
 * 1, and libusb_free_device_list() can optionally decrease the reference count
 * on all devices in the list. libusb_open() adds another reference which is
 * later destroyed by libusb_close().
 */
/* * \ingroup libusb_dev
 * Structure representing a handle on a USB device. This is an opaque type for
 * which you are only ever provided with a pointer, usually originating from
 * libusb_open().
 *
 * A device handle is used to perform I/O and other operations. When finished
 * with a device handle, you should call libusb_close().
 */
/* * \ingroup libusb_dev
 * Speed codes. Indicates the speed at which the device is operating.
 */
/* * The OS doesn't report or know the device speed. */
/* * The device is operating at low speed (1.5MBit/s). */
/* * The device is operating at full speed (12MBit/s). */
/* * The device is operating at high speed (480MBit/s). */
/* * The device is operating at super speed (5000MBit/s). */
/* * The device is operating at super speed plus (10000MBit/s). */
/* * \ingroup libusb_misc
 * Error codes. Most libusb functions return 0 on success or one of these
 * codes on failure.
 * You can call libusb_error_name() to retrieve a string representation of an
 * error code or libusb_strerror() to get an end-user suitable description of
 * an error code.
 */
/* * Success (no error) */
/* * Input/output error */
/* * Invalid parameter */
/* * Access denied (insufficient permissions) */
/* * No such device (it may have been disconnected) */
/* * Entity not found */
/* * Resource busy */
/* * Operation timed out */
/* * Overflow */
/* * Pipe error */
/* * System call interrupted (perhaps due to signal) */
/* * Insufficient memory */
/* * Operation not supported or unimplemented on this platform */
/* NB: Remember to update LIBUSB_ERROR_COUNT below as well as the
message strings in strerror.c when adding new error codes here. */
/* * Other error */
/* Total number of error codes in enum libusb_error */
/* * \ingroup libusb_asyncio
 * Transfer type */
/* * Control transfer */
/* * Isochronous transfer */
/* * Bulk transfer */
/* * Interrupt transfer */
/* * Bulk stream transfer */
/* * \ingroup libusb_asyncio
 * Transfer status codes */
/* * Transfer completed without error. Note that this does not indicate
 * that the entire amount of requested data was transferred. */
/* * Transfer failed */
/* * Transfer timed out */
/* * Transfer was cancelled */
/* * For bulk/interrupt endpoints: halt condition detected (endpoint
 * stalled). For control endpoints: control request not supported. */
/* * Device was disconnected */
/* * Device sent more data than requested */
/* NB! Remember to update libusb_error_name()
when adding new status codes here. */
/* * \ingroup libusb_asyncio
 * libusb_transfer.flags values */
/* * Report short frames as errors */
/* * Automatically free() transfer buffer during libusb_free_transfer().
 * Note that buffers allocated with libusb_dev_mem_alloc() should not
 * be attempted freed in this way, since free() is not an appropriate
 * way to release such memory. */
/* * Automatically call libusb_free_transfer() after callback returns.
 * If this flag is set, it is illegal to call libusb_free_transfer()
 * from your transfer callback, as this will result in a double-free
 * when this flag is acted upon. */
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
/* * \ingroup libusb_asyncio
 * Isochronous packet descriptor. */
/* * Length of data to request in this packet */
/* * Amount of data that was actually transferred */
/* * Status code for this packet */
/* * \ingroup libusb_asyncio
 * Asynchronous transfer callback function type. When submitting asynchronous
 * transfers, you pass a pointer to a callback function of this type via the
 * \ref libusb_transfer::callback "callback" member of the libusb_transfer
 * structure. libusb will call this function later, when the transfer has
 * completed or failed. See \ref libusb_asyncio for more information.
 * \param transfer The libusb_transfer struct the callback function is being
 * notified about.
 */
/* * \ingroup libusb_asyncio
 * The generic USB transfer structure. The user populates this structure and
 * then submits it in order to request a transfer. After the transfer has
 * completed, the library populates the transfer with the results and passes
 * it back to the user.
 */
/* * Handle of the device that this transfer will be submitted to */
/* * A bitwise OR combination of \ref libusb_transfer_flags. */
/* * Address of the endpoint where this transfer will be sent. */
/* * Type of the transfer from \ref libusb_transfer_type */
/* * Timeout for this transfer in milliseconds. A value of 0 indicates no
 * timeout. */
/* * The status of the transfer. Read-only, and only for use within
 * transfer callback function.
 *
 * If this is an isochronous transfer, this field may read COMPLETED even
 * if there were errors in the frames. Use the
 * \ref libusb_iso_packet_descriptor::status "status" field in each packet
 * to determine if errors occurred. */
/* * Length of the data buffer. Must be non-negative. */
/* * Actual length of data that was transferred. Read-only, and only for
 * use within transfer callback function. Not valid for isochronous
 * endpoint transfers. */
/* * Callback function. This will be invoked when the transfer completes,
 * fails, or is cancelled. */
/* * User context data to pass to the callback function. */
/* * Data buffer */
/* * Number of isochronous packets. Only used for I/O with isochronous
 * endpoints. Must be non-negative. */
/* * Isochronous packet descriptors, for isochronous transfers only. */
/* * \ingroup libusb_misc
 * Capabilities supported by an instance of libusb on the current running
 * platform. Test if the loaded library supports a given capability by calling
 * \ref libusb_has_capability().
 */
/* * The libusb_has_capability() API is available. */
/* * Hotplug support is available on this platform. */
/* * The library can access HID devices without requiring user intervention.
 * Note that before being able to actually access an HID device, you may
 * still have to call additional libusb functions such as
 * \ref libusb_detach_kernel_driver(). */
/* * The library supports detaching of the default USB driver, using
 * \ref libusb_detach_kernel_driver(), if one is set by the OS kernel */
/* * \ingroup libusb_lib
 *  Log message levels.
 */
/* * (0) : No messages ever emitted by the library (default) */
/* * (1) : Error messages are emitted */
/* * (2) : Warning and error messages are emitted */
/* * (3) : Informational, warning and error messages are emitted */
/* * (4) : All messages are emitted */
/* * \ingroup libusb_lib
 *  Log callback mode.
 * \see libusb_set_log_cb()
 */
/* * Callback function handling all log mesages. */
/* * Callback function handling context related log mesages. */
/* * \ingroup libusb_lib
 * Callback function for handling log messages.
 * \param ctx the context which is related to the log message, or NULL if it
 * is a global log message
 * \param level the log level, see \ref libusb_log_level for a description
 * \param str the log message
 * \see libusb_set_log_cb()
 */
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
pub type libusb_log_cb = Option<
    unsafe extern "C" fn(_: *mut libusb_context, _: libusb_log_level, _: *const libc::c_char) -> (),
>;
pub type libusb_log_level = libc::c_uint;
pub const LIBUSB_LOG_LEVEL_DEBUG: libusb_log_level = 4;
pub const LIBUSB_LOG_LEVEL_INFO: libusb_log_level = 3;
pub const LIBUSB_LOG_LEVEL_WARNING: libusb_log_level = 2;
pub const LIBUSB_LOG_LEVEL_ERROR: libusb_log_level = 1;
pub const LIBUSB_LOG_LEVEL_NONE: libusb_log_level = 0;
pub type libusb_error = libc::c_int;
pub const LIBUSB_ERROR_OTHER: libusb_error = -99;
pub const LIBUSB_ERROR_NOT_SUPPORTED: libusb_error = -12;
pub const LIBUSB_ERROR_NO_MEM: libusb_error = -11;
pub const LIBUSB_ERROR_INTERRUPTED: libusb_error = -10;
pub const LIBUSB_ERROR_PIPE: libusb_error = -9;
pub const LIBUSB_ERROR_OVERFLOW: libusb_error = -8;
pub const LIBUSB_ERROR_TIMEOUT: libusb_error = -7;
pub const LIBUSB_ERROR_BUSY: libusb_error = -6;
pub const LIBUSB_ERROR_NOT_FOUND: libusb_error = -5;
pub const LIBUSB_ERROR_NO_DEVICE: libusb_error = -4;
pub const LIBUSB_ERROR_ACCESS: libusb_error = -3;
pub const LIBUSB_ERROR_INVALID_PARAM: libusb_error = -2;
pub const LIBUSB_ERROR_IO: libusb_error = -1;
pub const LIBUSB_SUCCESS: libusb_error = 0;
pub type usbi_mutex_static_t = pthread_mutex_t;
#[inline]
unsafe extern "C" fn usbi_mutex_static_lock(mutex: *mut usbi_mutex_static_t) {
    pthread_mutex_lock(mutex);
}
#[inline]
unsafe extern "C" fn usbi_mutex_static_unlock(mutex: *mut usbi_mutex_static_t) {
    pthread_mutex_unlock(mutex);
}
/* -*- Mode: C; c-basic-offset:8 ; indent-tabs-mode:t -*- */
/*
 * Linux usbfs backend for libusb
 * Copyright (C) 2007-2009 Daniel Drake <dsd@gentoo.org>
 * Copyright (c) 2001 Johannes Erdfelt <johannes@erdfelt.com>
 * Copyright (c) 2012-2013 Nathan Hjelm <hjelmn@mac.com>
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
/* udev context */
static mut udev_ctx: *mut udev = 0 as *const udev as *mut udev;
static mut udev_monitor_fd: libc::c_int = -(1 as libc::c_int);
static mut udev_control_pipe: [libc::c_int; 2] = [-(1 as libc::c_int), -(1 as libc::c_int)];
static mut udev_monitor: *mut udev_monitor = 0 as *const udev_monitor as *mut udev_monitor;
static mut linux_event_thread: pthread_t = 0;
#[no_mangle]
pub unsafe extern "C" fn linux_udev_start_event_monitor() -> libc::c_int {
    let mut current_block: u64;
    let mut r: libc::c_int = 0;
    if udev_ctx.is_null() {
    } else {
        __assert_fail(
            b"udev_ctx == NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/konstantin/Projects/tests/rust/libusb2rs/libusb/libusb/os/linux_udev.c\x00"
                as *const u8 as *const libc::c_char,
            48 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 41], &[libc::c_char; 41]>(
                b"int linux_udev_start_event_monitor(void)\x00",
            ))
            .as_ptr(),
        );
    }
    udev_ctx = udev_new();
    if udev_ctx.is_null() {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                b"linux_udev_start_event_monitor\x00",
            ))
            .as_ptr(),
            b"could not create udev context\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        udev_monitor = udev_monitor_new_from_netlink(
            udev_ctx,
            b"udev\x00" as *const u8 as *const libc::c_char,
        );
        if udev_monitor.is_null() {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"linux_udev_start_event_monitor\x00",
                ))
                .as_ptr(),
                b"could not initialize udev monitor\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            r = udev_monitor_filter_add_match_subsystem_devtype(
                udev_monitor,
                b"usb\x00" as *const u8 as *const libc::c_char,
                b"usb_device\x00" as *const u8 as *const libc::c_char,
            );
            if r != 0 {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                        b"linux_udev_start_event_monitor\x00",
                    ))
                    .as_ptr(),
                    b"could not initialize udev monitor filter for \"usb\" subsystem\x00"
                        as *const u8 as *const libc::c_char,
                );
            } else if udev_monitor_enable_receiving(udev_monitor) != 0 {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                        b"linux_udev_start_event_monitor\x00",
                    ))
                    .as_ptr(),
                    b"failed to enable the udev monitor\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                udev_monitor_fd = udev_monitor_get_fd(udev_monitor);
                /* Make sure the udev file descriptor is marked as CLOEXEC */
                r = fcntl(udev_monitor_fd, 1 as libc::c_int);
                if r == -(1 as libc::c_int) {
                    usbi_log(
                        0 as *mut libusb_context,
                        LIBUSB_LOG_LEVEL_ERROR,
                        (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                            b"linux_udev_start_event_monitor\x00",
                        ))
                        .as_ptr(),
                        b"failed to get udev monitor fd flags, errno=%d\x00" as *const u8
                            as *const libc::c_char,
                        *__errno_location(),
                    );
                } else {
                    if r & 1 as libc::c_int == 0 {
                        if fcntl(udev_monitor_fd, 2 as libc::c_int, r | 1 as libc::c_int)
                            == -(1 as libc::c_int)
                        {
                            usbi_log(
                                0 as *mut libusb_context,
                                LIBUSB_LOG_LEVEL_ERROR,
                                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                    b"linux_udev_start_event_monitor\x00",
                                ))
                                .as_ptr(),
                                b"failed to set udev monitor fd flags, errno=%d\x00" as *const u8
                                    as *const libc::c_char,
                                *__errno_location(),
                            );
                            current_block = 8416519292278917778;
                        } else {
                            current_block = 18317007320854588510;
                        }
                    } else {
                        current_block = 18317007320854588510;
                    }
                    match current_block {
                        8416519292278917778 => {}
                        _ => {
                            /* Some older versions of udev are not non-blocking by default,
                             * so make sure this is set */
                            r = fcntl(udev_monitor_fd, 3 as libc::c_int);
                            if r == -(1 as libc::c_int) {
                                usbi_log(
                                    0 as *mut libusb_context,
                                    LIBUSB_LOG_LEVEL_ERROR,
                                    (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                                        b"linux_udev_start_event_monitor\x00",
                                    ))
                                    .as_ptr(),
                                    b"failed to get udev monitor fd status flags, errno=%d\x00"
                                        as *const u8
                                        as *const libc::c_char,
                                    *__errno_location(),
                                );
                            } else {
                                if r & 0o4000 as libc::c_int == 0 {
                                    if fcntl(
                                        udev_monitor_fd,
                                        4 as libc::c_int,
                                        r | 0o4000 as libc::c_int,
                                    ) == -(1 as libc::c_int)
                                    {
                                        usbi_log(0 as *mut libusb_context,
                                                 LIBUSB_LOG_LEVEL_ERROR,
                                                 (*::std::mem::transmute::<&[u8; 31],
                                                                           &[libc::c_char; 31]>(b"linux_udev_start_event_monitor\x00")).as_ptr(),
                                                 b"failed to set udev monitor fd status flags, errno=%d\x00"
                                                     as *const u8 as
                                                     *const libc::c_char,
                                                 *__errno_location());
                                        current_block = 8416519292278917778;
                                    } else {
                                        current_block = 7205609094909031804;
                                    }
                                } else {
                                    current_block = 7205609094909031804;
                                }
                                match current_block {
                                    8416519292278917778 => {}
                                    _ => {
                                        r = usbi_pipe(udev_control_pipe.as_mut_ptr());
                                        if r != 0 {
                                            usbi_log(
                                                0 as *mut libusb_context,
                                                LIBUSB_LOG_LEVEL_ERROR,
                                                (*::std::mem::transmute::<
                                                    &[u8; 31],
                                                    &[libc::c_char; 31],
                                                >(
                                                    b"linux_udev_start_event_monitor\x00"
                                                ))
                                                .as_ptr(),
                                                b"could not create udev control pipe\x00"
                                                    as *const u8
                                                    as *const libc::c_char,
                                            );
                                        } else {
                                            r =
                                                pthread_create(&mut linux_event_thread,
                                                               0 as
                                                                   *const pthread_attr_t,
                                                               Some(linux_udev_event_thread_main
                                                                        as
                                                                        unsafe extern "C" fn(_:
                                                                                                 *mut libc::c_void)
                                                                            ->
                                                                                *mut libc::c_void),
                                                               0 as
                                                                   *mut libc::c_void);
                                            if r != 0 {
                                                usbi_log(
                                                    0 as *mut libusb_context,
                                                    LIBUSB_LOG_LEVEL_ERROR,
                                                    (*::std::mem::transmute::<
                                                        &[u8; 31],
                                                        &[libc::c_char; 31],
                                                    >(
                                                        b"linux_udev_start_event_monitor\x00"
                                                    ))
                                                    .as_ptr(),
                                                    b"creating hotplug event thread (%d)\x00"
                                                        as *const u8
                                                        as *const libc::c_char,
                                                    r,
                                                );
                                                close(udev_control_pipe[0 as libc::c_int as usize]);
                                                close(udev_control_pipe[1 as libc::c_int as usize]);
                                            } else {
                                                return LIBUSB_SUCCESS as libc::c_int;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            udev_monitor_unref(udev_monitor);
            udev_monitor = 0 as *mut udev_monitor;
            udev_monitor_fd = -(1 as libc::c_int)
        }
        udev_unref(udev_ctx);
    }
    udev_ctx = 0 as *mut udev;
    return LIBUSB_ERROR_OTHER as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn linux_udev_stop_event_monitor() -> libc::c_int {
    let mut dummy: libc::c_char = 1 as libc::c_int as libc::c_char;
    let mut r: ssize_t = 0;
    if !udev_ctx.is_null() {
    } else {
        __assert_fail(
            b"udev_ctx != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/konstantin/Projects/tests/rust/libusb2rs/libusb/libusb/os/linux_udev.c\x00"
                as *const u8 as *const libc::c_char,
            136 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"int linux_udev_stop_event_monitor(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if !udev_monitor.is_null() {
    } else {
        __assert_fail(
            b"udev_monitor != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/konstantin/Projects/tests/rust/libusb2rs/libusb/libusb/os/linux_udev.c\x00"
                as *const u8 as *const libc::c_char,
            137 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"int linux_udev_stop_event_monitor(void)\x00",
            ))
            .as_ptr(),
        );
    }
    if udev_monitor_fd != -(1 as libc::c_int) {
    } else {
        __assert_fail(
            b"udev_monitor_fd != -1\x00" as *const u8 as *const libc::c_char,
            b"/home/konstantin/Projects/tests/rust/libusb2rs/libusb/libusb/os/linux_udev.c\x00"
                as *const u8 as *const libc::c_char,
            138 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 40], &[libc::c_char; 40]>(
                b"int linux_udev_stop_event_monitor(void)\x00",
            ))
            .as_ptr(),
        );
    }
    /* Write some dummy data to the control pipe and
     * wait for the thread to exit */
    r = write(
        udev_control_pipe[1 as libc::c_int as usize],
        &mut dummy as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    );
    if r <= 0 as libc::c_int as libc::c_long {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_WARNING,
            (*::std::mem::transmute::<&[u8; 30], &[libc::c_char; 30]>(
                b"linux_udev_stop_event_monitor\x00",
            ))
            .as_ptr(),
            b"udev control pipe signal failed\x00" as *const u8 as *const libc::c_char,
        );
    }
    pthread_join(linux_event_thread, 0 as *mut *mut libc::c_void);
    /* Release the udev monitor */
    udev_monitor_unref(udev_monitor);
    udev_monitor = 0 as *mut udev_monitor;
    udev_monitor_fd = -(1 as libc::c_int);
    /* Clean up the udev context */
    udev_unref(udev_ctx);
    udev_ctx = 0 as *mut udev;
    /* close and reset control pipe */
    close(udev_control_pipe[0 as libc::c_int as usize]);
    close(udev_control_pipe[1 as libc::c_int as usize]);
    udev_control_pipe[0 as libc::c_int as usize] = -(1 as libc::c_int);
    udev_control_pipe[1 as libc::c_int as usize] = -(1 as libc::c_int);
    return LIBUSB_SUCCESS as libc::c_int;
}
unsafe extern "C" fn linux_udev_event_thread_main(_arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut dummy: libc::c_char = 0;
    let mut r: libc::c_int = 0;
    let mut nb: ssize_t = 0;
    let mut udev_dev: *mut udev_device = 0 as *mut udev_device;
    let mut fds: [pollfd; 2] = [
        {
            let init = pollfd {
                fd: udev_control_pipe[0 as libc::c_int as usize],
                events: 0x1 as libc::c_int as libc::c_short,
                revents: 0,
            };
            init
        },
        {
            let init = pollfd {
                fd: udev_monitor_fd,
                events: 0x1 as libc::c_int as libc::c_short,
                revents: 0,
            };
            init
        },
    ];
    r = pthread_setname_np(
        pthread_self(),
        b"libusb_event\x00" as *const u8 as *const libc::c_char,
    );
    if r != 0 {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_WARNING,
            (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                b"linux_udev_event_thread_main\x00",
            ))
            .as_ptr(),
            b"failed to set hotplug event thread name, error=%d\x00" as *const u8
                as *const libc::c_char,
            r,
        );
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
            b"linux_udev_event_thread_main\x00",
        ))
        .as_ptr(),
        b"udev event thread entering\x00" as *const u8 as *const libc::c_char,
    );
    loop
    /* temporary failure */
    {
        r = poll(
            fds.as_mut_ptr(),
            2 as libc::c_int as nfds_t,
            -(1 as libc::c_int),
        );
        if !(r >= 0 as libc::c_int || *__errno_location() == 4 as libc::c_int) {
            break;
        }
        if r < 0 as libc::c_int {
            continue;
        }
        if fds[0 as libc::c_int as usize].revents as libc::c_int & 0x1 as libc::c_int != 0 {
            /* activity on control pipe, read the byte and exit */
            nb = read(
                udev_control_pipe[0 as libc::c_int as usize],
                &mut dummy as *mut libc::c_char as *mut libc::c_void,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            );
            if nb <= 0 as libc::c_int as libc::c_long {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_WARNING,
                    (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
                        b"linux_udev_event_thread_main\x00",
                    ))
                    .as_ptr(),
                    b"udev control pipe read failed\x00" as *const u8 as *const libc::c_char,
                );
            }
            break;
        } else if fds[1 as libc::c_int as usize].revents as libc::c_int & 0x1 as libc::c_int != 0 {
            usbi_mutex_static_lock(&mut linux_hotplug_lock);
            udev_dev = udev_monitor_receive_device(udev_monitor);
            if !udev_dev.is_null() {
                udev_hotplug_event(udev_dev);
            }
            usbi_mutex_static_unlock(&mut linux_hotplug_lock);
        }
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 29], &[libc::c_char; 29]>(
            b"linux_udev_event_thread_main\x00",
        ))
        .as_ptr(),
        b"udev event thread exiting\x00" as *const u8 as *const libc::c_char,
    );
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn udev_device_info(
    ctx: *mut libusb_context,
    detached: libc::c_int,
    udev_dev: *mut udev_device,
    busnum: *mut uint8_t,
    devaddr: *mut uint8_t,
    sys_name: *mut *const libc::c_char,
) -> libc::c_int {
    let mut dev_node: *const libc::c_char = 0 as *const libc::c_char;
    dev_node = udev_device_get_devnode(udev_dev);
    if dev_node.is_null() {
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    *sys_name = udev_device_get_sysname(udev_dev);
    if (*sys_name).is_null() {
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    return linux_get_device_address(
        ctx,
        detached,
        busnum,
        devaddr,
        dev_node,
        *sys_name,
        -(1 as libc::c_int),
    );
}
unsafe extern "C" fn udev_hotplug_event(udev_dev: *mut udev_device) {
    let mut udev_action: *const libc::c_char = 0 as *const libc::c_char;
    let mut sys_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut busnum: uint8_t = 0 as libc::c_int as uint8_t;
    let mut devaddr: uint8_t = 0 as libc::c_int as uint8_t;
    let mut detached: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    udev_action = udev_device_get_action(udev_dev);
    if !udev_action.is_null() {
        detached = (strncmp(
            udev_action,
            b"remove\x00" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0) as libc::c_int;
        r = udev_device_info(
            0 as *mut libusb_context,
            detached,
            udev_dev,
            &mut busnum,
            &mut devaddr,
            &mut sys_name,
        );
        if !(LIBUSB_SUCCESS as libc::c_int != r) {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"udev_hotplug_event\x00",
                ))
                .as_ptr(),
                b"udev hotplug event. action: %s.\x00" as *const u8 as *const libc::c_char,
                udev_action,
            );
            if strncmp(
                udev_action,
                b"add\x00" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                linux_hotplug_enumerate(busnum, devaddr, sys_name);
            } else if detached != 0 {
                linux_device_disconnected(busnum, devaddr);
            } else if !(strncmp(
                udev_action,
                b"bind\x00" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int)
            {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                        b"udev_hotplug_event\x00",
                    ))
                    .as_ptr(),
                    b"ignoring udev action %s\x00" as *const u8 as *const libc::c_char,
                    udev_action,
                );
            }
        }
    }
    udev_device_unref(udev_dev);
}
#[no_mangle]
pub unsafe extern "C" fn linux_udev_scan_devices(ctx: *mut libusb_context) -> libc::c_int {
    let mut enumerator: *mut udev_enumerate = 0 as *mut udev_enumerate;
    let mut devices: *mut udev_list_entry = 0 as *mut udev_list_entry;
    let mut entry: *mut udev_list_entry = 0 as *mut udev_list_entry;
    let mut udev_dev: *mut udev_device = 0 as *mut udev_device;
    let mut sys_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    if !udev_ctx.is_null() {
    } else {
        __assert_fail(
            b"udev_ctx != NULL\x00" as *const u8 as *const libc::c_char,
            b"/home/konstantin/Projects/tests/rust/libusb2rs/libusb/libusb/os/linux_udev.c\x00"
                as *const u8 as *const libc::c_char,
            280 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 53], &[libc::c_char; 53]>(
                b"int linux_udev_scan_devices(struct libusb_context *)\x00",
            ))
            .as_ptr(),
        );
    }
    enumerator = udev_enumerate_new(udev_ctx);
    if enumerator.is_null() {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"linux_udev_scan_devices\x00",
            ))
            .as_ptr(),
            b"error creating udev enumerator\x00" as *const u8 as *const libc::c_char,
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    udev_enumerate_add_match_subsystem(enumerator, b"usb\x00" as *const u8 as *const libc::c_char);
    udev_enumerate_add_match_property(
        enumerator,
        b"DEVTYPE\x00" as *const u8 as *const libc::c_char,
        b"usb_device\x00" as *const u8 as *const libc::c_char,
    );
    udev_enumerate_scan_devices(enumerator);
    devices = udev_enumerate_get_list_entry(enumerator);
    entry = 0 as *mut udev_list_entry;
    entry = devices;
    while !entry.is_null() {
        let path: *const libc::c_char = udev_list_entry_get_name(entry);
        let mut busnum: uint8_t = 0 as libc::c_int as uint8_t;
        let mut devaddr: uint8_t = 0 as libc::c_int as uint8_t;
        udev_dev = udev_device_new_from_syspath(udev_ctx, path);
        r = udev_device_info(
            ctx,
            0 as libc::c_int,
            udev_dev,
            &mut busnum,
            &mut devaddr,
            &mut sys_name,
        );
        if r != 0 {
            udev_device_unref(udev_dev);
        } else {
            linux_enumerate_device(ctx, busnum, devaddr, sys_name);
            udev_device_unref(udev_dev);
        }
        entry = udev_list_entry_get_next(entry)
    }
    udev_enumerate_unref(enumerator);
    return LIBUSB_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn linux_udev_hotplug_poll() {
    let mut udev_dev: *mut udev_device = 0 as *mut udev_device;
    usbi_mutex_static_lock(&mut linux_hotplug_lock);
    loop {
        udev_dev = udev_monitor_receive_device(udev_monitor);
        if !udev_dev.is_null() {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                    b"linux_udev_hotplug_poll\x00",
                ))
                .as_ptr(),
                b"Handling hotplug event from hotplug_poll\x00" as *const u8 as *const libc::c_char,
            );
            udev_hotplug_event(udev_dev);
        }
        if udev_dev.is_null() {
            break;
        }
    }
    usbi_mutex_static_unlock(&mut linux_hotplug_lock);
}
