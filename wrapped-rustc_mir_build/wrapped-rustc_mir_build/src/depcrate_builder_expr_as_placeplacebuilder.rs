// Generated macro for PlaceBuilder (struct)
macro_rules! Depcrate_builder_expr_as_placePlaceBuilder {
() => {
// Module: crate::builder::expr::as_place
// Provides: {"PlaceBuilder"}
// Dependencies: {}
# [doc = " `PlaceBuilder` is used to create places during MIR construction. It allows you to \"build up\" a"] # [doc = " place by pushing more and more projections onto the end, and then convert the final set into a"] # [doc = " place using the `to_place` method."] # [doc = ""] # [doc = " This is used internally when building a place for an expression like `a.b.c`. The fields `b`"] # [doc = " and `c` can be progressively pushed onto the place builder that is created when converting `a`."] # [derive (Clone , Debug , PartialEq)] pub (in crate :: builder) struct PlaceBuilder < 'tcx > { base : PlaceBase , projection : Vec < PlaceElem < 'tcx > > , }
};
}
