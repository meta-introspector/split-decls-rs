macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! Setter {
    () => {
        deps!();
        # [doc = " Implements the pattern for `ioctl`s where a pointer argument is given to"] # [doc = " the `ioctl`."] # [doc = ""] # [doc = " The opcode must be read-only."] # [doc = ""] # [doc = " To compute a value for the `OPCODE` argument, see the functions in the"] # [doc = " [`opcode`] module."] # [doc = ""] # [doc = " [`opcode`]: crate::ioctl::opcode"] pub struct Setter < const OPCODE : Opcode , Input > { # [doc = " The input data."] input : Input , }
    };
}

Setter!()