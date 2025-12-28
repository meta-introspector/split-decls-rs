macro_rules! macro_72 {
    () => {
        ast_enum ! { # [doc = " Content of a compile-time structured attribute."] # [doc = ""] # [doc = " ## Path"] # [doc = ""] # [doc = " A meta path is like the `test` in `#[test]`."] # [doc = ""] # [doc = " ## List"] # [doc = ""] # [doc = " A meta list is like the `derive(Copy)` in `#[derive(Copy)]`."] # [doc = ""] # [doc = " ## NameValue"] # [doc = ""] # [doc = " A name-value meta is like the `path = \"...\"` in `#[path ="] # [doc = " \"sys/windows.rs\"]`."] # [doc = ""] # [doc = " # Syntax tree enum"] # [doc = ""] # [doc = " This type is a [syntax tree enum]."] # [doc = ""] # [doc = " [syntax tree enum]: crate::expr::Expr#syntax-tree-enums"] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum Meta { Path (Path) , # [doc = " A structured list within an attribute, like `derive(Copy, Clone)`."] List (MetaList) , # [doc = " A name-value pair within an attribute, like `feature = \"nightly\"`."] NameValue (MetaNameValue) , } }
    };
}

macro_72!();