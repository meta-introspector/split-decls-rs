// Generated macro for impl_1090 (impl)
macro_rules! Depcrate_combinator_implsimpl_1090 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1090"}
// Dependencies: {}
impl < F , I , O , E , E2 > Parser < I , O , E2 > for ErrInto < F , I , O , E , E2 > where F : Parser < I , O , E > , E : Into < E2 > , { # [inline] fn parse_next (& mut self , i : & mut I) -> Result < O , E2 > { self . parser . parse_next (i) . map_err (| err | err . into ()) } }
};
}
