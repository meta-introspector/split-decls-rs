// Generated macro for impl_60 (impl)
macro_rules! Depcrate_tokenimpl_60 {
() => {
// Module: crate::token
// Provides: {"impl_60"}
// Dependencies: {}
# [cfg (feature = "parsing")] impl < T : CustomToken > Token for T { fn peek (cursor : Cursor) -> bool { < Self as CustomToken > :: peek (cursor) } fn display () -> & 'static str { < Self as CustomToken > :: display () } }
};
}
