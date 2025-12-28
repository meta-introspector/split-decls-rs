macro_rules! ShiftDiff {
    () => {
        # [doc = " Performs `Shl` on `Lhs` so that `SizeOf(Lhs) = SizeOf(Rhs)`"] # [doc = " Fails if `SizeOf(Lhs) > SizeOf(Rhs)`"] pub trait ShiftDiff < Rhs > { type Output ; }
    };
}

ShiftDiff!()