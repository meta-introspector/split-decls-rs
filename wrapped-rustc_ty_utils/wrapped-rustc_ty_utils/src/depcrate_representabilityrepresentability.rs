// Generated macro for representability (function)
macro_rules! Depcrate_representabilityrepresentability {
() => {
// Module: crate::representability
// Provides: {"representability"}
// Dependencies: {}
fn representability (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Representability { match tcx . def_kind (def_id) { DefKind :: Struct | DefKind :: Union | DefKind :: Enum => { for variant in tcx . adt_def (def_id) . variants () { for field in variant . fields . iter () { rtry ! (tcx . representability (field . did . expect_local ())) ; } } Representability :: Representable } DefKind :: Field => representability_ty (tcx , tcx . type_of (def_id) . instantiate_identity ()) , def_kind => bug ! ("unexpected {def_kind:?}") , } }
};
}
