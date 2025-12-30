// Generated macro for impl_1074 (impl)
macro_rules! Depcrate_combinator_implsimpl_1074 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1074"}
// Dependencies: {}
impl < F , I , O , O2 , E > Parser < I , O2 , E > for Value < F , I , O , O2 , E > where F : Parser < I , O , E > , O2 : Clone , { # [inline] fn parse_next (& mut self , input : & mut I) -> Result < O2 , E > { (self . parser) . parse_next (input) . map (| _ | self . val . clone ()) } }
};
}
