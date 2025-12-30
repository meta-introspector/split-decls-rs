// Generated macro for impl_369 (impl)
macro_rules! Depcrate_rustc_peekimpl_369 {
() => {
// Module: crate::rustc_peek
// Provides: {"impl_369"}
// Dependencies: {}
impl < 'tcx , A , D > RustcPeekAt < 'tcx > for A where A : Analysis < 'tcx , Domain = D > + HasMoveData < 'tcx > , D : JoinSemiLattice + Clone + BitSetExt < MovePathIndex > , { fn peek_at (& self , tcx : TyCtxt < 'tcx > , place : mir :: Place < 'tcx > , state : & Self :: Domain , call : PeekCall ,) { match self . move_data () . rev_lookup . find (place . as_ref ()) { LookupResult :: Exact (peek_mpi) => { let bit_state = state . contains (peek_mpi) ; debug ! ("rustc_peek({:?} = &{:?}) bit_state: {}" , call . arg , place , bit_state) ; if ! bit_state { tcx . dcx () . emit_err (PeekBitNotSet { span : call . span }) ; } } LookupResult :: Parent (..) => { tcx . dcx () . emit_err (PeekArgumentUntracked { span : call . span }) ; } } } }
};
}
