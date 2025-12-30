// Generated macro for normalize_data_to_unordered (function)
macro_rules! Depcrate_filter_patternnormalize_data_to_unordered {
() => {
// Module: crate::filter::pattern
// Provides: {"normalize_data_to_unordered"}
// Dependencies: {}
fn normalize_data_to_unordered (actual : Data , expected : & Data) -> Data { let source = actual . source ; let filters = actual . filters ; let inner = match (actual . inner , & expected . inner) { (DataInner :: Error (err) , _) => DataInner :: Error (err) , (DataInner :: Binary (bin) , _) => DataInner :: Binary (bin) , (DataInner :: Text (text) , _) => { if let Some (pattern) = expected . render () { let lines = normalize_str_to_unordered (& text , & pattern) ; DataInner :: Text (lines) } else { DataInner :: Text (text) } } # [cfg (feature = "json")] (DataInner :: Json (value) , DataInner :: Json (exp)) => { let mut value = value ; normalize_value_to_unordered (& mut value , exp) ; DataInner :: Json (value) } # [cfg (feature = "json")] (DataInner :: JsonLines (value) , DataInner :: JsonLines (exp)) => { let mut value = value ; normalize_value_to_unordered (& mut value , exp) ; DataInner :: JsonLines (value) } # [cfg (feature = "term-svg")] (DataInner :: TermSvg (text) , DataInner :: TermSvg (exp)) => { if let (Some ((header , body , footer)) , Some ((_ , exp , _))) = (crate :: data :: split_term_svg (& text) , crate :: data :: split_term_svg (exp) ,) { let lines = normalize_str_to_unordered (body , exp) ; DataInner :: TermSvg (format ! ("{header}{lines}{footer}")) } else { DataInner :: TermSvg (text) } } # [allow (unreachable_patterns)] (inner , _) => inner , } ; Data { inner , source , filters , } }
};
}
