// Generated macro for impl_1068 (impl)
macro_rules! Depcrate_combinator_implsimpl_1068 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1068"}
// Dependencies: {}
impl < F , G , H , I , O , O2 , E > Parser < I , O2 , E > for FlatMap < F , G , H , I , O , O2 , E > where F : Parser < I , O , E > , G : FnMut (O) -> H , H : Parser < I , O2 , E > , { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < O2 , E > { let o = self . f . parse_next (i) ? ; (self . g) (o) . parse_next (i) } }
};
}
