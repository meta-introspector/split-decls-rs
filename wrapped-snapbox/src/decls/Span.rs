macro_rules! Span {
    () => {
        # [derive (Clone , Debug)] struct Span { # [doc = " The byte range of the argument to `expect!`, including the inner `[]` if it exists."] literal_range : std :: ops :: Range < usize > , }
    };
}

Span!();