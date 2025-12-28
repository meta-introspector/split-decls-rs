macro_rules! deps {
    () => {
        MoveCheckVisitor!();
    };
}

macro_rules! check_moves {
    () => {
        deps!();
        pub (crate) fn check_moves < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , body : & 'tcx mir :: Body < 'tcx > ,) { let mut visitor = MoveCheckVisitor { tcx , instance , body , move_size_spans : vec ! [] } ; for (bb , data) in traversal :: mono_reachable (body , tcx , instance) { visitor . visit_basic_block_data (bb , data) } }
    };
}

check_moves!();