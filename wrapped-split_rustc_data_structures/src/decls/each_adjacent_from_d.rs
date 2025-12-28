macro_rules! deps {
    () => {
        NodeIndex!();
    };
}

macro_rules! each_adjacent_from_d {
    () => {
        deps!();
        # [test] fn each_adjacent_from_d () { let graph = create_graph () ; test_adjacent_edges (& graph , NodeIndex (3) , "D" , & [("BD" , "B")] , & [("DE" , "E")]) ; }
    };
}

each_adjacent_from_d!()