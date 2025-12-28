macro_rules! deps {
    () => {
        Direction!();
        EdgeIndex!();
        LinkedGraph!();
    };
}

macro_rules! AdjacentEdges {
    () => {
        deps!();
        pub struct AdjacentEdges < 'g , N , E > { graph : & 'g LinkedGraph < N , E > , direction : Direction , next : EdgeIndex , }
    };
}

AdjacentEdges!();