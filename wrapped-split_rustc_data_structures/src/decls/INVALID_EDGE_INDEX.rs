macro_rules! deps {
    () => {
        EdgeIndex!();
    };
}

macro_rules! INVALID_EDGE_INDEX {
    () => {
        deps!();
        pub const INVALID_EDGE_INDEX : EdgeIndex = EdgeIndex (usize :: MAX) ;
    };
}

INVALID_EDGE_INDEX!();