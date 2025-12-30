// Generated macro for impl_232 (impl)
macro_rules! Depcrate_impls_initializedimpl_232 {
() => {
// Module: crate::impls::initialized
// Provides: {"impl_232"}
// Dependencies: {}
impl < 'a , 'tcx > MaybeInitializedPlaces < 'a , 'tcx > { pub fn new (tcx : TyCtxt < 'tcx > , body : & 'a Body < 'tcx > , move_data : & 'a MoveData < 'tcx >) -> Self { MaybeInitializedPlaces { tcx , body , move_data , exclude_inactive_in_otherwise : false , skip_unreachable_unwind : false , } } # [doc = " Ensures definitely inactive variants are excluded from the set of initialized places for"] # [doc = " blocks reached through an `otherwise` edge."] pub fn exclude_inactive_in_otherwise (mut self) -> Self { self . exclude_inactive_in_otherwise = true ; self } pub fn skipping_unreachable_unwind (mut self) -> Self { self . skip_unreachable_unwind = true ; self } pub fn is_unwind_dead (& self , place : mir :: Place < 'tcx > , state : & < Self as Analysis < 'tcx > > :: Domain ,) -> bool { if let LookupResult :: Exact (path) = self . move_data () . rev_lookup . find (place . as_ref ()) { let mut maybe_live = false ; on_all_children_bits (self . move_data () , path , | child | { maybe_live |= state . contains (child) ; }) ; ! maybe_live } else { false } } }
};
}
