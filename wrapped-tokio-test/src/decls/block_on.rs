macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! block_on {
    () => {
        deps!();
        # [doc = " Runs the provided future, blocking the current thread until the"] # [doc = " future completes."] # [doc = ""] # [doc = " For more information, see the documentation for"] # [doc = " [`tokio::runtime::Runtime::block_on`][runtime-block-on]."] # [doc = ""] # [doc = " [runtime-block-on]: https://docs.rs/tokio/1.3.0/tokio/runtime/struct.Runtime.html#method.block_on"] pub fn block_on < F : std :: future :: Future > (future : F) -> F :: Output { use tokio :: runtime ; let rt = runtime :: Builder :: new_current_thread () . enable_all () . build () . unwrap () ; rt . block_on (future) }
    };
}

block_on!();