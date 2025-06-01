crate::ix!();

///////////////////////////////////////////////////////////////////////////////
// Function pointer types matching the extern "C" signatures in libamidi.
///////////////////////////////////////////////////////////////////////////////

pub type AMidiDeviceFromJavaFn = unsafe extern "C" fn(
    env: *mut JNIEnv,
    midi_device_obj: jobject,
    out_device_ptr: *mut *mut AMidiDevice,
) -> media_status_t;

pub type AMidiDeviceReleaseFn = unsafe extern "C" fn(
    midi_device: *const AMidiDevice,
) -> media_status_t;

pub type AMidiDeviceGetTypeFn = unsafe extern "C" fn(
    device: *const AMidiDevice,
) -> i32;

pub type AMidiDeviceGetNumInputPortsFn = unsafe extern "C" fn(
    device: *const AMidiDevice,
) -> isize;

pub type AMidiDeviceGetNumOutputPortsFn = unsafe extern "C" fn(
    device: *const AMidiDevice,
) -> isize;

pub type AMidiDeviceGetDefaultProtocolFn = unsafe extern "C" fn(
    device: *const AMidiDevice,
) -> AMidiDevice_Protocol;

pub type AMidiOutputPortOpenFn = unsafe extern "C" fn(
    device: *const AMidiDevice,
    port_number: i32,
    out_port_ptr: *mut *mut AMidiOutputPort,
) -> media_status_t;

pub type AMidiOutputPortCloseFn = unsafe extern "C" fn(
    output_port: *const AMidiOutputPort,
);

pub type AMidiOutputPortReceiveFn = unsafe extern "C" fn(
    output_port:            *const AMidiOutputPort,
    opcode_ptr:             *mut i32,
    buffer:                 *mut u8,
    max_bytes:              usize,
    num_bytes_received_ptr: *mut usize,
    out_timestamp_ptr:      *mut i64,
) -> isize;

pub type AMidiInputPortOpenFn = unsafe extern "C" fn(
    device:       *const AMidiDevice,
    port_number:  i32,
    out_port_ptr: *mut *mut AMidiInputPort,
) -> media_status_t;

pub type AMidiInputPortSendFn = unsafe extern "C" fn(
    input_port: *const AMidiInputPort,
    buffer:     *const u8,
    num_bytes:  usize,
) -> isize;

pub type AMidiInputPortSendWithTimestampFn = unsafe extern "C" fn(
    input_port: *const AMidiInputPort,
    buffer:     *const u8,
    num_bytes:  usize,
    timestamp:  i64,
) -> isize;

pub type AMidiInputPortSendFlushFn = unsafe extern "C" fn(
    input_port: *const AMidiInputPort,
) -> media_status_t;

pub type AMidiInputPortCloseFn = unsafe extern "C" fn(
    input_port: *const AMidiInputPort,
);
