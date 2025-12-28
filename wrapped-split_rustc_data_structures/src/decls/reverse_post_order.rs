macro_rules! deps {
    () => {
        Node!();
        DirectedGraph!();
        Successors!();
    };
}

macro_rules! reverse_post_order {
    () => {
        deps!();
        pub fn reverse_post_order < G : DirectedGraph + Successors > (graph : & G , start_node : G :: Node ,) -> Vec < G :: Node > { let mut vec = post_order_from (graph , start_node) ; vec . reverse () ; vec }
    };
}

reverse_post_order!()