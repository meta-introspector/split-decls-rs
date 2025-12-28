macro_rules! macro_720 {
    () => {
        ast_struct ! { # [doc = " A braced block containing Rust statements."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct Block { pub brace_token : token :: Brace , # [doc = " Statements in a block"] pub stmts : Vec < Stmt >, } }
    };
}

macro_720!()