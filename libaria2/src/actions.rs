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
}
