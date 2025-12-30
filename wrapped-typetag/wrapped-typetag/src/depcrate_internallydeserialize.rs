// Generated macro for deserialize (function)
macro_rules! Depcrate_internallydeserialize {
() => {
// Module: crate::internally
// Provides: {"deserialize"}
// Dependencies: {}
pub fn deserialize < 'de , D , T > (deserializer : D , trait_object : & 'static str , tag : & 'static str , default_variant : Option < & 'static str > , registry : & 'static Registry < T > ,) -> Result < Box < T > , D :: Error > where D : Deserializer < 'de > , T : ? Sized , { let visitor = TaggedVisitor { trait_object , tag , default_variant , registry , } ; deserializer . deserialize_map (visitor) }
};
}
