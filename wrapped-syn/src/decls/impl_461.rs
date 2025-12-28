macro_rules! deps {
    () => {
        End!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl Copy for End { }
    };
}

impl_461!()