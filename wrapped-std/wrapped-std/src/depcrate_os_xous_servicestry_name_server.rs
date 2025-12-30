// Generated macro for try_name_server (function)
macro_rules! Depcrate_os_xous_servicestry_name_server {
() => {
// Module: crate::os::xous::services
// Provides: {"try_name_server"}
// Dependencies: {}
fn try_name_server () -> Option < Connection > { let cid = NAME_SERVER_CONNECTION . load (Ordering :: Relaxed) ; if cid != 0 { return Some (cid . into ()) ; } if let Ok (Some (cid)) = crate :: os :: xous :: ffi :: try_connect ("xous-name-server" . try_into () . unwrap ()) { NAME_SERVER_CONNECTION . store (cid . into () , Ordering :: Relaxed) ; Some (cid) } else { None } }
};
}
