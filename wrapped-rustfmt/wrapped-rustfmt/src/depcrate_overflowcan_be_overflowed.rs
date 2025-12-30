// Generated macro for can_be_overflowed (function)
macro_rules! Depcrate_overflowcan_be_overflowed {
() => {
// Module: crate::overflow
// Provides: {"can_be_overflowed"}
// Dependencies: {}
fn can_be_overflowed (context : & RewriteContext < '_ > , items : & [OverflowableItem < '_ >]) -> bool { items . last () . map_or (false , | x | x . can_be_overflowed (context , items . len ())) }
};
}
