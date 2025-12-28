macro_rules! deps {
    () => {
        WithSubscriber!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < T : Sized > WithSubscriber for T { }
    };
}

impl_18!()