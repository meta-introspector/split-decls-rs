macro_rules! deps {
    () => {
        Value!();
        Index!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl Index for String { fn index < 'a > (& self , val : & 'a Value) -> Option < & 'a Value > { self [..] . index (val) } fn index_mut < 'a > (& self , val : & 'a mut Value) -> Option < & 'a mut Value > { self [..] . index_mut (val) } }
    };
}

impl_81!()