// Generated macro for undermine (function)
macro_rules! Depcrate_inflateundermine {
() => {
// Module: crate::inflate
// Provides: {"undermine"}
// Dependencies: {}
pub fn undermine (stream : & mut InflateStream , subvert : i32) -> ReturnCode { stream . state . flags . update (Flags :: SANE , (! subvert) != 0) ; ReturnCode :: Ok }
};
}
