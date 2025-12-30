// Generated macro for ReadWindow (struct)
macro_rules! Depcrate_channelReadWindow {
() => {
// Module: crate::channel
// Provides: {"ReadWindow"}
// Dependencies: {}
# [doc = " Description of the read window as returned by `Channel::read_window`"] # [derive (Copy , Clone)] pub struct ReadWindow { # [doc = " The number of bytes which the remote end may send without overflowing"] # [doc = " the window limit."] pub remaining : u32 , # [doc = " The number of bytes actually available to be read."] pub available : u32 , # [doc = " The window_size_initial as defined by the channel open request"] pub window_size_initial : u32 , }
};
}
