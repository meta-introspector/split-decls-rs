macro_rules! deps {
    () => {
        ModifierInfo!();
        InlineAsmArch!();
        InlineAsmType!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl Msp430InlineAsmRegClass { pub fn valid_modifiers (self , _arch : super :: InlineAsmArch) -> & 'static [char] { & [] } pub fn suggest_class (self , _arch : InlineAsmArch , _ty : InlineAsmType) -> Option < Self > { None } pub fn suggest_modifier (self , _arch : InlineAsmArch , _ty : InlineAsmType ,) -> Option < ModifierInfo > { None } pub fn default_modifier (self , _arch : InlineAsmArch) -> Option < ModifierInfo > { None } pub fn supported_types (self , arch : InlineAsmArch ,) -> & 'static [(InlineAsmType , Option < Symbol >)] { match (self , arch) { (Self :: reg , _) => types ! { _ : I8 , I16 ; } , } } }
    };
}

impl_60!();