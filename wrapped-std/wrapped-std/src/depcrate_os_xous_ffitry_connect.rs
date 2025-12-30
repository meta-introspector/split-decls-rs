// Generated macro for try_connect (function)
macro_rules! Depcrate_os_xous_ffitry_connect {
() => {
// Module: crate::os::xous::ffi
// Provides: {"try_connect"}
// Dependencies: {}
# [doc = " Attempts to connect to a Xous server represented by the specified `address`."] # [doc = ""] # [doc = " If the server does not exist then None is returned."] pub (crate) fn try_connect (address : ServerAddress) -> Result < Option < Connection > , Error > { match connect_impl (address , false) { Ok (conn) => Ok (Some (conn)) , Err (Error :: ServerNotFound) => Ok (None) , Err (e) => Err (e) , } }
};
}
