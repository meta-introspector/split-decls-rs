macro_rules! macro_568 {
    () => {
        ast_struct ! { # [doc = " A single field in a struct pattern."] # [doc = ""] # [doc = " Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated"] # [doc = " the same as `x: x, y: ref y, z: ref mut z` but there is no colon token."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct FieldPat { pub attrs : Vec < Attribute >, pub member : Member , pub colon_token : Option < Token ! [:] >, pub pat : Box < Pat >, } }
    };
}

macro_568!();