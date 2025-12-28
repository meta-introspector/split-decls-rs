macro_rules! deps {
    () => {
        NodeIndex!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl NodeIndex { # [doc = " Returns unique ID (unique with respect to the graph holding associated node)."] pub fn node_id (self) -> usize { self . 0 } }
    };
}

impl_130!()