#![allow(non_camel_case_types)]

use crate::ffi::SessionHandle;

pub type A2Gid = u64;
pub type RunMode = ffi::RUN_MODE;

#[derive(Copy, Clone, Debug)]
pub enum DownloadEvent {
    Start,
    Pause,
    Stop,
    Complete,
    Error,
    BtComplete,
}

impl From<ffi::DownloadEvent> for DownloadEvent {
    fn from(raw: ffi::DownloadEvent) -> Self {
        match raw {
            ffi::DownloadEvent::EVENT_ON_DOWNLOAD_START => DownloadEvent::Start,
            ffi::DownloadEvent::EVENT_ON_DOWNLOAD_PAUSE => DownloadEvent::Pause,
            ffi::DownloadEvent::EVENT_ON_DOWNLOAD_STOP => DownloadEvent::Stop,
            ffi::DownloadEvent::EVENT_ON_DOWNLOAD_COMPLETE => DownloadEvent::Complete,
            ffi::DownloadEvent::EVENT_ON_DOWNLOAD_ERROR => DownloadEvent::Error,
            ffi::DownloadEvent::EVENT_ON_BT_DOWNLOAD_COMPLETE => DownloadEvent::BtComplete,
            _ => unreachable!(),
        }
    }
}

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

    unsafe extern "C++" {
        include!("libaria2/include/aria2_bridge.hpp");

        #[namespace = "aria2"]
        type DownloadEvent;
        #[namespace = "aria2"]
        type RUN_MODE;

        pub unsafe fn library_init() -> i32;
        pub unsafe fn library_deinit() -> i32;

        pub unsafe fn session_new(
            options: &Vec<KeyVal>,
            config: &SessionConfigFfi,
            // cb: fn(SessionHandle, DownloadEvent, A2Gid, *const c_void) -> i32,
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

        pub unsafe fn add_uri(
            session: SessionHandle,
            gid: &mut u64,
            uris: &Vec<String>,
            options: &Vec<KeyVal>,
            position: i32
        ) -> i32;

        pub unsafe fn add_metalink(
            session: SessionHandle,
            gids: &mut Vec<u64>,
            metalink_file: &str,
            options: &Vec<KeyVal>,
            position: i32
        ) -> i32;

        pub unsafe fn add_torrent(
            session: SessionHandle,
            gid: &mut u64,
            torrent_file: &str,
            options: &Vec<KeyVal>,
            position: i32,
        ) -> i32;

        pub unsafe fn add_torrent_with_webseed_uris(
            session: SessionHandle,
            gid: &mut u64,
            torrent_file: &str,
            webseed_uris: &Vec<String>,
            options: &Vec<KeyVal>,
            position: i32,
        ) -> i32;

        pub unsafe fn get_active_download(session: SessionHandle) -> Vec<u64>;

        pub unsafe fn remove_download(session: SessionHandle, gid: u64, force: bool) -> i32;

        pub unsafe fn pause_download(session: SessionHandle, gid: u64, force: bool) -> i32;

        pub unsafe fn unpause_download(session: SessionHandle, gid: u64) -> i32;

        pub unsafe fn change_option(session: SessionHandle, gid: u64, options: &Vec<KeyVal>) -> i32;

        pub unsafe fn get_global_option(session: SessionHandle, name: &str) -> &str;

        pub unsafe fn get_global_options(session: SessionHandle) -> Vec<KeyVal>;

        pub unsafe fn change_global_option(session: SessionHandle, options: &Vec<KeyVal>) -> i32;
    }
}

impl SessionHandle {
    pub fn is_valid(&self) -> bool {
        self.ptr != 0
    }
}
