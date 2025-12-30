// Generated macro for deserialize (function)
macro_rules! Depcrate_adjacentlydeserialize {
() => {
// Module: crate::adjacently
// Provides: {"deserialize"}
// Dependencies: {}
pub fn deserialize < 'de , D , T > (deserializer : D , trait_object : & 'static str , field_names : & 'static [& 'static str ; 2] , default_variant : Option < & 'static str > , registry : & 'static Registry < T > , deny_unknown_fields : bool ,) -> Result < Box < T > , D :: Error > where D : Deserializer < 'de > , T : ? Sized , { let visitor = TaggedVisitor { trait_object , field_names , default_variant , registry , deny_unknown_fields , } ; deserializer . deserialize_struct (trait_object , field_names , visitor) }
};
}
