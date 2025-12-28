macro_rules! deps {
    () => {
        Trim!();
        Xor!();
    };
}

macro_rules! PrivateXor {
    () => {
        deps!();
        # [doc = " Does the real xoring for `UInt`s; `Xor` just calls this and then `Trim`."] pub trait PrivateXor < Rhs = Self > { type Output ; fn private_xor (self , rhs : Rhs) -> Self :: Output ; }
    };
}

PrivateXor!()