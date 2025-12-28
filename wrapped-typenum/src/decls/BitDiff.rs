macro_rules! BitDiff {
    () => {
        # [doc = " Gives `SizeOf(Lhs) - SizeOf(Rhs)`"] pub trait BitDiff < Rhs > { type Output ; }
    };
}

BitDiff!();