macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! NoArg {
    () => {
        deps!();
        # [doc = " Implements an `ioctl` with no real arguments."] # [doc = ""] # [doc = " To compute a value for the `OPCODE` argument, see the functions in the"] # [doc = " [`opcode`] module."] # [doc = ""] # [doc = " [`opcode`]: crate::ioctl::opcode"] pub struct NoArg < const OPCODE : Opcode > { }
    };
}

NoArg!();