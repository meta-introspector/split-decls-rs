macro_rules! deps {
    () => {
        Index!();
        Item!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < T : ? Sized > Index for & T where T : Index , { fn index < 'v > (& self , v : & 'v Item) -> Option < & 'v Item > { (* * self) . index (v) } fn index_mut < 'v > (& self , v : & 'v mut Item) -> Option < & 'v mut Item > { (* * self) . index_mut (v) } }
    };
}

impl_79!()