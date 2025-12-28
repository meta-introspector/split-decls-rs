macro_rules! deps {
    () => {
        DirectedGraph!();
        TestGraph!();
        Node!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl DirectedGraph for TestGraph { type Node = usize ; fn num_nodes (& self) -> usize { self . num_nodes } }
    };
}

impl_218!();