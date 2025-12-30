// Generated macro for impl_133 (impl)
macro_rules! Depcrate_zone_ianaimpl_133 {
() => {
// Module: crate::zone::iana
// Provides: {"impl_133"}
// Dependencies: {}
impl < 'a > Iterator for TimeZoneAndCanonicalAndNormalizedIter < 'a > { type Item = TimeZoneAndCanonicalAndNormalized < 'a > ; fn next (& mut self) -> Option < Self :: Item > { if let (Some (time_zone) , Some (canonical)) = (self . 1 . inner . data . bcp47_ids . get (self . 0) , self . 1 . data . normalized_iana_ids . get (self . 0) ,) { self . 0 += 1 ; Some (TimeZoneAndCanonicalAndNormalized { time_zone , canonical , normalized : canonical , }) } else if let Some (normalized) = self . 1 . data . normalized_iana_ids . get (self . 0) { let Some (trie_value) = self . 1 . inner . trie_value (normalized . as_bytes ()) else { debug_assert ! (false , "normalized value should be in trie") ; return None ; } ; let (Some (time_zone) , Some (canonical)) = (self . 1 . inner . data . bcp47_ids . get (trie_value . index ()) , self . 1 . data . normalized_iana_ids . get (trie_value . index ()) ,) else { debug_assert ! (false , "index should be in range") ; return None ; } ; self . 0 += 1 ; Some (TimeZoneAndCanonicalAndNormalized { time_zone , canonical , normalized , }) } else { None } } }
};
}
