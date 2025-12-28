macro_rules! deps {
    () => {
        UTerm!();
        UInt!();
        PartialDiv!();
        Unsigned!();
        Bit!();
    };
}

macro_rules! impl_482 {
    () => {
        deps!();
        impl < Ur : Unsigned , Br : Bit > PartialDiv < UInt < Ur , Br > > for UTerm { type Output = UTerm ; # [inline] fn partial_div (self , _ : UInt < Ur , Br >) -> Self :: Output { UTerm } }
    };
}

impl_482!();