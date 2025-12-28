macro_rules! deps {
    () => {
        ABI!();
    };
}

macro_rules! X86Abi {
    () => {
        deps!();
        # [doc = " x86 (32-bit) abi options."] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub struct X86Abi { # [doc = " On x86-32 targets, the regparm N causes the compiler to pass arguments"] # [doc = " in registers EAX, EDX, and ECX instead of on the stack."] pub regparm : Option < u32 > , # [doc = " Override the default ABI to return small structs in registers"] pub reg_struct_return : bool , }
    };
}

X86Abi!()