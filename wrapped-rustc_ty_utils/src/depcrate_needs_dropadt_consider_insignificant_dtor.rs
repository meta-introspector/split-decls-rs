// Generated macro for adt_consider_insignificant_dtor (function)
macro_rules! Depcrate_needs_dropadt_consider_insignificant_dtor {
() => {
// Module: crate::needs_drop
// Provides: {"adt_consider_insignificant_dtor"}
// Dependencies: {}
fn adt_consider_insignificant_dtor < 'tcx > (tcx : TyCtxt < 'tcx > ,) -> impl Fn (ty :: AdtDef < 'tcx >) -> Option < DtorType > { move | adt_def : ty :: AdtDef < 'tcx > | { let is_marked_insig = tcx . has_attr (adt_def . did () , sym :: rustc_insignificant_dtor) ; if is_marked_insig { Some (DtorType :: Insignificant) } else if adt_def . destructor (tcx) . is_some () { Some (DtorType :: Significant) } else { None } } }
};
}
