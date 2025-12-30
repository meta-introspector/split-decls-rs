// Generated macro for WriteWindow (struct)
macro_rules! Depcrate_channelWriteWindow {
() => {
// Module: crate::channel
// Provides: {"WriteWindow"}
// Dependencies: {}
# [doc = " Description of the write window as returned by `Channel::write_window`"] # [derive (Copy , Clone)] pub struct WriteWindow { # [doc = " The number of bytes which may be safely written on the channel without"] # [doc = " blocking."] pub remaining : u32 , # [doc = " The window_size_initial as defined by the channel open request"] pub window_size_initial : u32 , }
};
}
