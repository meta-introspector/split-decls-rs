macro_rules! deps {
    () => {
        Unsigned!();
        ShiftDiff!();
        BitDiff!();
        Shleft!();
        BitDiffOut!();
    };
}

macro_rules! impl_439 {
    () => {
        deps!();
        impl < Ul : Unsigned , Ur : Unsigned > ShiftDiff < Ur > for Ul where Ur : BitDiff < Ul > , Ul : Shl < BitDiffOut < Ur , Ul > > , { type Output = Shleft < Ul , BitDiffOut < Ur , Ul > > ; }
    };
}

impl_439!()