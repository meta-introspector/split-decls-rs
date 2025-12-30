// Generated macro for impl_106 (impl)
macro_rules! Depcrate_de_valueimpl_106 {
() => {
// Module: crate::de::value
// Provides: {"impl_106"}
// Dependencies: {}
impl < I , E > SeqDeserializer < I , E > where I : Iterator , { # [doc = " Construct a new `SeqDeserializer<I, E>`."] pub fn new (iter : I) -> Self { SeqDeserializer { iter : iter . fuse () , count : 0 , marker : PhantomData , } } }
};
}
