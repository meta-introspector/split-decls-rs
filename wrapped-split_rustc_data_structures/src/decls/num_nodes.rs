macro_rules! num_nodes {
    () => {
        # [test] fn num_nodes () { let graph = create_graph () ; assert_eq ! (graph . num_nodes () , 7) ; let graph = create_graph_with_back_refs () ; assert_eq ! (graph . num_nodes () , 7) ; }
    };
}

num_nodes!();