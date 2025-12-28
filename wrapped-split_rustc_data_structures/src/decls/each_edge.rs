macro_rules! each_edge {
    () => {
        # [test] fn each_edge () { let graph = create_graph () ; let expected = ["AB" , "BC" , "BD" , "DE" , "EC" , "FB"] ; graph . each_edge (| idx , edge | { assert_eq ! (expected [idx . 0] , edge . data) ; true }) ; }
    };
}

each_edge!();