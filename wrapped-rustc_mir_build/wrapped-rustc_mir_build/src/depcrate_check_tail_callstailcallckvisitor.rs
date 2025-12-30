// Generated macro for TailCallCkVisitor (struct)
macro_rules! Depcrate_check_tail_callsTailCallCkVisitor {
() => {
// Module: crate::check_tail_calls
// Provides: {"TailCallCkVisitor"}
// Dependencies: {}
struct TailCallCkVisitor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , thir : & 'a Thir < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , # [doc = " Whatever the currently checked body is one of a closure"] is_closure : bool , # [doc = " The result of the checks, `Err(_)` if there was a problem with some"] # [doc = " tail call, `Ok(())` if all of them were fine."] found_errors : Result < () , ErrorGuaranteed > , # [doc = " Type of the caller function."] caller_ty : Ty < 'tcx > , }
};
}
