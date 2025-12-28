macro_rules! deps {
    () => {
        InlineAsmRegOrRegClass!();
        InlineAsmRegClass!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl InlineAsmRegOrRegClass { pub fn reg_class (self) -> InlineAsmRegClass { match self { Self :: Reg (r) => r . reg_class () , Self :: RegClass (r) => r , } } }
    };
}

impl_117!()