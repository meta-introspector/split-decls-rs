// Generated macro for NeedsDropTypes (struct)
macro_rules! Depcrate_needs_dropNeedsDropTypes {
() => {
// Module: crate::needs_drop
// Provides: {"NeedsDropTypes"}
// Dependencies: {}
struct NeedsDropTypes < 'tcx , F > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , query_ty : Ty < 'tcx > , seen_tys : FxHashSet < Ty < 'tcx > > , # [doc = " A stack of types left to process, and the recursion depth when we"] # [doc = " pushed that type. Each round, we pop something from the stack and check"] # [doc = " if it needs drop. If the result depends on whether some other types"] # [doc = " need drop we push them onto the stack."] unchecked_tys : Vec < (Ty < 'tcx > , usize) > , recursion_limit : Limit , adt_components : F , # [doc = " Set this to true if an exhaustive list of types involved in"] # [doc = " drop obligation is requested."] exhaustive : bool , }
};
}
