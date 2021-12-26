use libaria2_sys::{cxx, ffi, A2Gid};

use crate::session::PollContext;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DownloadStatus {
    Active,
    Waiting,
    Paused,
    Complete,
    Error,
    Removed,
}

impl From<ffi::DownloadStatus> for DownloadStatus {
    fn from(s: ffi::DownloadStatus) -> Self {
        match s {
            ffi::DownloadStatus::DOWNLOAD_ACTIVE => DownloadStatus::Active,
            ffi::DownloadStatus::DOWNLOAD_WAITING => DownloadStatus::Waiting,
            ffi::DownloadStatus::DOWNLOAD_PAUSED => DownloadStatus::Paused,
            ffi::DownloadStatus::DOWNLOAD_COMPLETE => DownloadStatus::Complete,
            ffi::DownloadStatus::DOWNLOAD_ERROR => DownloadStatus::Error,
            ffi::DownloadStatus::DOWNLOAD_REMOVED => DownloadStatus::Removed,
            _ => unreachable!(),
        }
    }
}

pub struct DownloadHandle<'a> {
    handle: cxx::UniquePtr<ffi::DownloadHandleWrapper>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl DownloadHandle<'_> {
    pub fn status(&self) -> DownloadStatus {
        unsafe { self.handle.status().into() }
    }

    pub fn total_len(&self) -> usize {
        unsafe { self.handle.total_len() }
    }

    pub fn completed_len(&self) -> usize {
        unsafe { self.handle.completed_len() }
    }

    pub fn upload_len(&self) -> usize {
        unsafe { self.handle.upload_len() }
    }

    pub fn bitfield(&self) -> String {
        unsafe { self.handle.bitfield() }
    }

    pub fn download_speed(&self) -> u32 {
        unsafe { self.handle.download_speed() }
    }

    pub fn upload_speed(&self) -> u32 {
        unsafe { self.handle.upload_speed() }
    }

    pub fn info_hash(&self) -> &str {
        unsafe { self.handle.info_hash() }.to_str().unwrap()
    }

    pub fn piece_len(&self) -> usize {
        unsafe { self.handle.piece_len() }
    }

    pub fn num_pieces(&self) -> u32 {
        unsafe { self.handle.num_pieces() }
    }

    pub fn connections(&self) -> u32 {
        unsafe { self.handle.connections() }
    }

    pub fn error_code(&self) -> i32 {
        unsafe { self.handle.error_code() }
    }

    pub fn followed_by(&self) -> &[A2Gid] {
        unsafe { self.handle.followed_by().as_slice() }
    }

    pub fn following(&self) -> Option<A2Gid> {
        let following = unsafe { self.handle.following() };
        if ffi::is_gid_null(following) {
            None
        } else {
            Some(following)
        }
    }

    pub fn belongs_to(&self) -> Option<A2Gid> {
        let belongs = unsafe { self.handle.belongs_to() };
        if ffi::is_gid_null(belongs) {
            None
        } else {
            Some(belongs)
        }
    }

    pub fn dir(&self) -> &str {
        unsafe { self.handle.dir().to_str().unwrap() }
    }

    pub fn files(&self) -> Vec<FileData> {
        (0..self.num_files())
            .into_iter()
            .map(|i| self.get_file(i))
            .collect()
    }

    pub fn num_files(&self) -> u32 {
        unsafe { self.handle.num_files() }
    }

    pub fn get_file(&self, index: u32) -> FileData {
        let file = unsafe { self.handle.get_file(index) };
        FileData { inner: file }
    }

    /*pub fn bt_meta_info(&self) -> ffi::BtMetaInfo {
        unsafe { self.handle.bt_meta_info() }
    }*/

    pub fn get_option(&self, name: &str) -> &str {
        unsafe { self.handle.get_option(name) }.to_str().unwrap()
    }

    pub fn options(&self) -> Vec<ffi::KeyVal> {
        unsafe { self.handle.options() }
    }
}

pub struct FileData {
    inner: cxx::UniquePtr<ffi::FileDataWrapper>,
}

impl PollContext<'_> {
    pub fn acquire_handle(&self, gid: A2Gid) -> Option<DownloadHandle> {
        debug_assert!(!ffi::is_gid_null(gid));
        let handle = unsafe { ffi::get_download_handle(self.handle, gid) };

        if handle.is_null() {
            None
        } else {
            Some(DownloadHandle {
                handle,
                _phantom: Default::default(),
            })
        }
    }
}
