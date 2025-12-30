// Generated macro for impl_27 (impl)
macro_rules! Depcrate_linesimpl_27 {
() => {
// Module: crate::lines
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'b > LineIter < 'b > { # [doc = " Create a new line iterator that yields lines in the given bytes that"] # [doc = " are terminated by `line_term`."] pub fn new (line_term : u8 , bytes : & 'b [u8]) -> LineIter < 'b > { let stepper = LineStep :: new (line_term , 0 , bytes . len ()) ; LineIter { bytes , stepper } } }
};
}
