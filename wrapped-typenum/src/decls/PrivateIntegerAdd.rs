macro_rules! PrivateIntegerAdd {
    () => {
        # [doc = " Used for addition of signed integers; `C = P.cmp(N)`"] # [doc = " Assumes `P = Self` is positive and `N` is negative"] # [doc = " where `P` and `N` are both passed as unsigned integers"] pub trait PrivateIntegerAdd < C , N > { type Output ; fn private_integer_add (self , _ : C , _ : N) -> Self :: Output ; }
    };
}

PrivateIntegerAdd!()