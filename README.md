# libaria2-rs
Provides unsafe rust bindings for [aria2](https://aria2.github.io/).

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