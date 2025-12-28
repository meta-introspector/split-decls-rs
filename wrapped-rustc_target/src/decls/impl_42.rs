macro_rules! deps {
    () => {
        InlineAsmArch!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl HexagonInlineAsmReg { pub fn emit (self , out : & mut dyn fmt :: Write , _arch : InlineAsmArch , _modifier : Option < char > ,) -> fmt :: Result { out . write_str (self . name ()) } pub fn overlapping_regs (self , mut _cb : impl FnMut (HexagonInlineAsmReg)) { } }
    };
}

impl_42!();