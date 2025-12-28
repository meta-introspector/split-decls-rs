macro_rules! deps {
    () => {
        TestGraph!();
        StartNode!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl StartNode for TestGraph { fn start_node (& self) -> usize { self . start_node } }
    };
}

impl_219!();