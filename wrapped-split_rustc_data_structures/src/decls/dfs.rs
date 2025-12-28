macro_rules! dfs {
    () => {
        # [test] fn dfs () { let graph = create_graph () ; let dfs : Vec < _ > = graph :: depth_first_search (& graph , 0) . collect () ; assert_eq ! (dfs , vec ! [0 , 1 , 3 , 4 , 2]) ; let graph = create_graph_with_back_refs () ; let dfs : Vec < _ > = graph :: depth_first_search (& graph , 0) . collect () ; assert_eq ! (dfs , vec ! [0 , 1 , 3 , 4 , 2]) ; }
    };
}

dfs!()