// Generated macro for impl_1088 (impl)
macro_rules! Depcrate_combinator_implsimpl_1088 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1088"}
// Dependencies: {}
impl < F , I , O , O2 , E > Parser < I , O2 , E > for OutputInto < F , I , O , O2 , E > where F : Parser < I , O , E > , O : Into < O2 > , { # [inline] fn parse_next (& mut self , i : & mut I) -> Result < O2 , E > { self . parser . parse_next (i) . map (| o | o . into ()) } }
};
}
