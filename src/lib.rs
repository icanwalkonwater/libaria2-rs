pub type A2Gid = u64;

#[cxx::bridge(namespace = "aria2::bridge")]
pub mod ffi {
    #[derive(Copy, Clone)]
    pub struct SessionHandle {
        // Not really a usize but a `*mut Session`
        ptr: usize,
    }

    pub enum DownloadEventFfi {
        Start = 1,
        Pause,
        Stop,
        Complete,
        Error,
        BtComplete,
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

    pub enum RunModeFfi {
        Default,
        Once,
    }

    extern "Rust" {
        fn download_event_callback(session: SessionHandle, event: DownloadEventFfi, gid: u64, user_data: usize) -> i32;
    }

    extern "C++" {
        include!("libaria2/include/aria2_bridge.hpp");

        pub unsafe fn library_init() -> i32;
        pub unsafe fn library_deinit() -> i32;
        pub unsafe fn session_new(options: &Vec<KeyVal>, config: &SessionConfigFfi) -> SessionHandle;
        pub unsafe fn session_final(session: SessionHandle) -> i32;
    }
}

fn download_event_callback(session: ffi::SessionHandle, event: ffi::DownloadEventFfi, gid: A2Gid, user_data: usize) -> i32 {
    println!("Event callback");
    0
}
