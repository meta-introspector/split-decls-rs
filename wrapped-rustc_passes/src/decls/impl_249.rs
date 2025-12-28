macro_rules! deps {
    () => {
        NodeStats!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl NodeStats { fn new () -> NodeStats { NodeStats { count : 0 , size : 0 } } fn accum_size (& self) -> usize { self . count * self . size } }
    };
}

impl_249!()