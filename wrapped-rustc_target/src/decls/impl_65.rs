macro_rules! deps {
    () => {
        ModifierInfo!();
        InlineAsmArch!();
        InlineAsmType!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl NvptxInlineAsmRegClass { pub fn valid_modifiers (self , _arch : InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , _arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match self { Self :: reg16 => types ! { _ : I8 , I16 ; } , Self :: reg32 => types ! { _ : I8 , I16 , I32 , F32 ; } , Self :: reg64 => types ! { _ : I8 , I16 , I32 , F32 , I64 , F64 ; } , } } }
    };
}

impl_65!();