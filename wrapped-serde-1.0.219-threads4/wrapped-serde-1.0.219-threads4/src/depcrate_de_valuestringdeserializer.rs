// Generated macro for StringDeserializer (struct)
macro_rules! Depcrate_de_valueStringDeserializer {
() => {
// Module: crate::de::value
// Provides: {"StringDeserializer"}
// Dependencies: {}
# [doc = " A deserializer holding a `String`."] # [cfg (any (feature = "std" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] pub struct StringDeserializer < E > { value : String , marker : PhantomData < E > , }
};
}
