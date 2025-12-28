macro_rules! deps {
    () => {
        Bit!();
    };
}

macro_rules! IsLess {
    () => {
        deps!();
        # [doc = " A **type operator** that returns `True` if `Self < Rhs`, otherwise returns `False`."] pub trait IsLess < Rhs = Self > { # [doc = " The type representing either `True` or `False`"] type Output : Bit ; # [doc = " Method returning `True` or `False`."] # [allow (clippy :: wrong_self_convention)] fn is_less (self , rhs : Rhs) -> Self :: Output ; }
    };
}

IsLess!()