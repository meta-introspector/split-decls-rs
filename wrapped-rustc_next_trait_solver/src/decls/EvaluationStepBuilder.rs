macro_rules! deps {
    () => {
        WipEvaluationStep!();
        SolverDelegate!();
    };
}

macro_rules! EvaluationStepBuilder {
    () => {
        deps!();
        pub (crate) struct EvaluationStepBuilder < D , I = < D as SolverDelegate > :: Interner > where D : SolverDelegate < Interner = I > , I : Interner , { state : Option < Box < WipEvaluationStep < I > > > , _infcx : PhantomData < D > , }
    };
}

EvaluationStepBuilder!();