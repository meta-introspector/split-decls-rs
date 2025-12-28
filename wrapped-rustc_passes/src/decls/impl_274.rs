macro_rules! deps {
    () => {
        UnwrapLayoutCx!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl < 'tcx > HasDataLayout for UnwrapLayoutCx < 'tcx > { fn data_layout (& self) -> & TargetDataLayout { self . tcx . data_layout () } }
    };
}

impl_274!()