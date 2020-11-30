#![allow(non_camel_case_types)]

use crate::ffi::SessionHandle;
use std::fmt::{Debug, Formatter};

pub use cxx;

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

    #[derive(Copy, Clone)]
    pub struct GlobalStat {
        pub download_speed: i32,
        pub upload_speed: i32,
        pub num_active: i32,
        pub num_waiting: i32,
        pub num_stopped: i32,
    }

    #[repr(u32)]
    pub enum OffsetMode {
        OFFSET_MODE_SET,
        OFFSET_MODE_CUR,
        OFFSET_MODE_END,
    }

    #[repr(u32)]
    pub enum UriStatus {
        URI_USED,
        URI_WAITING,
    }

    /*#[derive(Clone)]
    #[repr(C)]
    pub struct FileData {
        pub index: i32,
        pub path: String,
        pub length: isize,
        pub completed_length: isize,
        pub selected: bool,
        pub uris: Vec<ffi::UriData>,
    }*/

    /*#[derive(Clone)]
    pub struct UriData {
        pub uri: String,
        pub status: UriStatus,
    }*/

    #[repr(u32)]
    pub enum BtFileMode {
        BT_FILE_MODE_NONE,
        BT_FILE_MODE_SINGLE,
        BT_FILE_MODE_MULTI,
    }

    /*#[derive(Clone)]
    pub struct BtMetaInfoData {
        pub announce_list: Vec<Vec<String>>,
        pub comment: String,
        pub creation_date: i32,
        pub mode: BtFileMode,
        pub name: String,
    }*/

    #[repr(u32)]
    pub enum DownloadStatus {
        DOWNLOAD_ACTIVE,
        DOWNLOAD_WAITING,
        DOWNLOAD_PAUSED,
        DOWNLOAD_COMPLETE,
        DOWNLOAD_ERROR,
        DOWNLOAD_REMOVED,
    }

    unsafe extern "C++" {
        include!("libaria2-sys/include/aria2_bridge.hpp");
        include!("libaria2-sys/include/DownloadHandleWrapper.hpp");

        #[namespace = "aria2"]
        type DownloadEvent;
        #[namespace = "aria2"]
        type RUN_MODE;
        #[namespace = "aria2"]
        type OffsetMode;
        /*#[namespace = "aria2"]
        type GlobalStat;*/
        #[namespace = "aria2"]
        type UriStatus;
        #[namespace = "aria2"]
        type BtFileMode;
        #[namespace = "aria2"]
        type DownloadStatus;
        #[namespace = "aria2"]
        type UriData;
        #[namespace = "aria2"]
        type BtMetaInfoData;

        #[namespace = "aria2"]
        #[cxx_name = "libraryInit"]
        pub unsafe fn library_init() -> i32;
        #[namespace = "aria2"]
        #[cxx_name = "libraryDeinit"]
        pub unsafe fn library_deinit() -> i32;

        #[cxx_name = "sessionNew"]
        pub unsafe fn session_new(
            options: &Vec<KeyVal>,
            config: &SessionConfigFfi,
            cb: fn(SessionHandle, DownloadEvent, u64, usize) -> i32,
        ) -> SessionHandle;

        #[cxx_name = "sessionFinal"]
        pub unsafe fn session_final(session: SessionHandle) -> i32;

        #[cxx_name = "run"]
        pub unsafe fn run(session: SessionHandle, run_mode: RUN_MODE) -> i32;
        #[cxx_name = "shutdown"]
        pub unsafe fn shutdown(session: SessionHandle, force: bool) -> i32;

        #[cxx_name = "gidToHex"]
        pub fn gid_to_hex(gid: u64) -> String;
        #[cxx_name = "hexToGid"]
        pub fn hex_to_gid(hex: &str) -> u64;
        #[cxx_name = "isGidNull"]
        pub fn is_gid_null(gid: u64) -> bool;

        #[cxx_name = "addUri"]
        pub unsafe fn add_uri(
            session: SessionHandle,
            gid: &mut u64,
            uris: &Vec<String>,
            options: &Vec<KeyVal>,
            position: i32
        ) -> i32;

        #[cxx_name = "addMetalink"]
        pub unsafe fn add_metalink(
            session: SessionHandle,
            gids: &mut Vec<u64>,
            metalink_file: &str,
            options: &Vec<KeyVal>,
            position: i32
        ) -> i32;

        #[cxx_name = "addTorrent"]
        pub unsafe fn add_torrent(
            session: SessionHandle,
            gid: &mut u64,
            torrent_file: &str,
            options: &Vec<KeyVal>,
            position: i32,
        ) -> i32;

        #[cxx_name = "addTorrentWithWebseedUris"]
        pub unsafe fn add_torrent_with_webseed_uris(
            session: SessionHandle,
            gid: &mut u64,
            torrent_file: &str,
            webseed_uris: &Vec<String>,
            options: &Vec<KeyVal>,
            position: i32,
        ) -> i32;

        #[cxx_name = "getActiveDownload"]
        pub unsafe fn get_active_download(session: SessionHandle) -> Vec<u64>;
        #[cxx_name = "removeDownload"]
        pub unsafe fn remove_download(session: SessionHandle, gid: u64, force: bool) -> i32;
        #[cxx_name = "pauseDownload"]
        pub unsafe fn pause_download(session: SessionHandle, gid: u64, force: bool) -> i32;
        #[cxx_name = "unpauseDownload"]
        pub unsafe fn unpause_download(session: SessionHandle, gid: u64) -> i32;
        #[cxx_name = "changePosition"]
        pub unsafe fn change_position(session: SessionHandle, gid: u64, pos: i32, how: OffsetMode) -> i32;

        #[cxx_name = "changeOption"]
        pub unsafe fn change_option(session: SessionHandle, gid: u64, options: &Vec<KeyVal>) -> i32;
        #[cxx_name = "getGlobalOption"]
        pub unsafe fn get_global_option(session: SessionHandle, name: &str) -> &str;
        #[cxx_name = "getGlobalOptions"]
        pub unsafe fn get_global_options(session: SessionHandle) -> Vec<KeyVal>;
        #[cxx_name = "changeGlobalOption"]
        pub unsafe fn change_global_option(session: SessionHandle, options: &Vec<KeyVal>) -> i32;

        #[cxx_name = "getGlobalStat"]
        pub unsafe fn get_global_stat(session: SessionHandle) -> GlobalStat;

        type UriDataWrapper;
        #[cxx_name = "getUri"]
        pub unsafe fn uri(self: &UriDataWrapper) -> &CxxString;
        #[cxx_name = "getStatus"]
        pub unsafe fn status(self: &UriDataWrapper) -> UriStatus;

        type FileDataWrapper;
        #[cxx_name = "getIndex"]
        pub unsafe fn index(self: &FileDataWrapper) -> u32;
        #[cxx_name = "getPath"]
        pub unsafe fn path(self: &FileDataWrapper) -> &CxxString;
        #[cxx_name = "getLength"]
        pub unsafe fn len(self: &FileDataWrapper) -> u64;
        #[cxx_name = "getCompletedLength"]
        pub unsafe fn completed_len(self: &FileDataWrapper) -> u64;
        #[cxx_name = "isSelected"]
        pub unsafe fn selected(self: &FileDataWrapper) -> bool;
        #[cxx_name = "getUris"]
        pub unsafe fn uris(self: &FileDataWrapper) -> &CxxVector<UriDataWrapper>;

        type DownloadHandleWrapper;
        #[cxx_name = "getStatus"]
        pub unsafe fn status(self: &DownloadHandleWrapper) -> DownloadStatus;
        #[cxx_name = "getTotalLength"]
        pub unsafe fn total_length(self: &DownloadHandleWrapper) -> usize;
        #[cxx_name = "getCompletedLength"]
        pub unsafe fn completed_length(self: &DownloadHandleWrapper) -> usize;
        #[cxx_name = "getUploadLength"]
        pub unsafe fn upload_length(self: &DownloadHandleWrapper) -> usize;
        #[cxx_name = "getBitfield"]
        pub unsafe fn bitfield(self: &DownloadHandleWrapper) -> String;
        #[cxx_name = "getDownloadSpeed"]
        pub unsafe fn download_speed(self: &DownloadHandleWrapper) -> u32;
        #[cxx_name = "getUploadSpeed"]
        pub unsafe fn upload_speed(self: &DownloadHandleWrapper) -> u32;
        #[cxx_name = "getInfoHash"]
        pub unsafe fn info_hash(self: &DownloadHandleWrapper) -> &CxxString;
        #[cxx_name = "getPieceLength"]
        pub unsafe fn piece_length(self: &DownloadHandleWrapper) -> usize;
        #[cxx_name = "getNumPieces"]
        pub unsafe fn num_pieces(self: &DownloadHandleWrapper) -> u32;
        #[cxx_name = "getConnections"]
        pub unsafe fn connections(self: &DownloadHandleWrapper) -> u32;
        #[cxx_name = "getErrorCode"]
        pub unsafe fn error_code(self: &DownloadHandleWrapper) -> i32;
        #[cxx_name = "getFollowedBy"]
        pub unsafe fn followed_by(self: &DownloadHandleWrapper) -> &CxxVector<u64>;
        #[cxx_name = "getFollowing"]
        pub unsafe fn following(self: &DownloadHandleWrapper) -> u64;
        #[cxx_name = "getBelongsTo"]
        pub unsafe fn belongs_to(self: &DownloadHandleWrapper) -> u64;
        #[cxx_name = "getDir"]
        pub unsafe fn directory(self: &DownloadHandleWrapper) -> &CxxString;
        #[cxx_name = "getFiles"]
        pub unsafe fn files(self: &DownloadHandleWrapper) -> UniquePtr<CxxVector<FileDataWrapper>>;
        #[cxx_name = "getNumFiles"]
        pub unsafe fn num_files(self: &DownloadHandleWrapper) -> u32;
        #[cxx_name = "getFile"]
        pub unsafe fn get_file(self: &DownloadHandleWrapper, index: u32) -> UniquePtr<FileDataWrapper>;
        #[cxx_name = "getBtMetaInfo"]
        pub unsafe fn bt_meta_info(self: &DownloadHandleWrapper) -> UniquePtr<BtMetaInfoData>;
        #[cxx_name = "getOption"]
        pub unsafe fn get_option<'a>(self: &'a DownloadHandleWrapper, name: &str) -> &'a CxxString;
        #[cxx_name = "getOptions"]
        pub unsafe fn options(self: &DownloadHandleWrapper) -> Vec<KeyVal>;

        #[cxx_name = "getDownloadHandle"]
        pub unsafe fn get_download_handle(session: SessionHandle, gid: u64) -> UniquePtr<DownloadHandleWrapper>;
        #[cxx_name = "deleteDownloadHandle"]
        pub unsafe fn delete_download_handle(handle: UniquePtr<DownloadHandleWrapper>);
    }
}

