// Generated macro for impl_1066 (impl)
macro_rules! Depcrate_combinator_implsimpl_1066 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1066"}
// Dependencies: {}
impl < P , I , O , O2 , E > Parser < I , O2 , E > for ParseTo < P , I , O , O2 , E > where P : Parser < I , O , E > , I : Stream , O : crate :: stream :: ParseSlice < O2 > , E : ParserError < I > , { # [inline] fn parse_next (& mut self , i : & mut I) -> Result < O2 , E > { let start = i . checkpoint () ; let o = self . p . parse_next (i) ? ; let res = o . parse_slice () . ok_or_else (| | { i . reset (& start) ; ParserError :: from_input (i) }) ; trace_result ("verify" , & res) ; res } }
};
}
