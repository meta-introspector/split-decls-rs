// Generated macro for impl_52 (impl)
macro_rules! Depcrate_contentimpl_52 {
() => {
// Module: crate::content
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'de , E > MapDeserializer < 'de , E > where E : de :: Error , { fn new (map : Vec < (Content < 'de > , Content < 'de >) >) -> Self { MapDeserializer { iter : map . into_iter () , value : None , err : PhantomData , } } }
};
}
