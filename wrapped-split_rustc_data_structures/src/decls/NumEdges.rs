macro_rules! deps {
    () => {
        DirectedGraph!();
    };
}

macro_rules! NumEdges {
    () => {
        deps!();
        pub trait NumEdges : DirectedGraph { fn num_edges (& self) -> usize ; }
    };
}

NumEdges!();