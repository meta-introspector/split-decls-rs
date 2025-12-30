// Generated macro for sysproto (module)
macro_rules! Depcrate_net_typessysproto {
() => {
// Module: crate::net::types
// Provides: {"sysproto"}
// Dependencies: {}
# [doc = " `SYSPROTO_*` constants."] pub mod sysproto { # [cfg (apple)] use { super :: { new_raw_protocol , Protocol } , crate :: backend :: c , } ; # [doc = " `SYSPROTO_EVENT`"] # [cfg (apple)] pub const EVENT : Protocol = Protocol (new_raw_protocol (c :: SYSPROTO_EVENT as _)) ; # [doc = " `SYSPROTO_CONTROL`"] # [cfg (apple)] pub const CONTROL : Protocol = Protocol (new_raw_protocol (c :: SYSPROTO_CONTROL as _)) ; }
};
}
