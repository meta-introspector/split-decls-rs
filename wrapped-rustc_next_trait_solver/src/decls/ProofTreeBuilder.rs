macro_rules! deps {
    () => {
        EvaluationStepBuilder!();
        SolverDelegate!();
        EvalCtxt!();
    };
}

macro_rules! ProofTreeBuilder {
    () => {
        deps!();
        # [doc = " We need to know whether to build a prove tree while evaluating. We"] # [doc = " pass a `ProofTreeBuilder` with `state: Some(None)` into the search"] # [doc = " graph which then causes the initial `EvalCtxt::compute_goal` to actually"] # [doc = " build a proof tree which then gets written into the `state`."] # [doc = ""] # [doc = " Building the proof tree for a single evaluation step happens via the"] # [doc = " [EvaluationStepBuilder] which is updated by the `EvalCtxt` when"] # [doc = " appropriate."] pub (crate) struct ProofTreeBuilder < D , I = < D as SolverDelegate > :: Interner > where D : SolverDelegate < Interner = I > , I : Interner , { state : Option < Box < Option < inspect :: Probe < I > > > > , _infcx : PhantomData < D > , }
    };
}

ProofTreeBuilder!()