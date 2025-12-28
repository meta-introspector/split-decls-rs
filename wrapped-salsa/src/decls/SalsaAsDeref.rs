macro_rules! SalsaAsDeref {
    () => {
        # [doc = " Used to determine the return type and value for tracked fields and functions annotated with `returns(as_deref)`."] pub trait SalsaAsDeref { type AsDeref < 'a > where Self : 'a ; fn as_deref (& self) -> Self :: AsDeref < '_ > ; }
    };
}

SalsaAsDeref!();