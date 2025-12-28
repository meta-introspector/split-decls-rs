macro_rules! deps {
    () => {
        Node!();
        Successors!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < 'graph , G : Successors > Successors for & 'graph G { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { (* * self) . successors (node) } }
    };
}

impl_143!()