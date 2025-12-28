macro_rules! deps {
    () => {
        InlineAsmType!();
        InlineAsmArch!();
        ModifierInfo!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl M68kInlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , _arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match self { Self :: reg => types ! { _ : I16 , I32 ; } , Self :: reg_data => types ! { _ : I8 , I16 , I32 ; } , Self :: reg_addr => types ! { _ : I16 , I32 ; } , } } }
    };
}

impl_50!()