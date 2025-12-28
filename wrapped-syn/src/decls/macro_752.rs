macro_rules! macro_752 {
    () => {
        ast_struct ! { # [doc = " A path like `std::slice::Iter`, optionally qualified with a"] # [doc = " self-type as in `<Vec<T> as SomeTrait>::Associated`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypePath { pub qself : Option < QSelf >, pub path : Path , } }
    };
}

macro_752!()