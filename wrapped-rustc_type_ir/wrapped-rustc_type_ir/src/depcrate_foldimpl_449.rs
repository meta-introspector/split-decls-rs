// Generated macro for impl_449 (impl)
macro_rules! Depcrate_foldimpl_449 {
() => {
// Module: crate::fold
// Provides: {"impl_449"}
// Dependencies: {}
impl < I , F > RegionFolder < I , F > { # [inline] pub fn new (cx : I , fold_region_fn : F) -> RegionFolder < I , F > { RegionFolder { cx , current_index : ty :: INNERMOST , fold_region_fn } } }
};
}
