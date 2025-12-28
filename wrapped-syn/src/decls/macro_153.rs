macro_rules! macro_153 {
    () => {
        ast_struct ! { # [doc = " A struct input to a `proc_macro_derive` macro."] # [cfg_attr (docsrs , doc (cfg (feature = "derive")))] pub struct DataStruct { pub struct_token : Token ! [struct] , pub fields : Fields , pub semi_token : Option < Token ! [;] >, } }
    };
}

macro_153!()