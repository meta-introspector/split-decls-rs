macro_rules! macro_256 {
    () => {
        # [cfg (feature = "full")] ast_enum ! { # [doc = " Limit types of a range, inclusive or exclusive."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub enum RangeLimits { # [doc = " Inclusive at the beginning, exclusive at the end."] HalfOpen (Token ! [..]) , # [doc = " Inclusive at the beginning and end."] Closed (Token ! [..=]) , } }
    };
}

macro_256!();