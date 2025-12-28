macro_rules! AsLog {
    () => {
        # [doc = " Trait implemented for `tracing` types that can be converted to a `log`"] # [doc = " equivalent."] pub trait AsLog : crate :: sealed :: Sealed { # [doc = " The `log` type that this type can be converted into."] type Log ; # [doc = " Returns the `log` equivalent of `self`."] fn as_log (& self) -> Self :: Log ; }
    };
}

AsLog!();