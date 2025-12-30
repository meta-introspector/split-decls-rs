// Generated macro for impl_82 (impl)
macro_rules! Depcrate_runnerimpl_82 {
() => {
// Module: crate::runner
// Provides: {"impl_82"}
// Dependencies: {}
impl Stream { fn make_text (mut self) -> Self { let content = self . content . coerce_to (DataFormat :: Text) ; if content . format () != DataFormat :: Text { self . status = StreamStatus :: Failure ("Unable to convert underlying Data to Text" . into ()) ; } self . content = FilterNewlines . filter (FilterPaths . filter (content)) ; self } fn is_ok (& self) -> bool { self . status . is_ok () } }
};
}
