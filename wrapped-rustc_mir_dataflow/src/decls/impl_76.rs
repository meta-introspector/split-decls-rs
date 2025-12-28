macro_rules! deps {
    () => {
        Background!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl ops :: Not for Background { type Output = Self ; fn not (self) -> Self { match self { Self :: Light => Self :: Dark , Self :: Dark => Self :: Light , } } }
    };
}

impl_76!();