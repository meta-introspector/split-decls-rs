macro_rules! deps {
    () => {
        EvaluationStepBuilder!();
        WipProbe!();
        SolverDelegate!();
        WipEvaluationStep!();
        ProofTreeBuilder!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < D : SolverDelegate < Interner = I > , I : Interner > ProofTreeBuilder < D > { pub (crate) fn new () -> ProofTreeBuilder < D > { ProofTreeBuilder { state : Some (Box :: new (None)) , _infcx : PhantomData } } pub (crate) fn new_noop () -> ProofTreeBuilder < D > { ProofTreeBuilder { state : None , _infcx : PhantomData } } pub (crate) fn is_noop (& self) -> bool { self . state . is_none () } pub (crate) fn new_evaluation_step (& mut self , var_values : ty :: CanonicalVarValues < I > ,) -> EvaluationStepBuilder < D > { if self . is_noop () { EvaluationStepBuilder { state : None , _infcx : PhantomData } } else { EvaluationStepBuilder { state : Some (Box :: new (WipEvaluationStep { var_values : var_values . var_values . to_vec () , evaluation : WipProbe { initial_num_var_values : var_values . len () , steps : vec ! [] , kind : None , final_state : None , } , probe_depth : 0 , })) , _infcx : PhantomData , } } } pub (crate) fn finish_evaluation_step (& mut self , goal_evaluation_step : EvaluationStepBuilder < D > ,) { if let Some (this) = self . state . as_deref_mut () { * this = Some (goal_evaluation_step . state . unwrap () . finalize ()) ; } } pub (crate) fn unwrap (self) -> inspect :: Probe < I > { self . state . unwrap () . unwrap () } }
    };
}

impl_95!()