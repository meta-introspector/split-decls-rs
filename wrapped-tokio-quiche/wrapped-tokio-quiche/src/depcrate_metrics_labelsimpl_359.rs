// Generated macro for impl_359 (impl)
macro_rules! Depcrate_metrics_labelsimpl_359 {
() => {
// Module: crate::metrics::labels
// Provides: {"impl_359"}
// Dependencies: {}
impl std :: hash :: Hash for QuicInvalidInitialPacketError { fn hash < H > (& self , state : & mut H) where H : std :: hash :: Hasher , { std :: mem :: discriminant (self) . hash (state) ; if let Self :: WrongType (ty) = self { std :: mem :: discriminant (ty) . hash (state) ; } } }
};
}
