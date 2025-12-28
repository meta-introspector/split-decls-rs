macro_rules! deps {
    () => {
        InlineAsmType!();
        ModifierInfo!();
        InlineAsmArch!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl SparcInlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match self { Self :: reg => { if arch == InlineAsmArch :: Sparc { types ! { _ : I8 , I16 , I32 ; } } else { types ! { _ : I8 , I16 , I32 , I64 ; } } } Self :: yreg => & [] , } } }
    };
}

impl_88!();