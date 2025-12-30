// Generated macro for Channel (struct)
macro_rules! Depcrate_channelChannel {
() => {
// Module: crate::channel
// Provides: {"Channel"}
// Dependencies: {}
# [doc = " A channel represents a portion of an SSH connection on which data can be"] # [doc = " read and written."] # [doc = ""] # [doc = " Channels denote all of SCP uploads and downloads, shell sessions, remote"] # [doc = " process executions, and other general-purpose sessions. Each channel"] # [doc = " implements the `Reader` and `Writer` traits to send and receive data."] # [doc = " Whether or not I/O operations are blocking is mandated by the `blocking`"] # [doc = " flag on a channel's corresponding `Session`."] # [doc = ""] # [doc = " You may clone a `Channel` to obtain another handle to the same underlying"] # [doc = " channel, but note that all clones will share the same underlying SSH"] # [doc = " session and will be subject to the same blocking behavior. For more details"] # [doc = " on the implications of cloning and blocking operations, refer to the"] # [doc = " `Session` documentation."] # [derive (Clone)] pub struct Channel { channel_inner : Arc < ChannelInner > , }
};
}
