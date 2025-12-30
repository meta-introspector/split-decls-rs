// Generated macro for impl_14 (impl)
macro_rules! Depcrate_termimpl_14 {
() => {
// Module: crate::term
// Provides: {"impl_14"}
// Dependencies: {}
impl Term { fn new () -> Self { Term { spec : ColorSpec :: new () , stream : Stream :: stderr (ColorChoice :: Auto) , start_of_line : true , } } fn set_color (& mut self , spec : & ColorSpec) { if self . spec != * spec { self . spec = spec . clone () ; self . start_of_line = true ; } } fn reset (& mut self) { self . spec = ColorSpec :: new () ; let _ = self . stream . reset () ; } }
};
}
