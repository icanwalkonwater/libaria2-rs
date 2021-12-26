use libaria2::download_handle::DownloadStatus;
use libaria2::prelude::*;
use libaria2::session::RunResult;

fn main() {
    if std::env::args().len() == 1 {
        eprintln!("Usage: {} <url> [dir]", std::env::args().next().unwrap());
        std::process::exit(1);
    }

    let url = std::env::args().nth(1).unwrap();
    let dir = std::env::args().nth(2).unwrap_or(".".to_owned());

    let mut aria = Aria2Context::new().unwrap();
    let mut session = aria.new_session(false, &[("dir", &dir)]);

    let gid = session.add_uri(&url).unwrap();

    println!("Downloading {} until completion ...", url);
    let (res, ctx) = session.poll(false).unwrap();
    assert_eq!(res, RunResult::Done);

    let handle = ctx.acquire_handle(gid).unwrap();
    assert_eq!(handle.status(), DownloadStatus::Complete);

    println!("Done ! Saved in: {}", handle.dir());
}