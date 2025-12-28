macro_rules! macro_388 {
    () => {
        ast_enum_of_structs ! { # [doc = " An argument in a function signature: the `n: usize` in `fn f(n: usize)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub enum FnArg { # [doc = " The `self` argument of an associated method."] Receiver (Receiver) , # [doc = " A function argument accepted by pattern and type."] Typed (PatType) , } }
    };
}

macro_388!()