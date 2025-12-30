// Generated macro for rustc (module)
macro_rules! Depcrate_layoutrustc {
() => {
// Module: crate::layout
// Provides: {"rustc"}
// Dependencies: {}
# [cfg (feature = "rustc")] pub mod rustc { use rustc_abi :: Layout ; use rustc_middle :: ty :: layout :: { HasTyCtxt , LayoutCx , LayoutError } ; use rustc_middle :: ty :: { self , Region , Ty } ; # [doc = " A visibility node in the layout."] # [derive (Debug , Hash , Eq , PartialEq , Clone , Copy)] pub enum Def < 'tcx > { Adt (ty :: AdtDef < 'tcx >) , Variant (& 'tcx ty :: VariantDef) , Field (& 'tcx ty :: FieldDef) , Primitive , } impl < 'tcx > super :: Def for Def < 'tcx > { fn has_safety_invariants (& self) -> bool { self != & Self :: Primitive } } impl < 'tcx > super :: Region for Region < 'tcx > { } impl < 'tcx > super :: Type for Ty < 'tcx > { } pub (crate) fn layout_of < 'tcx > (cx : LayoutCx < 'tcx > , ty : Ty < 'tcx > ,) -> Result < Layout < 'tcx > , & 'tcx LayoutError < 'tcx > > { use rustc_middle :: ty :: layout :: LayoutOf ; let ty = cx . tcx () . erase_and_anonymize_regions (ty) ; cx . layout_of (ty) . map (| tl | tl . layout) } }
};
}
