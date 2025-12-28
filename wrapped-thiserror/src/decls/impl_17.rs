macro_rules! deps {
    () => {
        AsDisplay!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'a , T > AsDisplay < 'a > for & T where T : Display + ? Sized + 'a , { type Target = & 'a T ; fn as_display (& 'a self) -> Self :: Target { * self } }
    };
}

impl_17!()