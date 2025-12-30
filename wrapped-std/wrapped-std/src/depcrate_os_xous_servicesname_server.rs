// Generated macro for name_server (function)
macro_rules! Depcrate_os_xous_servicesname_server {
() => {
// Module: crate::os::xous::services
// Provides: {"name_server"}
// Dependencies: {}
# [doc = " Returns a `Connection` to the name server. If the name server has not been started,"] # [doc = " then this call will block until the name server has been started. The `Connection`"] # [doc = " will be shared among all connections in a process, so it is safe to call this"] # [doc = " multiple times."] pub (crate) fn name_server () -> Connection { let cid = NAME_SERVER_CONNECTION . load (Ordering :: Relaxed) ; if cid != 0 { return cid . into () ; } let cid = crate :: os :: xous :: ffi :: connect ("xous-name-server" . try_into () . unwrap ()) . unwrap () ; NAME_SERVER_CONNECTION . store (cid . into () , Ordering :: Relaxed) ; cid }
};
}
