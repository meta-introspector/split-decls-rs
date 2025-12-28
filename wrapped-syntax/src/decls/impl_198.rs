macro_rules! deps {
    () => {
        SyntaxNode!();
        Parse!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl SourceFile { pub fn parse (text : & str , edition : Edition) -> Parse < SourceFile > { let _p = tracing :: info_span ! ("SourceFile::parse") . entered () ; let (green , errors) = parsing :: parse_text (text , edition) ; let root = SyntaxNode :: new_root (green . clone ()) ; assert_eq ! (root . kind () , SyntaxKind :: SOURCE_FILE) ; Parse :: new (green , errors) } }
    };
}

impl_198!();