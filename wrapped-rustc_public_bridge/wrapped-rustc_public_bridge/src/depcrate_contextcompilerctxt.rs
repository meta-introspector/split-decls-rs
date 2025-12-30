// Generated macro for CompilerCtxt (struct)
macro_rules! Depcrate_contextCompilerCtxt {
() => {
// Module: crate::context
// Provides: {"CompilerCtxt"}
// Dependencies: {}
# [doc = " Provides direct access to rustc's internal queries."] # [doc = ""] # [doc = " `CompilerInterface` must go through"] # [doc = " this context to obtain internal information."] pub struct CompilerCtxt < 'tcx , B : Bridge > { pub tcx : TyCtxt < 'tcx > , _marker : PhantomData < B > , }
};
}
