// Generated macro for impl_881 (impl)
macro_rules! Depcrate_combinator_branchimpl_881 {
() => {
// Module: crate::combinator::branch
// Provides: {"impl_881"}
// Dependencies: {}
impl < I : Stream , O , E : ParserError < I > , A : Parser < I , O , E > > Alt < I , O , E > for (A ,) { fn choice (& mut self , input : & mut I) -> Result < O , E > { self . 0 . parse_next (input) } }
};
}
