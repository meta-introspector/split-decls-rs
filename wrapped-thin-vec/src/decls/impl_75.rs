macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        # [cfg (feature = "unstable")] unsafe impl < T > core :: iter :: TrustedLen for Drain < '_ , T > { }
    };
}

impl_75!()