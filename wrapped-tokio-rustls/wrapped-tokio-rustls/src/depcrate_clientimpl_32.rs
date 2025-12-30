// Generated macro for impl_32 (impl)
macro_rules! Depcrate_clientimpl_32 {
() => {
// Module: crate::client
// Provides: {"impl_32"}
// Dependencies: {}
impl < IO > TlsStream < IO > { # [inline] pub fn get_ref (& self) -> (& IO , & ClientConnection) { (& self . io , & self . session) } # [inline] pub fn get_mut (& mut self) -> (& mut IO , & mut ClientConnection) { (& mut self . io , & mut self . session) } # [inline] pub fn into_inner (self) -> (IO , ClientConnection) { (self . io , self . session) } }
};
}
