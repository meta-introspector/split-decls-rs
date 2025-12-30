// Generated macro for impl_387 (impl)
macro_rules! Depcrate_filterimpl_387 {
() => {
// Module: crate::filter
// Provides: {"impl_387"}
// Dependencies: {}
impl Filter for NormalizeRedactions < '_ > { fn filter (& self , data : Data) -> Data { let source = data . source ; let filters = data . filters ; let inner = match data . inner { DataInner :: Error (err) => DataInner :: Error (err) , DataInner :: Binary (bin) => DataInner :: Binary (bin) , DataInner :: Text (text) => { let lines = self . redactions . redact (& text) ; DataInner :: Text (lines) } # [cfg (feature = "json")] DataInner :: Json (value) => { let mut value = value ; normalize_json_string (& mut value , & | s | self . redactions . redact (s)) ; DataInner :: Json (value) } # [cfg (feature = "json")] DataInner :: JsonLines (value) => { let mut value = value ; normalize_json_string (& mut value , & | s | self . redactions . redact (s)) ; DataInner :: JsonLines (value) } # [cfg (feature = "term-svg")] DataInner :: TermSvg (text) => { let lines = self . redactions . redact (& text) ; DataInner :: TermSvg (lines) } } ; Data { inner , source , filters , } } }
};
}
