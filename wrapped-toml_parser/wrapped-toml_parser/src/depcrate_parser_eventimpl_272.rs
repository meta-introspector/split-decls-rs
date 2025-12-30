// Generated macro for impl_272 (impl)
macro_rules! Depcrate_parser_eventimpl_272 {
() => {
// Module: crate::parser::event
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'r > RecursionGuard < 'r > { pub fn new (receiver : & 'r mut dyn EventReceiver , max_depth : u32) -> Self { Self { receiver , max_depth , depth : 0 , } } fn within_depth (& self) -> bool { self . depth <= self . max_depth as i64 } }
};
}
