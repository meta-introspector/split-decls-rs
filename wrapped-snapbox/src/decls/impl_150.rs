macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < 's > From < & 's str > for Data { fn from (other : & 's str) -> Self { other . into_data () } }
    };
}

impl_150!()