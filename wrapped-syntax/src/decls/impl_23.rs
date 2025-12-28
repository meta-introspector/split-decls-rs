macro_rules! deps {
    () => {
        RustLanguage!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Language for RustLanguage { type Kind = SyntaxKind ; fn kind_from_raw (raw : rowan :: SyntaxKind) -> SyntaxKind { SyntaxKind :: from (raw . 0) } fn kind_to_raw (kind : SyntaxKind) -> rowan :: SyntaxKind { rowan :: SyntaxKind (kind . into ()) } }
    };
}

impl_23!()