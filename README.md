# libaria2-rs
Provides unsafe rust bindings for [aria2](https://aria2.github.io/).

## Goals and non-goals

* Providing unsafe bindings to the official [libaria2 API](http://aria2.github.io/manual/en/html/libaria2.html).
* Providing a safe wrapper around these unsafe bindings.
* Not being a Rust version of `aria2c`.
* Not rewriting `aria2` in Rust.

## Roadmap

(unsafe) refers to `libaria2-sys` whereas (safe) refers to `libaria2`.

- [x] (unsafe) Link to the installed `libaria2.so`.
- [x] (unsafe) Provide bindings to all functions and opaque types.
- [x] (safe) Safe context and session.
- [x] (safe) `DownloadHandle` must live only for one poll.
- [ ] (unsafe) Provide access to all fields of opaque types.
- [ ] (unsafe) Allow closure instead of function for the event callback.
- [ ] (unsafe) Avoid copying as much as possible at the FFI frontier.
- [ ] (safe) Avoid copying as much as possible between the unsafe bindings and the safe wrapper.
- [ ] (unsafe & safe) Add the documentation from the original library.
- [ ] (unsafe) Support for windows

## Testing
Since `libaria2` make heavy use of static objets and don't seem
to be able to be init and deinit multiple times in the same process,
tests are run in a different process (1 process per test).

The special harness that forks and wait for the test result has
some limitations:
* It doesn't support non-unix platforms (because of the use of `fork`).
* It only report a success or a failure without details.

Thanks to this harness, you can just `cargo test` like usual on
unix but you'll need the workaround to inspect any failure.

#### Workaround
You can still run 1 test and disable the harness to have it run
like a normal test.

For example:
```bash
NO_HARNESS=1 cargo test --test ffi session_create
``` 