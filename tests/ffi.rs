use libaria2::ffi::*;
use libaria2::*;
use nix::sys::wait::WaitStatus;
use nix::unistd::ForkResult;

fn test_harness(test: unsafe fn()) {
    // Skip harness if env variable say so
    if let Some(opt) = option_env!("NO_HARNESS") {
        if opt != "0" && opt.to_lowercase() != "false" {
            unsafe {
                test();
            }
            return;
        }
    }

    // Can't setup harness if not in a unix environment
    if cfg!(not(unix)) {
        println!("== WARNING: non-unix OS aren't supported by the test harness !");
        println!("== Running the test anyway but expect weird errors");
        unsafe {
            test();
        }
        return;
    }

    let fork_res = unsafe { nix::unistd::fork().unwrap() };

    if fork_res.is_child() {
        let res = std::panic::catch_unwind(|| unsafe {
            test();
        });

        if let Err(e) = res {
            eprintln!("{:?}", e);
            std::process::exit(1);
        } else {
            std::process::exit(0);
        }
    } else if let ForkResult::Parent { child } = fork_res {
        let res = nix::sys::wait::waitpid(child, None).unwrap();
        if let WaitStatus::Exited(_, 0) = res {
            // Ok
        } else {
            panic!("Child failed !");
        }
    }
}

unsafe fn get_session() -> SessionHandle {
    session_new(
        &vec![
            KeyVal {
                key: "dir".into(),
                val: std::env::temp_dir().to_string_lossy().into_owned(),
            },
            KeyVal {
                key: "max-overall-download-limit".into(),
                val: "1".into(),
            },
        ],
        &SessionConfigFfi {
            keep_running: false,
            use_signal_handler: false,
            user_data: 0,
        },
        |_, _, _, _| 0,
    )
}

unsafe fn tick(session: SessionHandle) -> i32 {
    run(session, RunMode::RUN_ONCE)
}

#[test]
fn session_create() {
    test_harness(|| unsafe {
        library_init();

        let session = get_session();

        assert!(session.is_valid());

        session_final(session);
        library_deinit();
    });
}

#[test]
fn download_http() {
    test_harness(|| unsafe {
        library_init();
        let session = get_session();

        let mut gid = 0;
        let res = add_uri(
            session,
            &mut gid,
            &vec!["https://via.placeholder.com/150".into()],
            &vec![],
            -1,
        );
        assert_eq!(res, 0);
        assert_ne!(gid, 0);

        assert_eq!(tick(session), 1);

        let active = get_active_download(session);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0], gid);

        let res = remove_download(session, gid, true);
        assert_eq!(res, 0);

        assert_eq!(tick(session), 1);

        let active = get_active_download(session);
        assert!(active.is_empty());

        session_final(session);
        library_deinit();
    });
}

#[test]
fn get_change_options() {
    test_harness(|| unsafe {
        library_init();
        let session = get_session();

        assert_eq!(get_global_option(session, "qsdqsdqsd"), "");
        assert_eq!(
            get_global_option(session, "dir"),
            std::env::temp_dir().to_str().unwrap()
        );

        let options = get_global_options(session);
        println!(
            "{:?}",
            options
                .iter()
                .map(|KeyVal { key, val }| format!("{}: {}", key, val))
                .collect::<Vec<_>>()
        );
        assert!(!options.is_empty());
        assert!(options
            .iter()
            .find(
                |KeyVal { key, val }| key == "dir" && val == std::env::temp_dir().to_str().unwrap()
            )
            .is_some());
        assert!(options
            .iter()
            .find(|KeyVal { key, val }| key == "max-overall-download-limit" && val == "1")
            .is_some());

        let res = change_global_option(
            session,
            &vec![KeyVal {
                key: "max-overall-download-limit".into(),
                val: "2".into(),
            }],
        );
        assert_eq!(res, 0);

        assert_eq!(
            get_global_option(session, "max-overall-download-limit"),
            "2"
        );

        session_final(session);
        library_deinit();
    })
}
