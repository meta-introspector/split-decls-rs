macro_rules! deps {
    () => {
        UInt!();
        Unsigned!();
        UTerm!();
        Bit!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < Ur : Unsigned , Br : Bit > Rem < UInt < Ur , Br > > for UTerm { type Output = UTerm ; # [inline] fn rem (self , _ : UInt < Ur , Br >) -> Self :: Output { UTerm } }
    };
}

impl_472!();