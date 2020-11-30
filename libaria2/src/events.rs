use crate::session::{Session, EVENT_SENDER};
use libaria2_sys::{ffi, A2Gid};
use log::error;

#[derive(Debug, PartialEq)]
pub enum DownloadEvent {
    Started(A2Gid),
    Paused(A2Gid),
    Stopped(A2Gid),
    Completed(A2Gid, bool),
    Error(A2Gid),
}

impl<U> Session<'_, U> {
    pub(crate) fn static_event_callback(
        _: ffi::SessionHandle,
        event: ffi::DownloadEvent,
        gid: A2Gid,
        user_data: usize,
    ) -> i32 {
        let user_data = user_data as *mut std::ffi::c_void;
        unsafe {
            if let Err(e) = EVENT_SENDER.as_ref().unwrap().send((event, gid, user_data)) {
                error!("{}", e);
            }
        };

        0
    }

    pub(crate) fn handle_event(
        &mut self,
        event: (ffi::DownloadEvent, A2Gid, *mut std::ffi::c_void),
    ) {
        let (event, gid, _) = event;

        let event = match event {
            ffi::DownloadEvent::EVENT_ON_DOWNLOAD_START => DownloadEvent::Started(gid),
            ffi::DownloadEvent::EVENT_ON_DOWNLOAD_PAUSE => DownloadEvent::Paused(gid),
            ffi::DownloadEvent::EVENT_ON_DOWNLOAD_STOP => DownloadEvent::Stopped(gid),
            ffi::DownloadEvent::EVENT_ON_DOWNLOAD_COMPLETE => DownloadEvent::Completed(gid, false),
            ffi::DownloadEvent::EVENT_ON_BT_DOWNLOAD_COMPLETE => {
                DownloadEvent::Completed(gid, true)
            }
            ffi::DownloadEvent::EVENT_ON_DOWNLOAD_ERROR => DownloadEvent::Error(gid),
            _ => unreachable!(),
        };

        self.event_queue.push_back(event);
    }
}

impl<U> Session<'_, U> {
    /*fn cast_user_data(ptr: *mut std::ffi::c_void) -> Box<&'static U> {
        if ptr.is_null() {
            unsafe { Box::new(MaybeUninit::zeroed().assume_init()) }
        } else {
            unsafe { Box::new(&mut *(ptr as *mut U)) }
        }
    }*/
}
