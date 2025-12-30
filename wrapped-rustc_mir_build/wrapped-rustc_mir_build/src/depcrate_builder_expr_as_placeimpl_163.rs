// Generated macro for impl_163 (impl)
macro_rules! Depcrate_builder_expr_as_placeimpl_163 {
() => {
// Module: crate::builder::expr::as_place
// Provides: {"impl_163"}
// Dependencies: {}
impl < 'tcx > From < Local > for PlaceBuilder < 'tcx > { fn from (local : Local) -> Self { Self { base : PlaceBase :: Local (local) , projection : Vec :: new () } } }
};
}
