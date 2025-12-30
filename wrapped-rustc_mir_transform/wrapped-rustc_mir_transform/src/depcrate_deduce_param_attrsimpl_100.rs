// Generated macro for impl_100 (impl)
macro_rules! Depcrate_deduce_param_attrsimpl_100 {
() => {
// Module: crate::deduce_param_attrs
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for DeduceReadOnly { fn visit_place (& mut self , place : & Place < 'tcx > , context : PlaceContext , _location : Location) { if place . local == RETURN_PLACE || place . local . index () > self . mutable_args . domain_size () { return ; } let mark_as_mutable = match context { PlaceContext :: MutatingUse (..) => { true } PlaceContext :: NonMutatingUse (NonMutatingUseContext :: RawBorrow) => { ! place . is_indirect () } PlaceContext :: NonMutatingUse (..) | PlaceContext :: NonUse (..) => { false } } ; if mark_as_mutable { self . mutable_args . insert (place . local . index () - 1) ; } } fn visit_terminator (& mut self , terminator : & Terminator < 'tcx > , location : Location) { if let TerminatorKind :: Call { ref args , .. } = terminator . kind { for arg in args { if let Operand :: Move (place) = arg . node { let local = place . local ; if place . is_indirect () || local == RETURN_PLACE || local . index () > self . mutable_args . domain_size () { continue ; } self . mutable_args . insert (local . index () - 1) ; } } } ; self . super_terminator (terminator , location) ; } }
};
}
