macro_rules! deps {
    () => {
        DirectedGraph!();
        Node!();
    };
}

macro_rules! StartNode {
    () => {
        deps!();
        pub trait StartNode : DirectedGraph { fn start_node (& self) -> Self :: Node ; }
    };
}

StartNode!();