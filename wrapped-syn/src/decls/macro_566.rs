macro_rules! macro_566 {
    () => {
        ast_struct ! { # [doc = " A type ascription pattern: `foo: f64`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct PatType { pub attrs : Vec < Attribute >, pub pat : Box < Pat >, pub colon_token : Token ! [:] , pub ty : Box < Type >, } }
    };
}

macro_566!();