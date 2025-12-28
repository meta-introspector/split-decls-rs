macro_rules! deps {
    () => {
        NodeStats!();
    };
}

macro_rules! Node {
    () => {
        deps!();
        struct Node { stats : NodeStats , subnodes : FxHashMap < & 'static str , NodeStats > , }
    };
}

Node!();