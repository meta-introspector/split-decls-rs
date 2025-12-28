macro_rules! deps {
    () => {
        MatchTreeSubBranch!();
    };
}

macro_rules! MatchTreeBranch {
    () => {
        deps!();
        # [doc = " A branch in the output of match lowering."] # [derive (Debug , Clone)] struct MatchTreeBranch < 'tcx > { sub_branches : Vec < MatchTreeSubBranch < 'tcx > > , }
    };
}

MatchTreeBranch!()