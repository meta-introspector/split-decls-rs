macro_rules! deps {
    () => {
        EvalCtxt!();
        ProbeCtxt!();
        TraitProbeCtxt!();
        SolverDelegate!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < 'a , D , I > EvalCtxt < 'a , D , I > where D : SolverDelegate < Interner = I > , I : Interner , { # [doc = " `probe_kind` is only called when proof tree building is enabled so it can be"] # [doc = " as expensive as necessary to output the desired information."] pub (in crate :: solve) fn probe < F , T > (& mut self , probe_kind : F) -> ProbeCtxt < '_ , 'a , D , I , F , T > where F : FnOnce (& T) -> inspect :: ProbeKind < I > , { ProbeCtxt { ecx : self , probe_kind , _result : PhantomData } } pub (in crate :: solve) fn probe_builtin_trait_candidate (& mut self , source : BuiltinImplSource ,) -> TraitProbeCtxt < '_ , 'a , D , I , impl FnOnce (& QueryResult < I >) -> inspect :: ProbeKind < I > > { self . probe_trait_candidate (CandidateSource :: BuiltinImpl (source)) } pub (in crate :: solve) fn probe_trait_candidate (& mut self , source : CandidateSource < I > ,) -> TraitProbeCtxt < '_ , 'a , D , I , impl FnOnce (& QueryResult < I >) -> inspect :: ProbeKind < I > > { TraitProbeCtxt { cx : ProbeCtxt { ecx : self , probe_kind : move | result : & QueryResult < I > | inspect :: ProbeKind :: TraitCandidate { source , result : * result , } , _result : PhantomData , } , source , } } }
    };
}

impl_79!();