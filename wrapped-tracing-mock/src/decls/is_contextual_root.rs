macro_rules! deps {
    () => {
        ExpectedAncestry!();
    };
}

macro_rules! is_contextual_root {
    () => {
        deps!();
        # [doc = " Convenience function that returns [`ExpectedAncestry::IsContextualRoot`]."] pub fn is_contextual_root () -> ExpectedAncestry { ExpectedAncestry :: IsContextualRoot }
    };
}

is_contextual_root!();