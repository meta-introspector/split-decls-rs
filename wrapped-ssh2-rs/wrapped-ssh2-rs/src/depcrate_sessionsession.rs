// Generated macro for Session (struct)
macro_rules! Depcrate_sessionSession {
() => {
// Module: crate::session
// Provides: {"Session"}
// Dependencies: {}
# [doc = " An SSH session, typically representing one TCP connection."] # [doc = ""] # [doc = " All other structures are based on an SSH session and cannot outlive a"] # [doc = " session. Sessions are created and then have the TCP socket handed to them"] # [doc = " (via the `set_tcp_stream` method)."] # [doc = ""] # [doc = " `Session`, and any objects its methods return, hold a reference to the underlying"] # [doc = " SSH session.  You may clone `Session` to obtain another handle referencing"] # [doc = " the same session, and create multiple `Channel` and `Stream` objects"] # [doc = " from that same underlying session, which can all be passed across thread"] # [doc = " boundaries (they are `Send` and `Sync`).  These are all related objects and"] # [doc = " are internally synchronized via a `Mutex` to make it safe to pass them"] # [doc = " around in this way."] # [doc = ""] # [doc = " This means that a blocking read from a `Channel` or `Stream` will block"] # [doc = " all other calls on objects created from the same underlying `Session`."] # [doc = " If you need the ability to perform concurrent operations then you will"] # [doc = " need to create separate `Session` instances, or employ non-blocking mode."] # [derive (Clone)] pub struct Session { inner : Arc < Mutex < SessionInner > > , }
};
}
