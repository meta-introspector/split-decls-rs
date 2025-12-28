macro_rules! deps {
    () => {
        Trim!();
        And!();
    };
}

macro_rules! PrivateAnd {
    () => {
        deps!();
        # [doc = " Does the real anding for `UInt`s; `And` just calls this and then `Trim`."] pub trait PrivateAnd < Rhs = Self > { type Output ; fn private_and (self , rhs : Rhs) -> Self :: Output ; }
    };
}

PrivateAnd!();