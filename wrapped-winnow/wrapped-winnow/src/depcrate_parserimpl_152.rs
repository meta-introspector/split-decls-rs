// Generated macro for impl_152 (impl)
macro_rules! Depcrate_parserimpl_152 {
() => {
// Module: crate::parser
// Provides: {"impl_152"}
// Dependencies: {}
impl < I , O , E , F > Parser < I , O , E > for F where F : FnMut (& mut I) -> Result < O , E > , I : Stream , { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < O , E > { self (i) } }
};
}
