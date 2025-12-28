macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! macro_365 {
    () => {
        deps!();
        ast_enum_of_structs ! { # [doc = " A suffix of an import tree in a `use` item: `Type as Renamed` or `*`."] # [doc = ""] # [doc = " # Syntax tree enum"] # [doc = ""] # [doc = " This type is a [syntax tree enum]."] # [doc = ""] # [doc = " [syntax tree enum]: crate::expr::Expr#syntax-tree-enums"] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub enum UseTree { # [doc = " A path prefix of imports in a `use` item: `std::...`."] Path (UsePath) , # [doc = " An identifier imported by a `use` item: `HashMap`."] Name (UseName) , # [doc = " An renamed identifier imported by a `use` item: `HashMap as Map`."] Rename (UseRename) , # [doc = " A glob import in a `use` item: `*`."] Glob (UseGlob) , # [doc = " A braced group of imports in a `use` item: `{A, B, C}`."] Group (UseGroup) , } }
    };
}

macro_365!()