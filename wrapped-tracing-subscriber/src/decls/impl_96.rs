macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < S > Context < '_ , S > { pub (crate) fn none () -> Self { Self { subscriber : None , # [cfg (feature = "registry")] filter : FilterId :: none () , } } }
    };
}

impl_96!();