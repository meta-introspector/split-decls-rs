macro_rules! deps {
    () => {
        ReturnCode!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl From < i32 > for ReturnCode { fn from (value : i32) -> Self { match Self :: try_from_c_int (value) { Some (value) => value , None => panic ! ("invalid return code {value}") , } } }
    };
}

impl_24!()