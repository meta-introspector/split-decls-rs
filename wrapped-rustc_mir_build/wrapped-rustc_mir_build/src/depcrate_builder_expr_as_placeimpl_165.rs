// Generated macro for impl_165 (impl)
macro_rules! Depcrate_builder_expr_as_placeimpl_165 {
() => {
// Module: crate::builder::expr::as_place
// Provides: {"impl_165"}
// Dependencies: {}
impl < 'tcx > From < Place < 'tcx > > for PlaceBuilder < 'tcx > { fn from (p : Place < 'tcx >) -> Self { Self { base : PlaceBase :: Local (p . local) , projection : p . projection . to_vec () } } }
};
}
