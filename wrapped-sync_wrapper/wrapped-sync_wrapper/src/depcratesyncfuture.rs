// Generated macro for SyncFuture (struct)
macro_rules! DepcrateSyncFuture {
() => {
// Module: crate
// Provides: {"SyncFuture"}
// Dependencies: {}
# [doc = " `Future` which is `Sync`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use sync_wrapper::{SyncWrapper, SyncFuture};"] # [doc = ""] # [doc = " let fut = async { 1 };"] # [doc = " let fut = SyncFuture::new(fut);"] # [doc = " ```"] pub struct SyncFuture < F > { inner : SyncWrapper < F > }
};
}
