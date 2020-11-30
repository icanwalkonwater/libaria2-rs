use libaria2_sys::{
    ffi::{KeyVal, SessionConfigFfi},
    A2Gid, RunMode,
};

fn main() {
    unsafe {
        libaria2_sys::ffi::library_init();

        let mut event_counter = 0;

        let session = libaria2_sys::ffi::session_new(
            &vec![KeyVal {
                key: "dir".into(),
                val: "/tmp".into(),
            }],
            &SessionConfigFfi {
                keep_running: false,
                use_signal_handler: false,
                user_data: (&mut event_counter) as *mut i32 as usize,
            },
            |_, event, gid, counter| {
                println!("[Event] {:?} gid {:x}", event, gid);
                let counter = counter as *mut i32;
                counter.write(*counter + 1);
                0
            },
        );
        println!("Create session: {}", session.is_valid());

        if !session.is_valid() {
            eprintln!("Failed to create session, abort");
            libaria2_sys::ffi::library_deinit();
            return;
        }

        let mut gid = A2Gid::default();
        let res = libaria2_sys::ffi::add_uri(
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

        let mut gid2 = A2Gid::default();
        let res = libaria2_sys::ffi::add_uri(
            session,
            &mut gid2,
            &vec!["http://localhost/1".into()],
            &vec![],
            -1,
        );
        if res == 0 {
            println!("AddUri success, GID: {:x}", gid2);
        } else {
            eprintln!("AddUri failed: {}", res);
        }

        loop {
            event_counter = 0;
            match libaria2_sys::ffi::run(session, RunMode::RUN_ONCE) {
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
            println!("Events this loop: {}", event_counter);
        }

        libaria2_sys::ffi::session_final(session);
        libaria2_sys::ffi::library_deinit();
    }
}
