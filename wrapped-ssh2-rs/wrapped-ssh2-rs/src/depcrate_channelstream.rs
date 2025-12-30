// Generated macro for Stream (struct)
macro_rules! Depcrate_channelStream {
() => {
// Module: crate::channel
// Provides: {"Stream"}
// Dependencies: {}
# [doc = " A channel can have a number of streams, each identified by an id, each of"] # [doc = " which implements the `Read` and `Write` traits."] # [doc = ""] # [doc = " You may clone a `Stream` to obtain another handle to the same underlying"] # [doc = " stream, but note that all clones will share the same underlying SSH"] # [doc = " session and will be subject to the same blocking behavior. For more details"] # [doc = " on the implications of cloning and blocking operations, refer to the"] # [doc = " `Session` documentation."] # [derive (Clone)] pub struct Stream { channel_inner : Arc < ChannelInner > , id : i32 , }
};
}
