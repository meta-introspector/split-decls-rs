// Generated macro for encode_formatted (function)
macro_rules! Depcrate_encodeencode_formatted {
() => {
// Module: crate::encode
// Provides: {"encode_formatted"}
// Dependencies: {}
pub (crate) fn encode_formatted < T : ValueRepr > (this : & Formatted < T > , buf : & mut dyn Write , input : Option < & str > , default_decor : (& str , & str) ,) -> Result { let decor = this . decor () ; decor . prefix_encode (buf , input , default_decor . 0) ? ; if let Some (input) = input { let repr = this . as_repr () . map (Cow :: Borrowed) . unwrap_or_else (| | Cow :: Owned (this . default_repr ())) ; repr . encode (buf , input) ? ; } else { let repr = this . display_repr () ; write ! (buf , "{repr}") ? ; } ; decor . suffix_encode (buf , input , default_decor . 1) ? ; Ok (()) }
};
}
