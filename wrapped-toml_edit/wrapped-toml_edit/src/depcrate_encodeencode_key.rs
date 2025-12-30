// Generated macro for encode_key (function)
macro_rules! Depcrate_encodeencode_key {
() => {
// Module: crate::encode
// Provides: {"encode_key"}
// Dependencies: {}
pub (crate) fn encode_key (this : & Key , buf : & mut dyn Write , input : Option < & str >) -> Result { if let Some (input) = input { let repr = this . as_repr () . map (Cow :: Borrowed) . unwrap_or_else (| | Cow :: Owned (this . default_repr ())) ; repr . encode (buf , input) ? ; } else { let repr = this . display_repr () ; write ! (buf , "{repr}") ? ; } ; Ok (()) }
};
}
