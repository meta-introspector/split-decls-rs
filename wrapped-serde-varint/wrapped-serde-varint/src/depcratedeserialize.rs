// Generated macro for deserialize (function)
macro_rules! Depcratedeserialize {
() => {
// Module: crate
// Provides: {"deserialize"}
// Dependencies: {}
pub fn deserialize < 'de , D , T > (deserializer : D) -> Result < T , D :: Error > where D : Deserializer < 'de > , T : VarInt , { deserializer . deserialize_tuple ((std :: mem :: size_of :: < T > () * 8) . div_ceil (7) , VarIntVisitor { phantom : PhantomData , } ,) }
};
}
