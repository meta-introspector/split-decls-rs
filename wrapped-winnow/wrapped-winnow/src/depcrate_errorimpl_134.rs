// Generated macro for impl_134 (impl)
macro_rules! Depcrate_errorimpl_134 {
() => {
// Module: crate::error
// Provides: {"impl_134"}
// Dependencies: {}
impl < I : Stream , E : ParserError < I > > ParseError < I , E > { pub (crate) fn new (mut input : I , start : I :: Checkpoint , inner : E) -> Self { let offset = input . offset_from (& start) ; input . reset (& start) ; Self { input , offset , inner , } } }
};
}
