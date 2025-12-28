macro_rules! deps {
    () => {
        UInt!();
        Unsigned!();
        Bit!();
        BitDiffOut!();
        BitDiff!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        impl < Ul , Bl , Ur , Br > BitDiff < UInt < Ur , Br > > for UInt < Ul , Bl > where Ul : Unsigned , Bl : Bit , Ur : Unsigned , Br : Bit , Ul : BitDiff < Ur > , { type Output = BitDiffOut < Ul , Ur > ; }
    };
}

impl_437!()