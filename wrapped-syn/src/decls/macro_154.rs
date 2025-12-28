macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_154 {
    () => {
        deps!();
        ast_struct ! { # [doc = " An enum input to a `proc_macro_derive` macro."] # [cfg_attr (docsrs , doc (cfg (feature = "derive")))] pub struct DataEnum { pub enum_token : Token ! [enum] , pub brace_token : token :: Brace , pub variants : Punctuated < Variant , Token ! [,] >, } }
    };
}

macro_154!()