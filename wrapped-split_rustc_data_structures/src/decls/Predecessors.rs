macro_rules! deps {
    () => {
        DirectedGraph!();
        Node!();
    };
}

macro_rules! Predecessors {
    () => {
        deps!();
        pub trait Predecessors : DirectedGraph { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > ; }
    };
}

Predecessors!()