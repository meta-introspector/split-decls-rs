macro_rules! CFG {
    () => {
        struct CFG < 'tcx > { basic_blocks : IndexVec < BasicBlock , BasicBlockData < 'tcx > > , }
    };
}

CFG!();