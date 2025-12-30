// Generated macro for impl_1427 (impl)
macro_rules! Depcrate_serverimpl_1427 {
() => {
// Module: crate::server
// Provides: {"impl_1427"}
// Dependencies: {}
impl < R > futures :: Stream for Body < R > { type Item = Result < R > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> std :: task :: Poll < Option < Self :: Item > > { Pin :: new (& mut self . receiver) . poll_next (cx) } }
};
}
