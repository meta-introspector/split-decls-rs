macro_rules! macro_722 {
    () => {
        ast_struct ! { # [doc = " A local `let` binding: `let x: u64 = s.parse()?;`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct Local { pub attrs : Vec < Attribute >, pub let_token : Token ! [let] , pub pat : Pat , pub init : Option < LocalInit >, pub semi_token : Token ! [;] , } }
    };
}

macro_722!()