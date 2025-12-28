macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl From < Vec < u8 > > for Data { fn from (other : Vec < u8 >) -> Self { other . into_data () } }
    };
}

impl_146!();