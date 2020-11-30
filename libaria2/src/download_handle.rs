use crate::session::PollContext;
use libaria2_sys::{cxx, ffi, A2Gid};

#[derive(Copy, Clone, Debug)]
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
