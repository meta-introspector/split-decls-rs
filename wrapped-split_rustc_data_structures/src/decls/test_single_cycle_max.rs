macro_rules! deps {
    () => {
        Maxes!();
        Sccs!();
        TestGraph!();
    };
}

macro_rules! test_single_cycle_max {
    () => {
        deps!();
        # [test] fn test_single_cycle_max () { let graph = TestGraph :: new (0 , & [(0 , 2) , (2 , 3) , (2 , 4) , (4 , 1) , (1 , 2)]) ; let mut annotations = Maxes (IndexVec :: new () , | n | n) ; let sccs = Sccs :: new_with_annotation (& graph , & mut annotations) ; assert_eq ! (annotations . 0 [sccs . scc (2)] , 4) ; assert_eq ! (annotations . 0 [sccs . scc (0)] , 4) ; }
    };
}

test_single_cycle_max!();