// Generated macro for new_parse_buffer (function)
macro_rules! Depcrate_parsenew_parse_buffer {
() => {
// Module: crate::parse
// Provides: {"new_parse_buffer"}
// Dependencies: {}
pub (crate) fn new_parse_buffer (scope : Span , cursor : Cursor , unexpected : Rc < Cell < Unexpected > > ,) -> ParseBuffer { ParseBuffer { scope , cell : Cell :: new (unsafe { mem :: transmute :: < Cursor , Cursor < 'static > > (cursor) }) , marker : PhantomData , unexpected : Cell :: new (Some (unexpected)) , } }
};
}
