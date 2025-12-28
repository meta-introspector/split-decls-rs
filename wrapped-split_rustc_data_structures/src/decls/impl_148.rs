macro_rules! deps {
    () => {
        ReversedGraph!();
        DirectedGraph!();
        Node!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < G : DirectedGraph > DirectedGraph for ReversedGraph < G > { type Node = G :: Node ; fn num_nodes (& self) -> usize { self . inner . num_nodes () } }
    };
}

impl_148!()