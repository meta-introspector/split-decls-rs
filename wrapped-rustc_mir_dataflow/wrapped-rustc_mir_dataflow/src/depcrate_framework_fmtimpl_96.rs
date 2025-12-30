// Generated macro for impl_96 (impl)
macro_rules! Depcrate_framework_fmtimpl_96 {
() => {
// Module: crate::framework::fmt
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'tcx , C > DebugWithContext < C > for crate :: move_paths :: MovePathIndex where C : crate :: move_paths :: HasMoveData < 'tcx > , { fn fmt_with (& self , ctxt : & C , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , ctxt . move_data () . move_paths [* self]) } }
};
}
