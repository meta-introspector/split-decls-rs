macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < 'b > From < & 'b [u8] > for Data { fn from (other : & 'b [u8]) -> Self { other . into_data () } }
    };
}

impl_147!();