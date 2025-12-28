macro_rules! deps {
    () => {
        Node!();
        Successors!();
        DirectedGraph!();
    };
}

macro_rules! post_order_from {
    () => {
        deps!();
        pub fn post_order_from < G : DirectedGraph + Successors > (graph : & G , start_node : G :: Node ,) -> Vec < G :: Node > { post_order_from_to (graph , start_node , None) }
    };
}

post_order_from!()