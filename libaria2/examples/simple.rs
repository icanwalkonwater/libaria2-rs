use libaria2::{prelude::*, session::RunResult};

fn main() {
    let mut aria = Aria2Context::new().unwrap();
    let mut session = aria.new_session(true, &[]);

    let gid = session.add_uri("http://localhost/1").unwrap();

    let (res, ctx) = session.poll(true).unwrap();
    assert_eq!(res, RunResult::Continue);

    let handle = ctx.acquire_handle(gid).unwrap();
    println!("{:?}", handle.status());

    session.shutdown(false);
}
