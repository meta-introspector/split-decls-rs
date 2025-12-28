macro_rules! deps {
    () => {
        Equal!();
        PrivateIntegerAdd!();
        Z0!();
        Unsigned!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        # [doc = " `P + N = 0` where `P == N`"] impl < N : Unsigned , P : Unsigned > PrivateIntegerAdd < Equal , N > for P { type Output = Z0 ; # [inline] fn private_integer_add (self , _ : Equal , _ : N) -> Self :: Output { Z0 } }
    };
}

impl_66!();