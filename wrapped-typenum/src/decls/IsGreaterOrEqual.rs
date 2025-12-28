macro_rules! deps {
    () => {
        Bit!();
    };
}

macro_rules! IsGreaterOrEqual {
    () => {
        deps!();
        # [doc = " A **type operator** that returns `True` if `Self >= Rhs`, otherwise returns `False`."] pub trait IsGreaterOrEqual < Rhs = Self > { # [doc = " The type representing either `True` or `False`"] type Output : Bit ; # [doc = " Method returning `True` or `False`."] # [allow (clippy :: wrong_self_convention)] fn is_greater_or_equal (self , rhs : Rhs) -> Self :: Output ; }
    };
}

IsGreaterOrEqual!()