macro_rules! Max {
    () => {
        # [doc = " A **type operator** that returns the maximum of `Self` and `Rhs`."] pub trait Max < Rhs = Self > { # [doc = " The type of the maximum of `Self` and `Rhs`"] type Output ; # [doc = " Method returning the maximum"] fn max (self , rhs : Rhs) -> Self :: Output ; }
    };
}

Max!();