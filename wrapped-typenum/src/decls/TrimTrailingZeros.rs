macro_rules! TrimTrailingZeros {
    () => {
        # [doc = " Gets rid of all zeros until it hits a one."] pub trait TrimTrailingZeros { type Output ; fn trim_trailing_zeros (self) -> Self :: Output ; }
    };
}

TrimTrailingZeros!();