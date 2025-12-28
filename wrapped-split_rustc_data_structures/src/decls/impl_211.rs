macro_rules! deps {
    () => {
        VecGraph!();
        Node!();
        DirectedGraph!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < N : Idx , const BR : bool > DirectedGraph for VecGraph < N , BR > { type Node = N ; fn num_nodes (& self) -> usize { match BR { false => self . node_starts . len () - 1 , true => (self . node_starts . len () - 1) / 2 , } } }
    };
}

impl_211!();