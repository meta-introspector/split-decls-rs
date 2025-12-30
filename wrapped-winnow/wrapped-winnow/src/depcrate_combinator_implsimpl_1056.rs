// Generated macro for impl_1056 (impl)
macro_rules! Depcrate_combinator_implsimpl_1056 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1056"}
// Dependencies: {}
impl < I , O , E , P > Parser < I , O , E > for ByRef < '_ , P , I , O , E > where P : Parser < I , O , E > , { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < O , E > { self . p . parse_next (i) } }
};
}
