// Generated macro for impl_913 (impl)
macro_rules! Depcrate_combinator_debug_internalsimpl_913 {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"impl_913"}
// Dependencies: {}
impl < P , D , I , O , E > Trace < P , D , I , O , E > where P : Parser < I , O , E > , I : Stream , D : std :: fmt :: Display , E : ParserError < I > , { # [inline (always)] pub (crate) fn new (parser : P , name : D) -> Self { Self { parser , name , call_count : 0 , i : Default :: default () , o : Default :: default () , e : Default :: default () , } } }
};
}
