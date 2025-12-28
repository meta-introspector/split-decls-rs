macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < 's > From < & 's String > for Data { fn from (other : & 's String) -> Self { other . into_data () } }
    };
}

impl_149!();