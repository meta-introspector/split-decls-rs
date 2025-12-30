// Generated macro for ProbeCtxt (struct)
macro_rules! Depcrate_solve_eval_ctxt_probeProbeCtxt {
() => {
// Module: crate::solve::eval_ctxt::probe
// Provides: {"ProbeCtxt"}
// Dependencies: {}
pub (in crate :: solve) struct ProbeCtxt < 'me , 'a , D , I , F , T > where D : SolverDelegate < Interner = I > , I : Interner , { ecx : & 'me mut EvalCtxt < 'a , D , I > , probe_kind : F , _result : PhantomData < T > , }
};
}
