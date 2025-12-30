// Generated macro for impl_182 (impl)
macro_rules! Depcrate_solve_eval_ctxt_probeimpl_182 {
() => {
// Module: crate::solve::eval_ctxt::probe
// Provides: {"impl_182"}
// Dependencies: {}
impl < D , I , F > TraitProbeCtxt < '_ , '_ , D , I , F > where D : SolverDelegate < Interner = I > , I : Interner , F : FnOnce (& QueryResult < I >) -> inspect :: ProbeKind < I > , { # [instrument (level = "debug" , skip_all , fields (source = ? self . source))] pub (in crate :: solve) fn enter (self , f : impl FnOnce (& mut EvalCtxt < '_ , D >) -> QueryResult < I > ,) -> Result < Candidate < I > , NoSolution > { let (result , head_usages) = self . cx . enter_single_candidate (f) ; result . map (| result | Candidate { source : self . source , result , head_usages }) } }
};
}
