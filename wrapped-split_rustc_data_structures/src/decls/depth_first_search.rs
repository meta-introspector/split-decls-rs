macro_rules! deps {
    () => {
        DepthFirstSearch!();
        Node!();
        Successors!();
    };
}

macro_rules! depth_first_search {
    () => {
        deps!();
        pub fn depth_first_search < G > (graph : G , from : G :: Node) -> iterate :: DepthFirstSearch < G > where G : Successors , { iterate :: DepthFirstSearch :: new (graph) . with_start_node (from) }
    };
}

depth_first_search!()