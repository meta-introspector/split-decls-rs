// Generated macro for connect (function)
macro_rules! Depcrate_os_xous_servicesconnect {
() => {
// Module: crate::os::xous::services
// Provides: {"connect"}
// Dependencies: {}
# [doc = " Attempts to connect to a server by name. If the server does not exist, this will"] # [doc = " block until the server is created."] # [doc = ""] # [doc = " Note that this is different from connecting to a server by address. Server"] # [doc = " addresses are always 16 bytes long, whereas server names are arbitrary-length"] # [doc = " strings up to 64 bytes in length."] # [stable (feature = "rust1" , since = "1.0.0")] pub fn connect (name : & str) -> Option < Connection > { ns :: connect_with_name (name) }
};
}
