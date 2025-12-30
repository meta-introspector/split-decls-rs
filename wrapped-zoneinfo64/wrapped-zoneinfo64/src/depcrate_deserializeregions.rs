// Generated macro for regions (function)
macro_rules! Depcrate_deserializeregions {
() => {
// Module: crate::deserialize
// Provides: {"regions"}
// Dependencies: {}
fn regions < 'de , D : Deserializer < 'de > > (deserializer : D) -> Result < Vec < Region > , D :: Error > { struct RegionsVisitor ; impl < 'de > Visitor < 'de > for RegionsVisitor { type Value = Vec < Region > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (formatter , "a sequence of UTF-16 slices") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let mut vec = vec ! [] ; while let Some (bytes) = seq . next_element :: < & [u8] > () ? { let utf16 = potential_utf :: PotentialUtf16 :: from_slice (unsafe { resb :: binary :: helpers :: cast_bytes_to_slice :: < _ , A :: Error > (bytes) ? } ,) ; let mut utf16 = utf16 . chars () ; let Ok (region) = Region :: try_from_raw ([utf16 . next () . filter (char :: is_ascii) . unwrap_or_default () as u8 , utf16 . next () . filter (char :: is_ascii) . unwrap_or_default () as u8 , utf16 . next () . filter (char :: is_ascii) . unwrap_or_default () as u8 ,]) else { return Err (A :: Error :: custom ("Invalid region code")) ; } ; vec . push (region) ; } Ok (vec) } } deserializer . deserialize_seq (RegionsVisitor) }
};
}
