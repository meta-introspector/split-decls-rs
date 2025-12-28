macro_rules! deps {
    () => {
        InlineAsmArch!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Msp430InlineAsmReg { pub fn emit (self , out : & mut dyn fmt :: Write , _arch : InlineAsmArch , _modifier : Option < char > ,) -> fmt :: Result { out . write_str (self . name ()) } }
    };
}

impl_62!();