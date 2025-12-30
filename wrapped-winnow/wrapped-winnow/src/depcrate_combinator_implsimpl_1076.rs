// Generated macro for impl_1076 (impl)
macro_rules! Depcrate_combinator_implsimpl_1076 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1076"}
// Dependencies: {}
impl < F , I , O , O2 , E > Parser < I , O2 , E > for DefaultValue < F , I , O , O2 , E > where F : Parser < I , O , E > , O2 : core :: default :: Default , { # [inline] fn parse_next (& mut self , input : & mut I) -> Result < O2 , E > { (self . parser) . parse_next (input) . map (| _ | O2 :: default ()) } }
};
}
