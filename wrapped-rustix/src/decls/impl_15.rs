macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < T > Buffer < T > for & mut Vec < MaybeUninit < T > > { }
    };
}

impl_15!();