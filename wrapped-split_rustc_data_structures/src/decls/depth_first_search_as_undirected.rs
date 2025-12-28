macro_rules! deps {
    () => {
        Node!();
        Successors!();
        DirectedGraph!();
        Predecessors!();
        DepthFirstSearch!();
    };
}

macro_rules! depth_first_search_as_undirected {
    () => {
        deps!();
        pub fn depth_first_search_as_undirected < G > (graph : G , from : G :: Node ,) -> iterate :: DepthFirstSearch < impl Successors < Node = G :: Node > > where G : Successors + Predecessors , { struct AsUndirected < G > (G) ; impl < G : DirectedGraph > DirectedGraph for AsUndirected < G > { type Node = G :: Node ; fn num_nodes (& self) -> usize { self . 0 . num_nodes () } } impl < G : Successors + Predecessors > Successors for AsUndirected < G > { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . 0 . successors (node) . chain (self . 0 . predecessors (node)) } } iterate :: DepthFirstSearch :: new (AsUndirected (graph)) . with_start_node (from) }
    };
}

depth_first_search_as_undirected!()