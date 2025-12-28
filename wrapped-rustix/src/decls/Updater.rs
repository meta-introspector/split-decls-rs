macro_rules! deps {
    () => {
        Opcode!();
    };
}

macro_rules! Updater {
    () => {
        deps!();
        # [doc = " Implements an “updater” pattern for `ioctl`s."] # [doc = ""] # [doc = " The ioctl takes a reference to a struct that it reads its input from,"] # [doc = " then writes output to the same struct."] # [doc = ""] # [doc = " To compute a value for the `OPCODE` argument, see the functions in the"] # [doc = " [`opcode`] module."] # [doc = ""] # [doc = " [`opcode`]: crate::ioctl::opcode"] pub struct Updater < 'a , const OPCODE : Opcode , Value > { # [doc = " Reference to input/output data."] value : & 'a mut Value , }
    };
}

Updater!();