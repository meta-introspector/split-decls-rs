macro_rules! deps {
    () => {
        ExpectedAncestry!();
    };
}

macro_rules! is_explicit_root {
    () => {
        deps!();
        # [doc = " Convenience function that returns [`ExpectedAncestry::IsExplicitRoot`]."] pub fn is_explicit_root () -> ExpectedAncestry { ExpectedAncestry :: IsExplicitRoot }
    };
}

is_explicit_root!()