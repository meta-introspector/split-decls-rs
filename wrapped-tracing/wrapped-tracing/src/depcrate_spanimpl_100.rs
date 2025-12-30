// Generated macro for impl_100 (impl)
macro_rules! Depcrate_spanimpl_100 {
() => {
// Module: crate::span
// Provides: {"impl_100"}
// Dependencies: {}
impl EnteredSpan { # [doc = " Returns this span's `Id`, if it is enabled."] pub fn id (& self) -> Option < Id > { self . inner . as_ref () . map (Inner :: id) } # [doc = " Exits this span, returning the underlying [`Span`]."] # [inline] pub fn exit (mut self) -> Span { let span = mem :: replace (& mut self . span , Span :: none ()) ; span . do_exit () ; span } }
};
}
