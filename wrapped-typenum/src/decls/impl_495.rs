macro_rules! deps {
    () => {
        UTerm!();
        PrivateSquareRoot!();
    };
}

macro_rules! impl_495 {
    () => {
        deps!();
        impl PrivateSquareRoot for UTerm { type Output = UTerm ; }
    };
}

impl_495!()