macro_rules! deps {
    () => {
        Node!();
        Predecessors!();
        VecGraph!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl < N : Idx + Ord > Predecessors for VecGraph < N , true > { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . predecessors (node) . iter () . cloned () } }
    };
}

impl_214!();