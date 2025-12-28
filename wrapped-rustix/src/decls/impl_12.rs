macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < T > Buffer < T > for & mut Vec < T > { }
    };
}

impl_12!();