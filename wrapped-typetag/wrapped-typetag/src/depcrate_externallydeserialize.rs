// Generated macro for deserialize (function)
macro_rules! Depcrate_externallydeserialize {
() => {
// Module: crate::externally
// Provides: {"deserialize"}
// Dependencies: {}
pub fn deserialize < 'de , D , T > (deserializer : D , trait_object : & 'static str , registry : & 'static Registry < T > ,) -> Result < Box < T > , D :: Error > where D : Deserializer < 'de > , T : ? Sized , { let visitor = TaggedVisitor { trait_object , registry , } ; deserializer . deserialize_map (visitor) }
};
}
