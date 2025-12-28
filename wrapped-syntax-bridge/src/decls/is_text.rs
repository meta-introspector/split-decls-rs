macro_rules! is_text {
    () => {
        fn is_text (k : SyntaxKind) -> bool { k . is_any_identifier () || k . is_literal () || k == UNDERSCORE }
    };
}

is_text!()