macro_rules! deps {
    () => {
        EdgeIndex!();
        Direction!();
        LinkedGraph!();
    };
}

macro_rules! AdjacentEdges {
    () => {
        deps!();
        pub struct AdjacentEdges < 'g , N , E > { graph : & 'g LinkedGraph < N , E > , direction : Direction , next : EdgeIndex , }
    };
}

AdjacentEdges!()