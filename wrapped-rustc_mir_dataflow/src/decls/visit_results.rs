macro_rules! deps {
    () => {
        ResultsVisitor!();
        Results!();
        Direction!();
        Analysis!();
    };
}

macro_rules! visit_results {
    () => {
        deps!();
        # [doc = " Calls the corresponding method in `ResultsVisitor` for every location in a `mir::Body` with the"] # [doc = " dataflow state at that location."] pub fn visit_results < 'mir , 'tcx , A > (body : & 'mir mir :: Body < 'tcx > , blocks : impl IntoIterator < Item = BasicBlock > , analysis : & mut A , results : & Results < A :: Domain > , vis : & mut impl ResultsVisitor < 'tcx , A > ,) where A : Analysis < 'tcx > , { let mut state = analysis . bottom_value (body) ; # [cfg (debug_assertions)] let reachable_blocks = mir :: traversal :: reachable_as_bitset (body) ; for block in blocks { # [cfg (debug_assertions)] assert ! (reachable_blocks . contains (block)) ; let block_data = & body [block] ; state . clone_from (& results [block]) ; A :: Direction :: visit_results_in_block (& mut state , block , block_data , analysis , vis) ; } }
    };
}

visit_results!()