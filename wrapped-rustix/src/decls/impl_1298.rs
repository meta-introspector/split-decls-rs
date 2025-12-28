macro_rules! deps {
    () => {
        CompatCapability!();
    };
}

macro_rules! impl_1298 {
    () => {
        deps!();
        impl CompatCapability for CapabilitySet { fn as_capability_set (self , _ : private :: Token) -> CapabilitySet { self } }
    };
}

impl_1298!()