macro_rules! deps {
    () => {
        DirectedGraph!();
        Node!();
    };
}

macro_rules! Successors {
    () => {
        deps!();
        pub trait Successors : DirectedGraph { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > ; }
    };
}

Successors!()