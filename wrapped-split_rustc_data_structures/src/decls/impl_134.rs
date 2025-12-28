macro_rules! deps {
    () => {
        EdgeIndex!();
        AdjacentEdges!();
        Edge!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < 'g , N : Debug , E : Debug > Iterator for AdjacentEdges < 'g , N , E > { type Item = (EdgeIndex , & 'g Edge < E >) ; fn next (& mut self) -> Option < (EdgeIndex , & 'g Edge < E >) > { let edge_index = self . next ; if edge_index == INVALID_EDGE_INDEX { return None ; } let edge = self . graph . edge (edge_index) ; self . next = edge . next_edge [self . direction . repr] ; Some ((edge_index , edge)) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . graph . len_edges ())) } }
    };
}

impl_134!()