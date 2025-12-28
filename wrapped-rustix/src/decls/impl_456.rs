macro_rules! deps {
    () => {
        Getter!();
        Opcode!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        impl < const OPCODE : Opcode , Output > Getter < OPCODE , Output > { # [doc = " Create a new getter-style `ioctl` object."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = "  - `OPCODE` must provide a valid opcode."] # [doc = "  - For this opcode, `Output` must be the type that the kernel expects"] # [doc = "    to write into."] # [inline] pub const unsafe fn new () -> Self { Self { output : mem :: MaybeUninit :: uninit () , } } }
    };
}

impl_456!();