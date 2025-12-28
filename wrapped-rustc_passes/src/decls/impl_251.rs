macro_rules! deps {
    () => {
        NodeStats!();
        Node!();
    };
}

macro_rules! impl_251 {
    () => {
        deps!();
        impl Node { fn new () -> Node { Node { stats : NodeStats :: new () , subnodes : FxHashMap :: default () } } }
    };
}

impl_251!();