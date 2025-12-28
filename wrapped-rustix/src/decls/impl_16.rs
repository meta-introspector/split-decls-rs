macro_rules! deps {
    () => {
        Buffer!();
        SpareCapacity!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'a , T > Buffer < T > for SpareCapacity < 'a , T > { }
    };
}

impl_16!()