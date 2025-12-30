// Generated macro for impl_379 (impl)
macro_rules! Depcrate_filterimpl_379 {
() => {
// Module: crate::filter
// Provides: {"impl_379"}
// Dependencies: {}
impl Filter for FilterNewlines { fn filter (& self , data : Data) -> Data { let source = data . source ; let filters = data . filters ; let inner = match data . inner { DataInner :: Error (err) => DataInner :: Error (err) , DataInner :: Binary (bin) => DataInner :: Binary (bin) , DataInner :: Text (text) => { let lines = normalize_lines (& text) ; DataInner :: Text (lines) } # [cfg (feature = "json")] DataInner :: Json (value) => { let mut value = value ; normalize_json_string (& mut value , & normalize_lines) ; DataInner :: Json (value) } # [cfg (feature = "json")] DataInner :: JsonLines (value) => { let mut value = value ; normalize_json_string (& mut value , & normalize_lines) ; DataInner :: JsonLines (value) } # [cfg (feature = "term-svg")] DataInner :: TermSvg (text) => { let lines = normalize_lines (& text) ; DataInner :: TermSvg (lines) } } ; Data { inner , source , filters , } } }
};
}
