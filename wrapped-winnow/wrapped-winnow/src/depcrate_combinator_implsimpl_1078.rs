// Generated macro for impl_1078 (impl)
macro_rules! Depcrate_combinator_implsimpl_1078 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1078"}
// Dependencies: {}
impl < F , I , O , E > Parser < I , () , E > for Void < F , I , O , E > where F : Parser < I , O , E > , { # [inline (always)] fn parse_next (& mut self , input : & mut I) -> Result < () , E > { (self . parser) . parse_next (input) . map (| _ | ()) } }
};
}
