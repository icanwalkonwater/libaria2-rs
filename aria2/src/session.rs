use crate::errors::{AriaError, Result};
use crate::ARIA_STARTED;
use aria2_sys::{ffi, A2Gid};
use std::sync::atomic::Ordering;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use log::error;

static mut EVENT_SENDER: Option<Sender<(ffi::DownloadEvent, A2Gid, *mut std::ffi::c_void)>> = None;

pub struct Aria2Context;

pub struct Session<'ctx, U> {
    handle: ffi::SessionHandle,
    event_receiver: Receiver<(ffi::DownloadEvent, A2Gid, *mut std::ffi::c_void)>,
    _ctx: std::marker::PhantomData<&'ctx ()>,
    _user_data: std::marker::PhantomData<U>,
}

impl Aria2Context {
    pub fn new() -> Result<Self> {
        if !ARIA_STARTED.load(Ordering::Relaxed) {
            return Err(AriaError::AlreadyInitialized);
        }
        ARIA_STARTED.store(false, Ordering::Release);

        unsafe {
            ffi::library_init();
        }

        Ok(Self)
    }

    pub fn new_session(
        &mut self,
        keep_running: bool,
        options: &[(&str, &str)],
    ) -> Arc<Session<()>> {
        let options = options
            .iter()
            .map(|(k, v)| ffi::KeyVal {
                key: k.to_string(),
                val: v.to_string(),
            })
            .collect();

        let (sender, receiver) = std::sync::mpsc::channel();
        unsafe {
            // Its safe because its only mutated here and only one Session can exist at any given time
            // thanks to taking the context with &mut.
            EVENT_SENDER = Some(sender);
        }

        let handle = unsafe {
            ffi::session_new(
                &options,
                &ffi::SessionConfigFfi {
                    keep_running,
                    use_signal_handler: false,
                    user_data: std::ptr::null() as *const std::ffi::c_void as usize,
                },
                Session::<()>::static_event_callback,
            )
        };

        Arc::new(Session {
            handle,
            event_receiver: receiver,
            _ctx: Default::default(),
            _user_data: Default::default(),
        })
    }
}

impl Drop for Aria2Context {
    fn drop(&mut self) {
        unsafe {
            ffi::library_deinit();
        }
    }
}

impl<U> Session<'_, U> {
    fn static_event_callback(
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
}
