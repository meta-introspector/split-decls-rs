// Generated macro for impl_48 (impl)
macro_rules! Depcrate_contentimpl_48 {
() => {
// Module: crate::content
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'de , E > SeqDeserializer < 'de , E > where E : de :: Error , { fn new (vec : Vec < Content < 'de > >) -> Self { SeqDeserializer { iter : vec . into_iter () , err : PhantomData , } } }
};
}
