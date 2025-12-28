macro_rules! deps {
    () => {
        Len!();
        UTerm!();
        Length!();
        BitDiff!();
        Unsigned!();
    };
}

macro_rules! impl_438 {
    () => {
        deps!();
        impl < Ul > BitDiff < UTerm > for Ul where Ul : Unsigned + Len , { type Output = Length < Ul > ; }
    };
}

impl_438!()