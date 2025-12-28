macro_rules! deps {
    () => {
        Predecessors!();
        Successors!();
        ReversedGraph!();
        Node!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < G : Successors > Predecessors for ReversedGraph < G > { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . inner . successors (node) } }
    };
}

impl_150!()