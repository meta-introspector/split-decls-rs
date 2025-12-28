macro_rules! Candidate {
    () => {
        # [doc = " A candidate is a possible way to prove a goal."] # [doc = ""] # [doc = " It consists of both the `source`, which describes how that goal would be proven,"] # [doc = " and the `result` when using the given `source`."] # [derive_where (Debug ; I : Interner)] pub (super) struct Candidate < I : Interner > { pub (super) source : CandidateSource < I > , pub (super) result : CanonicalResponse < I > , pub (super) head_usages : CandidateHeadUsages , }
    };
}

Candidate!();