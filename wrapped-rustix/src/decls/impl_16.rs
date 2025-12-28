macro_rules! deps {
    () => {
        SpareCapacity!();
        Buffer!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'a , T > Buffer < T > for SpareCapacity < 'a , T > { }
    };
}

impl_16!();