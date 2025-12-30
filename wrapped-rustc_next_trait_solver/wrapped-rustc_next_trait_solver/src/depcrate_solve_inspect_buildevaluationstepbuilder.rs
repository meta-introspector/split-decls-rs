// Generated macro for EvaluationStepBuilder (struct)
macro_rules! Depcrate_solve_inspect_buildEvaluationStepBuilder {
() => {
// Module: crate::solve::inspect::build
// Provides: {"EvaluationStepBuilder"}
// Dependencies: {}
pub (crate) struct EvaluationStepBuilder < D , I = < D as SolverDelegate > :: Interner > where D : SolverDelegate < Interner = I > , I : Interner , { state : Option < Box < WipEvaluationStep < I > > > , _infcx : PhantomData < D > , }
};
}
