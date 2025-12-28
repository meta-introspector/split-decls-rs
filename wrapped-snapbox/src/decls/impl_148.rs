macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl From < String > for Data { fn from (other : String) -> Self { other . into_data () } }
    };
}

impl_148!()