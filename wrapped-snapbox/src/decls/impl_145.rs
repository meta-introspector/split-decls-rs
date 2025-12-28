macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < 'd > From < & 'd Data > for Data { fn from (other : & 'd Data) -> Self { other . into_data () } }
    };
}

impl_145!()