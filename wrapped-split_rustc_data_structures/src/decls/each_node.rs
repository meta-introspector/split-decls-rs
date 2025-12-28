macro_rules! each_node {
    () => {
        # [test] fn each_node () { let graph = create_graph () ; let expected = ["A" , "B" , "C" , "D" , "E" , "F"] ; graph . each_node (| idx , node | { assert_eq ! (& expected [idx . 0] , graph . node_data (idx)) ; assert_eq ! (expected [idx . 0] , node . data) ; true }) ; }
    };
}

each_node!();