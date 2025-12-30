// Generated macro for TimeoutFuture (struct)
macro_rules! Depcrate_futureTimeoutFuture {
() => {
// Module: crate::future
// Provides: {"TimeoutFuture"}
// Dependencies: {}
# [doc = " A scheduled timeout as a `Future`."] # [doc = ""] # [doc = " See `TimeoutFuture::new` for scheduling new timeouts."] # [doc = ""] # [doc = " Once scheduled, if you change your mind and don't want the timeout to fire,"] # [doc = " you can `drop` the future."] # [doc = ""] # [doc = " A timeout future will never resolve to `Err`. Its only failure mode is when"] # [doc = " the timeout is so long that it is effectively infinite and never fires."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use gloo_timers::future::TimeoutFuture;"] # [doc = " use futures_util::future::{select, Either};"] # [doc = " use wasm_bindgen_futures::spawn_local;"] # [doc = ""] # [doc = " spawn_local(async {"] # [doc = "     match select(TimeoutFuture::new(1_000), TimeoutFuture::new(2_000)).await {"] # [doc = "         Either::Left((val, b)) => {"] # [doc = "             // Drop the `2_000` ms timeout to cancel its timeout."] # [doc = "             drop(b);"] # [doc = "         }"] # [doc = "         Either::Right((a, val)) => {"] # [doc = "             panic!(\"the `1_000` ms timeout should have won this race\");"] # [doc = "         }"] # [doc = "     }"] # [doc = " });"] # [doc = " ```"] # [derive (Debug)] # [must_use = "futures do nothing unless polled or spawned"] pub struct TimeoutFuture { _inner : Timeout , rx : oneshot :: Receiver < () > , }
};
}
