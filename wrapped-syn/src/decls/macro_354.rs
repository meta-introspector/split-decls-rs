macro_rules! macro_354 {
    () => {
        ast_struct ! { # [doc = " A struct definition: `struct Foo<A> { x: A }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemStruct { pub attrs : Vec < Attribute >, pub vis : Visibility , pub struct_token : Token ! [struct] , pub ident : Ident , pub generics : Generics , pub fields : Fields , pub semi_token : Option < Token ! [;] >, } }
    };
}

macro_354!()