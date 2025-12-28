macro_rules! deps {
    () => {
        AddressSpace!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl AddressSpace { # [doc = " The default address space, corresponding to data space."] pub const DATA : Self = AddressSpace (0) ; }
    };
}

impl_38!();