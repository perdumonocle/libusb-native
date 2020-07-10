#![allow(non_camel_case_types,unused_assignments,unused_unsafe,non_snake_case,non_upper_case_globals,unused_must_use)]

use ::libc;
extern "C" {
    pub type __dirstream;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn nanosleep(__requested_time: *const timespec, __remaining: *mut timespec) -> libc::c_int;
    #[no_mangle]
    fn libusb_error_name(errcode: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn libusb_ref_device(dev: *mut libusb_device) -> *mut libusb_device;
    #[no_mangle]
    fn libusb_unref_device(dev: *mut libusb_device);
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
    /* A human-readable name for your backend, e.g. "Linux usbfs" */
    /* Binary mask for backend specific capabilities */
    /* Perform initialization of your backend. You might use this function
     * to determine specific capabilities of the system, allocate required
     * data structures for later, etc.
     *
     * This function is called when a libusb user initializes the library
     * prior to use.
     *
     * Return 0 on success, or a LIBUSB_ERROR code on failure.
     */
    /* Deinitialization. Optional. This function should destroy anything
     * that was set up by init.
     *
     * This function is called when the user deinitializes the library.
     */
    /* Set a backend-specific option. Optional.
     *
     * This function is called when the user calls libusb_set_option() and
     * the option is not handled by the core library.
     *
     * Return 0 on success, or a LIBUSB_ERROR code on failure.
     */
    /* Enumerate all the USB devices on the system, returning them in a list
     * of discovered devices.
     *
     * Your implementation should enumerate all devices on the system,
     * regardless of whether they have been seen before or not.
     *
     * When you have found a device, compute a session ID for it. The session
     * ID should uniquely represent that particular device for that particular
     * connection session since boot (i.e. if you disconnect and reconnect a
     * device immediately after, it should be assigned a different session ID).
     * If your OS cannot provide a unique session ID as described above,
     * presenting a session ID of (bus_number << 8 | device_address) should
     * be sufficient. Bus numbers and device addresses wrap and get reused,
     * but that is an unlikely case.
     *
     * After computing a session ID for a device, call
     * usbi_get_device_by_session_id(). This function checks if libusb already
     * knows about the device, and if so, it provides you with a reference
     * to a libusb_device structure for it.
     *
     * If usbi_get_device_by_session_id() returns NULL, it is time to allocate
     * a new device structure for the device. Call usbi_alloc_device() to
     * obtain a new libusb_device structure with reference count 1. Populate
     * the bus_number and device_address attributes of the new device, and
     * perform any other internal backend initialization you need to do. At
     * this point, you should be ready to provide device descriptors and so
     * on through the get_*_descriptor functions. Finally, call
     * usbi_sanitize_device() to perform some final sanity checks on the
     * device. Assuming all of the above succeeded, we can now continue.
     * If any of the above failed, remember to unreference the device that
     * was returned by usbi_alloc_device().
     *
     * At this stage we have a populated libusb_device structure (either one
     * that was found earlier, or one that we have just allocated and
     * populated). This can now be added to the discovered devices list
     * using discovered_devs_append(). Note that discovered_devs_append()
     * may reallocate the list, returning a new location for it, and also
     * note that reallocation can fail. Your backend should handle these
     * error conditions appropriately.
     *
     * This function should not generate any bus I/O and should not block.
     * If I/O is required (e.g. reading the active configuration value), it is
     * OK to ignore these suggestions :)
     *
     * This function is executed when the user wishes to retrieve a list
     * of USB devices connected to the system.
     *
     * If the backend has hotplug support, this function is not used!
     *
     * Return 0 on success, or a LIBUSB_ERROR code on failure.
     */
    /* Apps which were written before hotplug support, may listen for
     * hotplug events on their own and call libusb_get_device_list on
     * device addition. In this case libusb_get_device_list will likely
     * return a list without the new device in there, as the hotplug
     * event thread will still be busy enumerating the device, which may
     * take a while, or may not even have seen the event yet.
     *
     * To avoid this libusb_get_device_list will call this optional
     * function for backends with hotplug support before copying
     * ctx->usb_devs to the user. In this function the backend should
     * ensure any pending hotplug events are fully processed before
     * returning.
     *
     * Optional, should be implemented by backends with hotplug support.
     */
    /* Wrap a platform-specific device handle for I/O and other USB
     * operations. The device handle is preallocated for you.
     *
     * Your backend should allocate any internal resources required for I/O
     * and other operations so that those operations can happen (hopefully)
     * without hiccup. This is also a good place to inform libusb that it
     * should monitor certain file descriptors related to this device -
     * see the usbi_add_pollfd() function.
     *
     * Your backend should also initialize the device structure
     * (dev_handle->dev), which is NULL at the beginning of the call.
     *
     * This function should not generate any bus I/O and should not block.
     *
     * This function is called when the user attempts to wrap an existing
     * platform-specific device handle for a device.
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_ACCESS if the user has insufficient permissions
     * - another LIBUSB_ERROR code on other failure
     *
     * Do not worry about freeing the handle on failed open, the upper layers
     * do this for you.
     */
    /* Open a device for I/O and other USB operations. The device handle
     * is preallocated for you, you can retrieve the device in question
     * through handle->dev.
     *
     * Your backend should allocate any internal resources required for I/O
     * and other operations so that those operations can happen (hopefully)
     * without hiccup. This is also a good place to inform libusb that it
     * should monitor certain file descriptors related to this device -
     * see the usbi_add_pollfd() function.
     *
     * This function should not generate any bus I/O and should not block.
     *
     * This function is called when the user attempts to obtain a device
     * handle for a device.
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_ACCESS if the user has insufficient permissions
     * - LIBUSB_ERROR_NO_DEVICE if the device has been disconnected since
     *   discovery
     * - another LIBUSB_ERROR code on other failure
     *
     * Do not worry about freeing the handle on failed open, the upper layers
     * do this for you.
     */
    /* Close a device such that the handle cannot be used again. Your backend
     * should destroy any resources that were allocated in the open path.
     * This may also be a good place to call usbi_remove_pollfd() to inform
     * libusb of any file descriptors associated with this device that should
     * no longer be monitored.
     *
     * This function is called when the user closes a device handle.
     */
    /* Get the ACTIVE configuration descriptor for a device.
     *
     * The descriptor should be retrieved from memory, NOT via bus I/O to the
     * device. This means that you may have to cache it in a private structure
     * during get_device_list enumeration. You may also have to keep track
     * of which configuration is active when the user changes it.
     *
     * This function is expected to write len bytes of data into buffer, which
     * is guaranteed to be big enough. If you can only do a partial write,
     * return an error code.
     *
     * This function is expected to return the descriptor in bus-endian format
     * (LE).
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_NOT_FOUND if the device is in unconfigured state
     * - another LIBUSB_ERROR code on other failure
     */
    /* Get a specific configuration descriptor for a device.
     *
     * The descriptor should be retrieved from memory, NOT via bus I/O to the
     * device. This means that you may have to cache it in a private structure
     * during get_device_list enumeration.
     *
     * The requested descriptor is expressed as a zero-based index (i.e. 0
     * indicates that we are requesting the first descriptor). The index does
     * not (necessarily) equal the bConfigurationValue of the configuration
     * being requested.
     *
     * This function is expected to write len bytes of data into buffer, which
     * is guaranteed to be big enough. If you can only do a partial write,
     * return an error code.
     *
     * This function is expected to return the descriptor in bus-endian format
     * (LE).
     *
     * Return the length read on success or a LIBUSB_ERROR code on failure.
     */
    /* Like get_config_descriptor but then by bConfigurationValue instead
     * of by index.
     *
     * Optional, if not present the core will call get_config_descriptor
     * for all configs until it finds the desired bConfigurationValue.
     *
     * Returns a pointer to the raw-descriptor in *buffer, this memory
     * is valid as long as device is valid.
     *
     * Returns the length of the returned raw-descriptor on success,
     * or a LIBUSB_ERROR code on failure.
     */
    /* Get the bConfigurationValue for the active configuration for a device.
     * Optional. This should only be implemented if you can retrieve it from
     * cache (don't generate I/O).
     *
     * If you cannot retrieve this from cache, either do not implement this
     * function, or return LIBUSB_ERROR_NOT_SUPPORTED. This will cause
     * libusb to retrieve the information through a standard control transfer.
     *
     * This function must be non-blocking.
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_NO_DEVICE if the device has been disconnected since it
     *   was opened
     * - LIBUSB_ERROR_NOT_SUPPORTED if the value cannot be retrieved without
     *   blocking
     * - another LIBUSB_ERROR code on other failure.
     */
    /* Set the active configuration for a device.
     *
     * A configuration value of -1 should put the device in unconfigured state.
     *
     * This function can block.
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_NOT_FOUND if the configuration does not exist
     * - LIBUSB_ERROR_BUSY if interfaces are currently claimed (and hence
     *   configuration cannot be changed)
     * - LIBUSB_ERROR_NO_DEVICE if the device has been disconnected since it
     *   was opened
     * - another LIBUSB_ERROR code on other failure.
     */
    /* Claim an interface. When claimed, the application can then perform
     * I/O to an interface's endpoints.
     *
     * This function should not generate any bus I/O and should not block.
     * Interface claiming is a logical operation that simply ensures that
     * no other drivers/applications are using the interface, and after
     * claiming, no other drivers/applications can use the interface because
     * we now "own" it.
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_NOT_FOUND if the interface does not exist
     * - LIBUSB_ERROR_BUSY if the interface is in use by another driver/app
     * - LIBUSB_ERROR_NO_DEVICE if the device has been disconnected since it
     *   was opened
     * - another LIBUSB_ERROR code on other failure
     */
    /* Release a previously claimed interface.
     *
     * This function should also generate a SET_INTERFACE control request,
     * resetting the alternate setting of that interface to 0. It's OK for
     * this function to block as a result.
     *
     * You will only ever be asked to release an interface which was
     * successfully claimed earlier.
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_NO_DEVICE if the device has been disconnected since it
     *   was opened
     * - another LIBUSB_ERROR code on other failure
     */
    /* Set the alternate setting for an interface.
     *
     * You will only ever be asked to set the alternate setting for an
     * interface which was successfully claimed earlier.
     *
     * It's OK for this function to block.
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_NOT_FOUND if the alternate setting does not exist
     * - LIBUSB_ERROR_NO_DEVICE if the device has been disconnected since it
     *   was opened
     * - another LIBUSB_ERROR code on other failure
     */
    /* Clear a halt/stall condition on an endpoint.
     *
     * It's OK for this function to block.
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_NOT_FOUND if the endpoint does not exist
     * - LIBUSB_ERROR_NO_DEVICE if the device has been disconnected since it
     *   was opened
     * - another LIBUSB_ERROR code on other failure
     */
    /* Perform a USB port reset to reinitialize a device. Optional.
     *
     * If possible, the device handle should still be usable after the reset
     * completes, assuming that the device descriptors did not change during
     * reset and all previous interface state can be restored.
     *
     * If something changes, or you cannot easily locate/verify the resetted
     * device, return LIBUSB_ERROR_NOT_FOUND. This prompts the application
     * to close the old handle and re-enumerate the device.
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_NOT_FOUND if re-enumeration is required, or if the device
     *   has been disconnected since it was opened
     * - another LIBUSB_ERROR code on other failure
     */
    /* Alloc num_streams usb3 bulk streams on the passed in endpoints */
    /* Free usb3 bulk streams allocated with alloc_streams */
    /* Allocate persistent DMA memory for the given device, suitable for
     * zerocopy. May return NULL on failure. Optional to implement.
     */
    /* Free memory allocated by dev_mem_alloc. */
    /* Determine if a kernel driver is active on an interface. Optional.
     *
     * The presence of a kernel driver on an interface indicates that any
     * calls to claim_interface would fail with the LIBUSB_ERROR_BUSY code.
     *
     * Return:
     * - 0 if no driver is active
     * - 1 if a driver is active
     * - LIBUSB_ERROR_NO_DEVICE if the device has been disconnected since it
     *   was opened
     * - another LIBUSB_ERROR code on other failure
     */
    /* Detach a kernel driver from an interface. Optional.
     *
     * After detaching a kernel driver, the interface should be available
     * for claim.
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_NOT_FOUND if no kernel driver was active
     * - LIBUSB_ERROR_INVALID_PARAM if the interface does not exist
     * - LIBUSB_ERROR_NO_DEVICE if the device has been disconnected since it
     *   was opened
     * - another LIBUSB_ERROR code on other failure
     */
    /* Attach a kernel driver to an interface. Optional.
     *
     * Reattach a kernel driver to the device.
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_NOT_FOUND if no kernel driver was active
     * - LIBUSB_ERROR_INVALID_PARAM if the interface does not exist
     * - LIBUSB_ERROR_NO_DEVICE if the device has been disconnected since it
     *   was opened
     * - LIBUSB_ERROR_BUSY if a program or driver has claimed the interface,
     *   preventing reattachment
     * - another LIBUSB_ERROR code on other failure
     */
    /* Destroy a device. Optional.
     *
     * This function is called when the last reference to a device is
     * destroyed. It should free any resources allocated in the get_device_list
     * path.
     */
    /* Submit a transfer. Your implementation should take the transfer,
     * morph it into whatever form your platform requires, and submit it
     * asynchronously.
     *
     * This function must not block.
     *
     * This function gets called with the flying_transfers_lock locked!
     *
     * Return:
     * - 0 on success
     * - LIBUSB_ERROR_NO_DEVICE if the device has been disconnected
     * - another LIBUSB_ERROR code on other failure
     */
    /* Cancel a previously submitted transfer.
     *
     * This function must not block. The transfer cancellation must complete
     * later, resulting in a call to usbi_handle_transfer_cancellation()
     * from the context of handle_events.
     */
    /* Clear a transfer as if it has completed or cancelled, but do not
     * report any completion/cancellation to the library. You should free
     * all private data from the transfer as if you were just about to report
     * completion or cancellation.
     *
     * This function might seem a bit out of place. It is used when libusb
     * detects a disconnected device - it calls this function for all pending
     * transfers before reporting completion (with the disconnect code) to
     * the user. Maybe we can improve upon this internal interface in future.
     */
    /* Handle any pending events on file descriptors. Optional.
     *
     * Provide this function when file descriptors directly indicate device
     * or transfer activity. If your backend does not have such file descriptors,
     * implement the handle_transfer_completion function below.
     *
     * This involves monitoring any active transfers and processing their
     * completion or cancellation.
     *
     * The function is passed an array of pollfd structures (size nfds)
     * as a result of the poll() system call. The num_ready parameter
     * indicates the number of file descriptors that have reported events
     * (i.e. the poll() return value). This should be enough information
     * for you to determine which actions need to be taken on the currently
     * active transfers.
     *
     * For any cancelled transfers, call usbi_handle_transfer_cancellation().
     * For completed transfers, call usbi_handle_transfer_completion().
     * For control/bulk/interrupt transfers, populate the "transferred"
     * element of the appropriate usbi_transfer structure before calling the
     * above functions. For isochronous transfers, populate the status and
     * transferred fields of the iso packet descriptors of the transfer.
     *
     * This function should also be able to detect disconnection of the
     * device, reporting that situation with usbi_handle_disconnect().
     *
     * When processing an event related to a transfer, you probably want to
     * take usbi_transfer.lock to prevent races. See the documentation for
     * the usbi_transfer structure.
     *
     * Return 0 on success, or a LIBUSB_ERROR code on failure.
     */
    /* Handle transfer completion. Optional.
     *
     * Provide this function when there are no file descriptors available
     * that directly indicate device or transfer activity. If your backend does
     * have such file descriptors, implement the handle_events function above.
     *
     * Your backend must tell the library when a transfer has completed by
     * calling usbi_signal_transfer_completion(). You should store any private
     * information about the transfer and its completion status in the transfer's
     * private backend data.
     *
     * During event handling, this function will be called on each transfer for
     * which usbi_signal_transfer_completion() was called.
     *
     * For any cancelled transfers, call usbi_handle_transfer_cancellation().
     * For completed transfers, call usbi_handle_transfer_completion().
     * For control/bulk/interrupt transfers, populate the "transferred"
     * element of the appropriate usbi_transfer structure before calling the
     * above functions. For isochronous transfers, populate the status and
     * transferred fields of the iso packet descriptors of the transfer.
     *
     * Return 0 on success, or a LIBUSB_ERROR code on failure.
     */
    /* Number of bytes to reserve for per-context private backend data.
     * This private data area is accessible by calling
     * usbi_get_context_priv() on the libusb_context instance.
     */
    /* Number of bytes to reserve for per-device private backend data.
     * This private data area is accessible by calling
     * usbi_get_device_priv() on the libusb_device instance.
     */
    /* Number of bytes to reserve for per-handle private backend data.
     * This private data area is accessible by calling
     * usbi_get_device_handle_priv() on the libusb_device_handle instance.
     */
    /* Number of bytes to reserve for per-transfer private backend data.
     * This private data area is accessible by calling
     * usbi_get_transfer_priv() on the usbi_transfer instance.
     */
    #[no_mangle]
    static mut active_contexts_list: list_head;
    #[no_mangle]
    static mut active_contexts_lock: usbi_mutex_static_t;
    #[no_mangle]
    fn usbi_remove_pollfd(ctx: *mut libusb_context, fd: libc::c_int);
    #[no_mangle]
    fn usbi_add_pollfd(
        ctx: *mut libusb_context,
        fd: libc::c_int,
        events: libc::c_short,
    ) -> libc::c_int;
    #[no_mangle]
    fn usbi_disconnect_device(dev: *mut libusb_device);
    #[no_mangle]
    fn usbi_connect_device(dev: *mut libusb_device);
    #[no_mangle]
    fn usbi_handle_transfer_cancellation(itransfer: *mut usbi_transfer) -> libc::c_int;
    #[no_mangle]
    fn usbi_handle_transfer_completion(
        itransfer: *mut usbi_transfer,
        status: libusb_transfer_status,
    ) -> libc::c_int;
    #[no_mangle]
    fn usbi_handle_disconnect(dev_handle: *mut libusb_device_handle);
    #[no_mangle]
    fn usbi_sanitize_device(dev: *mut libusb_device) -> libc::c_int;
    #[no_mangle]
    fn usbi_get_device_by_session_id(
        ctx: *mut libusb_context,
        session_id: libc::c_ulong,
    ) -> *mut libusb_device;
    #[no_mangle]
    fn usbi_alloc_device(ctx: *mut libusb_context, session_id: libc::c_ulong)
        -> *mut libusb_device;
    #[no_mangle]
    fn usbi_log(
        ctx: *mut libusb_context,
        level: libusb_log_level,
        function: *const libc::c_char,
        format: *const libc::c_char,
        _: ...
    );
    #[no_mangle]
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    #[no_mangle]
    fn readlink(__path: *const libc::c_char, __buf: *mut libc::c_char, __len: size_t) -> ssize_t;
    #[no_mangle]
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn linux_udev_start_event_monitor() -> libc::c_int;
    #[no_mangle]
    fn linux_udev_scan_devices(ctx: *mut libusb_context) -> libc::c_int;
    #[no_mangle]
    fn linux_udev_stop_event_monitor() -> libc::c_int;
    #[no_mangle]
    fn linux_udev_hotplug_poll();
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    #[no_mangle]
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    #[no_mangle]
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn asprintf(__ptr: *mut *mut libc::c_char, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    #[no_mangle]
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    #[no_mangle]
    fn uname(__name: *mut utsname) -> libc::c_int;
    #[no_mangle]
    fn statfs(__file: *const libc::c_char, __buf: *mut statfs) -> libc::c_int;
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
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [libc::c_int; 2],
}
pub type __time_t = libc::c_long;
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub b8: [uint8_t; 2],
    pub b16: uint16_t,
}
pub type libusb_descriptor_type = libc::c_uint;
pub const LIBUSB_DT_SS_ENDPOINT_COMPANION: libusb_descriptor_type = 48;
pub const LIBUSB_DT_SUPERSPEED_HUB: libusb_descriptor_type = 42;
pub const LIBUSB_DT_HUB: libusb_descriptor_type = 41;
pub const LIBUSB_DT_PHYSICAL: libusb_descriptor_type = 35;
pub const LIBUSB_DT_REPORT: libusb_descriptor_type = 34;
pub const LIBUSB_DT_HID: libusb_descriptor_type = 33;
pub const LIBUSB_DT_DEVICE_CAPABILITY: libusb_descriptor_type = 16;
pub const LIBUSB_DT_BOS: libusb_descriptor_type = 15;
pub const LIBUSB_DT_ENDPOINT: libusb_descriptor_type = 5;
pub const LIBUSB_DT_INTERFACE: libusb_descriptor_type = 4;
pub const LIBUSB_DT_STRING: libusb_descriptor_type = 3;
pub const LIBUSB_DT_CONFIG: libusb_descriptor_type = 2;
pub const LIBUSB_DT_DEVICE: libusb_descriptor_type = 1;
pub type libusb_endpoint_direction = libc::c_uint;
pub const LIBUSB_ENDPOINT_IN: libusb_endpoint_direction = 128;
pub const LIBUSB_ENDPOINT_OUT: libusb_endpoint_direction = 0;
pub type libusb_standard_request = libc::c_uint;
pub const LIBUSB_SET_ISOCH_DELAY: libusb_standard_request = 49;
pub const LIBUSB_REQUEST_SET_SEL: libusb_standard_request = 48;
pub const LIBUSB_REQUEST_SYNCH_FRAME: libusb_standard_request = 12;
pub const LIBUSB_REQUEST_SET_INTERFACE: libusb_standard_request = 11;
pub const LIBUSB_REQUEST_GET_INTERFACE: libusb_standard_request = 10;
pub const LIBUSB_REQUEST_SET_CONFIGURATION: libusb_standard_request = 9;
pub const LIBUSB_REQUEST_GET_CONFIGURATION: libusb_standard_request = 8;
pub const LIBUSB_REQUEST_SET_DESCRIPTOR: libusb_standard_request = 7;
pub const LIBUSB_REQUEST_GET_DESCRIPTOR: libusb_standard_request = 6;
pub const LIBUSB_REQUEST_SET_ADDRESS: libusb_standard_request = 5;
pub const LIBUSB_REQUEST_SET_FEATURE: libusb_standard_request = 3;
pub const LIBUSB_REQUEST_CLEAR_FEATURE: libusb_standard_request = 1;
pub const LIBUSB_REQUEST_GET_STATUS: libusb_standard_request = 0;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct libusb_control_setup {
    pub bmRequestType: uint8_t,
    pub bRequest: uint8_t,
    pub wValue: uint16_t,
    pub wIndex: uint16_t,
    pub wLength: uint16_t,
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
pub type libusb_speed = libc::c_uint;
pub const LIBUSB_SPEED_SUPER_PLUS: libusb_speed = 5;
pub const LIBUSB_SPEED_SUPER: libusb_speed = 4;
pub const LIBUSB_SPEED_HIGH: libusb_speed = 3;
pub const LIBUSB_SPEED_FULL: libusb_speed = 2;
pub const LIBUSB_SPEED_LOW: libusb_speed = 1;
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
pub type libusb_transfer_type = libc::c_uint;
pub const LIBUSB_TRANSFER_TYPE_BULK_STREAM: libusb_transfer_type = 4;
pub const LIBUSB_TRANSFER_TYPE_INTERRUPT: libusb_transfer_type = 3;
pub const LIBUSB_TRANSFER_TYPE_BULK: libusb_transfer_type = 2;
pub const LIBUSB_TRANSFER_TYPE_ISOCHRONOUS: libusb_transfer_type = 1;
pub const LIBUSB_TRANSFER_TYPE_CONTROL: libusb_transfer_type = 0;
pub type libusb_transfer_status = libc::c_uint;
pub const LIBUSB_TRANSFER_OVERFLOW: libusb_transfer_status = 6;
pub const LIBUSB_TRANSFER_NO_DEVICE: libusb_transfer_status = 5;
pub const LIBUSB_TRANSFER_STALL: libusb_transfer_status = 4;
pub const LIBUSB_TRANSFER_CANCELLED: libusb_transfer_status = 3;
pub const LIBUSB_TRANSFER_TIMED_OUT: libusb_transfer_status = 2;
pub const LIBUSB_TRANSFER_ERROR: libusb_transfer_status = 1;
pub const LIBUSB_TRANSFER_COMPLETED: libusb_transfer_status = 0;
pub type libusb_transfer_flags = libc::c_uint;
pub const LIBUSB_TRANSFER_ADD_ZERO_PACKET: libusb_transfer_flags = 8;
pub const LIBUSB_TRANSFER_FREE_TRANSFER: libusb_transfer_flags = 4;
pub const LIBUSB_TRANSFER_FREE_BUFFER: libusb_transfer_flags = 2;
pub const LIBUSB_TRANSFER_SHORT_NOT_OK: libusb_transfer_flags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct libusb_iso_packet_descriptor {
    pub length: libc::c_uint,
    pub actual_length: libc::c_uint,
    pub status: libusb_transfer_status,
}
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
pub type libusb_transfer_cb_fn = Option<unsafe extern "C" fn(_: *mut libusb_transfer) -> ()>;
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
pub type usbi_mutex_static_t = pthread_mutex_t;
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
#[repr(C, packed)]
pub struct usbi_descriptor_header {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct usbi_device_descriptor {
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct usbi_configuration_descriptor {
    pub bLength: uint8_t,
    pub bDescriptorType: uint8_t,
    pub wTotalLength: uint16_t,
    pub bNumInterfaces: uint8_t,
    pub bConfigurationValue: uint8_t,
    pub iConfiguration: uint8_t,
    pub bmAttributes: uint8_t,
    pub bMaxPower: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct discovered_devs {
    pub len: size_t,
    pub capacity: size_t,
    pub devices: [*mut libusb_device; 0],
}
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
pub struct linux_transfer_priv {
    pub c2rust_unnamed: C2RustUnnamed_4,
    pub reap_action: reap_action,
    pub num_urbs: libc::c_int,
    pub num_retired: libc::c_int,
    pub reap_status: libusb_transfer_status,
    pub iso_packet_offset: libc::c_int,
}
pub type reap_action = libc::c_uint;
/* one or more urbs encountered a low-level error */
pub const ERROR: reap_action = 4;
/* completed multi-URB transfer in non-final URB */
pub const COMPLETED_EARLY: reap_action = 3;
/* cancelled by user or timeout */
pub const CANCELLED: reap_action = 2;
/* submission failed after the first URB, so await cancellation/completion
 * of all the others */
pub const SUBMIT_FAILED: reap_action = 1;
pub const NORMAL: reap_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub urbs: *mut usbfs_urb,
    pub iso_urbs: *mut *mut usbfs_urb,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbfs_urb {
    pub type_0: libc::c_uchar,
    pub endpoint: libc::c_uchar,
    pub status: libc::c_int,
    pub flags: libc::c_uint,
    pub buffer: *mut libc::c_void,
    pub buffer_length: libc::c_int,
    pub actual_length: libc::c_int,
    pub start_frame: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub error_count: libc::c_int,
    pub signr: libc::c_uint,
    pub usercontext: *mut libc::c_void,
    pub iso_frame_desc: [usbfs_iso_packet_desc; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbfs_iso_packet_desc {
    pub length: libc::c_uint,
    pub actual_length: libc::c_uint,
    pub status: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub number_of_packets: libc::c_int,
    pub stream_id: libc::c_uint,
}
/* cache val for !sysfs_available  */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_device_handle_priv {
    pub fd: libc::c_int,
    pub fd_removed: libc::c_int,
    pub fd_keep: libc::c_int,
    pub caps: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linux_device_priv {
    pub sysfs_dir: *mut libc::c_char,
    pub descriptors: *mut libc::c_void,
    pub descriptors_len: size_t,
    pub config_descriptors: *mut config_descriptor,
    pub active_config: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_descriptor {
    pub desc: *mut usbi_configuration_descriptor,
    pub actual_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbfs_ioctl {
    pub ifno: libc::c_int,
    pub ioctl_code: libc::c_int,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbfs_getdriver {
    pub interface: libc::c_uint,
    pub driver: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbfs_streams {
    pub num_streams: libc::c_uint,
    pub num_eps: libc::c_uint,
    pub eps: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbfs_disconnect_claim {
    pub interface: libc::c_uint,
    pub flags: libc::c_uint,
    pub driver: [libc::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbfs_setinterface {
    pub interface: libc::c_uint,
    pub altsetting: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbfs_ctrltransfer {
    pub bmRequestType: __u8,
    pub bRequest: __u8,
    pub wValue: __u16,
    pub wIndex: __u16,
    pub wLength: __u16,
    pub timeout: __u32,
    pub data: *mut libc::c_void,
}
pub type __u32 = libc::c_uint;
pub type __u16 = libc::c_ushort;
pub type __u8 = libc::c_uchar;
pub const _ISdigit: C2RustUnnamed_6 = 2048;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usbfs_connectinfo {
    pub devnum: libc::c_uint,
    pub slow: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kernel_version {
    pub major: libc::c_int,
    pub minor: libc::c_int,
    pub sublevel: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
pub type C2RustUnnamed_6 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_6 = 8;
pub const _ISpunct: C2RustUnnamed_6 = 4;
pub const _IScntrl: C2RustUnnamed_6 = 2;
pub const _ISblank: C2RustUnnamed_6 = 1;
pub const _ISgraph: C2RustUnnamed_6 = 32768;
pub const _ISprint: C2RustUnnamed_6 = 16384;
pub const _ISspace: C2RustUnnamed_6 = 8192;
pub const _ISxdigit: C2RustUnnamed_6 = 4096;
pub const _ISalpha: C2RustUnnamed_6 = 1024;
pub const _ISlower: C2RustUnnamed_6 = 512;
pub const _ISupper: C2RustUnnamed_6 = 256;
#[inline]
unsafe extern "C" fn libusb_cpu_to_le16(x: uint16_t) -> uint16_t {
    let mut _tmp: C2RustUnnamed_3 = C2RustUnnamed_3 { b8: [0; 2] };
    _tmp.b8[1 as libc::c_int as usize] = (x as libc::c_int >> 8 as libc::c_int) as uint8_t;
    _tmp.b8[0 as libc::c_int as usize] = (x as libc::c_int & 0xff as libc::c_int) as uint8_t;
    return _tmp.b16;
}
#[inline]
unsafe extern "C" fn usbi_get_transfer_priv(itransfer: *mut usbi_transfer) -> *mut libc::c_void {
    return (*itransfer).priv_0;
}
#[inline]
unsafe extern "C" fn usbi_get_device_handle_priv(
    dev_handle: *mut libusb_device_handle,
) -> *mut libc::c_void {
    return (dev_handle as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<libusb_device_handle>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn usbi_get_device_priv(dev: *mut libusb_device) -> *mut libc::c_void {
    return (dev as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<libusb_device>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn usbi_localize_device_descriptor(mut desc: *mut libusb_device_descriptor) {
    (*desc).bcdUSB = libusb_cpu_to_le16((*desc).bcdUSB);
    (*desc).idVendor = libusb_cpu_to_le16((*desc).idVendor);
    (*desc).idProduct = libusb_cpu_to_le16((*desc).idProduct);
    (*desc).bcdDevice = libusb_cpu_to_le16((*desc).bcdDevice);
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
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn usbi_mutex_static_lock(mutex: *mut usbi_mutex_static_t) {
    pthread_mutex_lock(mutex);
}
#[inline]
unsafe extern "C" fn usbi_mutex_static_unlock(mutex: *mut usbi_mutex_static_t) {
    pthread_mutex_unlock(mutex);
}
#[inline]
unsafe extern "C" fn usbi_mutex_lock(mutex: *mut usbi_mutex_t) {
    pthread_mutex_lock(mutex);
}
#[inline]
unsafe extern "C" fn usbi_mutex_unlock(mutex: *mut usbi_mutex_t) {
    pthread_mutex_unlock(mutex);
}
#[inline]
unsafe extern "C" fn linux_start_event_monitor() -> libc::c_int {
    return linux_udev_start_event_monitor();
}
#[inline]
unsafe extern "C" fn linux_stop_event_monitor() {
    linux_udev_stop_event_monitor();
}
#[inline]
unsafe extern "C" fn linux_hotplug_poll() {
    linux_udev_hotplug_poll();
}
/* use usbdev*.* device names in /dev instead of the usbfs bus directories */
static mut usbdev_names: libc::c_int = 0 as libc::c_int;
/* Linux has changed the maximum length of an individual isochronous packet
 * over time.  Initially this limit was 1,023 bytes, but Linux 2.6.18
 * (commit 3612242e527eb47ee4756b5350f8bdf791aa5ede) increased this value to
 * 8,192 bytes to support higher bandwidth devices.  Linux 3.10
 * (commit e2e2f0ea1c935edcf53feb4c4c8fdb4f86d57dd9) further increased this
 * value to 49,152 bytes to support super speed devices.  Linux 5.2
 * (commit 8a1dbc8d91d3d1602282c7e6b4222c7759c916fa) even further increased
 * this value to 98,304 bytes to support super speed plus devices.
 */
static mut max_iso_packet_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
/* is sysfs available (mounted) ? */
static mut sysfs_available: libc::c_int = -(1 as libc::c_int);
/* how many times have we initted (and not exited) ? */
static mut init_count: libc::c_int = 0 as libc::c_int;
/* Serialize hotplug start/stop */
static mut linux_hotplug_startstop_lock: usbi_mutex_static_t = pthread_mutex_t {
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
/* Serialize scan-devices, event-thread, and poll */
#[no_mangle]
pub static mut linux_hotplug_lock: usbi_mutex_static_t = pthread_mutex_t {
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
                }; /* Success */
                init
            },
        };
        init
    },
};
unsafe extern "C" fn get_usbfs_fd(
    dev: *mut libusb_device,
    mode: mode_t,
    silent: libc::c_int,
) -> libc::c_int {
    let ctx: *mut libusb_context = (*dev).ctx;
    let mut path: [libc::c_char; 24] = [0; 24];
    let mut fd: libc::c_int = 0;
    if usbdev_names != 0 {
        sprintf(
            path.as_mut_ptr(),
            b"/dev/usbdev%u.%u\x00" as *const u8 as *const libc::c_char,
            (*dev).bus_number as libc::c_int,
            (*dev).device_address as libc::c_int,
        );
    } else {
        sprintf(
            path.as_mut_ptr(),
            b"/dev/bus/usb/%03u/%03u\x00" as *const u8 as *const libc::c_char,
            (*dev).bus_number as libc::c_int,
            (*dev).device_address as libc::c_int,
        );
    }
    fd = open(
        path.as_mut_ptr(),
        (mode | 0o2000000 as libc::c_int as libc::c_uint) as libc::c_int,
    );
    if fd != -(1 as libc::c_int) {
        return fd;
    }
    if *__errno_location() == 2 as libc::c_int {
        let delay_ms: libc::c_long = 10 as libc::c_long;
        let delay_ts: timespec = {
            let init = timespec {
                tv_sec: 0 as libc::c_long,
                tv_nsec: delay_ms * 1000 as libc::c_long * 1000 as libc::c_long,
            };
            init
        };
        if silent == 0 {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"get_usbfs_fd\x00"))
                    .as_ptr(),
                b"File doesn\'t exist, wait %ld ms and try again\x00" as *const u8
                    as *const libc::c_char,
                delay_ms,
            );
        }
        /* Success */
        nanosleep(&delay_ts, 0 as *mut timespec);
        fd = open(
            path.as_mut_ptr(),
            (mode | 0o2000000 as libc::c_int as libc::c_uint) as libc::c_int,
        );
        if fd != -(1 as libc::c_int) {
            return fd;
        }
    }
    if silent == 0 {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"get_usbfs_fd\x00"))
                .as_ptr(),
            b"libusb couldn\'t open USB device %s, errno=%d\x00" as *const u8
                as *const libc::c_char,
            path.as_mut_ptr(),
            *__errno_location(),
        );
        if *__errno_location() == 13 as libc::c_int && mode == 0o2 as libc::c_int as libc::c_uint {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"get_usbfs_fd\x00"))
                    .as_ptr(),
                b"libusb requires write access to USB device nodes\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    if *__errno_location() == 13 as libc::c_int {
        return LIBUSB_ERROR_ACCESS as libc::c_int;
    }
    if *__errno_location() == 2 as libc::c_int {
        return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
    }
    return LIBUSB_ERROR_IO as libc::c_int;
}
/* Wait 10ms for USB device path creation.*/
/* check dirent for a /dev/usbdev%d.%d name
 * optionally return bus/device on success */
unsafe extern "C" fn is_usbdev_entry(
    name: *const libc::c_char,
    bus_p: *mut uint8_t,
    dev_p: *mut uint8_t,
) -> libc::c_int {
    let mut busnum: libc::c_int = 0;
    let mut devnum: libc::c_int = 0;
    if sscanf(
        name,
        b"usbdev%d.%d\x00" as *const u8 as *const libc::c_char,
        &mut busnum as *mut libc::c_int,
        &mut devnum as *mut libc::c_int,
    ) != 2 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if busnum < 0 as libc::c_int
        || busnum > 255 as libc::c_int
        || devnum < 0 as libc::c_int
        || devnum > 255 as libc::c_int
    {
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"is_usbdev_entry\x00"))
                .as_ptr(),
            b"invalid usbdev format \'%s\'\x00" as *const u8 as *const libc::c_char,
            name,
        );
        return 0 as libc::c_int;
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"is_usbdev_entry\x00")).as_ptr(),
        b"found: %s\x00" as *const u8 as *const libc::c_char,
        name,
    );
    if !bus_p.is_null() {
        *bus_p = busnum as uint8_t
    }
    if !dev_p.is_null() {
        *dev_p = devnum as uint8_t
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn find_usbfs_path() -> *const libc::c_char {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    path = b"/dev/bus/usb\x00" as *const u8 as *const libc::c_char;
    dir = opendir(path);
    if !dir.is_null() {
        loop {
            entry = readdir(dir);
            if entry.is_null() {
                break;
            }
            if !((*entry).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32) {
                break;
            }
        }
        closedir(dir);
        if !entry.is_null() {
            return path;
        }
    }
    /* look for /dev/usbdev*.* if the normal place fails */
    path = b"/dev\x00" as *const u8 as *const libc::c_char;
    dir = opendir(path);
    if !dir.is_null() {
        loop {
            entry = readdir(dir);
            if entry.is_null() {
                break;
            }
            if (*entry).d_name[0 as libc::c_int as usize] as libc::c_int == '.' as i32 {
                continue;
            }
            if is_usbdev_entry(
                (*entry).d_name.as_mut_ptr(),
                0 as *mut uint8_t,
                0 as *mut uint8_t,
            ) != 0
            {
                break;
            }
        }
        closedir(dir);
        if !entry.is_null() {
            usbdev_names = 1 as libc::c_int;
            return path;
        }
    }
    /* On udev based systems without any usb-devices /dev/bus/usb will not
     * exist. So if we've not found anything and we're using udev for hotplug
     * simply assume /dev/bus/usb rather then making libusb_init fail.
     * Make the same assumption for Android where SELinux policies might block us
     * from reading /dev on newer devices. */
    return b"/dev/bus/usb\x00" as *const u8 as *const libc::c_char;
}
unsafe extern "C" fn get_kernel_version(
    ctx: *mut libusb_context,
    mut ver: *mut kernel_version,
) -> libc::c_int {
    let mut uts: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    let mut atoms: libc::c_int = 0;
    if uname(&mut uts) < 0 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"get_kernel_version\x00"))
                .as_ptr(),
            b"uname failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return -(1 as libc::c_int);
    }
    atoms = sscanf(
        uts.release.as_mut_ptr(),
        b"%d.%d.%d\x00" as *const u8 as *const libc::c_char,
        &mut (*ver).major as *mut libc::c_int,
        &mut (*ver).minor as *mut libc::c_int,
        &mut (*ver).sublevel as *mut libc::c_int,
    );
    if atoms < 2 as libc::c_int {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"get_kernel_version\x00"))
                .as_ptr(),
            b"failed to parse uname release \'%s\'\x00" as *const u8 as *const libc::c_char,
            uts.release.as_mut_ptr(),
        );
        return -(1 as libc::c_int);
    }
    if atoms < 3 as libc::c_int {
        (*ver).sublevel = -(1 as libc::c_int)
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"get_kernel_version\x00"))
            .as_ptr(),
        b"reported kernel version is %s\x00" as *const u8 as *const libc::c_char,
        uts.release.as_mut_ptr(),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn kernel_version_ge(
    ver: *const kernel_version,
    major: libc::c_int,
    minor: libc::c_int,
    sublevel: libc::c_int,
) -> libc::c_int {
    if (*ver).major > major {
        return 1 as libc::c_int;
    } else {
        if (*ver).major < major {
            return 0 as libc::c_int;
        }
    }
    /* kmajor == major */
    if (*ver).minor > minor {
        return 1 as libc::c_int;
    } else {
        if (*ver).minor < minor {
            return 0 as libc::c_int;
        }
    }
    /* kminor == minor */
    if (*ver).sublevel == -(1 as libc::c_int) {
        return (sublevel == 0 as libc::c_int) as libc::c_int;
    }
    return ((*ver).sublevel >= sublevel) as libc::c_int;
}
unsafe extern "C" fn op_init(ctx: *mut libusb_context) -> libc::c_int {
    let mut kversion: kernel_version = kernel_version {
        major: 0,
        minor: 0,
        sublevel: 0,
    };
    let mut usbfs_path: *const libc::c_char = 0 as *const libc::c_char;
    let mut r: libc::c_int = 0;
    if get_kernel_version(ctx, &mut kversion) < 0 as libc::c_int {
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    if kernel_version_ge(
        &mut kversion,
        2 as libc::c_int,
        6 as libc::c_int,
        32 as libc::c_int,
    ) == 0
    {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"op_init\x00")).as_ptr(),
            b"kernel version is too old (reported as %d.%d.%d)\x00" as *const u8
                as *const libc::c_char,
            kversion.major,
            kversion.minor,
            if kversion.sublevel != -(1 as libc::c_int) {
                kversion.sublevel
            } else {
                0 as libc::c_int
            },
        );
        return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
    }
    usbfs_path = find_usbfs_path();
    if usbfs_path.is_null() {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"op_init\x00")).as_ptr(),
            b"could not find usbfs\x00" as *const u8 as *const libc::c_char,
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"op_init\x00")).as_ptr(),
        b"found usbfs at %s\x00" as *const u8 as *const libc::c_char,
        usbfs_path,
    );
    if max_iso_packet_len == 0 {
        if kernel_version_ge(
            &mut kversion,
            5 as libc::c_int,
            2 as libc::c_int,
            0 as libc::c_int,
        ) != 0
        {
            max_iso_packet_len = 98304 as libc::c_int as libc::c_uint
        } else if kernel_version_ge(
            &mut kversion,
            3 as libc::c_int,
            10 as libc::c_int,
            0 as libc::c_int,
        ) != 0
        {
            max_iso_packet_len = 49152 as libc::c_int as libc::c_uint
        } else {
            max_iso_packet_len = 8192 as libc::c_int as libc::c_uint
        }
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"op_init\x00")).as_ptr(),
        b"max iso packet length is (likely) %u bytes\x00" as *const u8 as *const libc::c_char,
        max_iso_packet_len,
    );
    if sysfs_available == -(1 as libc::c_int) {
        let mut statfsbuf: statfs = statfs {
            f_type: 0,
            f_bsize: 0,
            f_blocks: 0,
            f_bfree: 0,
            f_bavail: 0,
            f_files: 0,
            f_ffree: 0,
            f_fsid: __fsid_t { __val: [0; 2] },
            f_namelen: 0,
            f_frsize: 0,
            f_flags: 0,
            f_spare: [0; 4],
        };
        r = statfs(
            b"/sys\x00" as *const u8 as *const libc::c_char,
            &mut statfsbuf,
        );
        if r == 0 as libc::c_int && statfsbuf.f_type == 0x62656572 as libc::c_int as libc::c_long {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"op_init\x00")).as_ptr(),
                b"sysfs is available\x00" as *const u8 as *const libc::c_char,
            );
            sysfs_available = 1 as libc::c_int
        } else {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"op_init\x00")).as_ptr(),
                b"sysfs not mounted\x00" as *const u8 as *const libc::c_char,
            );
            sysfs_available = 0 as libc::c_int
        }
    }
    usbi_mutex_static_lock(&mut linux_hotplug_startstop_lock);
    r = LIBUSB_SUCCESS as libc::c_int;
    if init_count == 0 as libc::c_int {
        /* start up hotplug event handler */
        r = linux_start_event_monitor()
    }
    if r == LIBUSB_SUCCESS as libc::c_int {
        r = linux_scan_devices(ctx);
        if r == LIBUSB_SUCCESS as libc::c_int {
            init_count += 1
        } else if init_count == 0 as libc::c_int {
            linux_stop_event_monitor();
        }
    } else {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"op_init\x00")).as_ptr(),
            b"error starting hotplug event monitor\x00" as *const u8 as *const libc::c_char,
        );
    }
    usbi_mutex_static_unlock(&mut linux_hotplug_startstop_lock);
    return r;
}
unsafe extern "C" fn op_exit(_ctx: *mut libusb_context) {
    usbi_mutex_static_lock(&mut linux_hotplug_startstop_lock);
    if init_count != 0 as libc::c_int {
    } else {
        __assert_fail(
            b"init_count != 0\x00" as *const u8 as *const libc::c_char,
            b"/home/konstantin/Projects/tests/rust/libusb2rs/libusb/libusb/os/linux_usbfs.c\x00"
                as *const u8 as *const libc::c_char,
            407 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 38], &[libc::c_char; 38]>(
                b"void op_exit(struct libusb_context *)\x00",
            ))
            .as_ptr(),
        );
    }
    init_count -= 1;
    if init_count == 0 {
        /* tear down event handler */
        linux_stop_event_monitor();
    }
    usbi_mutex_static_unlock(&mut linux_hotplug_startstop_lock);
}
unsafe extern "C" fn linux_scan_devices(ctx: *mut libusb_context) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    usbi_mutex_static_lock(&mut linux_hotplug_lock);
    ret = linux_udev_scan_devices(ctx);
    usbi_mutex_static_unlock(&mut linux_hotplug_lock);
    return ret;
}
unsafe extern "C" fn op_hotplug_poll() {
    linux_hotplug_poll();
}
unsafe extern "C" fn open_sysfs_attr(
    ctx: *mut libusb_context,
    sysfs_dir: *const libc::c_char,
    attr: *const libc::c_char,
) -> libc::c_int {
    let mut filename: [libc::c_char; 256] = [0; 256];
    let mut fd: libc::c_int = 0;
    snprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        b"/sys/bus/usb/devices/%s/%s\x00" as *const u8 as *const libc::c_char,
        sysfs_dir,
        attr,
    );
    fd = open(
        filename.as_mut_ptr(),
        0 as libc::c_int | 0o2000000 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int {
            /* File doesn't exist. Assume the device has been
            disconnected (see trac ticket #70). */
            return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
        }
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"open_sysfs_attr\x00"))
                .as_ptr(),
            b"open %s failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
            *__errno_location(),
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    return fd;
}
/* Note only suitable for attributes which always read >= 0, < 0 is error */
unsafe extern "C" fn read_sysfs_attr(
    ctx: *mut libusb_context,
    sysfs_dir: *const libc::c_char,
    attr: *const libc::c_char,
    max_value: libc::c_int,
    value_p: *mut libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 20] = [0; 20];
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: libc::c_long = 0;
    let mut r: ssize_t = 0;
    let mut fd: libc::c_int = 0;
    fd = open_sysfs_attr(ctx, sysfs_dir, attr);
    if fd < 0 as libc::c_int {
        return fd;
    }
    r = read(
        fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
    );
    if r < 0 as libc::c_int as libc::c_long {
        r = *__errno_location() as ssize_t;
        close(fd);
        if r == 19 as libc::c_int as libc::c_long {
            return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
        }
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"read_sysfs_attr\x00"))
                .as_ptr(),
            b"attribute %s read failed, errno=%zd\x00" as *const u8 as *const libc::c_char,
            attr,
            r,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    close(fd);
    if r == 0 as libc::c_int as libc::c_long {
        /* Certain attributes (e.g. bConfigurationValue) are not
         * populated if the device is not configured. */
        *value_p = -(1 as libc::c_int);
        return 0 as libc::c_int;
    }
    /* The kernel does *not* NULL-terminate the string, but every attribute
     * should be terminated with a newline character. */
    if *(*__ctype_b_loc()).offset(buf[0 as libc::c_int as usize] as libc::c_int as isize)
        as libc::c_int
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"read_sysfs_attr\x00"))
                .as_ptr(),
            b"attribute %s doesn\'t have numeric value?\x00" as *const u8 as *const libc::c_char,
            attr,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    } else {
        if buf[(r - 1 as libc::c_int as libc::c_long) as usize] as libc::c_int != '\n' as i32 {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"read_sysfs_attr\x00"))
                    .as_ptr(),
                b"attribute %s doesn\'t end with newline?\x00" as *const u8 as *const libc::c_char,
                attr,
            );
            return LIBUSB_ERROR_IO as libc::c_int;
        }
    }
    buf[(r - 1 as libc::c_int as libc::c_long) as usize] = '\u{0}' as i32 as libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    value = strtol(buf.as_mut_ptr(), &mut endptr, 10 as libc::c_int);
    if value < 0 as libc::c_int as libc::c_long
        || value > max_value as libc::c_long
        || *__errno_location() != 0
    {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"read_sysfs_attr\x00"))
                .as_ptr(),
            b"attribute %s contains an invalid value: \'%s\'\x00" as *const u8
                as *const libc::c_char,
            attr,
            buf.as_mut_ptr(),
        );
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    } else {
        if *endptr as libc::c_int != '\u{0}' as i32 {
            /* Consider the value to be valid if the remainder is a '.'
             * character followed by numbers.  This occurs, for example,
             * when reading the "speed" attribute for a low-speed device
             * (e.g. "1.5") */
            if *endptr as libc::c_int == '.' as i32
                && *(*__ctype_b_loc())
                    .offset(*endptr.offset(1 as libc::c_int as isize) as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
            {
                endptr = endptr.offset(1);
                while *(*__ctype_b_loc()).offset(*endptr as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                {
                    endptr = endptr.offset(1)
                }
            }
            if *endptr as libc::c_int != '\u{0}' as i32 {
                usbi_log(
                    ctx,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                        b"read_sysfs_attr\x00",
                    ))
                    .as_ptr(),
                    b"attribute %s contains an invalid value: \'%s\'\x00" as *const u8
                        as *const libc::c_char,
                    attr,
                    buf.as_mut_ptr(),
                );
                return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
            }
        }
    }
    *value_p = value as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn sysfs_scan_device(
    ctx: *mut libusb_context,
    devname: *const libc::c_char,
) -> libc::c_int {
    let mut busnum: uint8_t = 0;
    let mut devaddr: uint8_t = 0;
    let mut ret: libc::c_int = 0;
    ret = linux_get_device_address(
        ctx,
        0 as libc::c_int,
        &mut busnum,
        &mut devaddr,
        0 as *const libc::c_char,
        devname,
        -(1 as libc::c_int),
    );
    if ret != LIBUSB_SUCCESS as libc::c_int {
        return ret;
    }
    return linux_enumerate_device(ctx, busnum, devaddr, devname);
}
/* read the bConfigurationValue for a device */
unsafe extern "C" fn sysfs_get_active_config(
    dev: *mut libusb_device,
    config: *mut uint8_t,
) -> libc::c_int {
    let priv_0: *mut linux_device_priv = usbi_get_device_priv(dev) as *mut linux_device_priv; /* unconfigured */
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    ret = read_sysfs_attr(
        (*dev).ctx,
        (*priv_0).sysfs_dir,
        b"bConfigurationValue\x00" as *const u8 as *const libc::c_char,
        255 as libc::c_int,
        &mut tmp,
    );
    if ret < 0 as libc::c_int {
        return ret;
    }
    if tmp == -(1 as libc::c_int) {
        tmp = 0 as libc::c_int
    }
    *config = tmp as uint8_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn linux_get_device_address(
    ctx: *mut libusb_context,
    detached: libc::c_int,
    busnum: *mut uint8_t,
    devaddr: *mut uint8_t,
    mut dev_node: *const libc::c_char,
    sys_name: *const libc::c_char,
    fd: libc::c_int,
) -> libc::c_int {
    let mut sysfs_val: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"linux_get_device_address\x00"))
            .as_ptr(),
        b"getting address for device: %s detached: %d\x00" as *const u8 as *const libc::c_char,
        sys_name,
        detached,
    );
    /* can't use sysfs to read the bus and device number if the
     * device has been detached */
    if sysfs_available == 0 || detached != 0 || sys_name.is_null() {
        if dev_node.is_null() && fd >= 0 as libc::c_int {
            let mut fresh0 =
                ::std::vec::from_elem(0, 4096 as libc::c_int as libc::c_ulong as usize);
            let fd_path: *mut libc::c_char = fresh0.as_mut_ptr() as *mut libc::c_char;
            let mut proc_path: [libc::c_char; 32] = [0; 32];
            /* try to retrieve the device node from fd */
            sprintf(
                proc_path.as_mut_ptr(),
                b"/proc/self/fd/%d\x00" as *const u8 as *const libc::c_char,
                fd,
            );
            r = readlink(
                proc_path.as_mut_ptr(),
                fd_path,
                (4096 as libc::c_int - 1 as libc::c_int) as size_t,
            ) as libc::c_int;
            if r > 0 as libc::c_int {
                *fd_path.offset(r as isize) = '\u{0}' as i32 as libc::c_char;
                dev_node = fd_path
            }
        }
        if dev_node.is_null() {
            return LIBUSB_ERROR_OTHER as libc::c_int;
        }
        /* will this work with all supported kernel versions? */
        if strncmp(
            dev_node,
            b"/dev/bus/usb\x00" as *const u8 as *const libc::c_char,
            12 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            sscanf(
                dev_node,
                b"/dev/bus/usb/%hhu/%hhu\x00" as *const u8 as *const libc::c_char,
                busnum,
                devaddr,
            );
        } else {
            return LIBUSB_ERROR_OTHER as libc::c_int;
        }
        return LIBUSB_SUCCESS as libc::c_int;
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"linux_get_device_address\x00"))
            .as_ptr(),
        b"scan %s\x00" as *const u8 as *const libc::c_char,
        sys_name,
    );
    r = read_sysfs_attr(
        ctx,
        sys_name,
        b"busnum\x00" as *const u8 as *const libc::c_char,
        255 as libc::c_int,
        &mut sysfs_val,
    );
    if r < 0 as libc::c_int {
        return r;
    }
    *busnum = sysfs_val as uint8_t;
    r = read_sysfs_attr(
        ctx,
        sys_name,
        b"devnum\x00" as *const u8 as *const libc::c_char,
        255 as libc::c_int,
        &mut sysfs_val,
    );
    if r < 0 as libc::c_int {
        return r;
    }
    *devaddr = sysfs_val as uint8_t;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(b"linux_get_device_address\x00"))
            .as_ptr(),
        b"bus=%u dev=%u\x00" as *const u8 as *const libc::c_char,
        *busnum as libc::c_int,
        *devaddr as libc::c_int,
    );
    return LIBUSB_SUCCESS as libc::c_int;
}
/* Return offset of the next config descriptor */
unsafe extern "C" fn seek_to_next_config(
    ctx: *mut libusb_context,
    mut buffer: *mut uint8_t,
    mut len: size_t,
) -> libc::c_int {
    let mut header: *mut usbi_descriptor_header = 0 as *mut usbi_descriptor_header; /* no configurations? */
    let mut offset: libc::c_int = 0 as libc::c_int;
    while len > 0 as libc::c_int as libc::c_ulong {
        if len < 2 as libc::c_int as libc::c_ulong {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                    b"seek_to_next_config\x00",
                ))
                .as_ptr(),
                b"short descriptor read %zu/2\x00" as *const u8 as *const libc::c_char,
                len,
            );
            return LIBUSB_ERROR_IO as libc::c_int;
        }
        header = buffer as *mut usbi_descriptor_header;
        if (*header).bDescriptorType as libc::c_int == LIBUSB_DT_CONFIG as libc::c_int {
            return offset;
        }
        if len < (*header).bLength as libc::c_ulong {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                    b"seek_to_next_config\x00",
                ))
                .as_ptr(),
                b"bLength overflow by %zu bytes\x00" as *const u8 as *const libc::c_char,
                ((*header).bLength as size_t).wrapping_sub(len),
            );
            return LIBUSB_ERROR_IO as libc::c_int;
        }
        offset += (*header).bLength as libc::c_int;
        buffer = buffer.offset((*header).bLength as libc::c_int as isize);
        len = (len as libc::c_ulong).wrapping_sub((*header).bLength as libc::c_ulong) as size_t
            as size_t
    }
    usbi_log(
        ctx,
        LIBUSB_LOG_LEVEL_ERROR,
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"seek_to_next_config\x00"))
            .as_ptr(),
        b"config descriptor not found\x00" as *const u8 as *const libc::c_char,
    );
    return LIBUSB_ERROR_IO as libc::c_int;
}
unsafe extern "C" fn parse_config_descriptors(dev: *mut libusb_device) -> libc::c_int {
    let ctx: *mut libusb_context = (*dev).ctx;
    let mut priv_0: *mut linux_device_priv = usbi_get_device_priv(dev) as *mut linux_device_priv;
    let mut device_desc: *mut usbi_device_descriptor = 0 as *mut usbi_device_descriptor;
    let mut idx: uint8_t = 0;
    let mut num_configs: uint8_t = 0;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut remaining: size_t = 0;
    device_desc = (*priv_0).descriptors as *mut usbi_device_descriptor;
    num_configs = (*device_desc).bNumConfigurations;
    if num_configs as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    (*priv_0).config_descriptors = malloc(
        (num_configs as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<config_descriptor>() as libc::c_ulong),
    ) as *mut config_descriptor;
    if (*priv_0).config_descriptors.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    buffer = (*priv_0).descriptors.offset(18 as libc::c_int as isize) as *mut uint8_t;
    remaining = (*priv_0)
        .descriptors_len
        .wrapping_sub(18 as libc::c_int as libc::c_ulong);
    idx = 0 as libc::c_int as uint8_t;
    while (idx as libc::c_int) < num_configs as libc::c_int {
        let mut config_desc: *mut usbi_configuration_descriptor =
            0 as *mut usbi_configuration_descriptor;
        let mut config_len: uint16_t = 0;
        if remaining < 9 as libc::c_int as libc::c_ulong {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                    b"parse_config_descriptors\x00",
                ))
                .as_ptr(),
                b"short descriptor read %zu/%d\x00" as *const u8 as *const libc::c_char,
                remaining,
                9 as libc::c_int,
            );
            return LIBUSB_ERROR_IO as libc::c_int;
        }
        config_desc = buffer as *mut usbi_configuration_descriptor;
        if (*config_desc).bDescriptorType as libc::c_int != LIBUSB_DT_CONFIG as libc::c_int {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                    b"parse_config_descriptors\x00",
                ))
                .as_ptr(),
                b"descriptor is not a config desc (type 0x%02x)\x00" as *const u8
                    as *const libc::c_char,
                (*config_desc).bDescriptorType as libc::c_int,
            );
            return LIBUSB_ERROR_IO as libc::c_int;
        } else {
            if ((*config_desc).bLength as libc::c_int) < 9 as libc::c_int {
                usbi_log(
                    ctx,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                        b"parse_config_descriptors\x00",
                    ))
                    .as_ptr(),
                    b"invalid descriptor bLength %u\x00" as *const u8 as *const libc::c_char,
                    (*config_desc).bLength as libc::c_int,
                );
                return LIBUSB_ERROR_IO as libc::c_int;
            }
        }
        config_len = libusb_cpu_to_le16((*config_desc).wTotalLength);
        if (config_len as libc::c_int) < 9 as libc::c_int {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                    b"parse_config_descriptors\x00",
                ))
                .as_ptr(),
                b"invalid wTotalLength %u\x00" as *const u8 as *const libc::c_char,
                config_len as libc::c_int,
            );
            return LIBUSB_ERROR_IO as libc::c_int;
        }
        if !(*priv_0).sysfs_dir.is_null() {
            /*
             * In sysfs wTotalLength is ignored, instead the kernel returns a
             * config descriptor with verified bLength fields, with descriptors
             * with an invalid bLength removed.
             */
            let mut sysfs_config_len: uint16_t = 0;
            let mut offset: libc::c_int = 0;
            if num_configs as libc::c_int > 1 as libc::c_int
                && (idx as libc::c_int) < num_configs as libc::c_int - 1 as libc::c_int
            {
                offset = seek_to_next_config(
                    ctx,
                    buffer.offset(9 as libc::c_int as isize),
                    remaining.wrapping_sub(9 as libc::c_int as libc::c_ulong),
                );
                if offset < 0 as libc::c_int {
                    return offset;
                }
                sysfs_config_len = offset as uint16_t
            } else {
                sysfs_config_len = remaining as uint16_t
            }
            if config_len as libc::c_int != sysfs_config_len as libc::c_int {
                usbi_log(
                    ctx,
                    LIBUSB_LOG_LEVEL_WARNING,
                    (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                        b"parse_config_descriptors\x00",
                    ))
                    .as_ptr(),
                    b"config length mismatch wTotalLength %u real %u\x00" as *const u8
                        as *const libc::c_char,
                    config_len as libc::c_int,
                    sysfs_config_len as libc::c_int,
                );
                config_len = sysfs_config_len
            }
        } else if config_len as libc::c_ulong > remaining {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 25], &[libc::c_char; 25]>(
                    b"parse_config_descriptors\x00",
                ))
                .as_ptr(),
                b"short descriptor read %zu/%u\x00" as *const u8 as *const libc::c_char,
                remaining,
                config_len as libc::c_int,
            );
            config_len = remaining as uint16_t
        }
        let ref mut fresh1 = (*(*priv_0).config_descriptors.offset(idx as isize)).desc;
        *fresh1 = config_desc;
        (*(*priv_0).config_descriptors.offset(idx as isize)).actual_len = config_len as size_t;
        buffer = buffer.offset(config_len as libc::c_int as isize);
        remaining = (remaining as libc::c_ulong).wrapping_sub(config_len as libc::c_ulong) as size_t
            as size_t;
        idx = idx.wrapping_add(1)
    }
    return LIBUSB_SUCCESS as libc::c_int;
}
unsafe extern "C" fn op_get_config_descriptor_by_value(
    dev: *mut libusb_device,
    value: uint8_t,
    buffer: *mut *mut libc::c_void,
) -> libc::c_int {
    let priv_0: *mut linux_device_priv = usbi_get_device_priv(dev) as *mut linux_device_priv;
    let mut config: *mut config_descriptor = 0 as *mut config_descriptor;
    let mut idx: uint8_t = 0;
    idx = 0 as libc::c_int as uint8_t;
    while (idx as libc::c_int) < (*dev).device_descriptor.bNumConfigurations as libc::c_int {
        config = &mut *(*priv_0).config_descriptors.offset(idx as isize) as *mut config_descriptor;
        if (*(*config).desc).bConfigurationValue as libc::c_int == value as libc::c_int {
            *buffer = (*config).desc as *mut libc::c_void;
            return (*config).actual_len as libc::c_int;
        }
        idx = idx.wrapping_add(1)
    }
    return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
}
unsafe extern "C" fn op_get_active_config_descriptor(
    dev: *mut libusb_device,
    buffer: *mut libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let priv_0: *mut linux_device_priv = usbi_get_device_priv(dev) as *mut linux_device_priv;
    let mut config_desc: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut active_config: uint8_t = 0;
    let mut r: libc::c_int = 0;
    if !(*priv_0).sysfs_dir.is_null() {
        r = sysfs_get_active_config(dev, &mut active_config);
        if r < 0 as libc::c_int {
            return r;
        }
    } else {
        /*
         * In usbfs the config descriptors are wTotalLength bytes apart,
         * with any short reads from the device appearing as holes in the file.
         */
        /* Use cached bConfigurationValue */
        active_config = (*priv_0).active_config
    }
    if active_config as libc::c_int == 0 as libc::c_int {
        usbi_log(
            (*dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                b"op_get_active_config_descriptor\x00",
            ))
            .as_ptr(),
            b"device unconfigured\x00" as *const u8 as *const libc::c_char,
        );
        return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
    }
    r = op_get_config_descriptor_by_value(dev, active_config, &mut config_desc);
    if r < 0 as libc::c_int {
        return r;
    }
    len = if len < r as size_t { len } else { r as size_t };
    memcpy(buffer, config_desc, len);
    return len as libc::c_int;
}
unsafe extern "C" fn op_get_config_descriptor(
    dev: *mut libusb_device,
    config_index: uint8_t,
    buffer: *mut libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let priv_0: *mut linux_device_priv = usbi_get_device_priv(dev) as *mut linux_device_priv;
    let mut config: *mut config_descriptor = 0 as *mut config_descriptor;
    if config_index as libc::c_int >= (*dev).device_descriptor.bNumConfigurations as libc::c_int {
        return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
    }
    config =
        &mut *(*priv_0).config_descriptors.offset(config_index as isize) as *mut config_descriptor;
    len = if len < (*config).actual_len {
        len
    } else {
        (*config).actual_len
    };
    memcpy(buffer, (*config).desc as *const libc::c_void, len);
    return len as libc::c_int;
}
/* send a control message to retrieve active configuration */
unsafe extern "C" fn usbfs_get_active_config(
    dev: *mut libusb_device,
    fd: libc::c_int,
) -> libc::c_int {
    let mut priv_0: *mut linux_device_priv = usbi_get_device_priv(dev) as *mut linux_device_priv;
    let mut active_config: uint8_t = 0 as libc::c_int as uint8_t;
    let mut r: libc::c_int = 0;
    let mut ctrl: usbfs_ctrltransfer = {
        let init = usbfs_ctrltransfer {
            bmRequestType: LIBUSB_ENDPOINT_IN as libc::c_int as __u8,
            bRequest: LIBUSB_REQUEST_GET_CONFIGURATION as libc::c_int as __u8,
            wValue: 0 as libc::c_int as __u16,
            wIndex: 0 as libc::c_int as __u16,
            wLength: 1 as libc::c_int as __u16,
            timeout: 1000 as libc::c_int as __u32,
            data: &mut active_config as *mut uint8_t as *mut libc::c_void,
        };
        init
    };
    r = ioctl(
        fd,
        ((2 as libc::c_uint | 1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<usbfs_ctrltransfer>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut ctrl as *mut usbfs_ctrltransfer,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 19 as libc::c_int {
            return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
        }
        /* we hit this error path frequently with buggy devices :( */
        usbi_log(
            (*dev).ctx,
            LIBUSB_LOG_LEVEL_WARNING,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"usbfs_get_active_config\x00",
            ))
            .as_ptr(),
            b"get configuration failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
    } else if active_config as libc::c_int == 0 as libc::c_int {
        /* some buggy devices have a configuration 0, but we're
         * reaching into the corner of a corner case here, so let's
         * not support buggy devices in these circumstances.
         * stick to the specs: a configuration value of 0 means
         * unconfigured. */
        usbi_log(
            (*dev).ctx,
            LIBUSB_LOG_LEVEL_WARNING,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"usbfs_get_active_config\x00",
            ))
            .as_ptr(),
            b"active cfg 0? assuming unconfigured device\x00" as *const u8 as *const libc::c_char,
        );
    }
    (*priv_0).active_config = active_config;
    return LIBUSB_SUCCESS as libc::c_int;
}
unsafe extern "C" fn initialize_device(
    mut dev: *mut libusb_device,
    busnum: uint8_t,
    devaddr: uint8_t,
    sysfs_dir: *const libc::c_char,
    wrapped_fd: libc::c_int,
) -> libc::c_int {
    let mut priv_0: *mut linux_device_priv = usbi_get_device_priv(dev) as *mut linux_device_priv;
    let ctx: *mut libusb_context = (*dev).ctx;
    let mut alloc_len: size_t = 0;
    let mut fd: libc::c_int = 0;
    let mut speed: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut nb: ssize_t = 0;
    (*dev).bus_number = busnum;
    (*dev).device_address = devaddr;
    if !sysfs_dir.is_null() {
        (*priv_0).sysfs_dir = strdup(sysfs_dir);
        if (*priv_0).sysfs_dir.is_null() {
            return LIBUSB_ERROR_NO_MEM as libc::c_int;
        }
        /* Note speed can contain 1.5, in this case read_sysfs_attr()
        will stop parsing at the '.' and return 1 */
        if read_sysfs_attr(
            ctx,
            sysfs_dir,
            b"speed\x00" as *const u8 as *const libc::c_char,
            2147483647 as libc::c_int,
            &mut speed,
        ) == 0 as libc::c_int
        {
            match speed {
                1 => (*dev).speed = LIBUSB_SPEED_LOW,
                12 => (*dev).speed = LIBUSB_SPEED_FULL,
                480 => (*dev).speed = LIBUSB_SPEED_HIGH,
                5000 => (*dev).speed = LIBUSB_SPEED_SUPER,
                10000 => (*dev).speed = LIBUSB_SPEED_SUPER_PLUS,
                _ => {
                    usbi_log(
                        ctx,
                        LIBUSB_LOG_LEVEL_WARNING,
                        (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                            b"initialize_device\x00",
                        ))
                        .as_ptr(),
                        b"unknown device speed: %d Mbps\x00" as *const u8 as *const libc::c_char,
                        speed,
                    );
                }
            }
        }
    }
    /* cache descriptors in memory */
    if !sysfs_dir.is_null() {
        fd = open_sysfs_attr(
            ctx,
            sysfs_dir,
            b"descriptors\x00" as *const u8 as *const libc::c_char,
        )
    } else if wrapped_fd < 0 as libc::c_int {
        fd = get_usbfs_fd(dev, 0 as libc::c_int as mode_t, 0 as libc::c_int)
    } else {
        fd = wrapped_fd;
        r = lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int) as libc::c_int;
        if r < 0 as libc::c_int {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                    b"initialize_device\x00",
                ))
                .as_ptr(),
                b"lseek failed, errno=%d\x00" as *const u8 as *const libc::c_char,
                *__errno_location(),
            );
            return LIBUSB_ERROR_IO as libc::c_int;
        }
    }
    if fd < 0 as libc::c_int {
        return fd;
    }
    alloc_len = 0 as libc::c_int as size_t;
    loop {
        alloc_len = (alloc_len as libc::c_ulong).wrapping_add(256 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        (*priv_0).descriptors = usbi_reallocf((*priv_0).descriptors, alloc_len);
        if (*priv_0).descriptors.is_null() {
            if fd != wrapped_fd {
                close(fd);
            }
            return LIBUSB_ERROR_NO_MEM as libc::c_int;
        }
        /* usbfs has holes in the file */
        if sysfs_dir.is_null() {
            memset(
                (*priv_0)
                    .descriptors
                    .offset((*priv_0).descriptors_len as isize),
                0 as libc::c_int,
                alloc_len.wrapping_sub((*priv_0).descriptors_len),
            );
        }
        nb = read(
            fd,
            (*priv_0)
                .descriptors
                .offset((*priv_0).descriptors_len as isize),
            alloc_len.wrapping_sub((*priv_0).descriptors_len),
        );
        if nb < 0 as libc::c_int as libc::c_long {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                    b"initialize_device\x00",
                ))
                .as_ptr(),
                b"read descriptor failed, errno=%d\x00" as *const u8 as *const libc::c_char,
                *__errno_location(),
            );
            if fd != wrapped_fd {
                close(fd);
            }
            return LIBUSB_ERROR_IO as libc::c_int;
        }
        (*priv_0).descriptors_len = ((*priv_0).descriptors_len as libc::c_ulong)
            .wrapping_add(nb as size_t) as size_t as size_t;
        if !((*priv_0).descriptors_len == alloc_len) {
            break;
        }
    }
    if fd != wrapped_fd {
        close(fd);
    }
    if (*priv_0).descriptors_len < 18 as libc::c_int as libc::c_ulong {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"initialize_device\x00"))
                .as_ptr(),
            b"short descriptor read (%zu)\x00" as *const u8 as *const libc::c_char,
            (*priv_0).descriptors_len,
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    r = parse_config_descriptors(dev);
    if r < 0 as libc::c_int {
        return r;
    }
    memcpy(
        &mut (*dev).device_descriptor as *mut libusb_device_descriptor as *mut libc::c_void,
        (*priv_0).descriptors,
        18 as libc::c_int as libc::c_ulong,
    );
    if !sysfs_dir.is_null() {
        /* sysfs descriptors are in bus-endian format */
        usbi_localize_device_descriptor(&mut (*dev).device_descriptor);
        return LIBUSB_SUCCESS as libc::c_int;
    }
    /* cache active config */
    if wrapped_fd < 0 as libc::c_int {
        fd = get_usbfs_fd(dev, 0o2 as libc::c_int as mode_t, 1 as libc::c_int)
    } else {
        fd = wrapped_fd
    }
    if fd < 0 as libc::c_int {
        /* cannot send a control message to determine the active
         * config. just assume the first one is active. */
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_WARNING,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"initialize_device\x00"))
                .as_ptr(),
            b"Missing rw usbfs access; cannot determine active configuration descriptor\x00"
                as *const u8 as *const libc::c_char,
        ); /* No config dt */
        if !(*priv_0).config_descriptors.is_null() {
            (*priv_0).active_config = (*(*(*priv_0)
                .config_descriptors
                .offset(0 as libc::c_int as isize))
            .desc)
                .bConfigurationValue
        } else {
            (*priv_0).active_config = 0 as libc::c_int as uint8_t
        }
        return LIBUSB_SUCCESS as libc::c_int;
    }
    r = usbfs_get_active_config(dev, fd);
    if fd != wrapped_fd {
        close(fd);
    }
    return r;
}
unsafe extern "C" fn linux_get_parent_info(
    mut dev: *mut libusb_device,
    sysfs_dir: *const libc::c_char,
) -> libc::c_int {
    let ctx: *mut libusb_context = (*dev).ctx;
    let mut it: *mut libusb_device = 0 as *mut libusb_device;
    let mut parent_sysfs_dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut add_parent: libc::c_int = 1 as libc::c_int;
    /* XXX -- can we figure out the topology when using usbfs? */
    if sysfs_dir.is_null()
        || strncmp(
            sysfs_dir,
            b"usb\x00" as *const u8 as *const libc::c_char,
            3 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        /* either using usbfs or finding the parent of a root hub */
        return LIBUSB_SUCCESS as libc::c_int;
    }
    parent_sysfs_dir = strdup(sysfs_dir);
    if parent_sysfs_dir.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    tmp = strrchr(parent_sysfs_dir, '.' as i32);
    if !tmp.is_null() || {
        tmp = strrchr(parent_sysfs_dir, '-' as i32);
        !tmp.is_null()
    } {
        (*dev).port_number = atoi(tmp.offset(1 as libc::c_int as isize)) as uint8_t;
        *tmp = '\u{0}' as i32 as libc::c_char
    } else {
        usbi_log(
            ctx,
            LIBUSB_LOG_LEVEL_WARNING,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"linux_get_parent_info\x00",
            ))
            .as_ptr(),
            b"Can not parse sysfs_dir: %s, no parent info\x00" as *const u8 as *const libc::c_char,
            parent_sysfs_dir,
        );
        free(parent_sysfs_dir as *mut libc::c_void);
        return LIBUSB_SUCCESS as libc::c_int;
    }
    /* is the parent a root hub? */
    if strchr(parent_sysfs_dir, '-' as i32).is_null() {
        tmp = parent_sysfs_dir;
        ret = asprintf(
            &mut parent_sysfs_dir as *mut *mut libc::c_char,
            b"usb%s\x00" as *const u8 as *const libc::c_char,
            tmp,
        );
        free(tmp as *mut libc::c_void);
        if ret < 0 as libc::c_int {
            return LIBUSB_ERROR_NO_MEM as libc::c_int;
        }
    }
    loop {
        /* find the parent in the context */
        usbi_mutex_lock(&mut (*ctx).usb_devs_lock);
        it = ((*ctx).usb_devs.next as uintptr_t).wrapping_sub(72 as libc::c_ulong)
            as *mut libusb_device;
        while &mut (*it).list as *mut list_head != &mut (*ctx).usb_devs as *mut list_head {
            let priv_0: *mut linux_device_priv = usbi_get_device_priv(it) as *mut linux_device_priv;
            if !(*priv_0).sysfs_dir.is_null() {
                if strcmp((*priv_0).sysfs_dir, parent_sysfs_dir) == 0 {
                    (*dev).parent_dev = libusb_ref_device(it);
                    break;
                }
            }
            it = ((*it).list.next as uintptr_t).wrapping_sub(72 as libc::c_ulong)
                as *mut libusb_device
        }
        usbi_mutex_unlock(&mut (*ctx).usb_devs_lock);
        if !((*dev).parent_dev.is_null() && add_parent != 0) {
            break;
        }
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"linux_get_parent_info\x00",
            ))
            .as_ptr(),
            b"parent_dev %s not enumerated yet, enumerating now\x00" as *const u8
                as *const libc::c_char,
            parent_sysfs_dir,
        );
        sysfs_scan_device(ctx, parent_sysfs_dir);
        add_parent = 0 as libc::c_int
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"linux_get_parent_info\x00"))
            .as_ptr(),
        b"dev %p (%s) has parent %p (%s) port %u\x00" as *const u8 as *const libc::c_char,
        dev,
        sysfs_dir,
        (*dev).parent_dev,
        parent_sysfs_dir,
        (*dev).port_number as libc::c_int,
    );
    free(parent_sysfs_dir as *mut libc::c_void);
    return LIBUSB_SUCCESS as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn linux_enumerate_device(
    ctx: *mut libusb_context,
    busnum: uint8_t,
    devaddr: uint8_t,
    sysfs_dir: *const libc::c_char,
) -> libc::c_int {
    let mut session_id: libc::c_ulong = 0;
    let mut dev: *mut libusb_device = 0 as *mut libusb_device;
    let mut r: libc::c_int = 0;
    /* FIXME: session ID is not guaranteed unique as addresses can wrap and
     * will be reused. instead we should add a simple sysfs attribute with
     * a session ID. */
    session_id =
        ((busnum as libc::c_int) << 8 as libc::c_int | devaddr as libc::c_int) as libc::c_ulong;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"linux_enumerate_device\x00"))
            .as_ptr(),
        b"busnum %u devaddr %u session_id %lu\x00" as *const u8 as *const libc::c_char,
        busnum as libc::c_int,
        devaddr as libc::c_int,
        session_id,
    );
    dev = usbi_get_device_by_session_id(ctx, session_id);
    if !dev.is_null() {
        /* device already exists in the context */
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"linux_enumerate_device\x00",
            ))
            .as_ptr(),
            b"session_id %lu already exists\x00" as *const u8 as *const libc::c_char,
            session_id,
        );
        libusb_unref_device(dev);
        return LIBUSB_SUCCESS as libc::c_int;
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"linux_enumerate_device\x00"))
            .as_ptr(),
        b"allocating new device for %u/%u (session %lu)\x00" as *const u8 as *const libc::c_char,
        busnum as libc::c_int,
        devaddr as libc::c_int,
        session_id,
    );
    dev = usbi_alloc_device(ctx, session_id);
    if dev.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    r = initialize_device(dev, busnum, devaddr, sysfs_dir, -(1 as libc::c_int));
    if !(r < 0 as libc::c_int) {
        r = usbi_sanitize_device(dev);
        if !(r < 0 as libc::c_int) {
            r = linux_get_parent_info(dev, sysfs_dir);
            (r) < 0 as libc::c_int;
        }
    }
    if r < 0 as libc::c_int {
        libusb_unref_device(dev);
    } else {
        usbi_connect_device(dev);
    }
    return r;
}
/* Only used for isoc urbs */
/* Only used with bulk streams */
/* interface 0..N ; negative numbers reserved */
/* MUST encode size + direction of data so the
 * macros in <asm/ioctl.h> give correct values */
