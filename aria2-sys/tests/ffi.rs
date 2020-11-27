use aria2_sys::{ffi::*, *};
use nix::{sys::wait::WaitStatus, unistd::ForkResult};

pub fn test_harness(test: unsafe fn()) {
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

pub unsafe fn get_session() -> SessionHandle {
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
            KeyVal {
                key: "no-conf".into(),
                val: "true".into(),
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

pub unsafe fn tick(session: SessionHandle) -> i32 {
    run(session, RunMode::RUN_ONCE)
}

#[test]
fn session_create() {
    test_harness(|| unsafe {
        library_init();

        let session = get_session();

        assert!(session.is_valid());

        shutdown(session, true);
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

        shutdown(session, true);
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

        shutdown(session, true);
        session_final(session);
        library_deinit();
    })
}

#[test]
fn stats() {
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

        let stat = get_global_stat(session);
        if stat.num_active == 1 {
            assert_eq!(stat.num_waiting, 0);
        } else if stat.num_waiting == 1 {
            assert_eq!(stat.num_active, 0);
        } else {
            assert!(false);
        }
        assert_eq!(stat.num_stopped, 0);

        remove_download(session, gid, true);

        assert_eq!(tick(session), 1);

        let stat = get_global_stat(session);
        assert_eq!(stat.num_active, 0);
        assert_eq!(stat.num_waiting, 0);
        assert_eq!(stat.num_stopped, 1);

        shutdown(session, true);
        session_final(session);
        library_deinit();
    })
}

#[test]
fn download_handle() {
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

        let handle = get_download_handle(session, gid);
        assert_eq!(handle.num_files(), 1);
        let file = handle.get_file(1);

        delete_download_handle(handle);

        shutdown(session, true);
        session_final(session);
        library_deinit();
    })
}

#[test]
fn multiple_session() {
    test_harness(|| unsafe {
        library_init();

        let session = get_session();
        assert_eq!(tick(session), 1);
        assert_eq!(session_final(session), 0);

        let session = get_session();
        let mut gid = A2Gid::default();
        assert_eq!(add_uri(session, &mut gid, &vec!["http://localhost/1".into()], &vec![], -1), 0);
        assert!(!is_gid_null(gid));
        assert_eq!(remove_download(session, gid, false), 0);
        assert_eq!(session_final(session), 0);

        library_deinit();
    })
}