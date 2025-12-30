// Generated macro for CowStrDeserializer (struct)
macro_rules! Depcrate_de_valueCowStrDeserializer {
() => {
// Module: crate::de::value
// Provides: {"CowStrDeserializer"}
// Dependencies: {}
# [doc = " A deserializer holding a `Cow<str>`."] # [cfg (any (feature = "std" , feature = "alloc"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] pub struct CowStrDeserializer < 'a , E > { value : Cow < 'a , str > , marker : PhantomData < E > , }
};
}
