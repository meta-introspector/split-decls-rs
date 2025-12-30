// Generated macro for RecursionGuard (struct)
macro_rules! Depcrate_parser_eventRecursionGuard {
() => {
// Module: crate::parser::event
// Provides: {"RecursionGuard"}
// Dependencies: {}
pub struct RecursionGuard < 'r > { receiver : & 'r mut dyn EventReceiver , max_depth : u32 , depth : i64 , }
};
}
