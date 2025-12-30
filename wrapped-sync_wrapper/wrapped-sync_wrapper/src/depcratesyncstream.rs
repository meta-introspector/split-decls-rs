// Generated macro for SyncStream (struct)
macro_rules! DepcrateSyncStream {
() => {
// Module: crate
// Provides: {"SyncStream"}
// Dependencies: {}
# [doc = " `Stream` which is `Sync`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use sync_wrapper::SyncStream;"] # [doc = " use futures::stream;"] # [doc = ""] # [doc = " let st = stream::iter(vec![1]);"] # [doc = " let st = SyncStream::new(st);"] # [doc = " ```"] # [cfg (feature = "futures")] pub struct SyncStream < S > { inner : SyncWrapper < S > }
};
}
