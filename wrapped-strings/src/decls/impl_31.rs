macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < & std :: path :: Path > for HSTRING { fn from (value : & std :: path :: Path) -> Self { value . as_os_str () . into () } }
    };
}

impl_31!()