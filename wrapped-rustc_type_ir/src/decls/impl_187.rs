macro_rules! deps {
    () => {
        CandidateSource!();
        Interner!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < I : Interner > Eq for CandidateSource < I > { }
    };
}

impl_187!()