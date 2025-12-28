macro_rules! deps {
    () => {
        Node!();
        Predecessors!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < 'graph , G : Predecessors > Predecessors for & 'graph G { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { (* * self) . predecessors (node) } }
    };
}

impl_144!();