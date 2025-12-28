macro_rules! macro_358 {
    () => {
        ast_struct ! { # [doc = " A union definition: `union Foo<A, B> { x: A, y: B }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemUnion { pub attrs : Vec < Attribute >, pub vis : Visibility , pub union_token : Token ! [union] , pub ident : Ident , pub generics : Generics , pub fields : FieldsNamed , } }
    };
}

macro_358!()