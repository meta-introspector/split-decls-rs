macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        # [cfg (feature = "unstable")] unsafe impl < T > core :: iter :: TrustedLen for IntoIter < T > { }
    };
}

impl_66!()