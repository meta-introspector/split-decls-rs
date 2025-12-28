macro_rules! deps {
    () => {
        End!();
        Peek!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl Peek for End { type Token = Self ; }
    };
}

impl_463!();