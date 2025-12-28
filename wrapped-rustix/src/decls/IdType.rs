macro_rules! deps {
    () => {
        Pid!();
    };
}

macro_rules! IdType {
    () => {
        deps!();
        # [doc = " Subset of `idtype_t` C enum, with only the values allowed by `procctl`."] # [repr (i32)] pub enum IdType { # [doc = " Process id."] Pid = 0 , # [doc = " Process group id."] Pgid = 2 , }
    };
}

IdType!()