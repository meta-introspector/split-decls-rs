macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! macro_291 {
    () => {
        deps!();
        ast_enum_of_structs ! { # [doc = " A generic type parameter, lifetime, or const generic: `T: Into<String>`,"] # [doc = " `'a: 'b`, `const LEN: usize`."] # [doc = ""] # [doc = " # Syntax tree enum"] # [doc = ""] # [doc = " This type is a [syntax tree enum]."] # [doc = ""] # [doc = " [syntax tree enum]: crate::expr::Expr#syntax-tree-enums"] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum GenericParam { # [doc = " A lifetime parameter: `'a: 'b + 'c + 'd`."] Lifetime (LifetimeParam) , # [doc = " A generic type parameter: `T: Into<String>`."] Type (TypeParam) , # [doc = " A const generic parameter: `const LENGTH: usize`."] Const (ConstParam) , } }
    };
}

macro_291!();