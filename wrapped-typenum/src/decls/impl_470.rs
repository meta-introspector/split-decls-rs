macro_rules! deps {
    () => {
        Unsigned!();
        UTerm!();
        UInt!();
        Bit!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl < Ur : Unsigned , Br : Bit > Div < UInt < Ur , Br > > for UTerm { type Output = UTerm ; # [inline] fn div (self , _ : UInt < Ur , Br >) -> Self :: Output { UTerm } }
    };
}

impl_470!()