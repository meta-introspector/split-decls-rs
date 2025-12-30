// Generated macro for impl_139 (impl)
macro_rules! Depcrate_serverimpl_139 {
() => {
// Module: crate::server
// Provides: {"impl_139"}
// Dependencies: {}
impl < IO > TlsStream < IO > { # [inline] pub fn get_ref (& self) -> (& IO , & ServerConnection) { (& self . io , & self . session) } # [inline] pub fn get_mut (& mut self) -> (& mut IO , & mut ServerConnection) { (& mut self . io , & mut self . session) } # [inline] pub fn into_inner (self) -> (IO , ServerConnection) { (self . io , self . session) } }
};
}
