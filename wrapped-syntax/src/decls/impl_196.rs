macro_rules! deps {
    () => {
        SyntaxNode!();
        Parse!();
    };
}

macro_rules! impl_196 {
    () => {
        deps!();
        impl ast :: Expr { # [doc = " Parses an `ast::Expr` from `text`."] # [doc = ""] # [doc = " Note that if the parsed root node is not a valid expression, [`Parse::tree`] will panic."] # [doc = " For example:"] # [doc = " ```rust,should_panic"] # [doc = " # use syntax::{ast, Edition};"] # [doc = " ast::Expr::parse(\"let fail = true;\", Edition::CURRENT).tree();"] # [doc = " ```"] pub fn parse (text : & str , edition : Edition) -> Parse < ast :: Expr > { let _p = tracing :: info_span ! ("Expr::parse") . entered () ; let (green , errors) = parsing :: parse_text_at (text , parser :: TopEntryPoint :: Expr , edition) ; let root = SyntaxNode :: new_root (green . clone ()) ; assert ! (ast :: Expr :: can_cast (root . kind ()) || root . kind () == SyntaxKind :: ERROR , "{:?} isn't an expression" , root . kind ()) ; Parse :: new (green , errors) } }
    };
}

impl_196!()