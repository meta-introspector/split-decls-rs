macro_rules! deps {
    () => {
        DebugWithContext!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < C > DebugWithContext < C > for rustc_middle :: mir :: Local { }
    };
}

impl_52!();