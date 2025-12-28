macro_rules! deps {
    () => {
        StartNode!();
        Node!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'graph , G : StartNode > StartNode for & 'graph G { fn start_node (& self) -> Self :: Node { (* * self) . start_node () } }
    };
}

impl_142!()