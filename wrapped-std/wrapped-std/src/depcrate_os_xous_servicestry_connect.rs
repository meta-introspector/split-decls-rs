// Generated macro for try_connect (function)
macro_rules! Depcrate_os_xous_servicestry_connect {
() => {
// Module: crate::os::xous::services
// Provides: {"try_connect"}
// Dependencies: {}
# [doc = " Attempts to connect to a server by name. If the server does not exist, this will"] # [doc = " immediately return `None`."] # [doc = ""] # [doc = " Note that this is different from connecting to a server by address. Server"] # [doc = " addresses are always 16 bytes long, whereas server names are arbitrary-length"] # [doc = " strings."] # [stable (feature = "rust1" , since = "1.0.0")] pub fn try_connect (name : & str) -> Option < Connection > { ns :: try_connect_with_name (name) }
};
}
