macro_rules! deps {
    () => {
        QueryCtxt!();
    };
}

macro_rules! try_mark_green {
    () => {
        deps!();
        pub (super) fn try_mark_green < 'tcx > (tcx : TyCtxt < 'tcx > , dep_node : & dep_graph :: DepNode) -> bool { tcx . dep_graph . try_mark_green (QueryCtxt :: new (tcx) , dep_node) . is_some () }
    };
}

try_mark_green!()