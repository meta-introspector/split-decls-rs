macro_rules! deps {
    () => {
        SolverDelegate!();
        ProbeCtxt!();
    };
}

macro_rules! TraitProbeCtxt {
    () => {
        deps!();
        pub (in crate :: solve) struct TraitProbeCtxt < 'me , 'a , D , I , F > where D : SolverDelegate < Interner = I > , I : Interner , { cx : ProbeCtxt < 'me , 'a , D , I , F , QueryResult < I > > , source : CandidateSource < I > , }
    };
}

TraitProbeCtxt!()