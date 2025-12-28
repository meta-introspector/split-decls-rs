macro_rules! deps {
    () => {
        WipProbeStep!();
        WipProbe!();
        WipEvaluationStep!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < I : Interner > WipEvaluationStep < I > { fn current_evaluation_scope (& mut self) -> & mut WipProbe < I > { let mut current = & mut self . evaluation ; for _ in 0 .. self . probe_depth { match current . steps . last_mut () { Some (WipProbeStep :: NestedProbe (p)) => current = p , _ => panic ! () , } } current } fn finalize (self) -> inspect :: Probe < I > { let evaluation = self . evaluation . finalize () ; match evaluation . kind { inspect :: ProbeKind :: Root { .. } => evaluation , _ => unreachable ! ("unexpected root evaluation: {evaluation:?}") , } } }
    };
}

impl_98!();