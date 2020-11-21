#![allow(non_camel_case_types)]

use crate::ffi::SessionHandle;

pub type A2Gid = u64;
pub type RunMode = ffi::RUN_MODE;

#[rustfmt::skip]
#[cxx::bridge(namespace = "aria2::bridge")]
pub mod ffi {
    #[derive(Copy, Clone)]
    pub struct SessionHandle {
        // Not really a usize but a `*mut Session`
        ptr: usize,
    }

    #[repr(u32)]
    pub enum DownloadEvent {
        EVENT_ON_DOWNLOAD_START = 1,
        EVENT_ON_DOWNLOAD_PAUSE,
        EVENT_ON_DOWNLOAD_STOP,
        EVENT_ON_DOWNLOAD_COMPLETE,
        EVENT_ON_DOWNLOAD_ERROR,
        EVENT_ON_BT_DOWNLOAD_COMPLETE,
    }

    pub struct SessionConfigFfi {
        pub keep_running: bool,
        pub use_signal_handler: bool,
        pub user_data: usize,
    }

    pub struct KeyVal {
        pub key: String,
        pub val: String,
    }

    #[repr(u32)]
    pub enum RUN_MODE {
        RUN_DEFAULT,
        RUN_ONCE,
    }

    extern "Rust" {
        fn download_event_callback(
            session: SessionHandle,
            event: DownloadEvent,
            gid: u64,
            user_data: usize,
        ) -> i32;
    }

    unsafe extern "C++" {
        include!("libaria2/include/aria2_bridge.hpp");

        #[namespace = "aria2"]
        type A2Gid;
        #[namespace = "aria2"]
        type DownloadEvent;
        #[namespace = "aria2"]
        type RUN_MODE;

        pub unsafe fn library_init() -> i32;
        pub unsafe fn library_deinit() -> i32;

        pub unsafe fn session_new(
            options: &Vec<KeyVal>,
            config: &SessionConfigFfi,
            // cb: fn(SessionHandle, DownloadEvent, A2Gid, usize) -> i32,
            cb: fn(SessionHandle, DownloadEvent, u64, usize) -> i32,
        ) -> SessionHandle;
        pub unsafe fn session_final(session: SessionHandle) -> i32;

        pub unsafe fn run(session: SessionHandle, run_mode: RUN_MODE) -> i32;

        // pub fn gid_to_hex(gid: A2Gid) -> String;
        pub fn gid_to_hex(gid: u64) -> String;

        // pub fn hex_to_gid(hex: &str) -> A2Gid;
        pub fn hex_to_gid(hex: &str) -> u64;

        // pub fn is_gid_null(gid: A2Gid) -> bool;
        pub fn is_gid_null(gid: u64) -> bool;
    }
}

impl SessionHandle {
    pub fn is_valid(&self) -> bool {
        self.ptr != 0
    }
}

fn download_event_callback(
    _session: ffi::SessionHandle,
    _event: ffi::DownloadEvent,
    _gid: A2Gid,
    _user_data: usize,
) -> i32 {
    println!("Event callback");
    0
}
