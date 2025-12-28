macro_rules! deps {
    () => {
        ResultsVisitor!();
        ResultsCursor!();
    };
}

macro_rules! Results {
    () => {
        deps!();
        # [doc = " The results of a dataflow analysis that has converged to fixpoint. It only holds the domain"] # [doc = " values at the entry of each basic block. Domain values in other parts of the block are"] # [doc = " recomputed on the fly by visitors (i.e. `ResultsCursor`, or `ResultsVisitor` impls)."] pub type Results < D > = IndexVec < BasicBlock , D > ;
    };
}

Results!()