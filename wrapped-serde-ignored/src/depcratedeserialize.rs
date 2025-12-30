// Generated macro for deserialize (function)
macro_rules! Depcratedeserialize {
() => {
// Module: crate
// Provides: {"deserialize"}
// Dependencies: {}
# [doc = " Entry point. See crate documentation for an example."] pub fn deserialize < 'de , D , F , T > (deserializer : D , mut callback : F) -> Result < T , D :: Error > where D : de :: Deserializer < 'de > , F : FnMut (Path) , T : Deserialize < 'de > , { T :: deserialize (Deserializer :: new (deserializer , & mut callback)) }
};
}
