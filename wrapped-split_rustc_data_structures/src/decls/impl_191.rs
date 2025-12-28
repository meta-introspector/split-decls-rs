macro_rules! deps {
    () => {
        Node!();
        DirectedGraph!();
        Sccs!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < N : Idx , S : Idx + Ord > DirectedGraph for Sccs < N , S > { type Node = S ; fn num_nodes (& self) -> usize { self . num_sccs () } }
    };
}

impl_191!()