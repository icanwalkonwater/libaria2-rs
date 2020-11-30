use crate::{
    errors::{AriaError, Result},
    events::DownloadEvent,
    ARIA_STARTED,
};
use libaria2_sys::{ffi, A2Gid};
use std::{
    collections::VecDeque,
    sync::{
        atomic::Ordering,
        mpsc::{Receiver, Sender},
    },
};

pub(crate) static mut EVENT_SENDER: Option<
    Sender<(ffi::DownloadEvent, A2Gid, *mut std::ffi::c_void)>,
> = None;

pub struct Aria2Context;

pub struct Session<'ctx, U> {
    pub(crate) handle: ffi::SessionHandle,
    pub(crate) event_receiver: Receiver<(ffi::DownloadEvent, A2Gid, *mut std::ffi::c_void)>,
    pub(crate) event_queue: VecDeque<DownloadEvent>,
    _ctx: std::marker::PhantomData<&'ctx ()>,
    _user_data: std::marker::PhantomData<U>,
}

// TODO: When stabilized
// impl<U> !Sync for Session<'_, U> {}
// impl<U> !Send for Session<'_, U> {}

impl Aria2Context {
    pub fn new() -> Result<Self> {
        if ARIA_STARTED.load(Ordering::Relaxed) {
            return Err(AriaError::AlreadyInitialized);
        }
        ARIA_STARTED.store(false, Ordering::Release);

        unsafe {
            ffi::library_init();
        }

        Ok(Self)
    }

    pub fn new_session(&mut self, keep_running: bool, options: &[(&str, &str)]) -> Session<()> {
        let options = options
            .iter()
            .map(|(k, v)| ffi::KeyVal {
                key: k.to_string(),
                val: v.to_string(),
            })
            .collect();

        let (sender, receiver) = std::sync::mpsc::channel();
        unsafe {
            // It's safe because its only mutated here and only one Session can exist at any given time
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

        Session {
            handle,
            event_receiver: receiver,
            event_queue: Default::default(),
            _ctx: Default::default(),
            _user_data: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RunResult {
    Event(DownloadEvent),
    Continue,
    Done,
}

pub struct PollContext<'a> {
    pub(crate) handle: ffi::SessionHandle,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl PollContext<'_> {
    fn new<U>(session: &Session<U>) -> Self {
        Self {
            handle: session.handle,
            _phantom: Default::default(),
        }
    }
}

impl<U> Session<'_, U> {
    pub fn is_event_queue_empty(&self) -> bool {
        self.event_queue.is_empty()
    }

    pub fn poll(&mut self, mode_once: bool) -> Result<(RunResult, PollContext)> {
        // Receive events stored in receiver.
        while let Ok(event) = self.event_receiver.try_recv() {
            self.handle_event(event);
        }

        // Create a context for things that can only live for this poll round.
        let ctx = PollContext::new(self);

        // If there are queued events that haven't been retrieved yet, return them first.
        if !self.event_queue.is_empty() {
            return Ok((RunResult::Event(self.event_queue.pop_front().unwrap()), ctx));
        }

        // Start polling aria for some events.
        let status = unsafe {
            if mode_once {
                ffi::run(self.handle, ffi::RUN_MODE::RUN_ONCE)
            } else {
                ffi::run(self.handle, ffi::RUN_MODE::RUN_DEFAULT)
            }
        };

        match status {
            1 => Ok((RunResult::Continue, ctx)),
            0 => Ok((RunResult::Done, ctx)),
            status if status < 0 => Err(AriaError::RunError(status)),
            _ => panic!("Very unexpected value from run()"),
        }
    }

    pub fn shutdown(&mut self, force: bool) {
        // TODO: Real return value with enum of error codes
        unsafe {
            ffi::shutdown(self.handle, force);
        }
    }
}

impl<U> Drop for Session<'_, U> {
    fn drop(&mut self) {
        unsafe {
            ffi::session_final(self.handle);
        }
    }
}

impl Drop for Aria2Context {
    fn drop(&mut self) {
        unsafe {
            ffi::library_deinit();
        }
    }
}
