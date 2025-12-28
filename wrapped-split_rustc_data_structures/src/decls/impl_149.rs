macro_rules! deps {
    () => {
        Predecessors!();
        Successors!();
        Node!();
        ReversedGraph!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < G : Predecessors > Successors for ReversedGraph < G > { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . inner . predecessors (node) } }
    };
}

impl_149!()