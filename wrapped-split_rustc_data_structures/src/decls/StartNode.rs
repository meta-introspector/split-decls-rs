macro_rules! deps {
    () => {
        Node!();
        DirectedGraph!();
    };
}

macro_rules! StartNode {
    () => {
        deps!();
        pub trait StartNode : DirectedGraph { fn start_node (& self) -> Self :: Node ; }
    };
}

StartNode!()