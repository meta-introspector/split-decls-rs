macro_rules! deps {
    () => {
        DepthFirstTraversal!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < 'g , N : Debug , E : Debug > ExactSizeIterator for DepthFirstTraversal < 'g , N , E > { }
    };
}

impl_138!();