impl SessionHandle {
    pub fn is_valid(&self) -> bool {
        self.ptr != 0
    }
}

impl Debug for ffi::DownloadStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::DOWNLOAD_ACTIVE => f.write_str("DOWNLOAD_ACTIVE"),
            Self::DOWNLOAD_WAITING => f.write_str("DOWNLOAD_WAITING"),
            Self::DOWNLOAD_PAUSED => f.write_str("DOWNLOAD_PAUSED"),
            Self::DOWNLOAD_COMPLETE => f.write_str("DOWNLOAD_COMPLETE"),
            Self::DOWNLOAD_ERROR => f.write_str("DOWNLOAD_ERROR"),
            Self::DOWNLOAD_REMOVED => f.write_str("DOWNLOAD_REMOVED"),
            _ => unreachable!(),
        }
    }
}

impl Debug for ffi::DownloadEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::EVENT_ON_DOWNLOAD_START => f.write_str("EVENT_ON_DOWNLOAD_START"),
            Self::EVENT_ON_DOWNLOAD_PAUSE => f.write_str("EVENT_ON_DOWNLOAD_PAUSE"),
            Self::EVENT_ON_DOWNLOAD_COMPLETE => f.write_str("EVENT_ON_DOWNLOAD_COMPLETE"),
            Self::EVENT_ON_BT_DOWNLOAD_COMPLETE => f.write_str("EVENT_ON_BT_DOWNLOAD_COMPLETE"),
            Self::EVENT_ON_DOWNLOAD_STOP => f.write_str("EVENT_ON_DOWNLOAD_STOP"),
            Self::EVENT_ON_DOWNLOAD_ERROR => f.write_str("EVENT_ON_DOWNLOAD_ERROR"),
            _ => unreachable!(),
        }
    }
}
