// Generated macro for TraitProbeCtxt (struct)
macro_rules! Depcrate_solve_eval_ctxt_probeTraitProbeCtxt {
() => {
// Module: crate::solve::eval_ctxt::probe
// Provides: {"TraitProbeCtxt"}
// Dependencies: {}
pub (in crate :: solve) struct TraitProbeCtxt < 'me , 'a , D , I , F > where D : SolverDelegate < Interner = I > , I : Interner , { cx : ProbeCtxt < 'me , 'a , D , I , F , QueryResult < I > > , source : CandidateSource < I > , }
};
}
