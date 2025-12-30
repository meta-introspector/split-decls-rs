// Generated macro for impl_122 (impl)
macro_rules! Depcrate_de_valueimpl_122 {
() => {
// Module: crate::de::value
// Provides: {"impl_122"}
// Dependencies: {}
impl < 'de , I , E > MapDeserializer < 'de , I , E > where I : Iterator , I :: Item : private :: Pair , { # [doc = " Construct a new `MapDeserializer<I, E>`."] pub fn new (iter : I) -> Self { MapDeserializer { iter : iter . fuse () , value : None , count : 0 , lifetime : PhantomData , error : PhantomData , } } }
};
}
