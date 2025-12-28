macro_rules! deps {
    () => {
        Opcode!();
        Setter!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl < const OPCODE : Opcode , Input > Setter < OPCODE , Input > { # [doc = " Create a new pointer setter-style `ioctl` object."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = "  - `OPCODE` must provide a valid opcode."] # [doc = "  - For this opcode, `Input` must be the type that the kernel expects to"] # [doc = "    get."] # [inline] pub const unsafe fn new (input : Input) -> Self { Self { input } } }
    };
}

impl_460!()