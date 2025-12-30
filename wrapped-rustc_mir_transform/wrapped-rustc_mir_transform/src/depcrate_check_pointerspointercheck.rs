// Generated macro for PointerCheck (struct)
macro_rules! Depcrate_check_pointersPointerCheck {
() => {
// Module: crate::check_pointers
// Provides: {"PointerCheck"}
// Dependencies: {}
# [doc = " Details of a pointer check, the condition on which we decide whether to"] # [doc = " fail the assert and an [AssertKind] that defines the behavior on failure."] pub (crate) struct PointerCheck < 'tcx > { pub (crate) cond : Operand < 'tcx > , pub (crate) assert_kind : Box < AssertKind < Operand < 'tcx > > > , }
};
}
