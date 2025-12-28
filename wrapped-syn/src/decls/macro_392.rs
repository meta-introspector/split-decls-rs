macro_rules! macro_392 {
    () => {
        ast_enum ! { # [doc = " The mutability of an `Item::Static` or `ForeignItem::Static`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] # [non_exhaustive] pub enum StaticMutability { Mut (Token ! [mut]) , None , } }
    };
}

macro_392!()