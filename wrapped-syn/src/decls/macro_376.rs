macro_rules! macro_376 {
    () => {
        ast_enum_of_structs ! { # [doc = " An item declaration within the definition of a trait."] # [doc = ""] # [doc = " # Syntax tree enum"] # [doc = ""] # [doc = " This type is a [syntax tree enum]."] # [doc = ""] # [doc = " [syntax tree enum]: crate::expr::Expr#syntax-tree-enums"] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] # [non_exhaustive] pub enum TraitItem { # [doc = " An associated constant within the definition of a trait."] Const (TraitItemConst) , # [doc = " An associated function within the definition of a trait."] Fn (TraitItemFn) , # [doc = " An associated type within the definition of a trait."] Type (TraitItemType) , # [doc = " A macro invocation within the definition of a trait."] Macro (TraitItemMacro) , # [doc = " Tokens within the definition of a trait not interpreted by Syn."] Verbatim (TokenStream) , } }
    };
}

macro_376!()