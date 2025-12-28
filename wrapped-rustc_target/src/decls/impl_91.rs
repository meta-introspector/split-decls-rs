macro_rules! deps {
    () => {
        InlineAsmArch!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl SparcInlineAsmReg { pub fn emit (self , out : & mut dyn fmt :: Write , _arch : InlineAsmArch , _modifier : Option < char > ,) -> fmt :: Result { write ! (out , "%{}" , self . name ()) } }
    };
}

impl_91!();