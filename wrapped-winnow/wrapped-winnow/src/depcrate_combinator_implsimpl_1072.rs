// Generated macro for impl_1072 (impl)
macro_rules! Depcrate_combinator_implsimpl_1072 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1072"}
// Dependencies: {}
impl < F , G , I , O , O2 , E > Parser < I , O , E > for Verify < F , G , I , O , O2 , E > where F : Parser < I , O , E > , G : FnMut (& O2) -> bool , I : Stream , O : Borrow < O2 > , O2 : ? Sized , E : ParserError < I > , { # [inline] fn parse_next (& mut self , input : & mut I) -> Result < O , E > { let start = input . checkpoint () ; let o = self . parser . parse_next (input) ? ; let res = (self . filter) (o . borrow ()) . then_some (o) . ok_or_else (| | { input . reset (& start) ; ParserError :: from_input (input) }) ; trace_result ("verify" , & res) ; res } }
};
}
