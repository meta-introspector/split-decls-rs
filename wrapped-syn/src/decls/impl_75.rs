macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Meta { # [doc = " Returns the path that begins this structured meta item."] # [doc = ""] # [doc = " For example this would return the `test` in `#[test]`, the `derive` in"] # [doc = " `#[derive(Copy)]`, and the `path` in `#[path = \"sys/windows.rs\"]`."] pub fn path (& self) -> & Path { match self { Meta :: Path (path) => path , Meta :: List (meta) => & meta . path , Meta :: NameValue (meta) => & meta . path , } } # [doc = " Error if this is a `Meta::List` or `Meta::NameValue`."] # [cfg (feature = "parsing")] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] pub fn require_path_only (& self) -> Result < & Path > { let error_span = match self { Meta :: Path (path) => return Ok (path) , Meta :: List (meta) => meta . delimiter . span () . open () , Meta :: NameValue (meta) => meta . eq_token . span , } ; Err (Error :: new (error_span , "unexpected token in attribute")) } # [doc = " Error if this is a `Meta::Path` or `Meta::NameValue`."] # [cfg (feature = "parsing")] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] pub fn require_list (& self) -> Result < & MetaList > { match self { Meta :: List (meta) => Ok (meta) , Meta :: Path (path) => Err (crate :: error :: new2 (path . segments . first () . unwrap () . ident . span () , path . segments . last () . unwrap () . ident . span () , format ! ("expected attribute arguments in parentheses: `{}(...)`" , parsing :: DisplayPath (path) ,) ,)) , Meta :: NameValue (meta) => Err (Error :: new (meta . eq_token . span , "expected `(`")) , } } # [doc = " Error if this is a `Meta::Path` or `Meta::List`."] # [cfg (feature = "parsing")] # [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] pub fn require_name_value (& self) -> Result < & MetaNameValue > { match self { Meta :: NameValue (meta) => Ok (meta) , Meta :: Path (path) => Err (crate :: error :: new2 (path . segments . first () . unwrap () . ident . span () , path . segments . last () . unwrap () . ident . span () , format ! ("expected a value for this attribute: `{} = ...`" , parsing :: DisplayPath (path) ,) ,)) , Meta :: List (meta) => Err (Error :: new (meta . delimiter . span () . open () , "expected `=`")) , } } }
    };
}

impl_75!()