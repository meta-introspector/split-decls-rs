// Generated macro for impl_256 (impl)
macro_rules! Depcrateimpl_256 {
() => {
// Module: crate
// Provides: {"impl_256"}
// Dependencies: {}
impl SourceFile { pub fn parse (text : & str , edition : Edition) -> Parse < SourceFile > { let _p = tracing :: info_span ! ("SourceFile::parse") . entered () ; let (green , errors) = parsing :: parse_text (text , edition) ; let root = SyntaxNode :: new_root (green . clone ()) ; assert_eq ! (root . kind () , SyntaxKind :: SOURCE_FILE) ; Parse :: new (green , errors) } }
};
}
