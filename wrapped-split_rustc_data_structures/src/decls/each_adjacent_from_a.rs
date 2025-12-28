macro_rules! deps {
    () => {
        NodeIndex!();
    };
}

macro_rules! each_adjacent_from_a {
    () => {
        deps!();
        # [test] fn each_adjacent_from_a () { let graph = create_graph () ; test_adjacent_edges (& graph , NodeIndex (0) , "A" , & [] , & [("AB" , "B")]) ; }
    };
}

each_adjacent_from_a!();