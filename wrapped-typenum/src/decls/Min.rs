macro_rules! Min {
    () => {
        # [doc = " A **type operator** that returns the minimum of `Self` and `Rhs`."] pub trait Min < Rhs = Self > { # [doc = " The type of the minimum of `Self` and `Rhs`"] type Output ; # [doc = " Method returning the minimum"] fn min (self , rhs : Rhs) -> Self :: Output ; }
    };
}

Min!()