macro_rules! deps {
    () => {
        ModifierInfo!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl From < (char , & 'static str , u16) > for ModifierInfo { fn from ((modifier , result , size) : (char , & 'static str , u16)) -> Self { Self { modifier , result , size } } }
    };
}

impl_1!();