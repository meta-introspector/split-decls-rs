macro_rules! deps {
    () => {
        Node!();
        DirectedGraph!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl < 'graph , G : DirectedGraph > DirectedGraph for & 'graph G { type Node = G :: Node ; fn num_nodes (& self) -> usize { (* * self) . num_nodes () } }
    };
}

impl_141!()