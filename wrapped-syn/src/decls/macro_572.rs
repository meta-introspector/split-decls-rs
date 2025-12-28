macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_572 {
    () => {
        deps!();
        ast_struct ! { # [doc = " A path at which a named item is exported (e.g. `std::collections::HashMap`)."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct Path { pub leading_colon : Option < Token ! [::] >, pub segments : Punctuated < PathSegment , Token ! [::] >, } }
    };
}

macro_572!();