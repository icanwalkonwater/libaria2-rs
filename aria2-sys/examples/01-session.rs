use aria2_sys::ffi::SessionConfigFfi;

fn main() {
    unsafe {
        println!("Library init");
        aria2_sys::ffi::library_init();

        println!("Create session");
        let session = aria2_sys::ffi::session_new(
            &vec![],
            &SessionConfigFfi {
                keep_running: false,
                use_signal_handler: false,
                user_data: 0,
            },
            |_, _, _, _| 0,
        );

        // Do something maybe ?

        println!("Destroy session");
        aria2_sys::ffi::session_final(session);

        println!("Library deinit");
        aria2_sys::ffi::library_deinit();
    }
}
