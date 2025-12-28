macro_rules! deps {
    () => {
        AsDisplay!();
        Sealed!();
    };
}

macro_rules! placeholder {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] mod placeholder { use super :: { AsDisplay , Sealed } ; use core :: fmt :: { self , Display } ; # [allow (dead_code)] pub struct Placeholder ; impl < 'a > AsDisplay < 'a > for Placeholder { type Target = Self ; # [inline] fn as_display (& 'a self) -> Self :: Target { Placeholder } } impl Display for Placeholder { fn fmt (& self , _formatter : & mut fmt :: Formatter) -> fmt :: Result { unreachable ! () } } impl Sealed for Placeholder { } }
    };
}

placeholder!()