// Generated macro for impl_1062 (impl)
macro_rules! Depcrate_combinator_implsimpl_1062 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1062"}
// Dependencies: {}
impl < F , G , I , O , O2 , E > Parser < I , O2 , E > for VerifyMap < F , G , I , O , O2 , E > where F : Parser < I , O , E > , G : FnMut (O) -> Option < O2 > , I : Stream , E : ParserError < I > , { # [inline] fn parse_next (& mut self , input : & mut I) -> Result < O2 , E > { let start = input . checkpoint () ; let o = self . parser . parse_next (input) ? ; let res = (self . map) (o) . ok_or_else (| | { input . reset (& start) ; ParserError :: from_input (input) }) ; trace_result ("verify" , & res) ; res } }
};
}
