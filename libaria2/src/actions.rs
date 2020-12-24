use std::path::Path;

use crate::{
    errors::{AriaError, Result},
    session::Session,
};
use libaria2_sys::{ffi, A2Gid};

impl<U> Session<'_, U> {
    pub fn add_uri(&mut self, uri: &str) -> Result<A2Gid> {
        let mut gid = A2Gid::default();
        let res =
            unsafe { ffi::add_uri(self.handle, &mut gid, &vec![uri.to_string()], &vec![], -1) };

        if res == 0 {
            Ok(gid)
        } else {
            Err(AriaError::AddError(res))
        }
    }

    pub fn add_metalink(&mut self, file: &Path) -> Result<Vec<A2Gid>> {
        let mut gids = Vec::new();
        let filename = file.to_string_lossy();
        let res = unsafe { ffi::add_metalink(self.handle, &mut gids, &filename, &vec![], -1) };

        if res == 0 {
            Ok(gids)
        } else {
            Err(AriaError::AddError(res))
        }
    }

    pub fn add_torrent(&mut self, file: &Path) -> Result<A2Gid> {
        let mut gid = A2Gid::default();
        let filename = file.to_string_lossy();
        let res = unsafe { ffi::add_torrent(self.handle, &mut gid, &filename, &vec![], -1) };

        if res == 0 {
            Ok(gid)
        } else {
            Err(AriaError::AddError(res))
        }
    }

    pub fn add_torrent_with_webseed_uris(
        &mut self,
        file: &Path,
        webseeds: &Vec<String>,
    ) -> Result<A2Gid> {
        let mut gid = A2Gid::default();
        let filename = file.to_string_lossy();
        let res = unsafe {
            ffi::add_torrent_with_webseed_uris(
                self.handle,
                &mut gid,
                &filename,
                webseeds,
                &vec![],
                -1,
            )
        };

        if res == 0 {
            Ok(gid)
        } else {
            Err(AriaError::AddError(res))
        }
    }
}
