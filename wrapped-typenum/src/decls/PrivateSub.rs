macro_rules! deps {
    () => {
        Trim!();
    };
}

macro_rules! PrivateSub {
    () => {
        deps!();
        # [doc = " Does the real subtraction for `UInt`s; `Sub` just calls this and then `Trim`."] pub trait PrivateSub < Rhs = Self > { type Output ; fn private_sub (self , rhs : Rhs) -> Self :: Output ; }
    };
}

PrivateSub!();