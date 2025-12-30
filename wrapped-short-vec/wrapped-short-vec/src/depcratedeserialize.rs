// Generated macro for deserialize (function)
macro_rules! Depcratedeserialize {
() => {
// Module: crate
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " If you don't want to use the ShortVec newtype, you can do ShortVec"] # [doc = " deserialization on an ordinary vector with the following field annotation:"] # [doc = ""] # [doc = " #[serde(with = \"short_vec\")]"] # [doc = ""] pub fn deserialize < 'de , D , T > (deserializer : D) -> Result < Vec < T > , D :: Error > where D : Deserializer < 'de > , T : Deserialize < 'de > , { let visitor = ShortVecVisitor { _t : PhantomData } ; deserializer . deserialize_tuple (usize :: MAX , visitor) }
};
}
