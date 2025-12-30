// Generated macro for impl_87 (impl)
macro_rules! Depcrate_de_valueimpl_87 {
() => {
// Module: crate::de::value
// Provides: {"impl_87"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'a , E > CowStrDeserializer < 'a , E > { # [allow (missing_docs)] pub fn new (value : Cow < 'a , str >) -> Self { CowStrDeserializer { value , marker : PhantomData , } } }
};
}
