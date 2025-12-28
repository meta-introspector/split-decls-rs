macro_rules! macro_323 {
    () => {
        ast_enum ! { # [doc = " A modifier on a trait bound, currently only used for the `?` in"] # [doc = " `?Sized`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum TraitBoundModifier { None , Maybe (Token ! [?]) , } }
    };
}

macro_323!()