macro_rules! deps {
    () => {
        HeadUsages!();
    };
}

macro_rules! CandidateHeadUsages {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct CandidateHeadUsages { usages : Option < Box < HashMap < StackDepth , HeadUsages > > > , }
    };
}

CandidateHeadUsages!()