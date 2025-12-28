macro_rules! deps {
    () => {
        OutputStyle!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl OutputStyle { fn num_state_columns (& self) -> usize { match self { Self :: AfterOnly => 1 , Self :: BeforeAndAfter => 2 , } } }
    };
}

impl_60!();