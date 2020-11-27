//! Those tests come from the original test of aria2's public API.
//! https://github.com/aria2/aria2/blob/master/test/Aria2ApiTest.cc

use nix::{sys::wait::WaitStatus, unistd::ForkResult};

use libaria2::{ffi::*, *};

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
        &vec![KeyVal {
            key: "no-conf".into(),
            val: "true".into(),
        }],
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
fn test_add_uri() {
    test_harness(|| unsafe {
        library_init();
        let session = get_session();

        let mut gid = A2Gid::default();
        let uris = vec!["http://localhost/1".into()];
        assert_eq!(add_uri(session, &mut gid, &uris, &vec![], -1), 0);
        assert!(!is_gid_null(gid));

        {
            let handle = get_download_handle(session, gid);
            assert!(!handle.is_null());
            assert_eq!(handle.num_files(), 1);
            let file = handle.get_file(1);
            assert_eq!(file.uris().len(), 1);
            assert_eq!(file.uris().get(0).unwrap().uri().to_string_lossy(), uris[0]);
        }

        assert_eq!(
            add_uri(
                session,
                &mut gid,
                &uris,
                &vec![KeyVal {
                    key: "file-allocation".into(),
                    val: "foo".into(),
                }],
                -1,
            ),
            -1
        );

        session_final(session);
        library_deinit();
    });
}

#[test]
fn test_add_metalink() {
    test_harness(|| unsafe {
        library_init();
        let session = get_session();

        let metalink_path = "./tests/metalink4.xml";
        let mut gids = vec![];

        assert_eq!(
            add_metalink(session, &mut gids, metalink_path, &vec![], -1),
            0
        );
        assert_eq!(gids.len(), 2);

        gids.clear();
        assert_eq!(
            add_metalink(
                session,
                &mut gids,
                metalink_path,
                &vec![KeyVal {
                    key: "file-allocation".into(),
                    val: "foo".into(),
                }],
                -1,
            ),
            -1
        );

        library_deinit();
    });
}

#[test]
fn test_add_torrent() {
    test_harness(|| unsafe {
        library_init();
        let session = get_session();

        let torrent_path = "./tests/test.torrent";
        let mut gid = A2Gid::default();

        assert_eq!(add_torrent(session, &mut gid, torrent_path, &vec![], -1), 0);
        assert!(!is_gid_null(gid));

        assert_eq!(
            add_torrent(
                session,
                &mut gid,
                torrent_path,
                &vec![KeyVal {
                    key: "file-allocation".into(),
                    val: "foo".into(),
                }],
                -1,
            ),
            -1
        );

        session_final(session);
        library_deinit();
    });
}

#[test]
fn test_remove_pause() {
    test_harness(|| unsafe {
        library_init();
        let session = get_session();

        let mut gid = A2Gid::default();
        assert_eq!(
            add_uri(
                session,
                &mut gid,
                &vec!["http://localhost/1".into()],
                &vec![],
                -1,
            ),
            0
        );

        {
            let handle = get_download_handle(session, gid);
            assert!(!handle.is_null());
            assert_eq!(handle.status(), DownloadStatus::DOWNLOAD_WAITING);
        }

        assert_eq!(pause_download(session, 0, false), -1);
        assert_eq!(pause_download(session, gid, false), 0);
        {
            let handle = get_download_handle(session, gid);
            assert!(!handle.is_null());
            assert_eq!(handle.status(), DownloadStatus::DOWNLOAD_PAUSED);
        }

        assert_eq!(unpause_download(session, 0), -1);
        assert_eq!(unpause_download(session, gid), 0);
        {
            let handle = get_download_handle(session, gid);
            assert!(!handle.is_null());
            assert_eq!(handle.status(), DownloadStatus::DOWNLOAD_WAITING);
        }

        assert_eq!(remove_download(session, 0, false), -1);
        assert_eq!(remove_download(session, gid, false), 0);
        {
            let handle = get_download_handle(session, gid);
            assert!(handle.is_null());
        }

        session_final(session);
        library_deinit();
    });
}

#[test]
fn test_change_position() {
    test_harness(|| unsafe {
        library_init();
        let session = get_session();

        const N: usize = 10;
        let uris = vec!["http://localhost/".into()];
        let mut gids = [A2Gid::default(); N];
        for i in 0..N {
            assert_eq!(add_uri(session, &mut gids[i], &uris, &vec![], -1), 0);
        }

        assert_eq!(
            change_position(session, 0, -2, OffsetMode::OFFSET_MODE_CUR),
            -1
        );
        assert_eq!(
            change_position(session, gids[4], -2, OffsetMode::OFFSET_MODE_CUR),
            2
        );
        assert_eq!(
            change_position(session, gids[4], 5, OffsetMode::OFFSET_MODE_SET),
            5
        );
        assert_eq!(
            change_position(session, gids[4], -2, OffsetMode::OFFSET_MODE_END),
            7
        );

        session_final(session);
        library_deinit();
    });
}

#[test]
fn test_change_option() {
    test_harness(|| unsafe {
        library_init();
        let session = get_session();

        let uris = vec!["http://localhost/1".into()];
        let options = vec![KeyVal {
            key: "dir".into(),
            val: "mydownload".into(),
        }];
        let mut gid = A2Gid::default();

        assert_eq!(add_uri(session, &mut gid, &uris, &options, -1), 0);

        {
            let handle = get_download_handle(session, gid);
            assert!(!handle.is_null());
            assert_eq!(handle.num_files(), 1);
            let file = handle.get_file(1);
            assert_eq!(
                file.uris().get(0).unwrap().uri().to_string_lossy(),
                uris[0].as_str()
            );

            assert_eq!(handle.get_option("dir"), "mydownload");
            assert!(handle.get_option("unknown").is_empty());
        }

        session_final(session);
        library_deinit();
    });
}

#[test]
fn test_change_global_option() {
    test_harness(|| unsafe {
        library_init();
        let session = get_session();

        assert_eq!(
            change_global_option(
                session,
                &vec![KeyVal {
                    key: "file-allocation".into(),
                    val: "none".into(),
                }]
            ),
            0
        );
        assert_eq!(get_global_option(session, "file-allocation"), "none");
        assert!(get_global_option(session, "startup-idle-time").is_empty());

        assert_eq!(
            change_global_option(
                session,
                &vec![KeyVal {
                    key: "file-allocation".into(),
                    val: "foo".into(),
                }]
            ),
            -1
        );

        session_final(session);
        library_deinit();
    });
}

// There is one more test (testDownloadResultDH) but it requires to access some internals of the
// library so we will assume that it works =).
