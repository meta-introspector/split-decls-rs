// Generated macro for is_unsafe_block (function)
macro_rules! Depcrate_expris_unsafe_block {
() => {
// Module: crate::expr
// Provides: {"is_unsafe_block"}
// Dependencies: {}
pub (crate) fn is_unsafe_block (block : & ast :: Block) -> bool { matches ! (block . rules , ast :: BlockCheckMode :: Unsafe (..)) }
};
}
