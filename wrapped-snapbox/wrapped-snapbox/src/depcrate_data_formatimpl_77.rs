// Generated macro for impl_77 (impl)
macro_rules! Depcrate_data_formatimpl_77 {
() => {
// Module: crate::data::format
// Provides: {"impl_77"}
// Dependencies: {}
impl DataFormat { # [doc = " Assumed file extension for the format"] pub fn ext (self) -> & 'static str { match self { Self :: Error => "txt" , Self :: Binary => "bin" , Self :: Text => "txt" , # [cfg (feature = "json")] Self :: Json => "json" , # [cfg (feature = "json")] Self :: JsonLines => "jsonl" , # [cfg (feature = "term-svg")] Self :: TermSvg => "term.svg" , } } }
};
}
