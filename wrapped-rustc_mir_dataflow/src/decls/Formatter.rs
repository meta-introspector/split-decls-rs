macro_rules! deps {
    () => {
        Results!();
        Analysis!();
        OutputStyle!();
    };
}

macro_rules! Formatter {
    () => {
        deps!();
        struct Formatter < 'mir , 'tcx , A > where A : Analysis < 'tcx > , { body : & 'mir Body < 'tcx > , analysis : RefCell < & 'mir mut A > , results : & 'mir Results < A :: Domain > , style : OutputStyle , reachable : DenseBitSet < BasicBlock > , }
    };
}

Formatter!()