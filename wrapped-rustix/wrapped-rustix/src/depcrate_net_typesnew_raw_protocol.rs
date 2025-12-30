// Generated macro for new_raw_protocol (function)
macro_rules! Depcrate_net_typesnew_raw_protocol {
() => {
// Module: crate::net::types
// Provides: {"new_raw_protocol"}
// Dependencies: {}
const fn new_raw_protocol (u : u32) -> RawProtocol { match RawProtocol :: new (u) { Some (p) => p , None => panic ! ("new_raw_protocol: protocol must be non-zero") , } }
};
}
