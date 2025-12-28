macro_rules! deps {
    () => {
        Updater!();
        Opcode!();
    };
}

macro_rules! impl_463 {
    () => {
        deps!();
        impl < 'a , const OPCODE : Opcode , Value > Updater < 'a , OPCODE , Value > { # [doc = " Create a new pointer updater-style `ioctl` object."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = "  - `OPCODE` must provide a valid opcode."] # [doc = "  - For this opcode, `Value` must be the type that the kernel expects to"] # [doc = "    get."] # [inline] pub unsafe fn new (value : & 'a mut Value) -> Self { Self { value } } }
    };
}

impl_463!();