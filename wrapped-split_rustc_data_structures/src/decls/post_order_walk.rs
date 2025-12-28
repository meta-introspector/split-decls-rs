macro_rules! deps {
    () => {
        Successors!();
        DirectedGraph!();
        Node!();
    };
}

macro_rules! post_order_walk {
    () => {
        deps!();
        fn post_order_walk < G : DirectedGraph + Successors > (graph : & G , node : G :: Node , result : & mut Vec < G :: Node > , visited : & mut IndexSlice < G :: Node , bool > ,) { struct PostOrderFrame < Node , Iter > { node : Node , iter : Iter , } if visited [node] { return ; } let mut stack = vec ! [PostOrderFrame { node , iter : graph . successors (node) }] ; 'recurse : while let Some (frame) = stack . last_mut () { let node = frame . node ; visited [node] = true ; for successor in frame . iter . by_ref () { if ! visited [successor] { stack . push (PostOrderFrame { node : successor , iter : graph . successors (successor) }) ; continue 'recurse ; } } let _ = stack . pop () ; result . push (node) ; } }
    };
}

post_order_walk!()