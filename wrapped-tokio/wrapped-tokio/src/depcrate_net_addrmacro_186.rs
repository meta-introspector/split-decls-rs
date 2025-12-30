// Generated macro for macro_186 (macro)
macro_rules! Depcrate_net_addrmacro_186 {
() => {
// Module: crate::net::addr
// Provides: {"macro_186"}
// Dependencies: {}
cfg_net ! { pub (crate) fn to_socket_addrs < T > (arg : T) -> T :: Future where T : ToSocketAddrs , { arg . to_socket_addrs (sealed :: Internal) } }
};
}
