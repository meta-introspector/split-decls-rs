macro_rules! macro_155 {
    () => {
        ast_struct ! { # [doc = " An untagged union input to a `proc_macro_derive` macro."] # [cfg_attr (docsrs , doc (cfg (feature = "derive")))] pub struct DataUnion { pub union_token : Token ! [union] , pub fields : FieldsNamed , } }
    };
}

macro_155!();