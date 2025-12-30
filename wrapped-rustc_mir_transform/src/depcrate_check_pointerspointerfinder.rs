// Generated macro for PointerFinder (struct)
macro_rules! Depcrate_check_pointersPointerFinder {
() => {
// Module: crate::check_pointers
// Provides: {"PointerFinder"}
// Dependencies: {}
struct PointerFinder < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , local_decls : & 'a mut LocalDecls < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , pointers : Vec < (Place < 'tcx > , Ty < 'tcx > , PlaceContext) > , excluded_pointees : & 'a [Ty < 'tcx >] , field_projection_mode : BorrowedFieldProjectionMode , }
};
}
