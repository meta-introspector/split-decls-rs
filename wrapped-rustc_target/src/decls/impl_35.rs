macro_rules! deps {
    () => {
        InlineAsmType!();
        ModifierInfo!();
        InlineAsmArch!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl CSKYInlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , _arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match self { Self :: reg => types ! { _ : I8 , I16 , I32 ; } , Self :: freg => types ! { _ : F32 ; } , } } }
    };
}

impl_35!();