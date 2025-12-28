macro_rules! PartialDiv {
    () => {
        # [doc = " Division as a partial function. This **type operator** performs division just as `Div`, but is"] # [doc = " only defined when the result is an integer (i.e. there is no remainder)."] pub trait PartialDiv < Rhs = Self > { # [doc = " The type of the result of the division"] type Output ; # [doc = " Method for performing the division"] fn partial_div (self , _ : Rhs) -> Self :: Output ; }
    };
}

PartialDiv!()