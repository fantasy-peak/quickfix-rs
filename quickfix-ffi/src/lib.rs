#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi, ptr::NonNull};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixSessionSettings_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixFileStoreFactory_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixFileLogFactory_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixApplication_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixSessionID_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixMessage_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixSocketAcceptor_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FixApplicationCallbacks_t {
    pub onCreate: extern "C" fn(*const ffi::c_void, FixSessionID_t),
    pub onLogon: extern "C" fn(*const ffi::c_void, FixSessionID_t),
    pub onLogout: extern "C" fn(*const ffi::c_void, FixSessionID_t),
    pub toAdmin: extern "C" fn(*const ffi::c_void, FixMessage_t, FixSessionID_t),
    pub toApp: extern "C" fn(*const ffi::c_void, FixMessage_t, FixSessionID_t),
    pub fromAdmin: extern "C" fn(*const ffi::c_void, FixMessage_t, FixSessionID_t),
    pub fromApp: extern "C" fn(*const ffi::c_void, FixMessage_t, FixSessionID_t),
}

#[link(name = "quickfixbind")]
extern "C" {
    pub fn FixSessionSettings_new() -> Option<FixSessionSettings_t>;
    pub fn FixSessionSettings_fromPath(
        configPath: *const ffi::c_char,
    ) -> Option<FixSessionSettings_t>;
    pub fn FixSessionSettings_delete(obj: FixSessionSettings_t);

    pub fn FixFileStoreFactory_new(settings: FixSessionSettings_t)
        -> Option<FixFileStoreFactory_t>;
    pub fn FixFileStoreFactory_delete(obj: FixFileStoreFactory_t);

    pub fn FixFileLogFactory_new(settings: FixSessionSettings_t) -> Option<FixFileLogFactory_t>;
    pub fn FixFileLogFactory_delete(obj: FixFileLogFactory_t);

    pub fn FixApplication_new(
        data: *const ffi::c_void,
        callbacks: *const FixApplicationCallbacks_t,
    ) -> Option<FixApplication_t>;
    pub fn FixApplication_delete(obj: FixApplication_t);

    pub fn FixSocketAcceptor_new(
        application: FixApplication_t,
        storeFactory: FixFileStoreFactory_t,
        settings: FixSessionSettings_t,
        logFactory: FixFileLogFactory_t,
    ) -> Option<FixSocketAcceptor_t>;
    #[must_use]
    pub fn FixSocketAcceptor_start(obj: FixSocketAcceptor_t) -> ffi::c_int;
    #[must_use]
    pub fn FixSocketAcceptor_block(obj: FixSocketAcceptor_t) -> ffi::c_int;
    #[must_use]
    pub fn FixSocketAcceptor_poll(obj: FixSocketAcceptor_t) -> ffi::c_int;
    #[must_use]
    pub fn FixSocketAcceptor_stop(obj: FixSocketAcceptor_t) -> ffi::c_int;
    #[must_use]
    pub fn FixSocketAcceptor_isLoggedOn(obj: FixSocketAcceptor_t) -> ffi::c_int;
    #[must_use]
    pub fn FixSocketAcceptor_isStopped(obj: FixSocketAcceptor_t) -> ffi::c_int;
    pub fn FixSocketAcceptor_delete(obj: FixSocketAcceptor_t);

    pub fn FixSessionID_new(
        beginString: *const ffi::c_char,
        senderCompID: *const ffi::c_char,
        targetCompID: *const ffi::c_char,
        sessionQualifier: *const ffi::c_char,
    ) -> Option<FixSessionID_t>;
    pub fn FixSessionID_copy(src: FixSessionID_t) -> Option<FixSessionID_t>;
    pub fn FixSessionID_getBeginString(obj: FixSessionID_t) -> Option<NonNull<ffi::c_char>>;
    pub fn FixSessionID_getSenderCompID(obj: FixSessionID_t) -> Option<NonNull<ffi::c_char>>;
    pub fn FixSessionID_getTargetCompID(obj: FixSessionID_t) -> Option<NonNull<ffi::c_char>>;
    pub fn FixSessionID_getSessionQualifier(obj: FixSessionID_t) -> Option<NonNull<ffi::c_char>>;
    pub fn FixSessionID_isFIXT(obj: FixSessionID_t) -> i8;
    pub fn FixSessionID_toString(obj: FixSessionID_t) -> Option<NonNull<ffi::c_char>>;
    pub fn FixSessionID_delete(obj: FixSessionID_t);

    pub fn FixMessage_new() -> Option<FixMessage_t>;
    #[must_use]
    pub fn FixMessage_setField(
        obj: FixMessage_t,
        tag: ffi::c_int,
        value: *const ffi::c_char,
    ) -> ffi::c_int;
    #[must_use]
    pub fn FixMessage_getField(obj: FixMessage_t, tag: ffi::c_int) -> Option<NonNull<ffi::c_char>>;
    #[must_use]
    pub fn FixMessage_removeField(obj: FixMessage_t, tag: ffi::c_int) -> ffi::c_int;
    #[must_use]
    pub fn FixMessage_toBuffer(
        obj: FixMessage_t,
        buffer: *mut ffi::c_char,
        length: ffi::c_long,
    ) -> ffi::c_int;
    pub fn FixMessage_delete(obj: FixMessage_t);
}
