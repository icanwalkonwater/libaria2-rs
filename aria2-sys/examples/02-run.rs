use aria2_sys::{ffi::SessionConfigFfi, DownloadEvent, RunMode};

fn main() {
    unsafe {
        println!("Library init");
        aria2_sys::ffi::library_init();

        let session = aria2_sys::ffi::session_new(
            &vec![],
            &SessionConfigFfi {
                keep_running: false,
                use_signal_handler: false,
                user_data: 0,
            },
            |_, event, gid, _| {
                let event: DownloadEvent = event.into();
                println!("Event {:?} for gid {:x}", event, gid);
                0
            },
        );
        println!("Create session: {}", session.is_valid());

        if !session.is_valid() {
            eprintln!("Failed to create session, abort");
            aria2_sys::ffi::library_deinit();
            return;
        }

        loop {
            match aria2_sys::ffi::run(session, RunMode::RUN_ONCE) {
                0 => {
                    println!("Done !");
                    break;
                }
                1 => {
                    println!("Run once");
                    continue;
                }
                err if err < 0 => {
                    eprintln!("Error: {}", err);
                    break;
                }
                _ => unreachable!(),
            }
        }

        println!("Destroy session");
        aria2_sys::ffi::session_final(session);

        println!("Library destroy");
        aria2_sys::ffi::library_deinit();
    }
}
