// Generated macro for impl_1267 (impl)
macro_rules! Depcrate_sortimpl_1267 {
() => {
// Module: crate::sort
// Provides: {"impl_1267"}
// Dependencies: {}
impl < 'a > VersionChunkIter < 'a > { pub (crate) fn new (ident : & 'a str) -> Self { Self { ident , start : 0 } } fn parse_numeric_chunk (& mut self , mut chars : std :: str :: CharIndices < 'a > ,) -> Option < VersionChunk < 'a > > { let mut end = self . start ; let mut is_end_of_chunk = false ; while let Some ((idx , c)) = chars . next () { end = self . start + idx ; if c . is_ascii_digit () { continue ; } is_end_of_chunk = true ; break ; } let source = if is_end_of_chunk { let value = & self . ident [self . start .. end] ; self . start = end ; value } else { let value = & self . ident [self . start ..] ; self . start = self . ident . len () ; value } ; let zeros = source . chars () . take_while (| c | * c == '0') . count () ; let value = source . parse :: < usize > () . ok () ? ; Some (VersionChunk :: Number { value , zeros , source , }) } fn parse_str_chunk (& mut self , mut chars : std :: str :: CharIndices < 'a > ,) -> Option < VersionChunk < 'a > > { let mut end = self . start ; let mut is_end_of_chunk = false ; while let Some ((idx , c)) = chars . next () { end = self . start + idx ; if c == '_' { is_end_of_chunk = true ; break ; } if ! c . is_numeric () { continue ; } is_end_of_chunk = true ; break ; } let source = if is_end_of_chunk { let value = & self . ident [self . start .. end] ; self . start = end ; value } else { let value = & self . ident [self . start ..] ; self . start = self . ident . len () ; value } ; Some (VersionChunk :: Str (source)) } }
};
}
