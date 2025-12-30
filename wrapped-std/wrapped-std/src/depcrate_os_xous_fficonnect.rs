// Generated macro for connect (function)
macro_rules! Depcrate_os_xous_fficonnect {
() => {
// Module: crate::os::xous::ffi
// Provides: {"connect"}
// Dependencies: {}
# [doc = " Connects to a Xous server represented by the specified `address`."] # [doc = ""] # [doc = " The current thread will block until the server is available. Returns"] # [doc = " an error if the server cannot accept any more connections."] pub (crate) fn connect (address : ServerAddress) -> Result < Connection , Error > { connect_impl (address , true) }
};
}
