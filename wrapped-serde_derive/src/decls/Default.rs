macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! Default {
    () => {
        deps!();
        # [doc = " Represents the default to use for a field when deserializing."] pub enum Default { # [doc = " Field must always be specified because it does not have a default."] None , # [doc = " The default is given by `std::default::Default::default()`."] Default , # [doc = " The default is given by this function."] Path (syn :: ExprPath) , }
    };
}

Default!()