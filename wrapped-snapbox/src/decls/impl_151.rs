macro_rules! deps {
    () => {
        Inline!();
        Data!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl From < Inline > for Data { fn from (other : Inline) -> Self { other . into_data () } }
    };
}

impl_151!();