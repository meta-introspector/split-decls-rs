macro_rules! macro_371 {
    () => {
        ast_enum_of_structs ! { # [doc = " An item within an `extern` block."] # [doc = ""] # [doc = " # Syntax tree enum"] # [doc = ""] # [doc = " This type is a [syntax tree enum]."] # [doc = ""] # [doc = " [syntax tree enum]: crate::expr::Expr#syntax-tree-enums"] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] # [non_exhaustive] pub enum ForeignItem { # [doc = " A foreign function in an `extern` block."] Fn (ForeignItemFn) , # [doc = " A foreign static item in an `extern` block: `static ext: u8`."] Static (ForeignItemStatic) , # [doc = " A foreign type in an `extern` block: `type void`."] Type (ForeignItemType) , # [doc = " A macro invocation within an extern block."] Macro (ForeignItemMacro) , # [doc = " Tokens in an `extern` block not interpreted by Syn."] Verbatim (TokenStream) , } }
    };
}

macro_371!()