// Generated macro for impl_1508 (impl)
macro_rules! Depcrate_typesimpl_1508 {
() => {
// Module: crate::types
// Provides: {"impl_1508"}
// Dependencies: {}
impl < 'a > SegmentParam < 'a > { fn from_generic_arg (arg : & ast :: GenericArg) -> SegmentParam < '_ > { match arg { ast :: GenericArg :: Lifetime (ref lt) => SegmentParam :: LifeTime (lt) , ast :: GenericArg :: Type (ref ty) => SegmentParam :: Type (ty) , ast :: GenericArg :: Const (const_) => SegmentParam :: Const (const_) , } } }
};
}
