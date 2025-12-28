macro_rules! deps {
    () => {
        Sccs!();
        TestGraph!();
        UsizeSccs!();
    };
}

macro_rules! test_big_scc {
    () => {
        deps!();
        # [test] fn test_big_scc () { let graph = TestGraph :: new (0 , & [(0 , 1) , (1 , 2) , (1 , 3) , (2 , 0) , (3 , 2)]) ; let sccs : UsizeSccs = Sccs :: new (& graph) ; assert_eq ! (sccs . num_sccs () , 1) ; }
    };
}

test_big_scc!();