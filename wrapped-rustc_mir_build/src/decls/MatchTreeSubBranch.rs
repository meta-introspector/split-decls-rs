macro_rules! deps {
    () => {
        Ascription!();
        Binding!();
    };
}

macro_rules! MatchTreeSubBranch {
    () => {
        deps!();
        # [doc = " A sub-branch in the output of match lowering. Match lowering has generated MIR code that will"] # [doc = " branch to `success_block` when the matched value matches the corresponding pattern. If there is"] # [doc = " a guard, its failure must continue to `otherwise_block`, which will resume testing patterns."] # [derive (Debug , Clone)] struct MatchTreeSubBranch < 'tcx > { span : Span , # [doc = " The block that is branched to if the corresponding subpattern matches."] success_block : BasicBlock , # [doc = " The block to branch to if this arm had a guard and the guard fails."] otherwise_block : BasicBlock , # [doc = " The bindings to set up in this sub-branch."] bindings : Vec < Binding < 'tcx > > , # [doc = " The ascriptions to set up in this sub-branch."] ascriptions : Vec < Ascription < 'tcx > > , # [doc = " Whether the sub-branch corresponds to a never pattern."] is_never : bool , }
    };
}

MatchTreeSubBranch!()