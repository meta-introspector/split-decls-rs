macro_rules! deps {
    () => {
        Value!();
        Index!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < T > Index for & T where T : Index + ? Sized , { fn index < 'a > (& self , val : & 'a Value) -> Option < & 'a Value > { (* * self) . index (val) } fn index_mut < 'a > (& self , val : & 'a mut Value) -> Option < & 'a mut Value > { (* * self) . index_mut (val) } }
    };
}

impl_82!()