/* param buffer (in, or out) */
/* Not used by USBDEVFS_FREE_STREAMS */
#[no_mangle]
pub unsafe extern "C" fn linux_hotplug_enumerate(
    busnum: uint8_t,
    devaddr: uint8_t,
    sys_name: *const libc::c_char,
) {
    let mut ctx: *mut libusb_context = 0 as *mut libusb_context;
    usbi_mutex_static_lock(&mut active_contexts_lock);
    ctx = (active_contexts_list.next as uintptr_t).wrapping_sub(552 as libc::c_ulong)
        as *mut libusb_context;
    while &mut (*ctx).list as *mut list_head != &mut active_contexts_list as *mut list_head {
        linux_enumerate_device(ctx, busnum, devaddr, sys_name);
        ctx = ((*ctx).list.next as uintptr_t).wrapping_sub(552 as libc::c_ulong)
            as *mut libusb_context
    }
    usbi_mutex_static_unlock(&mut active_contexts_lock);
}
#[no_mangle]
pub unsafe extern "C" fn linux_device_disconnected(busnum: uint8_t, devaddr: uint8_t) {
    let mut ctx: *mut libusb_context = 0 as *mut libusb_context;
    let mut dev: *mut libusb_device = 0 as *mut libusb_device;
    let session_id: libc::c_ulong =
        ((busnum as libc::c_int) << 8 as libc::c_int | devaddr as libc::c_int) as libc::c_ulong;
    usbi_mutex_static_lock(&mut active_contexts_lock);
    ctx = (active_contexts_list.next as uintptr_t).wrapping_sub(552 as libc::c_ulong)
        as *mut libusb_context;
    while &mut (*ctx).list as *mut list_head != &mut active_contexts_list as *mut list_head {
        dev = usbi_get_device_by_session_id(ctx, session_id);
        if !dev.is_null() {
            usbi_disconnect_device(dev);
            libusb_unref_device(dev);
        } else {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"linux_device_disconnected\x00",
                ))
                .as_ptr(),
                b"device not found for session %lx\x00" as *const u8 as *const libc::c_char,
                session_id,
            );
        }
        ctx = ((*ctx).list.next as uintptr_t).wrapping_sub(552 as libc::c_ulong)
            as *mut libusb_context
    }
    usbi_mutex_static_unlock(&mut active_contexts_lock);
}
unsafe extern "C" fn initialize_handle(
    handle: *mut libusb_device_handle,
    fd: libc::c_int,
) -> libc::c_int {
    let mut hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let mut r: libc::c_int = 0;
    (*hpriv).fd = fd;
    r = ioctl(
        fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((26 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<__u32>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut (*hpriv).caps as *mut uint32_t,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 25 as libc::c_int {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                    b"initialize_handle\x00",
                ))
                .as_ptr(),
                b"getcap not available\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            usbi_log(
                (*(*handle).dev).ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(
                    b"initialize_handle\x00",
                ))
                .as_ptr(),
                b"getcap failed, errno=%d\x00" as *const u8 as *const libc::c_char,
                *__errno_location(),
            );
        }
        (*hpriv).caps = 0x2 as libc::c_int as uint32_t
    }
    return usbi_add_pollfd(
        (*(*handle).dev).ctx,
        (*hpriv).fd,
        0x4 as libc::c_int as libc::c_short,
    );
}
unsafe extern "C" fn op_wrap_sys_device(
    ctx: *mut libusb_context,
    mut handle: *mut libusb_device_handle,
    sys_dev: intptr_t,
) -> libc::c_int {
    let mut hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let fd: libc::c_int = sys_dev as libc::c_int;
    let mut busnum: uint8_t = 0;
    let mut devaddr: uint8_t = 0;
    let mut ci: usbfs_connectinfo = usbfs_connectinfo { devnum: 0, slow: 0 };
    let mut dev: *mut libusb_device = 0 as *mut libusb_device;
    let mut r: libc::c_int = 0;
    r = linux_get_device_address(
        ctx,
        1 as libc::c_int,
        &mut busnum,
        &mut devaddr,
        0 as *const libc::c_char,
        0 as *const libc::c_char,
        fd,
    );
    if r < 0 as libc::c_int {
        r = ioctl(
            fd,
            ((1 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((17 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                as libc::c_ulong
                | (::std::mem::size_of::<usbfs_connectinfo>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            &mut ci as *mut usbfs_connectinfo,
        );
        if r < 0 as libc::c_int {
            usbi_log(
                ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"op_wrap_sys_device\x00",
                ))
                .as_ptr(),
                b"connectinfo failed, errno=%d\x00" as *const u8 as *const libc::c_char,
                *__errno_location(),
            );
            return LIBUSB_ERROR_IO as libc::c_int;
        }
        /* There is no ioctl to get the bus number. We choose 0 here
         * as linux starts numbering buses from 1. */
        busnum = 0 as libc::c_int as uint8_t;
        devaddr = ci.devnum as uint8_t
    }
    /* Session id is unused as we do not add the device to the list of
     * connected devices. */
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"op_wrap_sys_device\x00"))
            .as_ptr(),
        b"allocating new device for fd %d\x00" as *const u8 as *const libc::c_char,
        fd,
    );
    dev = usbi_alloc_device(ctx, 0 as libc::c_int as libc::c_ulong);
    if dev.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    r = initialize_device(dev, busnum, devaddr, 0 as *const libc::c_char, fd);
    if !(r < 0 as libc::c_int) {
        r = usbi_sanitize_device(dev);
        if !(r < 0 as libc::c_int) {
            /* Consider the device as connected, but do not add it to the managed
             * device list. */
            (*dev).attached = 1 as libc::c_int;
            (*handle).dev = dev;
            r = initialize_handle(handle, fd);
            (*hpriv).fd_keep = 1 as libc::c_int
        }
    }
    if r < 0 as libc::c_int {
        libusb_unref_device(dev);
    }
    return r;
}
unsafe extern "C" fn op_open(handle: *mut libusb_device_handle) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    fd = get_usbfs_fd(
        (*handle).dev,
        0o2 as libc::c_int as mode_t,
        0 as libc::c_int,
    );
    if fd < 0 as libc::c_int {
        if fd == LIBUSB_ERROR_NO_DEVICE as libc::c_int {
            /* device will still be marked as attached if hotplug monitor thread
             * hasn't processed remove event yet */
            usbi_mutex_static_lock(&mut linux_hotplug_lock);
            if (*(*handle).dev).attached != 0 {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 8], &[libc::c_char; 8]>(b"op_open\x00"))
                        .as_ptr(),
                    b"open failed with no device, but device still attached\x00" as *const u8
                        as *const libc::c_char,
                );
                linux_device_disconnected(
                    (*(*handle).dev).bus_number,
                    (*(*handle).dev).device_address,
                );
            }
            usbi_mutex_static_unlock(&mut linux_hotplug_lock);
        }
        return fd;
    }
    r = initialize_handle(handle, fd);
    if r < 0 as libc::c_int {
        close(fd);
    }
    return r;
}
unsafe extern "C" fn op_close(dev_handle: *mut libusb_device_handle) {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(dev_handle) as *mut linux_device_handle_priv;
    /* fd may have already been removed by POLLERR condition in op_handle_events() */
    if (*hpriv).fd_removed == 0 {
        usbi_remove_pollfd((*(*dev_handle).dev).ctx, (*hpriv).fd);
    }
    if (*hpriv).fd_keep == 0 {
        close((*hpriv).fd);
    };
}
unsafe extern "C" fn op_get_configuration(
    handle: *mut libusb_device_handle,
    config: *mut uint8_t,
) -> libc::c_int {
    let priv_0: *mut linux_device_priv =
        usbi_get_device_priv((*handle).dev) as *mut linux_device_priv;
    let mut r: libc::c_int = 0;
    if !(*priv_0).sysfs_dir.is_null() {
        r = sysfs_get_active_config((*handle).dev, config)
    } else {
        let hpriv: *mut linux_device_handle_priv =
            usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
        r = usbfs_get_active_config((*handle).dev, (*hpriv).fd);
        if r == LIBUSB_SUCCESS as libc::c_int {
            *config = (*priv_0).active_config
        }
    }
    if r < 0 as libc::c_int {
        return r;
    }
    if *config as libc::c_int == 0 as libc::c_int {
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"op_get_configuration\x00"))
                .as_ptr(),
            b"device unconfigured\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn op_set_configuration(
    handle: *mut libusb_device_handle,
    mut config: libc::c_int,
) -> libc::c_int {
    let mut priv_0: *mut linux_device_priv =
        usbi_get_device_priv((*handle).dev) as *mut linux_device_priv;
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let fd: libc::c_int = (*hpriv).fd;
    let r: libc::c_int = ioctl(
        fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((5 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut config as *mut libc::c_int,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 22 as libc::c_int {
            return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
        } else {
            if *__errno_location() == 16 as libc::c_int {
                return LIBUSB_ERROR_BUSY as libc::c_int;
            } else {
                if *__errno_location() == 19 as libc::c_int {
                    return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
                }
            }
        }
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"op_set_configuration\x00"))
                .as_ptr(),
            b"set configuration failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    if config == -(1 as libc::c_int) {
        config = 0 as libc::c_int
    }
    /* update our cached active config descriptor */
    (*priv_0).active_config = config as uint8_t;
    return LIBUSB_SUCCESS as libc::c_int;
}
unsafe extern "C" fn claim_interface(
    handle: *mut libusb_device_handle,
    mut iface: libc::c_uint,
) -> libc::c_int {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let fd: libc::c_int = (*hpriv).fd;
    let r: libc::c_int = ioctl(
        fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((15 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut iface as *mut libc::c_uint,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int {
            return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
        } else {
            if *__errno_location() == 16 as libc::c_int {
                return LIBUSB_ERROR_BUSY as libc::c_int;
            } else {
                if *__errno_location() == 19 as libc::c_int {
                    return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
                }
            }
        }
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"claim_interface\x00"))
                .as_ptr(),
            b"claim interface failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn release_interface(
    handle: *mut libusb_device_handle,
    mut iface: libc::c_uint,
) -> libc::c_int {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let fd: libc::c_int = (*hpriv).fd;
    let r: libc::c_int = ioctl(
        fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((16 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut iface as *mut libc::c_uint,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 19 as libc::c_int {
            return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
        }
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"release_interface\x00"))
                .as_ptr(),
            b"release interface failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn op_set_interface(
    handle: *mut libusb_device_handle,
    interface: uint8_t,
    altsetting: uint8_t,
) -> libc::c_int {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let fd: libc::c_int = (*hpriv).fd;
    let mut setintf: usbfs_setinterface = usbfs_setinterface {
        interface: 0,
        altsetting: 0,
    };
    let mut r: libc::c_int = 0;
    setintf.interface = interface as libc::c_uint;
    setintf.altsetting = altsetting as libc::c_uint;
    r = ioctl(
        fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((4 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<usbfs_setinterface>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut setintf as *mut usbfs_setinterface,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 22 as libc::c_int {
            return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
        } else {
            if *__errno_location() == 19 as libc::c_int {
                return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
            }
        }
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"op_set_interface\x00"))
                .as_ptr(),
            b"set interface failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn op_clear_halt(
    handle: *mut libusb_device_handle,
    endpoint: libc::c_uchar,
) -> libc::c_int {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let fd: libc::c_int = (*hpriv).fd;
    let mut _endpoint: libc::c_uint = endpoint as libc::c_uint;
    let r: libc::c_int = ioctl(
        fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((21 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut _endpoint as *mut libc::c_uint,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 2 as libc::c_int {
            return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
        } else {
            if *__errno_location() == 19 as libc::c_int {
                return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
            }
        }
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"op_clear_halt\x00"))
                .as_ptr(),
            b"clear halt failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn op_reset_device(mut handle: *mut libusb_device_handle) -> libc::c_int {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let fd: libc::c_int = (*hpriv).fd;
    let mut r: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut i: uint8_t = 0;
    /* Doing a device reset will cause the usbfs driver to get unbound
     * from any interfaces it is bound to. By voluntarily unbinding
     * the usbfs driver ourself, we stop the kernel from rebinding
     * the interface after reset (which would end up with the interface
     * getting bound to the in kernel driver if any). */
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < 32 as libc::c_int {
        if (*handle).claimed_interfaces & (1 as libc::c_ulong) << i as libc::c_int != 0 {
            release_interface(handle, i as libc::c_uint);
        }
        i = i.wrapping_add(1)
    }
    usbi_mutex_lock(&mut (*handle).lock);
    r = ioctl(
        fd,
        ((0 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((20 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            | ((0 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                as libc::c_uint) as libc::c_ulong,
        0 as *mut libc::c_void,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 19 as libc::c_int {
            ret = LIBUSB_ERROR_NOT_FOUND as libc::c_int
        } else {
            usbi_log(
                (*(*handle).dev).ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"op_reset_device\x00"))
                    .as_ptr(),
                b"reset failed, errno=%d\x00" as *const u8 as *const libc::c_char,
                *__errno_location(),
            );
            ret = LIBUSB_ERROR_OTHER as libc::c_int
        }
    } else {
        /* And re-claim any interfaces which were claimed before the reset */
        i = 0 as libc::c_int as uint8_t;
        while (i as libc::c_int) < 32 as libc::c_int {
            if !((*handle).claimed_interfaces & (1 as libc::c_ulong) << i as libc::c_int == 0) {
                /*
                 * A driver may have completed modprobing during
                 * IOCTL_USBFS_RESET, and bound itself as soon as
                 * IOCTL_USBFS_RESET released the device lock
                 */
                r = detach_kernel_driver_and_claim(handle, i);
                if r != 0 {
                    usbi_log(
                        (*(*handle).dev).ctx,
                        LIBUSB_LOG_LEVEL_WARNING,
                        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(
                            b"op_reset_device\x00",
                        ))
                        .as_ptr(),
                        b"failed to re-claim interface %u after reset: %s\x00" as *const u8
                            as *const libc::c_char,
                        i as libc::c_int,
                        libusb_error_name(r),
                    );
                    (*handle).claimed_interfaces &= !((1 as libc::c_ulong) << i as libc::c_int);
                    ret = LIBUSB_ERROR_NOT_FOUND as libc::c_int
                }
            }
            i = i.wrapping_add(1)
        }
    }
    usbi_mutex_unlock(&mut (*handle).lock);
    return ret;
}
unsafe extern "C" fn do_streams_ioctl(
    handle: *mut libusb_device_handle,
    req: libc::c_long,
    num_streams: uint32_t,
    endpoints: *mut libc::c_uchar,
    num_endpoints: libc::c_int,
) -> libc::c_int {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let mut r: libc::c_int = 0;
    let fd: libc::c_int = (*hpriv).fd;
    let mut streams: *mut usbfs_streams = 0 as *mut usbfs_streams;
    if num_endpoints > 30 as libc::c_int {
        /* Max 15 in + 15 out eps */
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    streams = malloc(
        (::std::mem::size_of::<usbfs_streams>() as libc::c_ulong)
            .wrapping_add(num_endpoints as libc::c_ulong),
    ) as *mut usbfs_streams;
    if streams.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    (*streams).num_streams = num_streams;
    (*streams).num_eps = num_endpoints as libc::c_uint;
    memcpy(
        (*streams).eps.as_mut_ptr() as *mut libc::c_void,
        endpoints as *const libc::c_void,
        num_endpoints as libc::c_ulong,
    );
    r = ioctl(fd, req as libc::c_ulong, streams);
    free(streams as *mut libc::c_void);
    if r < 0 as libc::c_int {
        if *__errno_location() == 25 as libc::c_int {
            return LIBUSB_ERROR_NOT_SUPPORTED as libc::c_int;
        } else {
            if *__errno_location() == 22 as libc::c_int {
                return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
            } else {
                if *__errno_location() == 19 as libc::c_int {
                    return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
                }
            }
        }
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"do_streams_ioctl\x00"))
                .as_ptr(),
            b"streams-ioctl failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    return r;
}
unsafe extern "C" fn op_alloc_streams(
    handle: *mut libusb_device_handle,
    num_streams: uint32_t,
    endpoints: *mut libc::c_uchar,
    num_endpoints: libc::c_int,
) -> libc::c_int {
    return do_streams_ioctl(
        handle,
        (((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((28 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<usbfs_streams>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int) as libc::c_long,
        num_streams,
        endpoints,
        num_endpoints,
    );
}
unsafe extern "C" fn op_free_streams(
    handle: *mut libusb_device_handle,
    endpoints: *mut libc::c_uchar,
    num_endpoints: libc::c_int,
) -> libc::c_int {
    return do_streams_ioctl(
        handle,
        (((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((29 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<usbfs_streams>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int) as libc::c_long,
        0 as libc::c_int as uint32_t,
        endpoints,
        num_endpoints,
    );
}
unsafe extern "C" fn op_dev_mem_alloc(
    handle: *mut libusb_device_handle,
    len: size_t,
) -> *mut libc::c_void {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let mut buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    buffer = mmap(
        0 as *mut libc::c_void,
        len,
        0x1 as libc::c_int | 0x2 as libc::c_int,
        0x1 as libc::c_int,
        (*hpriv).fd,
        0 as libc::c_int as __off_t,
    );
    if buffer == -(1 as libc::c_int) as *mut libc::c_void {
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(b"op_dev_mem_alloc\x00"))
                .as_ptr(),
            b"alloc dev mem failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return 0 as *mut libc::c_void;
    }
    return buffer;
}
unsafe extern "C" fn op_dev_mem_free(
    handle: *mut libusb_device_handle,
    buffer: *mut libc::c_void,
    len: size_t,
) -> libc::c_int {
    if munmap(buffer, len) != 0 as libc::c_int {
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"op_dev_mem_free\x00"))
                .as_ptr(),
            b"free dev mem failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    } else {
        return LIBUSB_SUCCESS as libc::c_int;
    };
}
unsafe extern "C" fn op_kernel_driver_active(
    handle: *mut libusb_device_handle,
    interface: uint8_t,
) -> libc::c_int {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let fd: libc::c_int = (*hpriv).fd;
    let mut getdrv: usbfs_getdriver = usbfs_getdriver {
        interface: 0,
        driver: [0; 256],
    };
    let mut r: libc::c_int = 0;
    getdrv.interface = interface as libc::c_uint;
    r = ioctl(
        fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((8 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<usbfs_getdriver>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut getdrv as *mut usbfs_getdriver,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 61 as libc::c_int {
            return 0 as libc::c_int;
        } else {
            if *__errno_location() == 19 as libc::c_int {
                return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
            }
        }
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"op_kernel_driver_active\x00",
            ))
            .as_ptr(),
            b"get driver failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    return (strcmp(
        getdrv.driver.as_mut_ptr(),
        b"usbfs\x00" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn op_detach_kernel_driver(
    handle: *mut libusb_device_handle,
    interface: uint8_t,
) -> libc::c_int {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let fd: libc::c_int = (*hpriv).fd;
    let mut command: usbfs_ioctl = usbfs_ioctl {
        ifno: 0,
        ioctl_code: 0,
        data: 0 as *mut libc::c_void,
    };
    let mut getdrv: usbfs_getdriver = usbfs_getdriver {
        interface: 0,
        driver: [0; 256],
    };
    let mut r: libc::c_int = 0;
    command.ifno = interface as libc::c_int;
    command.ioctl_code = ((0 as libc::c_uint)
        << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
        | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
        | ((22 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
        | ((0 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
            as libc::c_uint) as libc::c_int;
    command.data = 0 as *mut libc::c_void;
    getdrv.interface = interface as libc::c_uint;
    r = ioctl(
        fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((8 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<usbfs_getdriver>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut getdrv as *mut usbfs_getdriver,
    );
    if r == 0 as libc::c_int
        && strcmp(
            getdrv.driver.as_mut_ptr(),
            b"usbfs\x00" as *const u8 as *const libc::c_char,
        ) == 0
    {
        return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
    }
    r = ioctl(
        fd,
        ((2 as libc::c_uint | 1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((18 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<usbfs_ioctl>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut command as *mut usbfs_ioctl,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 61 as libc::c_int {
            return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
        } else {
            if *__errno_location() == 22 as libc::c_int {
                return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
            } else {
                if *__errno_location() == 19 as libc::c_int {
                    return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
                }
            }
        }
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"op_detach_kernel_driver\x00",
            ))
            .as_ptr(),
            b"detach failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn op_attach_kernel_driver(
    handle: *mut libusb_device_handle,
    interface: uint8_t,
) -> libc::c_int {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let fd: libc::c_int = (*hpriv).fd;
    let mut command: usbfs_ioctl = usbfs_ioctl {
        ifno: 0,
        ioctl_code: 0,
        data: 0 as *mut libc::c_void,
    };
    let mut r: libc::c_int = 0;
    command.ifno = interface as libc::c_int;
    command.ioctl_code = ((0 as libc::c_uint)
        << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
        | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
        | ((23 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
        | ((0 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
            as libc::c_uint) as libc::c_int;
    command.data = 0 as *mut libc::c_void;
    r = ioctl(
        fd,
        ((2 as libc::c_uint | 1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((18 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<usbfs_ioctl>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut command as *mut usbfs_ioctl,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 61 as libc::c_int {
            return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
        } else {
            if *__errno_location() == 22 as libc::c_int {
                return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
            } else {
                if *__errno_location() == 19 as libc::c_int {
                    return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
                } else {
                    if *__errno_location() == 16 as libc::c_int {
                        return LIBUSB_ERROR_BUSY as libc::c_int;
                    }
                }
            }
        }
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"op_attach_kernel_driver\x00",
            ))
            .as_ptr(),
            b"attach failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_OTHER as libc::c_int;
    } else {
        if r == 0 as libc::c_int {
            return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn detach_kernel_driver_and_claim(
    handle: *mut libusb_device_handle,
    interface: uint8_t,
) -> libc::c_int {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let mut dc: usbfs_disconnect_claim = usbfs_disconnect_claim {
        interface: 0,
        flags: 0,
        driver: [0; 256],
    };
    let mut r: libc::c_int = 0;
    let fd: libc::c_int = (*hpriv).fd;
    dc.interface = interface as libc::c_uint;
    strcpy(
        dc.driver.as_mut_ptr(),
        b"usbfs\x00" as *const u8 as *const libc::c_char,
    );
    dc.flags = 0x2 as libc::c_int as libc::c_uint;
    r = ioctl(
        fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((27 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<usbfs_disconnect_claim>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut dc as *mut usbfs_disconnect_claim,
    );
    if r == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    match *__errno_location() {
        25 => {}
        16 => return LIBUSB_ERROR_BUSY as libc::c_int,
        22 => return LIBUSB_ERROR_INVALID_PARAM as libc::c_int,
        19 => return LIBUSB_ERROR_NO_DEVICE as libc::c_int,
        _ => {
            usbi_log(
                (*(*handle).dev).ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                    b"detach_kernel_driver_and_claim\x00",
                ))
                .as_ptr(),
                b"disconnect-and-claim failed, errno=%d\x00" as *const u8 as *const libc::c_char,
                *__errno_location(),
            );
            return LIBUSB_ERROR_OTHER as libc::c_int;
        }
    }
    /* Fallback code for kernels which don't support the
    disconnect-and-claim ioctl */
    r = op_detach_kernel_driver(handle, interface);
    if r != 0 as libc::c_int && r != LIBUSB_ERROR_NOT_FOUND as libc::c_int {
        return r;
    }
    return claim_interface(handle, interface as libc::c_uint);
}
unsafe extern "C" fn op_claim_interface(
    handle: *mut libusb_device_handle,
    interface: uint8_t,
) -> libc::c_int {
    if (*handle).auto_detach_kernel_driver != 0 {
        return detach_kernel_driver_and_claim(handle, interface);
    } else {
        return claim_interface(handle, interface as libc::c_uint);
    };
}
unsafe extern "C" fn op_release_interface(
    handle: *mut libusb_device_handle,
    interface: uint8_t,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = release_interface(handle, interface as libc::c_uint);
    if r != 0 {
        return r;
    }
    if (*handle).auto_detach_kernel_driver != 0 {
        op_attach_kernel_driver(handle, interface);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn op_destroy_device(dev: *mut libusb_device) {
    let priv_0: *mut linux_device_priv = usbi_get_device_priv(dev) as *mut linux_device_priv;
    free((*priv_0).config_descriptors as *mut libc::c_void);
    free((*priv_0).descriptors);
    free((*priv_0).sysfs_dir as *mut libc::c_void);
}
/* URBs are discarded in reverse order of submission to avoid races. */
unsafe extern "C" fn discard_urbs(
    itransfer: *mut usbi_transfer,
    first: libc::c_int,
    last_plus_one: libc::c_int,
) -> libc::c_int {
    let transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    let tpriv: *mut linux_transfer_priv =
        usbi_get_transfer_priv(itransfer) as *mut linux_transfer_priv;
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv((*transfer).dev_handle) as *mut linux_device_handle_priv;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut urb: *mut usbfs_urb = 0 as *mut usbfs_urb;
    i = last_plus_one - 1 as libc::c_int;
    while i >= first {
        if (*transfer).type_0 as libc::c_int == LIBUSB_TRANSFER_TYPE_ISOCHRONOUS as libc::c_int {
            urb = *(*tpriv).c2rust_unnamed.iso_urbs.offset(i as isize)
        } else {
            urb = &mut *(*tpriv).c2rust_unnamed.urbs.offset(i as isize) as *mut usbfs_urb
        }
        if !(ioctl(
            (*hpriv).fd,
            ((0 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((11 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
                | ((0 as libc::c_int) << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int)
                    as libc::c_uint) as libc::c_ulong,
            urb,
        ) == 0 as libc::c_int)
        {
            if *__errno_location() == 22 as libc::c_int {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"discard_urbs\x00"))
                        .as_ptr(),
                    b"URB not found --> assuming ready to be reaped\x00" as *const u8
                        as *const libc::c_char,
                );
                if i == last_plus_one - 1 as libc::c_int {
                    ret = LIBUSB_ERROR_NOT_FOUND as libc::c_int
                }
            } else if *__errno_location() == 19 as libc::c_int {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"discard_urbs\x00"))
                        .as_ptr(),
                    b"Device not found for URB --> assuming ready to be reaped\x00" as *const u8
                        as *const libc::c_char,
                );
                ret = LIBUSB_ERROR_NO_DEVICE as libc::c_int
            } else {
                usbi_log(
                    (*(*(*transfer).dev_handle).dev).ctx,
                    LIBUSB_LOG_LEVEL_WARNING,
                    (*::std::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"discard_urbs\x00"))
                        .as_ptr(),
                    b"unrecognised discard errno %d\x00" as *const u8 as *const libc::c_char,
                    *__errno_location(),
                );
                ret = LIBUSB_ERROR_OTHER as libc::c_int
            }
        }
        i -= 1
    }
    return ret;
}
unsafe extern "C" fn free_iso_urbs(mut tpriv: *mut linux_transfer_priv) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*tpriv).num_urbs {
        let urb: *mut usbfs_urb = *(*tpriv).c2rust_unnamed.iso_urbs.offset(i as isize);
        if urb.is_null() {
            break;
        }
        free(urb as *mut libc::c_void);
        i += 1
    }
    free((*tpriv).c2rust_unnamed.iso_urbs as *mut libc::c_void);
    (*tpriv).c2rust_unnamed.iso_urbs = 0 as *mut *mut usbfs_urb;
}
unsafe extern "C" fn submit_bulk_transfer(itransfer: *mut usbi_transfer) -> libc::c_int {
    let transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    let mut tpriv: *mut linux_transfer_priv =
        usbi_get_transfer_priv(itransfer) as *mut linux_transfer_priv;
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv((*transfer).dev_handle) as *mut linux_device_handle_priv;
    let mut urbs: *mut usbfs_urb = 0 as *mut usbfs_urb;
    let is_out: libc::c_int = !(0 as libc::c_int
        != (*transfer).endpoint as libc::c_int & LIBUSB_ENDPOINT_IN as libc::c_int)
        as libc::c_int;
    let mut bulk_buffer_len: libc::c_int = 0;
    let mut use_bulk_continuation: libc::c_int = 0;
    let mut num_urbs: libc::c_int = 0;
    let mut last_urb_partial: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    /*
     * Older versions of usbfs place a 16kb limit on bulk URBs. We work
     * around this by splitting large transfers into 16k blocks, and then
     * submit all urbs at once. it would be simpler to submit one urb at
     * a time, but there is a big performance gain doing it this way.
     *
     * Newer versions lift the 16k limit (USBFS_CAP_NO_PACKET_SIZE_LIM),
     * using arbritary large transfers can still be a bad idea though, as
     * the kernel needs to allocate physical contiguous memory for this,
     * which may fail for large buffers.
     *
     * The kernel solves this problem by splitting the transfer into
     * blocks itself when the host-controller is scatter-gather capable
     * (USBFS_CAP_BULK_SCATTER_GATHER), which most controllers are.
     *
     * Last, there is the issue of short-transfers when splitting, for
     * short split-transfers to work reliable USBFS_CAP_BULK_CONTINUATION
     * is needed, but this is not always available.
     */
    if (*hpriv).caps & 0x8 as libc::c_int as libc::c_uint != 0 {
        /* Good! Just submit everything in one go */
        bulk_buffer_len = if (*transfer).length != 0 {
            (*transfer).length
        } else {
            1 as libc::c_int
        };
        use_bulk_continuation = 0 as libc::c_int
    } else if (*hpriv).caps & 0x2 as libc::c_int as libc::c_uint != 0 {
        /* Split the transfers and use bulk-continuation to
        avoid issues with short-transfers */
        bulk_buffer_len = 16384 as libc::c_int;
        use_bulk_continuation = 1 as libc::c_int
    } else if (*hpriv).caps & 0x4 as libc::c_int as libc::c_uint != 0 {
        /* Don't split, assume the kernel can alloc the buffer
        (otherwise the submit will fail with -ENOMEM) */
        bulk_buffer_len = if (*transfer).length != 0 {
            (*transfer).length
        } else {
            1 as libc::c_int
        };
        use_bulk_continuation = 0 as libc::c_int
    } else {
        /* Bad, splitting without bulk-continuation, short transfers
        which end before the last urb will not work reliable! */
        /* Note we don't warn here as this is "normal" on kernels <
        2.6.32 and not a problem for most applications */
        bulk_buffer_len = 16384 as libc::c_int;
        use_bulk_continuation = 0 as libc::c_int
    }
    num_urbs = (*transfer).length / bulk_buffer_len;
    if (*transfer).length == 0 as libc::c_int {
        num_urbs = 1 as libc::c_int
    } else if (*transfer).length % bulk_buffer_len > 0 as libc::c_int {
        last_urb_partial = 1 as libc::c_int;
        num_urbs += 1
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(b"submit_bulk_transfer\x00"))
            .as_ptr(),
        b"need %d urbs for new transfer with length %d\x00" as *const u8 as *const libc::c_char,
        num_urbs,
        (*transfer).length,
    );
    urbs = calloc(
        num_urbs as libc::c_ulong,
        ::std::mem::size_of::<usbfs_urb>() as libc::c_ulong,
    ) as *mut usbfs_urb;
    if urbs.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    (*tpriv).c2rust_unnamed.urbs = urbs;
    (*tpriv).num_urbs = num_urbs;
    (*tpriv).num_retired = 0 as libc::c_int;
    (*tpriv).reap_action = NORMAL;
    (*tpriv).reap_status = LIBUSB_TRANSFER_COMPLETED;
    i = 0 as libc::c_int;
    while i < num_urbs {
        let mut urb: *mut usbfs_urb = &mut *urbs.offset(i as isize) as *mut usbfs_urb;
        (*urb).usercontext = itransfer as *mut libc::c_void;
        match (*transfer).type_0 as libc::c_int {
            2 => {
                (*urb).type_0 = 3 as libc::c_int as libc::c_uchar;
                (*urb).c2rust_unnamed.stream_id = 0 as libc::c_int as libc::c_uint
            }
            4 => {
                (*urb).type_0 = 3 as libc::c_int as libc::c_uchar;
                (*urb).c2rust_unnamed.stream_id = (*itransfer).stream_id
            }
            3 => (*urb).type_0 = 1 as libc::c_int as libc::c_uchar,
            _ => {}
        }
        (*urb).endpoint = (*transfer).endpoint;
        (*urb).buffer =
            (*transfer).buffer.offset((i * bulk_buffer_len) as isize) as *mut libc::c_void;
        /* don't set the short not ok flag for the last URB */
        if use_bulk_continuation != 0 && is_out == 0 && i < num_urbs - 1 as libc::c_int {
            (*urb).flags = 0x1 as libc::c_int as libc::c_uint
        }
        if i == num_urbs - 1 as libc::c_int && last_urb_partial != 0 {
            (*urb).buffer_length = (*transfer).length % bulk_buffer_len
        } else if (*transfer).length == 0 as libc::c_int {
            (*urb).buffer_length = 0 as libc::c_int
        } else {
            (*urb).buffer_length = bulk_buffer_len
        }
        if i > 0 as libc::c_int && use_bulk_continuation != 0 {
            (*urb).flags |= 0x4 as libc::c_int as libc::c_uint
        }
        /* we have already checked that the flag is supported */
        if is_out != 0
            && i == num_urbs - 1 as libc::c_int
            && (*transfer).flags as libc::c_int & LIBUSB_TRANSFER_ADD_ZERO_PACKET as libc::c_int
                != 0
        {
            (*urb).flags |= 0x40 as libc::c_int as libc::c_uint
        }
        r = ioctl(
            (*hpriv).fd,
            ((2 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((10 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                as libc::c_ulong
                | (::std::mem::size_of::<usbfs_urb>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            urb,
        );
        if r == 0 as libc::c_int {
            i += 1
        } else {
            if *__errno_location() == 19 as libc::c_int {
                r = LIBUSB_ERROR_NO_DEVICE as libc::c_int
            } else if *__errno_location() == 12 as libc::c_int {
                r = LIBUSB_ERROR_NO_MEM as libc::c_int
            } else {
                usbi_log(
                    (*(*(*transfer).dev_handle).dev).ctx,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                        b"submit_bulk_transfer\x00",
                    ))
                    .as_ptr(),
                    b"submiturb failed, errno=%d\x00" as *const u8 as *const libc::c_char,
                    *__errno_location(),
                );
                r = LIBUSB_ERROR_IO as libc::c_int
            }
            /* if the first URB submission fails, we can simply free up and
             * return failure immediately. */
            if i == 0 as libc::c_int {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 21], &[libc::c_char; 21]>(
                        b"submit_bulk_transfer\x00",
                    ))
                    .as_ptr(),
                    b"first URB failed, easy peasy\x00" as *const u8 as *const libc::c_char,
                );
                free(urbs as *mut libc::c_void);
                (*tpriv).c2rust_unnamed.urbs = 0 as *mut usbfs_urb;
                return r;
            }
            /* if it's not the first URB that failed, the situation is a bit
             * tricky. we may need to discard all previous URBs. there are
             * complications:
             *  - discarding is asynchronous - discarded urbs will be reaped
             *    later. the user must not have freed the transfer when the
             *    discarded URBs are reaped, otherwise libusb will be using
             *    freed memory.
             *  - the earlier URBs may have completed successfully and we do
             *    not want to throw away any data.
             *  - this URB failing may be no error; EREMOTEIO means that
             *    this transfer simply didn't need all the URBs we submitted
             * so, we report that the transfer was submitted successfully and
             * in case of error we discard all previous URBs. later when
             * the final reap completes we can report error to the user,
             * or success if an earlier URB was completed successfully.
             */
            (*tpriv).reap_action = if *__errno_location() == 121 as libc::c_int {
                COMPLETED_EARLY as libc::c_int
            } else {
                SUBMIT_FAILED as libc::c_int
            } as reap_action;
            /* The URBs we haven't submitted yet we count as already
             * retired. */
            (*tpriv).num_retired += num_urbs - i;
            /* If we completed short then don't try to discard. */
            if (*tpriv).reap_action as libc::c_uint
                == COMPLETED_EARLY as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int;
            }
            discard_urbs(itransfer, 0 as libc::c_int, i);
            usbi_log(0 as *mut libusb_context, LIBUSB_LOG_LEVEL_DEBUG,
                     (*::std::mem::transmute::<&[u8; 21],
                                               &[libc::c_char; 21]>(b"submit_bulk_transfer\x00")).as_ptr(),
                     b"reporting successful submission but waiting for %d discards before reporting error\x00"
                         as *const u8 as *const libc::c_char, i);
            return 0 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn submit_iso_transfer(itransfer: *mut usbi_transfer) -> libc::c_int {
    let transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    let mut tpriv: *mut linux_transfer_priv =
        usbi_get_transfer_priv(itransfer) as *mut linux_transfer_priv;
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv((*transfer).dev_handle) as *mut linux_device_handle_priv;
    let mut urbs: *mut *mut usbfs_urb = 0 as *mut *mut usbfs_urb;
    let num_packets: libc::c_int = (*transfer).num_iso_packets;
    let mut num_packets_remaining: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut num_urbs: libc::c_int = 0;
    let mut packet_len: libc::c_uint = 0;
    let mut total_len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut urb_buffer: *mut libc::c_uchar = (*transfer).buffer;
    if num_packets < 1 as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    /* usbfs places arbitrary limits on iso URBs. this limit has changed
     * at least three times, but we attempt to detect this limit during
     * init and check it here. if the kernel rejects the request due to
     * its size, we return an error indicating such to the user.
     */
    i = 0 as libc::c_int;
    while i < num_packets {
        packet_len = (*(*transfer).iso_packet_desc.as_mut_ptr().offset(i as isize)).length;
        if packet_len > max_iso_packet_len {
            usbi_log(
                (*(*(*transfer).dev_handle).dev).ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                    b"submit_iso_transfer\x00",
                ))
                .as_ptr(),
                b"iso packet length of %u bytes exceeds maximum of %u bytes\x00" as *const u8
                    as *const libc::c_char,
                packet_len,
                max_iso_packet_len,
            );
            return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
        }
        total_len = total_len.wrapping_add(packet_len);
        i += 1
    }
    if (*transfer).length < total_len as libc::c_int {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    /* usbfs limits the number of iso packets per URB */
    num_urbs = (num_packets + (128 as libc::c_int - 1 as libc::c_int)) / 128 as libc::c_int;
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(b"submit_iso_transfer\x00"))
            .as_ptr(),
        b"need %d urbs for new transfer with length %d\x00" as *const u8 as *const libc::c_char,
        num_urbs,
        (*transfer).length,
    );
    urbs = calloc(
        num_urbs as libc::c_ulong,
        ::std::mem::size_of::<*mut usbfs_urb>() as libc::c_ulong,
    ) as *mut *mut usbfs_urb;
    if urbs.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    (*tpriv).c2rust_unnamed.iso_urbs = urbs;
    (*tpriv).num_urbs = num_urbs;
    (*tpriv).num_retired = 0 as libc::c_int;
    (*tpriv).reap_action = NORMAL;
    (*tpriv).iso_packet_offset = 0 as libc::c_int;
    /* allocate + initialize each URB with the correct number of packets */
    num_packets_remaining = num_packets;
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while i < num_urbs {
        let num_packets_in_urb: libc::c_int = if num_packets_remaining < 128 as libc::c_int {
            num_packets_remaining
        } else {
            128 as libc::c_int
        };
        let mut urb: *mut usbfs_urb = 0 as *mut usbfs_urb;
        let mut alloc_size: size_t = 0;
        let mut k: libc::c_int = 0;
        alloc_size = (::std::mem::size_of::<usbfs_urb>() as libc::c_ulong).wrapping_add(
            (num_packets_in_urb as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<usbfs_iso_packet_desc>() as libc::c_ulong),
        );
        urb = calloc(1 as libc::c_int as libc::c_ulong, alloc_size) as *mut usbfs_urb;
        if urb.is_null() {
            free_iso_urbs(tpriv);
            return LIBUSB_ERROR_NO_MEM as libc::c_int;
        }
        let ref mut fresh2 = *urbs.offset(i as isize);
        *fresh2 = urb;
        /* populate packet lengths */
        k = 0 as libc::c_int;
        while k < num_packets_in_urb {
            packet_len = (*(*transfer).iso_packet_desc.as_mut_ptr().offset(j as isize)).length;
            (*urb).buffer_length = ((*urb).buffer_length as libc::c_uint).wrapping_add(packet_len)
                as libc::c_int as libc::c_int;
            (*(*urb).iso_frame_desc.as_mut_ptr().offset(k as isize)).length = packet_len;
            j += 1;
            k += 1
        }
        (*urb).usercontext = itransfer as *mut libc::c_void;
        (*urb).type_0 = 0 as libc::c_int as libc::c_uchar;
        /* FIXME: interface for non-ASAP data? */
        (*urb).flags = 0x2 as libc::c_int as libc::c_uint;
        (*urb).endpoint = (*transfer).endpoint;
        (*urb).c2rust_unnamed.number_of_packets = num_packets_in_urb;
        (*urb).buffer = urb_buffer as *mut libc::c_void;
        urb_buffer = urb_buffer.offset((*urb).buffer_length as isize);
        num_packets_remaining -= num_packets_in_urb;
        i += 1
    }
    /* submit URBs */
    i = 0 as libc::c_int;
    while i < num_urbs {
        let mut r: libc::c_int = ioctl(
            (*hpriv).fd,
            ((2 as libc::c_uint)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
                | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
                | ((10 as libc::c_int) << 0 as libc::c_int) as libc::c_uint)
                as libc::c_ulong
                | (::std::mem::size_of::<usbfs_urb>() as libc::c_ulong)
                    << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
            *urbs.offset(i as isize),
        );
        if r == 0 as libc::c_int {
            i += 1
        } else {
            if *__errno_location() == 19 as libc::c_int {
                r = LIBUSB_ERROR_NO_DEVICE as libc::c_int
            } else if *__errno_location() == 22 as libc::c_int {
                usbi_log(
                    (*(*(*transfer).dev_handle).dev).ctx,
                    LIBUSB_LOG_LEVEL_WARNING,
                    (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                        b"submit_iso_transfer\x00",
                    ))
                    .as_ptr(),
                    b"submiturb failed, transfer too large\x00" as *const u8 as *const libc::c_char,
                );
                r = LIBUSB_ERROR_INVALID_PARAM as libc::c_int
            } else if *__errno_location() == 90 as libc::c_int {
                usbi_log(
                    (*(*(*transfer).dev_handle).dev).ctx,
                    LIBUSB_LOG_LEVEL_WARNING,
                    (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                        b"submit_iso_transfer\x00",
                    ))
                    .as_ptr(),
                    b"submiturb failed, iso packet length too large\x00" as *const u8
                        as *const libc::c_char,
                );
                r = LIBUSB_ERROR_INVALID_PARAM as libc::c_int
            } else {
                usbi_log(
                    (*(*(*transfer).dev_handle).dev).ctx,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                        b"submit_iso_transfer\x00",
                    ))
                    .as_ptr(),
                    b"submiturb failed, errno=%d\x00" as *const u8 as *const libc::c_char,
                    *__errno_location(),
                );
                r = LIBUSB_ERROR_IO as libc::c_int
            }
            /* if the first URB submission fails, we can simply free up and
             * return failure immediately. */
            if i == 0 as libc::c_int {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 20], &[libc::c_char; 20]>(
                        b"submit_iso_transfer\x00",
                    ))
                    .as_ptr(),
                    b"first URB failed, easy peasy\x00" as *const u8 as *const libc::c_char,
                );
                free_iso_urbs(tpriv);
                return r;
            }
            /* if it's not the first URB that failed, the situation is a bit
             * tricky. we must discard all previous URBs. there are
             * complications:
             *  - discarding is asynchronous - discarded urbs will be reaped
             *    later. the user must not have freed the transfer when the
             *    discarded URBs are reaped, otherwise libusb will be using
             *    freed memory.
             *  - the earlier URBs may have completed successfully and we do
             *    not want to throw away any data.
             * so, in this case we discard all the previous URBs BUT we report
             * that the transfer was submitted successfully. then later when
             * the final discard completes we can report error to the user.
             */
            (*tpriv).reap_action = SUBMIT_FAILED;
            /* The URBs we haven't submitted yet we count as already
             * retired. */
            (*tpriv).num_retired = num_urbs - i;
            discard_urbs(itransfer, 0 as libc::c_int, i);
            usbi_log(0 as *mut libusb_context, LIBUSB_LOG_LEVEL_DEBUG,
                     (*::std::mem::transmute::<&[u8; 20],
                                               &[libc::c_char; 20]>(b"submit_iso_transfer\x00")).as_ptr(),
                     b"reporting successful submission but waiting for %d discards before reporting error\x00"
                         as *const u8 as *const libc::c_char, i);
            return 0 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn submit_control_transfer(itransfer: *mut usbi_transfer) -> libc::c_int {
    let mut tpriv: *mut linux_transfer_priv =
        usbi_get_transfer_priv(itransfer) as *mut linux_transfer_priv;
    let transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv((*transfer).dev_handle) as *mut linux_device_handle_priv;
    let mut urb: *mut usbfs_urb = 0 as *mut usbfs_urb;
    let mut r: libc::c_int = 0;
    if ((*transfer).length as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<libusb_control_setup>() as libc::c_ulong)
        > 4096 as libc::c_int as libc::c_ulong
    {
        return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
    }
    urb = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<usbfs_urb>() as libc::c_ulong,
    ) as *mut usbfs_urb;
    if urb.is_null() {
        return LIBUSB_ERROR_NO_MEM as libc::c_int;
    }
    (*tpriv).c2rust_unnamed.urbs = urb;
    (*tpriv).num_urbs = 1 as libc::c_int;
    (*tpriv).reap_action = NORMAL;
    (*urb).usercontext = itransfer as *mut libc::c_void;
    (*urb).type_0 = 2 as libc::c_int as libc::c_uchar;
    (*urb).endpoint = (*transfer).endpoint;
    (*urb).buffer = (*transfer).buffer as *mut libc::c_void;
    (*urb).buffer_length = (*transfer).length;
    r = ioctl(
        (*hpriv).fd,
        ((2 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((10 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<usbfs_urb>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        urb,
    );
    if r < 0 as libc::c_int {
        free(urb as *mut libc::c_void);
        (*tpriv).c2rust_unnamed.urbs = 0 as *mut usbfs_urb;
        if *__errno_location() == 19 as libc::c_int {
            return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
        }
        usbi_log(
            (*(*(*transfer).dev_handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 24], &[libc::c_char; 24]>(
                b"submit_control_transfer\x00",
            ))
            .as_ptr(),
            b"submiturb failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn op_submit_transfer(itransfer: *mut usbi_transfer) -> libc::c_int {
    let transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    match (*transfer).type_0 as libc::c_int {
        0 => return submit_control_transfer(itransfer),
        2 | 4 => return submit_bulk_transfer(itransfer),
        3 => return submit_bulk_transfer(itransfer),
        1 => return submit_iso_transfer(itransfer),
        _ => {
            usbi_log(
                (*(*(*transfer).dev_handle).dev).ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                    b"op_submit_transfer\x00",
                ))
                .as_ptr(),
                b"unknown transfer type %u\x00" as *const u8 as *const libc::c_char,
                (*transfer).type_0 as libc::c_int,
            );
            return LIBUSB_ERROR_INVALID_PARAM as libc::c_int;
        }
    };
}
unsafe extern "C" fn op_cancel_transfer(itransfer: *mut usbi_transfer) -> libc::c_int {
    let mut tpriv: *mut linux_transfer_priv =
        usbi_get_transfer_priv(itransfer) as *mut linux_transfer_priv;
    let transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    let mut r: libc::c_int = 0;
    if (*tpriv).c2rust_unnamed.urbs.is_null() {
        return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
    }
    r = discard_urbs(itransfer, 0 as libc::c_int, (*tpriv).num_urbs);
    if r != 0 as libc::c_int {
        return r;
    }
    let current_block_5: u64;
    match (*transfer).type_0 as libc::c_int {
        2 | 4 => {
            if (*tpriv).reap_action as libc::c_uint == ERROR as libc::c_int as libc::c_uint {
                current_block_5 = 1917311967535052937;
            } else {
                current_block_5 = 10488797066620554094;
            }
        }
        _ => {
            current_block_5 = 10488797066620554094;
        }
    }
    match current_block_5 {
        10488797066620554094 =>
        /* else, fall through */
        {
            (*tpriv).reap_action = CANCELLED
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn op_clear_transfer_priv(itransfer: *mut usbi_transfer) {
    let transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    let mut tpriv: *mut linux_transfer_priv =
        usbi_get_transfer_priv(itransfer) as *mut linux_transfer_priv;
    match (*transfer).type_0 as libc::c_int {
        0 | 2 | 4 | 3 => {
            if !(*tpriv).c2rust_unnamed.urbs.is_null() {
                free((*tpriv).c2rust_unnamed.urbs as *mut libc::c_void);
                (*tpriv).c2rust_unnamed.urbs = 0 as *mut usbfs_urb
            }
        }
        1 => {
            if !(*tpriv).c2rust_unnamed.iso_urbs.is_null() {
                free_iso_urbs(tpriv);
                (*tpriv).c2rust_unnamed.iso_urbs = 0 as *mut *mut usbfs_urb
            }
        }
        _ => {
            usbi_log(
                (*(*(*transfer).dev_handle).dev).ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"op_clear_transfer_priv\x00",
                ))
                .as_ptr(),
                b"unknown transfer type %u\x00" as *const u8 as *const libc::c_char,
                (*transfer).type_0 as libc::c_int,
            );
        }
    };
}
unsafe extern "C" fn handle_bulk_completion(
    mut itransfer: *mut usbi_transfer,
    urb: *mut usbfs_urb,
) -> libc::c_int {
    let mut current_block: u64;
    let mut tpriv: *mut linux_transfer_priv =
        usbi_get_transfer_priv(itransfer) as *mut linux_transfer_priv;
    let transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    let urb_idx: libc::c_int =
        urb.wrapping_offset_from((*tpriv).c2rust_unnamed.urbs) as libc::c_long as libc::c_int;
    usbi_mutex_lock(&mut (*itransfer).lock);
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(b"handle_bulk_completion\x00"))
            .as_ptr(),
        b"handling completion status %d of bulk urb %d/%d\x00" as *const u8 as *const libc::c_char,
        (*urb).status,
        urb_idx + 1 as libc::c_int,
        (*tpriv).num_urbs,
    );
    (*tpriv).num_retired += 1;
    if (*tpriv).reap_action as libc::c_uint != NORMAL as libc::c_int as libc::c_uint {
        /* cancelled, submit_fail, or completed early */
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                b"handle_bulk_completion\x00",
            ))
            .as_ptr(),
            b"abnormal reap: urb status %d\x00" as *const u8 as *const libc::c_char,
            (*urb).status,
        );
        /* even though we're in the process of cancelling, it's possible that
         * we may receive some data in these URBs that we don't want to lose.
         * examples:
         * 1. while the kernel is cancelling all the packets that make up an
         *    URB, a few of them might complete. so we get back a successful
         *    cancellation *and* some data.
         * 2. we receive a short URB which marks the early completion condition,
         *    so we start cancelling the remaining URBs. however, we're too
         *    slow and another URB completes (or at least completes partially).
         *    (this can't happen since we always use BULK_CONTINUATION.)
         *
         * When this happens, our objectives are not to lose any "surplus" data,
         * and also to stick it at the end of the previously-received data
         * (closing any holes), so that libusb reports the total amount of
         * transferred data and presents it in a contiguous chunk.
         */
        if (*urb).actual_length > 0 as libc::c_int {
            let target: *mut libc::c_uchar =
                (*transfer).buffer.offset((*itransfer).transferred as isize);
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"handle_bulk_completion\x00",
                ))
                .as_ptr(),
                b"received %d bytes of surplus data\x00" as *const u8 as *const libc::c_char,
                (*urb).actual_length,
            );
            if (*urb).buffer != target as *mut libc::c_void {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"handle_bulk_completion\x00",
                    ))
                    .as_ptr(),
                    b"moving surplus data from offset %zu to offset %zu\x00" as *const u8
                        as *const libc::c_char,
                    ((*urb).buffer as *mut libc::c_uchar).wrapping_offset_from((*transfer).buffer)
                        as libc::c_long,
                    target.wrapping_offset_from((*transfer).buffer) as libc::c_long,
                );
                memmove(
                    target as *mut libc::c_void,
                    (*urb).buffer,
                    (*urb).actual_length as libc::c_ulong,
                );
            }
            (*itransfer).transferred += (*urb).actual_length
        }
        if (*tpriv).num_retired == (*tpriv).num_urbs {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                    b"handle_bulk_completion\x00",
                ))
                .as_ptr(),
                b"abnormal reap: last URB handled, reporting\x00" as *const u8
                    as *const libc::c_char,
            );
            if (*tpriv).reap_action as libc::c_uint
                != COMPLETED_EARLY as libc::c_int as libc::c_uint
                && (*tpriv).reap_status as libc::c_uint
                    == LIBUSB_TRANSFER_COMPLETED as libc::c_int as libc::c_uint
            {
                (*tpriv).reap_status = LIBUSB_TRANSFER_ERROR
            }
            current_block = 5507434738182830386;
        } else {
            current_block = 15484932834559361664;
        }
    } else {
        (*itransfer).transferred += (*urb).actual_length;
        /* Many of these errors can occur on *any* urb of a multi-urb
         * transfer.  When they do, we tear down the rest of the transfer.
         */
        match (*urb).status {
            -121 => {
                /* short transfer */
                current_block = 14072441030219150333;
            }
            -2 => {
                /* cancelled */
                current_block = 14072441030219150333;
            }
            0 | -104 => {
                current_block = 14072441030219150333;
            }
            -19 | -108 => {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"handle_bulk_completion\x00",
                    ))
                    .as_ptr(),
                    b"device removed\x00" as *const u8 as *const libc::c_char,
                );
                (*tpriv).reap_status = LIBUSB_TRANSFER_NO_DEVICE;
                current_block = 18374883566288093865;
            }
            -32 => {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"handle_bulk_completion\x00",
                    ))
                    .as_ptr(),
                    b"detected endpoint stall\x00" as *const u8 as *const libc::c_char,
                );
                if (*tpriv).reap_status as libc::c_uint
                    == LIBUSB_TRANSFER_COMPLETED as libc::c_int as libc::c_uint
                {
                    (*tpriv).reap_status = LIBUSB_TRANSFER_STALL
                }
                current_block = 18374883566288093865;
            }
            -75 => {
                /* overflow can only ever occur in the last urb */
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"handle_bulk_completion\x00",
                    ))
                    .as_ptr(),
                    b"overflow, actual_length=%d\x00" as *const u8 as *const libc::c_char,
                    (*urb).actual_length,
                );
                if (*tpriv).reap_status as libc::c_uint
                    == LIBUSB_TRANSFER_COMPLETED as libc::c_int as libc::c_uint
                {
                    (*tpriv).reap_status = LIBUSB_TRANSFER_OVERFLOW
                }
                current_block = 5507434738182830386;
            }
            -62 | -71 | -84 | -70 | -63 => {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"handle_bulk_completion\x00",
                    ))
                    .as_ptr(),
                    b"low-level bus error %d\x00" as *const u8 as *const libc::c_char,
                    (*urb).status,
                );
                (*tpriv).reap_action = ERROR;
                current_block = 18374883566288093865;
            }
            _ => {
                usbi_log(
                    (*(*(*((itransfer as *mut libc::c_uchar).offset(
                        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
                            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                            as isize,
                    ) as *mut libusb_transfer))
                        .dev_handle)
                        .dev)
                        .ctx,
                    LIBUSB_LOG_LEVEL_WARNING,
                    (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        b"handle_bulk_completion\x00",
                    ))
                    .as_ptr(),
                    b"unrecognised urb status %d\x00" as *const u8 as *const libc::c_char,
                    (*urb).status,
                );
                (*tpriv).reap_action = ERROR;
                current_block = 18374883566288093865;
            }
        }
        match current_block {
            5507434738182830386 => {}
            _ => {
                match current_block {
                    14072441030219150333 =>
                    /* if we've reaped all urbs or we got less data than requested then we're
                     * done */
                    {
                        if (*tpriv).num_retired == (*tpriv).num_urbs {
                            usbi_log(
                                0 as *mut libusb_context,
                                LIBUSB_LOG_LEVEL_DEBUG,
                                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                                    b"handle_bulk_completion\x00",
                                ))
                                .as_ptr(),
                                b"all URBs in transfer reaped --> complete!\x00" as *const u8
                                    as *const libc::c_char,
                            );
                            current_block = 5507434738182830386;
                        } else if (*urb).actual_length < (*urb).buffer_length {
                            usbi_log(
                                0 as *mut libusb_context,
                                LIBUSB_LOG_LEVEL_DEBUG,
                                (*::std::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                                    b"handle_bulk_completion\x00",
                                ))
                                .as_ptr(),
                                b"short transfer %d/%d --> complete!\x00" as *const u8
                                    as *const libc::c_char,
                                (*urb).actual_length,
                                (*urb).buffer_length,
                            );
                            if (*tpriv).reap_action as libc::c_uint
                                == NORMAL as libc::c_int as libc::c_uint
                            {
                                (*tpriv).reap_action = COMPLETED_EARLY
                            }
                            current_block = 18374883566288093865;
                        } else {
                            current_block = 15484932834559361664;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    5507434738182830386 => {}
                    15484932834559361664 => {}
                    _ => {
                        if (*tpriv).reap_action as libc::c_uint
                            == ERROR as libc::c_int as libc::c_uint
                            && (*tpriv).reap_status as libc::c_uint
                                == LIBUSB_TRANSFER_COMPLETED as libc::c_int as libc::c_uint
                        {
                            (*tpriv).reap_status = LIBUSB_TRANSFER_ERROR
                        }
                        if (*tpriv).num_retired == (*tpriv).num_urbs {
                            current_block = 5507434738182830386;
                        } else {
                            /* cancel remaining urbs and wait for their completion before
                             * reporting results */
                            discard_urbs(itransfer, urb_idx + 1 as libc::c_int, (*tpriv).num_urbs);
                            current_block = 15484932834559361664;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        5507434738182830386 =>
        /* nothing to cancel */
        {
            free((*tpriv).c2rust_unnamed.urbs as *mut libc::c_void);
            (*tpriv).c2rust_unnamed.urbs = 0 as *mut usbfs_urb;
            usbi_mutex_unlock(&mut (*itransfer).lock);
            return if (*tpriv).reap_action as libc::c_uint
                == CANCELLED as libc::c_int as libc::c_uint
            {
                usbi_handle_transfer_cancellation(itransfer)
            } else {
                usbi_handle_transfer_completion(itransfer, (*tpriv).reap_status)
            };
        }
        _ => {
            usbi_mutex_unlock(&mut (*itransfer).lock);
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn handle_iso_completion(
    itransfer: *mut usbi_transfer,
    urb: *mut usbfs_urb,
) -> libc::c_int {
    let transfer: *mut libusb_transfer = (itransfer as *mut libc::c_uchar).offset(
        ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
    ) as *mut libusb_transfer;
    let mut tpriv: *mut linux_transfer_priv =
        usbi_get_transfer_priv(itransfer) as *mut linux_transfer_priv;
    let num_urbs: libc::c_int = (*tpriv).num_urbs;
    let mut urb_idx: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut status: libusb_transfer_status = LIBUSB_TRANSFER_COMPLETED;
    usbi_mutex_lock(&mut (*itransfer).lock);
    i = 0 as libc::c_int;
    while i < num_urbs {
        if urb == *(*tpriv).c2rust_unnamed.iso_urbs.offset(i as isize) {
            urb_idx = i + 1 as libc::c_int;
            break;
        } else {
            i += 1
        }
    }
    if urb_idx == 0 as libc::c_int {
        usbi_log(
            (*(*(*transfer).dev_handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"handle_iso_completion\x00",
            ))
            .as_ptr(),
            b"could not locate urb!\x00" as *const u8 as *const libc::c_char,
        );
        usbi_mutex_unlock(&mut (*itransfer).lock);
        return LIBUSB_ERROR_NOT_FOUND as libc::c_int;
    }
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"handle_iso_completion\x00"))
            .as_ptr(),
        b"handling completion status %d of iso urb %d/%d\x00" as *const u8 as *const libc::c_char,
        (*urb).status,
        urb_idx,
        num_urbs,
    );
    /* copy isochronous results back in */
    i = 0 as libc::c_int;
    while i < (*urb).c2rust_unnamed.number_of_packets {
        let urb_desc: *mut usbfs_iso_packet_desc =
            &mut *(*urb).iso_frame_desc.as_mut_ptr().offset(i as isize)
                as *mut usbfs_iso_packet_desc;
        let fresh3 = (*tpriv).iso_packet_offset;
        (*tpriv).iso_packet_offset = (*tpriv).iso_packet_offset + 1;
        let mut lib_desc: *mut libusb_iso_packet_descriptor = &mut *(*transfer)
            .iso_packet_desc
            .as_mut_ptr()
            .offset(fresh3 as isize)
            as *mut libusb_iso_packet_descriptor;
        (*lib_desc).status = LIBUSB_TRANSFER_COMPLETED;
        match (*urb_desc).status {
            0 | 4294967294 | 4294967192 => {}
            4294967277 | 4294967188 => {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"handle_iso_completion\x00",
                    ))
                    .as_ptr(),
                    b"packet %d - device removed\x00" as *const u8 as *const libc::c_char,
                    i,
                );
                (*lib_desc).status = LIBUSB_TRANSFER_NO_DEVICE
            }
            4294967264 => {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"handle_iso_completion\x00",
                    ))
                    .as_ptr(),
                    b"packet %d - detected endpoint stall\x00" as *const u8 as *const libc::c_char,
                    i,
                );
                (*lib_desc).status = LIBUSB_TRANSFER_STALL
            }
            4294967221 => {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"handle_iso_completion\x00",
                    ))
                    .as_ptr(),
                    b"packet %d - overflow error\x00" as *const u8 as *const libc::c_char,
                    i,
                );
                (*lib_desc).status = LIBUSB_TRANSFER_OVERFLOW
            }
            4294967234 | 4294967225 | 4294967212 | 4294967226 | 4294967233 | 4294967278 => {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"handle_iso_completion\x00",
                    ))
                    .as_ptr(),
                    b"packet %d - low-level USB error %d\x00" as *const u8 as *const libc::c_char,
                    i,
                    (*urb_desc).status,
                );
                (*lib_desc).status = LIBUSB_TRANSFER_ERROR
            }
            _ => {
                usbi_log(
                    (*(*(*transfer).dev_handle).dev).ctx,
                    LIBUSB_LOG_LEVEL_WARNING,
                    (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"handle_iso_completion\x00",
                    ))
                    .as_ptr(),
                    b"packet %d - unrecognised urb status %d\x00" as *const u8
                        as *const libc::c_char,
                    i,
                    (*urb_desc).status,
                );
                (*lib_desc).status = LIBUSB_TRANSFER_ERROR
            }
        }
        (*lib_desc).actual_length = (*urb_desc).actual_length;
        i += 1
    }
    (*tpriv).num_retired += 1;
    if (*tpriv).reap_action as libc::c_uint != NORMAL as libc::c_int as libc::c_uint {
        /* cancelled or submit_fail */
        usbi_log(
            0 as *mut libusb_context,
            LIBUSB_LOG_LEVEL_DEBUG,
            (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                b"handle_iso_completion\x00",
            ))
            .as_ptr(),
            b"CANCEL: urb status %d\x00" as *const u8 as *const libc::c_char,
            (*urb).status,
        );
        if (*tpriv).num_retired == num_urbs {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"handle_iso_completion\x00",
                ))
                .as_ptr(),
                b"CANCEL: last URB handled, reporting\x00" as *const u8 as *const libc::c_char,
            );
            free_iso_urbs(tpriv);
            if (*tpriv).reap_action as libc::c_uint == CANCELLED as libc::c_int as libc::c_uint {
                usbi_mutex_unlock(&mut (*itransfer).lock);
                return usbi_handle_transfer_cancellation(itransfer);
            } else {
                usbi_mutex_unlock(&mut (*itransfer).lock);
                return usbi_handle_transfer_completion(itransfer, LIBUSB_TRANSFER_ERROR);
            }
        }
    } else {
        match (*urb).status {
            0 | -2 | -104 => {}
            -108 => {
                usbi_log(
                    0 as *mut libusb_context,
                    LIBUSB_LOG_LEVEL_DEBUG,
                    (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"handle_iso_completion\x00",
                    ))
                    .as_ptr(),
                    b"device removed\x00" as *const u8 as *const libc::c_char,
                );
                status = LIBUSB_TRANSFER_NO_DEVICE
            }
            _ => {
                usbi_log(
                    (*(*(*transfer).dev_handle).dev).ctx,
                    LIBUSB_LOG_LEVEL_WARNING,
                    (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                        b"handle_iso_completion\x00",
                    ))
                    .as_ptr(),
                    b"unrecognised urb status %d\x00" as *const u8 as *const libc::c_char,
                    (*urb).status,
                );
                status = LIBUSB_TRANSFER_ERROR
            }
        }
        /* if we've reaped all urbs then we're done */
        if (*tpriv).num_retired == num_urbs {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                    b"handle_iso_completion\x00",
                ))
                .as_ptr(),
                b"all URBs in transfer reaped --> complete!\x00" as *const u8
                    as *const libc::c_char,
            );
            free_iso_urbs(tpriv);
            usbi_mutex_unlock(&mut (*itransfer).lock);
            return usbi_handle_transfer_completion(itransfer, status);
        }
    }
    usbi_mutex_unlock(&mut (*itransfer).lock);
    return 0 as libc::c_int;
}
unsafe extern "C" fn handle_control_completion(
    mut itransfer: *mut usbi_transfer,
    urb: *mut usbfs_urb,
) -> libc::c_int {
    let mut tpriv: *mut linux_transfer_priv =
        usbi_get_transfer_priv(itransfer) as *mut linux_transfer_priv;
    let mut status: libc::c_int = 0;
    usbi_mutex_lock(&mut (*itransfer).lock);
    usbi_log(
        0 as *mut libusb_context,
        LIBUSB_LOG_LEVEL_DEBUG,
        (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
            b"handle_control_completion\x00",
        ))
        .as_ptr(),
        b"handling completion status %d\x00" as *const u8 as *const libc::c_char,
        (*urb).status,
    );
    (*itransfer).transferred += (*urb).actual_length;
    if (*tpriv).reap_action as libc::c_uint == CANCELLED as libc::c_int as libc::c_uint {
        if (*urb).status != 0 && (*urb).status != -(2 as libc::c_int) {
            usbi_log(
                (*(*(*((itransfer as *mut libc::c_uchar).offset(
                    ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
                        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as isize,
                ) as *mut libusb_transfer))
                    .dev_handle)
                    .dev)
                    .ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"handle_control_completion\x00",
                ))
                .as_ptr(),
                b"cancel: unrecognised urb status %d\x00" as *const u8 as *const libc::c_char,
                (*urb).status,
            );
        }
        free((*tpriv).c2rust_unnamed.urbs as *mut libc::c_void);
        (*tpriv).c2rust_unnamed.urbs = 0 as *mut usbfs_urb;
        usbi_mutex_unlock(&mut (*itransfer).lock);
        return usbi_handle_transfer_cancellation(itransfer);
    }
    match (*urb).status {
        0 => status = LIBUSB_TRANSFER_COMPLETED as libc::c_int,
        -2 => {
            /* cancelled */
            status = LIBUSB_TRANSFER_CANCELLED as libc::c_int
        }
        -19 | -108 => {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"handle_control_completion\x00",
                ))
                .as_ptr(),
                b"device removed\x00" as *const u8 as *const libc::c_char,
            );
            status = LIBUSB_TRANSFER_NO_DEVICE as libc::c_int
        }
        -32 => {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"handle_control_completion\x00",
                ))
                .as_ptr(),
                b"unsupported control request\x00" as *const u8 as *const libc::c_char,
            );
            status = LIBUSB_TRANSFER_STALL as libc::c_int
        }
        -75 => {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"handle_control_completion\x00",
                ))
                .as_ptr(),
                b"overflow, actual_length=%d\x00" as *const u8 as *const libc::c_char,
                (*urb).actual_length,
            );
            status = LIBUSB_TRANSFER_OVERFLOW as libc::c_int
        }
        -62 | -71 | -84 | -70 | -63 => {
            usbi_log(
                0 as *mut libusb_context,
                LIBUSB_LOG_LEVEL_DEBUG,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"handle_control_completion\x00",
                ))
                .as_ptr(),
                b"low-level bus error %d\x00" as *const u8 as *const libc::c_char,
                (*urb).status,
            );
            status = LIBUSB_TRANSFER_ERROR as libc::c_int
        }
        _ => {
            usbi_log(
                (*(*(*((itransfer as *mut libc::c_uchar).offset(
                    ((::std::mem::size_of::<usbi_transfer>() as libc::c_ulong).wrapping_add(
                        (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    ) & !(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        as isize,
                ) as *mut libusb_transfer))
                    .dev_handle)
                    .dev)
                    .ctx,
                LIBUSB_LOG_LEVEL_WARNING,
                (*::std::mem::transmute::<&[u8; 26], &[libc::c_char; 26]>(
                    b"handle_control_completion\x00",
                ))
                .as_ptr(),
                b"unrecognised urb status %d\x00" as *const u8 as *const libc::c_char,
                (*urb).status,
            );
            status = LIBUSB_TRANSFER_ERROR as libc::c_int
        }
    }
    free((*tpriv).c2rust_unnamed.urbs as *mut libc::c_void);
    (*tpriv).c2rust_unnamed.urbs = 0 as *mut usbfs_urb;
    usbi_mutex_unlock(&mut (*itransfer).lock);
    return usbi_handle_transfer_completion(itransfer, status as libusb_transfer_status);
}
unsafe extern "C" fn reap_for_handle(handle: *mut libusb_device_handle) -> libc::c_int {
    let hpriv: *mut linux_device_handle_priv =
        usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
    let mut r: libc::c_int = 0;
    let mut urb: *mut usbfs_urb = 0 as *mut usbfs_urb;
    let mut itransfer: *mut usbi_transfer = 0 as *mut usbi_transfer;
    let mut transfer: *mut libusb_transfer = 0 as *mut libusb_transfer;
    r = ioctl(
        (*hpriv).fd,
        ((1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int + 14 as libc::c_int
            | (('U' as i32) << 0 as libc::c_int + 8 as libc::c_int) as libc::c_uint
            | ((13 as libc::c_int) << 0 as libc::c_int) as libc::c_uint) as libc::c_ulong
            | (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                << 0 as libc::c_int + 8 as libc::c_int + 8 as libc::c_int,
        &mut urb as *mut *mut usbfs_urb,
    );
    if r < 0 as libc::c_int {
        if *__errno_location() == 11 as libc::c_int {
            return 1 as libc::c_int;
        }
        if *__errno_location() == 19 as libc::c_int {
            return LIBUSB_ERROR_NO_DEVICE as libc::c_int;
        }
        usbi_log(
            (*(*handle).dev).ctx,
            LIBUSB_LOG_LEVEL_ERROR,
            (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"reap_for_handle\x00"))
                .as_ptr(),
            b"reap failed, errno=%d\x00" as *const u8 as *const libc::c_char,
            *__errno_location(),
        );
        return LIBUSB_ERROR_IO as libc::c_int;
    }
    itransfer = (*urb).usercontext as *mut usbi_transfer;
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
        (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"reap_for_handle\x00")).as_ptr(),
        b"urb type=%u status=%d transferred=%d\x00" as *const u8 as *const libc::c_char,
        (*urb).type_0 as libc::c_int,
        (*urb).status,
        (*urb).actual_length,
    );
    match (*transfer).type_0 as libc::c_int {
        1 => return handle_iso_completion(itransfer, urb),
        2 | 4 | 3 => return handle_bulk_completion(itransfer, urb),
        0 => return handle_control_completion(itransfer, urb),
        _ => {
            usbi_log(
                (*(*handle).dev).ctx,
                LIBUSB_LOG_LEVEL_ERROR,
                (*::std::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"reap_for_handle\x00"))
                    .as_ptr(),
                b"unrecognised transfer type %u\x00" as *const u8 as *const libc::c_char,
                (*transfer).type_0 as libc::c_int,
            );
            return LIBUSB_ERROR_OTHER as libc::c_int;
        }
    };
}
unsafe extern "C" fn op_handle_events(
    ctx: *mut libusb_context,
    fds: *mut pollfd,
    nfds: usbi_nfds_t,
    mut num_ready: libc::c_int,
) -> libc::c_int {
    let current_block: u64;
    let mut n: usbi_nfds_t = 0;
    let mut r: libc::c_int = 0;
    usbi_mutex_lock(&mut (*ctx).open_devs_lock);
    n = 0 as libc::c_int as usbi_nfds_t;
    loop {
        if !(n < nfds && num_ready > 0 as libc::c_int) {
            current_block = 14434620278749266018;
            break;
        }
        let pollfd: *mut pollfd = &mut *fds.offset(n as isize) as *mut pollfd;
        let mut handle: *mut libusb_device_handle = 0 as *mut libusb_device_handle;
        let mut hpriv: *mut linux_device_handle_priv = 0 as *mut linux_device_handle_priv;
        if !((*pollfd).revents == 0) {
            num_ready -= 1;
            handle = ((*ctx).open_devs.next as uintptr_t).wrapping_sub(48 as libc::c_ulong)
                as *mut libusb_device_handle;
            while &mut (*handle).list as *mut list_head != &mut (*ctx).open_devs as *mut list_head {
                hpriv = usbi_get_device_handle_priv(handle) as *mut linux_device_handle_priv;
                if (*hpriv).fd == (*pollfd).fd {
                    break;
                }
                handle = ((*handle).list.next as uintptr_t).wrapping_sub(48 as libc::c_ulong)
                    as *mut libusb_device_handle
            }
            if hpriv.is_null() || (*hpriv).fd != (*pollfd).fd {
                usbi_log(
                    ctx,
                    LIBUSB_LOG_LEVEL_ERROR,
                    (*::std::mem::transmute::<&[u8; 17], &[libc::c_char; 17]>(
                        b"op_handle_events\x00",
                    ))
                    .as_ptr(),
                    b"cannot find handle for fd %d\x00" as *const u8 as *const libc::c_char,
                    (*pollfd).fd,
                );
            } else if (*pollfd).revents as libc::c_int & 0x8 as libc::c_int != 0 {
                /* remove the fd from the pollfd set so that it doesn't continuously
                 * trigger an event, and flag that it has been removed so op_close()
                 * doesn't try to remove it a second time */
                usbi_remove_pollfd((*(*handle).dev).ctx, (*hpriv).fd);
                (*hpriv).fd_removed = 1 as libc::c_int;
                /* device will still be marked as attached if hotplug monitor thread
                 * hasn't processed remove event yet */
                usbi_mutex_static_lock(&mut linux_hotplug_lock);
                if (*(*handle).dev).attached != 0 {
                    linux_device_disconnected(
                        (*(*handle).dev).bus_number,
                        (*(*handle).dev).device_address,
                    );
                }
                usbi_mutex_static_unlock(&mut linux_hotplug_lock);
                if (*hpriv).caps & 0x10 as libc::c_int as libc::c_uint != 0 {
                    loop {
                        r = reap_for_handle(handle);
                        if !(r == 0 as libc::c_int) {
                            break;
                        }
                    }
                }
                usbi_handle_disconnect(handle);
            } else {
                loop {
                    r = reap_for_handle(handle);
                    if !(r == 0 as libc::c_int) {
                        break;
                    }
                }
                if !(r == 1 as libc::c_int || r == LIBUSB_ERROR_NO_DEVICE as libc::c_int) {
                    if r < 0 as libc::c_int {
                        current_block = 1774598534424886974;
                        break;
                    }
                }
            }
        }
        n = n.wrapping_add(1)
    }
    match current_block {
        14434620278749266018 => r = 0 as libc::c_int,
        _ => {}
    }
    usbi_mutex_unlock(&mut (*ctx).open_devs_lock);
    return r;
}
#[no_mangle]
pub static mut usbi_backend: usbi_os_backend = unsafe {
    {
        let init = usbi_os_backend {
            name: b"Linux usbfs\x00" as *const u8 as *const libc::c_char,
            caps: (0x10000 as libc::c_int | 0x20000 as libc::c_int) as uint32_t,
            init: Some(op_init as unsafe extern "C" fn(_: *mut libusb_context) -> libc::c_int),
            exit: Some(op_exit as unsafe extern "C" fn(_: *mut libusb_context) -> ()),
            set_option: None,
            get_device_list: None,
            hotplug_poll: Some(op_hotplug_poll as unsafe extern "C" fn() -> ()),
            wrap_sys_device: Some(
                op_wrap_sys_device
                    as unsafe extern "C" fn(
                        _: *mut libusb_context,
                        _: *mut libusb_device_handle,
                        _: intptr_t,
                    ) -> libc::c_int,
            ),
            open: Some(
                op_open as unsafe extern "C" fn(_: *mut libusb_device_handle) -> libc::c_int,
            ),
            close: Some(op_close as unsafe extern "C" fn(_: *mut libusb_device_handle) -> ()),
            get_active_config_descriptor: Some(
                op_get_active_config_descriptor
                    as unsafe extern "C" fn(
                        _: *mut libusb_device,
                        _: *mut libc::c_void,
                        _: size_t,
                    ) -> libc::c_int,
            ),
            get_config_descriptor: Some(
                op_get_config_descriptor
                    as unsafe extern "C" fn(
                        _: *mut libusb_device,
                        _: uint8_t,
                        _: *mut libc::c_void,
                        _: size_t,
                    ) -> libc::c_int,
            ),
            get_config_descriptor_by_value: Some(
                op_get_config_descriptor_by_value
                    as unsafe extern "C" fn(
                        _: *mut libusb_device,
                        _: uint8_t,
                        _: *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            get_configuration: Some(
                op_get_configuration
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: *mut uint8_t,
                    ) -> libc::c_int,
            ),
            set_configuration: Some(
                op_set_configuration
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: libc::c_int,
                    ) -> libc::c_int,
            ),
            claim_interface: Some(
                op_claim_interface
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: uint8_t,
                    ) -> libc::c_int,
            ),
            release_interface: Some(
                op_release_interface
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: uint8_t,
                    ) -> libc::c_int,
            ),
            set_interface_altsetting: Some(
                op_set_interface
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: uint8_t,
                        _: uint8_t,
                    ) -> libc::c_int,
            ),
            clear_halt: Some(
                op_clear_halt
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: libc::c_uchar,
                    ) -> libc::c_int,
            ),
            reset_device: Some(
                op_reset_device
                    as unsafe extern "C" fn(_: *mut libusb_device_handle) -> libc::c_int,
            ),
            alloc_streams: Some(
                op_alloc_streams
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: uint32_t,
                        _: *mut libc::c_uchar,
                        _: libc::c_int,
                    ) -> libc::c_int,
            ),
            free_streams: Some(
                op_free_streams
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: *mut libc::c_uchar,
                        _: libc::c_int,
                    ) -> libc::c_int,
            ),
            dev_mem_alloc: Some(
                op_dev_mem_alloc
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: size_t,
                    ) -> *mut libc::c_void,
            ),
            dev_mem_free: Some(
                op_dev_mem_free
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: *mut libc::c_void,
                        _: size_t,
                    ) -> libc::c_int,
            ),
            kernel_driver_active: Some(
                op_kernel_driver_active
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: uint8_t,
                    ) -> libc::c_int,
            ),
            detach_kernel_driver: Some(
                op_detach_kernel_driver
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: uint8_t,
                    ) -> libc::c_int,
            ),
            attach_kernel_driver: Some(
                op_attach_kernel_driver
                    as unsafe extern "C" fn(
                        _: *mut libusb_device_handle,
                        _: uint8_t,
                    ) -> libc::c_int,
            ),
            destroy_device: Some(
                op_destroy_device as unsafe extern "C" fn(_: *mut libusb_device) -> (),
            ),
            submit_transfer: Some(
                op_submit_transfer as unsafe extern "C" fn(_: *mut usbi_transfer) -> libc::c_int,
            ),
            cancel_transfer: Some(
                op_cancel_transfer as unsafe extern "C" fn(_: *mut usbi_transfer) -> libc::c_int,
            ),
            clear_transfer_priv: Some(
                op_clear_transfer_priv as unsafe extern "C" fn(_: *mut usbi_transfer) -> (),
            ),
            handle_events: Some(
                op_handle_events
                    as unsafe extern "C" fn(
                        _: *mut libusb_context,
                        _: *mut pollfd,
                        _: usbi_nfds_t,
                        _: libc::c_int,
                    ) -> libc::c_int,
            ),
            handle_transfer_completion: None,
            context_priv_size: 0,
            device_priv_size: ::std::mem::size_of::<linux_device_priv>() as libc::c_ulong,
            device_handle_priv_size: ::std::mem::size_of::<linux_device_handle_priv>()
                as libc::c_ulong,
            transfer_priv_size: ::std::mem::size_of::<linux_transfer_priv>() as libc::c_ulong,
        };
        init
    }
};
