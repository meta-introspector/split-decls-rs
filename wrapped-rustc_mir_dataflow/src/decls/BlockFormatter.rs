macro_rules! deps {
    () => {
        ResultsCursor!();
        Analysis!();
        Background!();
        OutputStyle!();
    };
}

macro_rules! BlockFormatter {
    () => {
        deps!();
        struct BlockFormatter < 'mir , 'tcx , A > where A : Analysis < 'tcx > , { cursor : ResultsCursor < 'mir , 'tcx , A > , bg : Background , style : OutputStyle , }
    };
}

BlockFormatter!()