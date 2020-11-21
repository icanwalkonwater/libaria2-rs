use libaria2::ffi::SessionConfigFfi;

fn main() {
    unsafe {
        println!("Library init");
        libaria2::ffi::library_init();

        println!("Create session");
        let session = libaria2::ffi::session_new(&vec![], &SessionConfigFfi {
            keep_running: false,
            use_signal_handler: false,
            user_data: 0,
        });

        // Do something maybe ?

        println!("Destroy session");
        libaria2::ffi::session_final(session);

        println!("Library deinit");
        libaria2::ffi::library_deinit();
    }
}
