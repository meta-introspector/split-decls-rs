macro_rules! deps {
    () => {
        FoldMul!();
        ATerm!();
    };
}

macro_rules! _ {
    () => {
        deps!();
        # [doc = " Hide our `Null` type"] const _ : () = { # [doc = " A type which contributes nothing when multiplying (i.e. a one)"] pub struct Null ; impl < T > Mul < T > for Null { type Output = T ; fn mul (self , rhs : T) -> Self :: Output { rhs } } impl FoldMul for ATerm { type Output = Null ; } } ;
    };
}

_!()