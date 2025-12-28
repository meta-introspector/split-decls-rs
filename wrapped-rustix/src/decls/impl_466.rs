macro_rules! deps {
    () => {
        Opcode!();
        Ioctl!();
        IntegerSetter!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        impl < const OPCODE : Opcode > IntegerSetter < OPCODE > { # [doc = " Create a new integer `Ioctl` helper containing a `usize`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = "  - `OPCODE` must provide a valid opcode."] # [doc = "  - For this opcode, it must expect an integer."] # [doc = "  - The integer is in the valid range for this opcode."] # [inline] pub const unsafe fn new_usize (value : usize) -> Self { Self { value : value as _ } } # [doc = " Create a new integer `Ioctl` helper containing a `*mut c_void`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = "  - `OPCODE` must provide a valid opcode."] # [doc = "  - For this opcode, it must expect an integer."] # [doc = "  - The integer is in the valid range for this opcode."] # [inline] pub const unsafe fn new_pointer (value : * mut c :: c_void) -> Self { Self { value } } }
    };
}

impl_466!();