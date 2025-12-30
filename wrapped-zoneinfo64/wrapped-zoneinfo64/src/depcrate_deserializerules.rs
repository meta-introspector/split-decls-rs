// Generated macro for rules (function)
macro_rules! Depcrate_deserializerules {
() => {
// Module: crate::deserialize
// Provides: {"rules"}
// Dependencies: {}
fn rules < 'de , D : Deserializer < 'de > > (deserializer : D) -> Result < Vec < (& 'de str , TzRule) > , D :: Error > { struct RulesVisitor ; impl < 'de > Visitor < 'de > for RulesVisitor { type Value = Vec < (& 'de str , TzRule) > ; fn expecting (& self , formatter : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (formatter , "a sequence of UTF-16 slices") } fn visit_map < A > (self , mut map : A) -> Result < Self :: Value , A :: Error > where A : MapAccess < 'de > , { let mut vec = vec ! [] ; while let Some ((key , value)) = map . next_entry :: < & str , & [u8] > () ? { if value . as_ptr () . align_offset (core :: mem :: align_of :: < [i32 ; 11] > ()) != 0 || value . len () != core :: mem :: size_of :: < [i32 ; 11] > () { return Err (A :: Error :: custom ("Wrong length or align")) ; } let value = unsafe { & * (value . as_ptr () as * const [i32 ; 11]) } ; vec . push ((key , TzRule :: from_raw (value))) ; } Ok (vec) } } deserializer . deserialize_map (RulesVisitor) }
};
}
