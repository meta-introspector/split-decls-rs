// Generated macro for NeverDeserializer (struct)
macro_rules! Depcrate_de_valueNeverDeserializer {
() => {
// Module: crate::de::value
// Provides: {"NeverDeserializer"}
// Dependencies: {}
# [doc = " A deserializer that cannot be instantiated."] # [cfg (feature = "unstable")] # [cfg_attr (docsrs , doc (cfg (feature = "unstable")))] pub struct NeverDeserializer < E > { never : ! , marker : PhantomData < E > , }
};
}
