use libaria2::{
    ffi::{KeyVal, SessionConfigFfi},
    A2Gid, DownloadEvent, RunMode,
};

fn main() {
    unsafe {
        libaria2::ffi::library_init();

        let session = libaria2::ffi::session_new(
            &vec![KeyVal {
                key: "dir".into(),
                val: "/tmp".into(),
            }],
            &SessionConfigFfi {
                keep_running: false,
                use_signal_handler: false,
                user_data: 0,
            },
            |_, event, gid, _| {
                let event: DownloadEvent = event.into();
                println!("[Event] {:?} gid {:x}", event, gid);
                0
            },
        );
        println!("Create session: {}", session.is_valid());

        if !session.is_valid() {
            eprintln!("Failed to create session, abort");
            libaria2::ffi::library_deinit();
            return;
        }

        let mut gid = A2Gid::default();
        let res = libaria2::ffi::add_uri(
            session,
            &mut gid,
            &vec!["https://via.placeholder.com/150".into()],
            &vec![],
            -1,
        );

        if res == 0 {
            println!("AddUri success, GID: {:x}", gid);
        } else {
            eprintln!("AddUri failed: {}", res);
        }

        loop {
            match libaria2::ffi::run(session, RunMode::RUN_ONCE) {
                0 => break,
                1 => {
                    println!("Running...");
                }
                err if err < 0 => {
                    eprintln!("Error: {}", err);
                    break;
                }
                _ => unreachable!(),
            }
        }

        libaria2::ffi::session_final(session);
        libaria2::ffi::library_deinit();
    }
}
