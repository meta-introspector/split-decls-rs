macro_rules! deps {
    () => {
        NodeIndex!();
    };
}

macro_rules! each_adjacent_from_c {
    () => {
        deps!();
        # [test] fn each_adjacent_from_c () { let graph = create_graph () ; test_adjacent_edges (& graph , NodeIndex (2) , "C" , & [("EC" , "E") , ("BC" , "B")] , & []) ; }
    };
}

each_adjacent_from_c!()