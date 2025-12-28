macro_rules! deps {
    () => {
        NoArg!();
        Opcode!();
    };
}

macro_rules! impl_452 {
    () => {
        deps!();
        impl < const OPCODE : Opcode > NoArg < OPCODE > { # [doc = " Create a new no-argument `ioctl` object."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = "  - `OPCODE` must provide a valid opcode."] # [inline] pub const unsafe fn new () -> Self { Self { } } }
    };
}

impl_452!()