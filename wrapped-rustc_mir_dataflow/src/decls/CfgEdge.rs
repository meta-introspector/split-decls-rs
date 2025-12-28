macro_rules! CfgEdge {
    () => {
        # [doc = " A pair of a basic block and an index into that basic blocks `successors`."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] struct CfgEdge { source : BasicBlock , index : usize , }
    };
}

CfgEdge!();