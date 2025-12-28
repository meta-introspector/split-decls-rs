macro_rules! deps {
    () => {
        EvalCtxt!();
        SolverDelegate!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < D , I > EvalCtxt < '_ , D > where D : SolverDelegate < Interner = I > , I : Interner , { # [instrument (level = "trace" , skip (self))] pub (super) fn compute_host_effect_goal (& mut self , goal : Goal < I , ty :: HostEffectPredicate < I > > ,) -> QueryResult < I > { let (_ , proven_via) = self . probe (| _ | ProbeKind :: ShadowedEnvProbing) . enter (| ecx | { let trait_goal : Goal < I , ty :: TraitPredicate < I > > = goal . with (ecx . cx () , goal . predicate . trait_ref) ; ecx . compute_trait_goal (trait_goal) }) ? ; self . assemble_and_merge_candidates (proven_via , goal , | _ecx | Err (NoSolution)) } }
    };
}

impl_66!();