// Generated macro for impl_1070 (impl)
macro_rules! Depcrate_combinator_implsimpl_1070 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1070"}
// Dependencies: {}
impl < P , I , O , E > Parser < I , O , E > for CompleteErr < P , I , O , E > where P : Parser < I , O , E > , I : Stream , E : ParserError < I > , { # [inline] fn parse_next (& mut self , input : & mut I) -> Result < O , E > { trace ("complete_err" , | input : & mut I | { match (self . p) . parse_next (input) { Err (err) => match err . needed () { Some (_) => Err (ParserError :: from_input (input)) , None => Err (err) , } , rest => rest , } }) . parse_next (input) } }
};
}
