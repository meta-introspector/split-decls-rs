use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Runs the provided future, blocking the current thread until the
/// future completes.
///
/// For more information, see the documentation for
/// [`tokio::runtime::Runtime::block_on`][runtime-block-on].
///
/// [runtime-block-on]: https://docs.rs/tokio/1.3.0/tokio/runtime/struct.Runtime.html#method.block_on
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    use tokio::runtime;
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(future)
}
