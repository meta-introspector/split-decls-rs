// Generated macro for impl_1060 (impl)
macro_rules! Depcrate_combinator_implsimpl_1060 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1060"}
// Dependencies: {}
impl < F , G , I , O , O2 , E , E2 > Parser < I , O2 , E > for TryMap < F , G , I , O , O2 , E , E2 > where F : Parser < I , O , E > , G : FnMut (O) -> Result < O2 , E2 > , I : Stream , E : FromExternalError < I , E2 > , E : ParserError < I > , { # [inline] fn parse_next (& mut self , input : & mut I) -> Result < O2 , E > { let start = input . checkpoint () ; let o = self . parser . parse_next (input) ? ; let res = (self . map) (o) . map_err (| err | { input . reset (& start) ; E :: from_external_error (input , err) }) ; trace_result ("verify" , & res) ; res } }
};
}
