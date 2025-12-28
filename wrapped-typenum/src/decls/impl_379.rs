macro_rules! deps {
    () => {
        Unsigned!();
        UTerm!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        # [doc = " 0 & X = 0"] impl < Ur : Unsigned > BitAnd < Ur > for UTerm { type Output = UTerm ; # [inline] fn bitand (self , _ : Ur) -> Self :: Output { UTerm } }
    };
}

impl_379!()