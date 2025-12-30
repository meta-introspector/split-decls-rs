// Generated macro for gather_moves (function)
macro_rules! Depcrate_move_paths_buildergather_moves {
() => {
// Module: crate::move_paths::builder
// Provides: {"gather_moves"}
// Dependencies: {}
pub (super) fn gather_moves < 'tcx > (body : & Body < 'tcx > , tcx : TyCtxt < 'tcx > , filter : impl Fn (Ty < 'tcx >) -> bool ,) -> MoveData < 'tcx > { let mut builder = MoveDataBuilder :: new (body , tcx , filter) ; builder . gather_args () ; for (bb , block) in body . basic_blocks . iter_enumerated () { for (i , stmt) in block . statements . iter () . enumerate () { builder . loc = Location { block : bb , statement_index : i } ; builder . gather_statement (stmt) ; } builder . loc = Location { block : bb , statement_index : block . statements . len () } ; builder . gather_terminator (block . terminator ()) ; } builder . finalize () }
};
}
